use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use serde_json::{Map, Value};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tf_app::{
    AppError, AppResult, RunMode, RunOptions, RunProgressEvent, RunRequest, RunStage,
    project_service, query, run_service,
};
use tf_cea::{NativeCeaBackend, Reactant};
use tf_eng::{
    DeviceStudyRequest, EquationStudyRequest, SweepAxisSpec, WorkflowStudyRequest,
    run_device_study, run_equation_study, run_workflow_study,
};
use tf_rpa::{
    CombustorModel, NozzleChemistryModel, NozzleConstraint, RocketAnalysisProblem,
    RocketAnalysisSolver,
};

#[derive(Parser)]
#[command(name = "tf-cli")]
#[command(about = "ThermoFlow CLI - Thermal-fluid network simulation tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate project file syntax and structure
    Validate {
        /// Path to the project YAML file
        project_path: PathBuf,
    },
    /// List systems in a project
    Systems {
        /// Path to the project YAML file
        project_path: PathBuf,
    },
    /// Run a simulation
    #[command(subcommand)]
    Run(RunCommands),
    /// List cached runs for a project
    Runs {
        /// Path to the project YAML file
        project_path: PathBuf,
        /// System ID to list runs for
        system_id: String,
    },
    /// Show details of a cached run
    ShowRun {
        /// Path to the project YAML file
        project_path: PathBuf,
        /// Run ID to display
        run_id: String,
    },
    /// Export time series data from a run
    ExportSeries {
        /// Path to the project YAML file
        project_path: PathBuf,
        /// Run ID
        run_id: String,
        /// Entity ID (node or component)
        entity_id: String,
        /// Variable name (e.g., pressure, temperature, mass_flow)
        variable: String,
        /// Output CSV file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Rocket performance workflows powered by tf-rpa + CEA backend
    #[command(subcommand)]
    Rocket(RocketCommands),
    /// Generic eng study bridge (equations/devices/workflows)
    #[command(subcommand)]
    Eng(EngCommands),
}

#[derive(Subcommand)]
enum EngCommands {
    /// Run a generic equation study and export plot-ready/table output.
    Equation {
        #[arg(long)]
        path_id: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        sweep_variable: String,
        #[arg(long)]
        from: f64,
        #[arg(long)]
        to: f64,
        #[arg(long, default_value_t = 50)]
        n: usize,
        #[arg(long, default_value = "{}")]
        fixed_inputs_json: String,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long, value_enum, default_value_t = CliStudyFormat::Json)]
        format: CliStudyFormat,
    },
    /// Run a generic device study and export plot-ready/table output.
    Device {
        #[arg(long)]
        device: String,
        #[arg(long)]
        sweep_arg: String,
        #[arg(long)]
        from: f64,
        #[arg(long)]
        to: f64,
        #[arg(long, default_value_t = 50)]
        n: usize,
        #[arg(long, default_value = "{}")]
        fixed_args_json: String,
        #[arg(long, default_value = "value,path_text")]
        outputs_csv: String,
        #[arg(long, default_value = "value")]
        output_key: String,
        #[arg(long, value_enum, default_value_t = CliStudyFormat::Json)]
        format: CliStudyFormat,
    },
    /// Run a generic workflow study and export plot-ready/table output.
    Workflow {
        #[arg(long)]
        workflow: String,
        #[arg(long)]
        sweep_arg: String,
        #[arg(long)]
        from: f64,
        #[arg(long)]
        to: f64,
        #[arg(long, default_value_t = 50)]
        n: usize,
        #[arg(long, default_value = "{}")]
        fixed_args_json: String,
        #[arg(long, default_value = "pre_shock_mach")]
        output_key: String,
        #[arg(long, value_enum, default_value_t = CliStudyFormat::Json)]
        format: CliStudyFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliStudyFormat {
    Json,
    Csv,
}

#[derive(Subcommand)]
enum RocketCommands {
    /// Solve a single rocket performance case
    Solve {
        /// Optional explicit CEA backend executable path
        #[arg(long)]
        cea_backend: Option<PathBuf>,
        /// CEA backend mode (default: native)
        #[arg(long, value_enum)]
        cea_backend_mode: Option<CliCeaBackendMode>,
        /// Case label for output readability
        #[arg(long, default_value = "O2/CH4 baseline")]
        case_name: String,
        /// Oxidizer species name understood by backend
        #[arg(long, default_value = "O2")]
        oxidizer: String,
        /// Fuel species name understood by backend
        #[arg(long, default_value = "CH4")]
        fuel: String,
        /// Oxidizer-to-fuel mixture ratio
        #[arg(long, default_value_t = 3.5)]
        mixture_ratio: f64,
        /// Sweep O/F and select an optimum automatically
        #[arg(long)]
        optimize_mixture_ratio: bool,
        /// Minimum O/F for optimization sweep
        #[arg(long, default_value_t = 0.5)]
        of_min: f64,
        /// Maximum O/F for optimization sweep
        #[arg(long, default_value_t = 12.0)]
        of_max: f64,
        /// Number of O/F sweep points
        #[arg(long, default_value_t = 121)]
        of_points: usize,
        /// Print O/F sweep diagnostics table
        #[arg(long)]
        of_diagnostics: bool,
        /// Chamber pressure in Pa
        #[arg(long, default_value_t = 3_000_000.0)]
        chamber_pressure_pa: f64,
        /// Ambient pressure in Pa
        #[arg(long, default_value_t = 101_325.0)]
        ambient_pressure_pa: f64,
        /// Oxidizer temperature in K
        #[arg(long, default_value_t = 298.0)]
        oxidizer_temperature_k: f64,
        /// Fuel temperature in K
        #[arg(long, default_value_t = 298.0)]
        fuel_temperature_k: f64,
        /// Combustor model selection
        #[arg(long, value_enum, default_value_t = CliCombustorModel::InfiniteArea)]
        combustor_model: CliCombustorModel,
        /// Contraction ratio for finite-area combustor model
        #[arg(long, default_value_t = 2.5)]
        contraction_ratio: f64,
        /// Nozzle chemistry model
        #[arg(long, value_enum, default_value_t = CliNozzleChemistryModel::ShiftingEquilibrium)]
        nozzle_chemistry_model: CliNozzleChemistryModel,
        /// Nozzle expansion ratio epsilon (mutually exclusive with --exit-pressure-pa)
        #[arg(long)]
        expansion_ratio: Option<f64>,
        /// Nozzle exit pressure in Pa (mutually exclusive with --expansion-ratio)
        #[arg(long)]
        exit_pressure_pa: Option<f64>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliCombustorModel {
    InfiniteArea,
    FiniteArea,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliNozzleChemistryModel {
    ShiftingEquilibrium,
    FrozenAtChamber,
    FrozenAtThroat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliCeaBackendMode {
    Process,
    InProcess,
}

#[derive(Subcommand)]
enum RunCommands {
    /// Run steady-state simulation
    Steady {
        /// Path to the project YAML file
        project_path: PathBuf,
        /// System ID to simulate
        system_id: String,
        /// Skip cache and force re-run
        #[arg(long)]
        no_cache: bool,
    },
    /// Run transient simulation
    Transient {
        /// Path to the project YAML file
        project_path: PathBuf,
        /// System ID to simulate
        system_id: String,
        /// Time step in seconds
        #[arg(long)]
        dt: f64,
        /// End time in seconds
        #[arg(long)]
        t_end: f64,
        /// Skip cache and force re-run
        #[arg(long)]
        no_cache: bool,
    },
}

fn main() -> AppResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { project_path } => cmd_validate(&project_path),
        Commands::Systems { project_path } => cmd_systems(&project_path),
        Commands::Run(run_cmd) => match run_cmd {
            RunCommands::Steady {
                project_path,
                system_id,
                no_cache,
            } => cmd_run_steady(&project_path, &system_id, !no_cache),
            RunCommands::Transient {
                project_path,
                system_id,
                dt,
                t_end,
                no_cache,
            } => cmd_run_transient(&project_path, &system_id, dt, t_end, !no_cache),
        },
        Commands::Runs {
            project_path,
            system_id,
        } => cmd_runs(&project_path, &system_id),
        Commands::ShowRun {
            project_path,
            run_id,
        } => cmd_show_run(&project_path, &run_id),
        Commands::ExportSeries {
            project_path,
            run_id,
            entity_id,
            variable,
            output,
        } => cmd_export_series(
            &project_path,
            &run_id,
            &entity_id,
            &variable,
            output.as_deref(),
        ),
        Commands::Rocket(rocket_cmd) => match rocket_cmd {
            RocketCommands::Solve {
                cea_backend,
                cea_backend_mode,
                case_name,
                oxidizer,
                fuel,
                mixture_ratio,
                optimize_mixture_ratio,
                of_min,
                of_max,
                of_points,
                of_diagnostics,
                chamber_pressure_pa,
                ambient_pressure_pa,
                oxidizer_temperature_k,
                fuel_temperature_k,
                combustor_model,
                contraction_ratio,
                nozzle_chemistry_model,
                expansion_ratio,
                exit_pressure_pa,
            } => cmd_rocket_solve(
                cea_backend,
                cea_backend_mode,
                &case_name,
                &oxidizer,
                &fuel,
                mixture_ratio,
                optimize_mixture_ratio,
                of_min,
                of_max,
                of_points,
                of_diagnostics,
                chamber_pressure_pa,
                ambient_pressure_pa,
                oxidizer_temperature_k,
                fuel_temperature_k,
                combustor_model,
                contraction_ratio,
                nozzle_chemistry_model,
                expansion_ratio,
                exit_pressure_pa,
            ),
        },
        Commands::Eng(eng_cmd) => match eng_cmd {
            EngCommands::Equation {
                path_id,
                target,
                sweep_variable,
                from,
                to,
                n,
                fixed_inputs_json,
                branch,
                format,
            } => cmd_eng_equation_study(
                &path_id,
                &target,
                &sweep_variable,
                from,
                to,
                n,
                &fixed_inputs_json,
                branch,
                format,
            ),
            EngCommands::Device {
                device,
                sweep_arg,
                from,
                to,
                n,
                fixed_args_json,
                outputs_csv,
                output_key,
                format,
            } => cmd_eng_device_study(
                &device,
                &sweep_arg,
                from,
                to,
                n,
                &fixed_args_json,
                &outputs_csv,
                &output_key,
                format,
            ),
            EngCommands::Workflow {
                workflow,
                sweep_arg,
                from,
                to,
                n,
                fixed_args_json,
                output_key,
                format,
            } => cmd_eng_workflow_study(
                &workflow,
                &sweep_arg,
                from,
                to,
                n,
                &fixed_args_json,
                &output_key,
                format,
            ),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn cmd_eng_equation_study(
    path_id: &str,
    target: &str,
    sweep_variable: &str,
    from: f64,
    to: f64,
    n: usize,
    fixed_inputs_json: &str,
    branch: Option<String>,
    format: CliStudyFormat,
) -> AppResult<()> {
    let fixed_inputs = parse_numeric_map(fixed_inputs_json)?;
    let result = run_equation_study(EquationStudyRequest {
        path_id: path_id.to_string(),
        target: target.to_string(),
        sweep_variable: sweep_variable.to_string(),
        axis: SweepAxisSpec::linspace(from, to, n),
        fixed_inputs,
        branch,
        output_key: Some(target.to_string()),
    })
    .map_err(|e| AppError::InvalidInput(e.to_string()))?;
    write_study_result(&result, format)
}

#[allow(clippy::too_many_arguments)]
fn cmd_eng_device_study(
    device: &str,
    sweep_arg: &str,
    from: f64,
    to: f64,
    n: usize,
    fixed_args_json: &str,
    outputs_csv: &str,
    output_key: &str,
    format: CliStudyFormat,
) -> AppResult<()> {
    let fixed_args = parse_json_object(fixed_args_json)?;
    let requested_outputs = outputs_csv
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let result = run_device_study(DeviceStudyRequest {
        device_key: device.to_string(),
        sweep_arg: sweep_arg.to_string(),
        axis: SweepAxisSpec::linspace(from, to, n),
        fixed_args,
        requested_outputs,
        output_key: Some(output_key.to_string()),
    })
    .map_err(|e| AppError::InvalidInput(e.to_string()))?;
    write_study_result(&result, format)
}

#[allow(clippy::too_many_arguments)]
fn cmd_eng_workflow_study(
    workflow: &str,
    sweep_arg: &str,
    from: f64,
    to: f64,
    n: usize,
    fixed_args_json: &str,
    output_key: &str,
    format: CliStudyFormat,
) -> AppResult<()> {
    let fixed_args = parse_json_object(fixed_args_json)?;
    let result = run_workflow_study(WorkflowStudyRequest {
        workflow_key: workflow.to_string(),
        sweep_arg: sweep_arg.to_string(),
        axis: SweepAxisSpec::linspace(from, to, n),
        fixed_args,
        output_key: Some(output_key.to_string()),
    })
    .map_err(|e| AppError::InvalidInput(e.to_string()))?;
    write_study_result(&result, format)
}

fn write_study_result(result: &tf_eng::StudyResult, format: CliStudyFormat) -> AppResult<()> {
    match format {
        CliStudyFormat::Json => print_json(result),
        CliStudyFormat::Csv => print_csv_table(&result.table),
    }
}

fn print_json<T: Serialize>(value: &T) -> AppResult<()> {
    let text = serde_json::to_string_pretty(value)
        .map_err(|e| AppError::InvalidInput(format!("JSON serialization failed: {e}")))?;
    println!("{text}");
    Ok(())
}

fn print_csv_table(table: &tf_eng::StudyTable) -> AppResult<()> {
    fn escape_csv_cell(cell: &str) -> String {
        if cell.contains(',') || cell.contains('"') || cell.contains('\n') || cell.contains('\r') {
            format!("\"{}\"", cell.replace('"', "\"\""))
        } else {
            cell.to_string()
        }
    }

    println!(
        "{}",
        table
            .columns
            .iter()
            .map(|c| escape_csv_cell(c))
            .collect::<Vec<_>>()
            .join(",")
    );
    for row in &table.rows {
        println!(
            "{}",
            row.iter()
                .map(|c| escape_csv_cell(c))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
    Ok(())
}

fn parse_json_object(text: &str) -> AppResult<Map<String, Value>> {
    match serde_json::from_str::<Value>(text).map_err(|e| AppError::InvalidInput(e.to_string()))? {
        Value::Object(obj) => Ok(obj),
        _ => Err(AppError::InvalidInput(
            "JSON payload must be an object".to_string(),
        )),
    }
}

fn parse_numeric_map(text: &str) -> AppResult<std::collections::BTreeMap<String, f64>> {
    let obj = parse_json_object(text)?;
    let mut out = std::collections::BTreeMap::new();
    for (k, v) in obj {
        let Some(n) = v.as_f64() else {
            return Err(AppError::InvalidInput(format!(
                "fixed input '{k}' must be numeric"
            )));
        };
        out.insert(k, n);
    }
    Ok(out)
}

fn cmd_validate(project_path: &Path) -> AppResult<()> {
    println!("Validating project: {}", project_path.display());
    let project = project_service::load_project(project_path)?;
    project_service::validate_project(&project)?;
    println!("✓ Project is valid");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_rocket_solve(
    cea_backend: Option<PathBuf>,
    cea_backend_mode: Option<CliCeaBackendMode>,
    case_name: &str,
    oxidizer: &str,
    fuel: &str,
    mixture_ratio: f64,
    optimize_mixture_ratio: bool,
    of_min: f64,
    of_max: f64,
    of_points: usize,
    of_diagnostics: bool,
    chamber_pressure_pa: f64,
    ambient_pressure_pa: f64,
    oxidizer_temperature_k: f64,
    fuel_temperature_k: f64,
    combustor_model: CliCombustorModel,
    contraction_ratio: f64,
    nozzle_chemistry_model: CliNozzleChemistryModel,
    expansion_ratio: Option<f64>,
    exit_pressure_pa: Option<f64>,
) -> AppResult<()> {
    if expansion_ratio.is_some() && exit_pressure_pa.is_some() {
        return Err(AppError::InvalidInput(
            "Provide only one of --expansion-ratio or --exit-pressure-pa".to_owned(),
        ));
    }

    let nozzle_constraint = if let Some(exit_pressure) = exit_pressure_pa {
        NozzleConstraint::ExitPressurePa(exit_pressure)
    } else {
        NozzleConstraint::ExpansionRatio(expansion_ratio.unwrap_or(25.0))
    };

    let combustor_model = match combustor_model {
        CliCombustorModel::InfiniteArea => CombustorModel::InfiniteArea,
        CliCombustorModel::FiniteArea => CombustorModel::FiniteArea { contraction_ratio },
    };

    let nozzle_chemistry_model = match nozzle_chemistry_model {
        CliNozzleChemistryModel::ShiftingEquilibrium => NozzleChemistryModel::ShiftingEquilibrium,
        CliNozzleChemistryModel::FrozenAtChamber => NozzleChemistryModel::FrozenAtChamber,
        CliNozzleChemistryModel::FrozenAtThroat => NozzleChemistryModel::FrozenAtThroat,
    };

    if cea_backend.is_some() || cea_backend_mode.is_some() {
        eprintln!(
            "Note: --cea-backend and --cea-backend-mode are currently ignored; native backend is used."
        );
    }
    let solver = RocketAnalysisSolver::new(NativeCeaBackend::new());

    let (selected_mr, of_samples, of_note) = if optimize_mixture_ratio {
        optimize_mixture_ratio_with_diagnostics(
            &solver,
            oxidizer,
            fuel,
            chamber_pressure_pa,
            ambient_pressure_pa,
            oxidizer_temperature_k,
            fuel_temperature_k,
            &combustor_model,
            &nozzle_chemistry_model,
            &nozzle_constraint,
            mixture_ratio,
            of_min,
            of_max,
            of_points,
        )?
    } else {
        (mixture_ratio, Vec::new(), None)
    };

    let problem = RocketAnalysisProblem {
        oxidizer: Reactant {
            name: oxidizer.to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(oxidizer_temperature_k),
        },
        fuel: Reactant {
            name: fuel.to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(fuel_temperature_k),
        },
        chamber_pressure_pa,
        mixture_ratio_oxidizer_to_fuel: selected_mr,
        nozzle_constraint,
        combustor_model,
        nozzle_chemistry_model,
        ambient_pressure_pa,
    };

    let result = solver.solve(&problem).map_err(|e| AppError::Backend {
        message: format!("Rocket performance solve failed: {e}"),
    })?;

    println!("Rocket case: {}", case_name);
    println!("  O/F used: {:.5}", selected_mr);
    if let Some(note) = of_note {
        println!("  O/F optimizer note: {}", note);
    }
    println!("  c* [m/s]: {:.3}", result.characteristic_velocity_m_per_s);
    println!("  Cf,vac:   {:.5}", result.thrust_coefficient_vac);
    println!("  Cf,amb:   {:.5}", result.thrust_coefficient_amb);
    println!("  Isp,vac [s]: {:.3}", result.specific_impulse_vac_s);
    println!("  Isp,amb [s]: {:.3}", result.specific_impulse_amb_s);
    println!(
        "  Chamber: T={:.2} K, gamma={:.5}, MW={:.4} kg/kmol",
        result.chamber.temperature_k.unwrap_or(f64::NAN),
        result.chamber.gamma.unwrap_or(f64::NAN),
        result
            .chamber
            .molecular_weight_kg_per_kmol
            .unwrap_or(f64::NAN)
    );
    println!("  Notes:");
    for note in &result.notes {
        println!("    - {}", note);
    }

    if optimize_mixture_ratio && of_diagnostics {
        println!("\nO/F sweep diagnostics (top 12 by metric):");
        let mut ranked = of_samples.clone();
        ranked.sort_by(|a, b| b.metric.total_cmp(&a.metric));
        for s in ranked.into_iter().take(12) {
            println!(
                "  MR={:.5} metric={:.5} c*={:.3} Isp_vac={:.3} Ve_vac={:.3}",
                s.mr, s.metric, s.c_star, s.isp_vac_s, s.ve_vac_mps
            );
        }
    }

    Ok(())
}

#[derive(Clone)]
struct OfSample {
    mr: f64,
    metric: f64,
    c_star: f64,
    isp_vac_s: f64,
    ve_vac_mps: f64,
}

#[allow(clippy::too_many_arguments)]
fn optimize_mixture_ratio_with_diagnostics(
    solver: &RocketAnalysisSolver<NativeCeaBackend>,
    oxidizer: &str,
    fuel: &str,
    chamber_pressure_pa: f64,
    ambient_pressure_pa: f64,
    oxidizer_temperature_k: f64,
    fuel_temperature_k: f64,
    combustor_model: &CombustorModel,
    nozzle_chemistry_model: &NozzleChemistryModel,
    nozzle_constraint: &NozzleConstraint,
    anchor_mr: f64,
    of_min: f64,
    of_max: f64,
    of_points: usize,
) -> AppResult<(f64, Vec<OfSample>, Option<String>)> {
    const G0: f64 = 9.80665;
    let min_mr = of_min.min(of_max).max(0.1);
    let max_mr = of_max.max(of_min).max(min_mr + 1.0e-6);
    let points = of_points.max(7);
    let mut best_mr = anchor_mr.clamp(min_mr, max_mr);
    let mut best_metric = f64::NEG_INFINITY;
    let mut samples = Vec::with_capacity(points);

    for i in 0..points {
        let f = i as f64 / (points - 1) as f64;
        let mr = min_mr + f * (max_mr - min_mr);
        let problem = RocketAnalysisProblem {
            oxidizer: Reactant {
                name: oxidizer.to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(oxidizer_temperature_k),
            },
            fuel: Reactant {
                name: fuel.to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(fuel_temperature_k),
            },
            chamber_pressure_pa,
            mixture_ratio_oxidizer_to_fuel: mr,
            nozzle_constraint: nozzle_constraint.clone(),
            combustor_model: combustor_model.clone(),
            nozzle_chemistry_model: nozzle_chemistry_model.clone(),
            ambient_pressure_pa,
        };
        let Ok(res) = solver.solve(&problem) else {
            continue;
        };
        let metric = if res.characteristic_velocity_m_per_s.is_finite()
            && res.characteristic_velocity_m_per_s > 10.0
        {
            res.characteristic_velocity_m_per_s
        } else if res.specific_impulse_vac_s.is_finite() && res.specific_impulse_vac_s > 1.0 {
            res.specific_impulse_vac_s * G0
        } else if res.effective_exhaust_velocity_vac_m_per_s.is_finite()
            && res.effective_exhaust_velocity_vac_m_per_s > 10.0
        {
            res.effective_exhaust_velocity_vac_m_per_s
        } else {
            continue;
        };
        samples.push(OfSample {
            mr,
            metric,
            c_star: res.characteristic_velocity_m_per_s,
            isp_vac_s: res.specific_impulse_vac_s,
            ve_vac_mps: res.effective_exhaust_velocity_vac_m_per_s,
        });
        let rel_delta = if best_metric.is_finite() && best_metric.abs() > 1.0e-9 {
            ((metric - best_metric) / best_metric.abs()).abs()
        } else {
            f64::INFINITY
        };
        let nearly_tied = rel_delta <= 5.0e-4;
        if metric > best_metric
            || (nearly_tied && (mr - anchor_mr).abs() < (best_mr - anchor_mr).abs())
        {
            best_metric = metric;
            best_mr = mr;
        }
    }

    if !best_metric.is_finite() {
        return Err(AppError::Backend {
            message: "Could not determine optimal O/F from sweep; all points invalid.".to_owned(),
        });
    }

    let mut note = None;
    if (best_mr - min_mr).abs() < 1.0e-6 || (best_mr - max_mr).abs() < 1.0e-6 {
        note = Some(
            "Selected optimum lies on sweep boundary; objective may be flat or sweep bounds may be too narrow."
                .to_owned(),
        );
    }
    Ok((best_mr, samples, note))
}

fn cmd_systems(project_path: &Path) -> AppResult<()> {
    let project = project_service::load_project(project_path)?;
    let systems = project_service::list_systems(&project);

    if systems.is_empty() {
        println!("No systems found in project");
    } else {
        println!("Systems in project:");
        for sys in systems {
            println!(
                "  {} - {} ({} nodes, {} components)",
                sys.id, sys.name, sys.node_count, sys.component_count
            );
        }
    }
    Ok(())
}

fn cmd_run_steady(project_path: &Path, system_id: &str, use_cache: bool) -> AppResult<()> {
    println!("Running steady-state simulation for system: {}", system_id);

    let request = RunRequest {
        project_path,
        system_id,
        mode: RunMode::Steady,
        options: RunOptions {
            use_cache,
            solver_version: "0.1.0".to_string(),
            initialization_strategy: None,
        },
    };

    let mut last_emit = Instant::now();
    let mut last_stage = String::new();
    let response = run_service::ensure_run_with_progress(
        &request,
        Some(&mut |event| {
            let stage_key = format!("{:?}", event.stage);
            let emit_now = stage_key != last_stage || last_emit.elapsed().as_millis() >= 100;
            if emit_now {
                render_cli_progress(&event);
                last_stage = stage_key;
                last_emit = Instant::now();
            }
        }),
    )?;
    clear_progress_line();

    if response.loaded_from_cache {
        println!("✓ Loaded from cache: {}", response.run_id);
    } else {
        println!("✓ Simulation completed: {}", response.run_id);
    }

    print_timing_summary(&request.mode, &response.timing);

    // Load results and show brief summary
    let (_manifest, records) = run_service::load_run(project_path, &response.run_id)?;
    let summary = query::get_run_summary(&records)?;
    println!("  Time points: {}", summary.record_count);
    println!("  Nodes: {}", summary.node_count);
    println!("  Components: {}", summary.component_count);

    Ok(())
}

fn cmd_run_transient(
    project_path: &Path,
    system_id: &str,
    dt: f64,
    t_end: f64,
    use_cache: bool,
) -> AppResult<()> {
    println!("Running transient simulation for system: {}", system_id);
    println!("  dt = {:.3} s, t_end = {:.3} s", dt, t_end);

    let request = RunRequest {
        project_path,
        system_id,
        mode: RunMode::Transient {
            dt_s: dt,
            t_end_s: t_end,
        },
        options: RunOptions {
            use_cache,
            solver_version: "0.1.0".to_string(),
            initialization_strategy: None,
        },
    };

    let mut last_emit = Instant::now();
    let mut last_fraction = -1.0f64;
    let response = run_service::ensure_run_with_progress(
        &request,
        Some(&mut |event| {
            let fraction = event
                .transient
                .as_ref()
                .map(|t| t.fraction_complete)
                .unwrap_or(-1.0);
            let emit_now = (fraction >= 0.0 && (fraction - last_fraction).abs() >= 0.005)
                || last_emit.elapsed().as_millis() >= 100;
            if emit_now {
                render_cli_progress(&event);
                if fraction >= 0.0 {
                    last_fraction = fraction;
                }
                last_emit = Instant::now();
            }
        }),
    )?;
    clear_progress_line();

    if response.loaded_from_cache {
        println!("✓ Loaded from cache: {}", response.run_id);
    } else {
        println!("✓ Simulation completed: {}", response.run_id);
    }

    print_timing_summary(&request.mode, &response.timing);

    // Load results and show brief summary
    let (_manifest, records) = run_service::load_run(project_path, &response.run_id)?;
    let summary = query::get_run_summary(&records)?;
    println!("  Time points: {}", summary.record_count);
    println!("  Nodes: {}", summary.node_count);
    println!("  Components: {}", summary.component_count);

    Ok(())
}

fn clear_progress_line() {
    print!("\r{}\r", " ".repeat(180));
    let _ = io::stdout().flush();
}

fn render_cli_progress(event: &RunProgressEvent) {
    match event.stage {
        RunStage::RunningTransient => {
            if let Some(t) = &event.transient {
                let width = 28usize;
                let filled = ((t.fraction_complete * width as f64).round() as usize).min(width);
                let bar = format!(
                    "{}{}",
                    "#".repeat(filled),
                    "-".repeat(width.saturating_sub(filled))
                );
                print!(
                    "\r[{}] {:>6.2}%  phase={}  t={:.3}/{:.3}s  step={}  cutbacks={}  elapsed={:.1}s",
                    bar,
                    t.fraction_complete * 100.0,
                    event.stage.label(),
                    t.sim_time_s,
                    t.t_end_s,
                    t.step,
                    t.cutback_retries,
                    event.elapsed_wall_s
                );
                let _ = io::stdout().flush();
            }
        }
        _ => {
            let spinner = ['|', '/', '-', '\\'];
            let spin_idx = ((event.elapsed_wall_s * 10.0) as usize) % spinner.len();
            let mut line = format!(
                "\r{} {}  elapsed={:.2}s",
                spinner[spin_idx],
                event.stage.label(),
                event.elapsed_wall_s
            );
            if let Some(strategy) = &event.initialization_strategy {
                line.push_str(&format!("  init={}", strategy));
            }
            if let Some(s) = &event.steady {
                if let Some(iter) = s.iteration {
                    line.push_str(&format!("  iter={}", iter));
                }
                if let Some(residual) = s.residual_norm {
                    line.push_str(&format!("  residual={:.3e}", residual));
                }
            }
            if let Some(msg) = &event.message {
                line.push_str(&format!("  {}", msg));
            }
            print!("{}", line);
            let _ = io::stdout().flush();
        }
    }
}

fn print_timing_summary(mode: &RunMode, timing: &tf_app::RunTimingSummary) {
    if let Some(strategy_name) = &timing.initialization_strategy {
        println!("\nInitialization: {}", strategy_name);
    }

    let total = timing.total_time_s.max(1.0e-12);
    let compile_pct = 100.0 * timing.compile_time_s / total;
    let build_pct = 100.0 * timing.build_time_s / total;
    let solve_pct = 100.0 * timing.solve_time_s / total;
    let save_pct = 100.0 * timing.save_time_s / total;

    println!("\nTiming summary:");
    println!(
        "  Compile: {:.3}s ({:.1}%)",
        timing.compile_time_s, compile_pct
    );
    if timing.build_time_s > 0.0 {
        println!("  Build:   {:.3}s ({:.1}%)", timing.build_time_s, build_pct);
    }
    println!("  Solve:   {:.3}s ({:.1}%)", timing.solve_time_s, solve_pct);

    // Phase 0 instrumentation: Show fine-grained solver timing if available
    if timing.solve_residual_time_s > 0.0 {
        let res_pct = 100.0 * timing.solve_residual_time_s / timing.solve_time_s;
        println!(
            "    Residual eval: {:.3}s ({:.1}%)",
            timing.solve_residual_time_s, res_pct
        );
    }
    if timing.solve_jacobian_time_s > 0.0 {
        let jac_pct = 100.0 * timing.solve_jacobian_time_s / timing.solve_time_s;
        println!(
            "    Jacobian eval: {:.3}s ({:.1}%)",
            timing.solve_jacobian_time_s, jac_pct
        );
    }
    if timing.solve_linearch_time_s > 0.0 {
        let ls_pct = 100.0 * timing.solve_linearch_time_s / timing.solve_time_s;
        println!(
            "    Line search:   {:.3}s ({:.1}%)",
            timing.solve_linearch_time_s, ls_pct
        );
    }
    if timing.solve_thermo_time_s > 0.0 {
        let th_pct = 100.0 * timing.solve_thermo_time_s / timing.solve_time_s;
        println!(
            "    Thermo state:  {:.3}s ({:.1}%)",
            timing.solve_thermo_time_s, th_pct
        );
    }
    if timing.rhs_snapshot_time_s > 0.0 {
        let pct = 100.0 * timing.rhs_snapshot_time_s / timing.solve_time_s;
        println!(
            "    RHS snapshot: {:.3}s ({:.1}%)",
            timing.rhs_snapshot_time_s, pct
        );
    }
    if timing.rhs_state_reconstruct_time_s > 0.0 {
        let pct = 100.0 * timing.rhs_state_reconstruct_time_s / timing.solve_time_s;
        println!(
            "    RHS state rebuild: {:.3}s ({:.1}%)",
            timing.rhs_state_reconstruct_time_s, pct
        );
    }
    if timing.rhs_flow_routing_time_s > 0.0 {
        let pct = 100.0 * timing.rhs_flow_routing_time_s / timing.solve_time_s;
        println!(
            "    RHS flow routing: {:.3}s ({:.1}%)",
            timing.rhs_flow_routing_time_s, pct
        );
    }
    if timing.rhs_surrogate_time_s > 0.0 {
        let pct = 100.0 * timing.rhs_surrogate_time_s / timing.solve_time_s;
        println!(
            "    RHS surrogate work: {:.3}s ({:.1}%)",
            timing.rhs_surrogate_time_s, pct
        );
    }

    println!("  Save:    {:.3}s ({:.1}%)", timing.save_time_s, save_pct);
    if timing.load_cache_time_s > 0.0 {
        println!("  Cache load: {:.3}s", timing.load_cache_time_s);
    }
    println!("  Total:   {:.3}s", timing.total_time_s);

    match mode {
        RunMode::Steady => {
            println!("  Steady iterations: {}", timing.steady_iterations);
            if timing.steady_residual_norm > 0.0 {
                println!("  Final residual: {:.3e}", timing.steady_residual_norm);
            }
            // Phase 0: Show residual eval counts
            if timing.solve_residual_eval_count > 0 {
                println!(
                    "  Residual evaluations: {}",
                    timing.solve_residual_eval_count
                );
            }
            if timing.solve_jacobian_eval_count > 0 {
                println!(
                    "  Jacobian evaluations: {}",
                    timing.solve_jacobian_eval_count
                );
            }
        }
        RunMode::Transient { .. } => {
            println!("  Transient steps: {}", timing.transient_steps);
            println!("  Cutback retries: {}", timing.transient_cutback_retries);
            println!("  Fallback uses:   {}", timing.transient_fallback_uses);
            if timing.transient_real_fluid_attempts > 0 {
                let success_pct = 100.0 * (timing.transient_real_fluid_successes as f64)
                    / (timing.transient_real_fluid_attempts as f64);
                println!(
                    "  Real-fluid:      {}/{} ({:.1}%)",
                    timing.transient_real_fluid_successes,
                    timing.transient_real_fluid_attempts,
                    success_pct
                );
            }
            println!(
                "  Surrogate updates: {}",
                timing.transient_surrogate_populations
            );
        }
    }
}

fn cmd_runs(project_path: &Path, system_id: &str) -> AppResult<()> {
    let runs = run_service::list_runs(project_path, system_id)?;

    if runs.is_empty() {
        println!("No cached runs found for system: {}", system_id);
    } else {
        println!("Cached runs for system '{}':", system_id);
        for manifest in runs {
            println!("  {} ({})", manifest.run_id, manifest.timestamp);
        }
    }
    Ok(())
}

fn cmd_show_run(project_path: &Path, run_id: &str) -> AppResult<()> {
    println!("Loading run: {}", run_id);

    let (_manifest, records) = run_service::load_run(project_path, run_id)?;
    let summary = query::get_run_summary(&records)?;

    println!("\nRun Summary:");
    println!("  Time points: {}", summary.record_count);
    println!(
        "  Time range: {:.3} - {:.3} s",
        summary.time_range.0, summary.time_range.1
    );
    println!("  Nodes: {}", summary.node_count);
    println!("  Components: {}", summary.component_count);

    let node_ids = query::list_node_ids(&records);
    println!("\nNodes:");
    for id in node_ids {
        println!("  {}", id);
    }

    let comp_ids = query::list_component_ids(&records);
    println!("\nComponents:");
    for id in comp_ids {
        println!("  {}", id);
    }

    Ok(())
}

fn cmd_export_series(
    project_path: &Path,
    run_id: &str,
    entity_id: &str,
    variable: &str,
    output: Option<&Path>,
) -> AppResult<()> {
    let (_manifest, records) = run_service::load_run(project_path, run_id)?;

    // Try node variable first
    let series = if let Ok(data) = query::extract_node_series(&records, entity_id, variable) {
        data
    } else {
        // Try component variable
        query::extract_component_series(&records, entity_id, variable)?
    };

    // Build CSV
    let mut csv = String::from("time_s,value\n");
    for (t, val) in &series {
        csv.push_str(&format!("{},{}\n", t, val));
    }

    // Write to file or stdout
    if let Some(path) = output {
        std::fs::write(path, csv)?;
        println!(
            "✓ Exported {} data points to {}",
            series.len(),
            path.display()
        );
    } else {
        print!("{}", csv);
    }

    Ok(())
}
