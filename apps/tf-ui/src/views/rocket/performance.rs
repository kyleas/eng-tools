use crate::rocket_workspace::{
    CEA_FUELS, CEA_OXIDIZERS, PerformanceConstraint, RocketWorkspace, format_state_summary,
    species_display_name,
};
use tf_cea::NativeCeaBackend;
use tf_fluids::{Quantity, parse_quantity};
use tf_rpa::{
    GeometrySizingMode, NozzleConstraint, RocketAnalysisProblem, RocketAnalysisResult,
    RocketAnalysisSolver, compute_geometry,
};

pub fn show(ui: &mut egui::Ui, workspace: &mut RocketWorkspace) {
    ui.heading("Rocket Performance");
    ui.label("Configure chamber/nozzle assumptions and run a backend-driven solve through tf-rpa.");
    if let Some(preset) = workspace.selected_propellant_preset() {
        ui.label(format!(
            "Propellant preset: {} ({})",
            preset.display_name, preset.category
        ));
    }
    ui.separator();

    egui::Grid::new("rocket_perf_inputs")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("Case name");
            ui.text_edit_singleline(&mut workspace.performance_case.case_name);
            ui.end_row();

            ui.label("Oxidizer");
            egui::ComboBox::new("oxidizer_combo", "")
                .selected_text(&workspace.performance_case.oxidizer_name)
                .show_ui(ui, |ui| {
                    for oxidizer in CEA_OXIDIZERS {
                        ui.selectable_value(
                            &mut workspace.performance_case.oxidizer_name,
                            oxidizer.to_string(),
                            *oxidizer,
                        );
                    }
                });
            ui.label(species_display_name(
                &workspace.performance_case.oxidizer_name,
            ));
            ui.end_row();

            ui.label("Fuel");
            egui::ComboBox::new("fuel_combo", "")
                .selected_text(&workspace.performance_case.fuel_name)
                .show_ui(ui, |ui| {
                    for fuel in CEA_FUELS {
                        ui.selectable_value(
                            &mut workspace.performance_case.fuel_name,
                            fuel.to_string(),
                            *fuel,
                        );
                    }
                });
            ui.label(species_display_name(&workspace.performance_case.fuel_name));
            ui.end_row();

            ui.label("O/F mixture ratio");
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut workspace.performance_case.use_optimal_mixture_ratio,
                    "Optimal",
                );
                let shown_mr = if workspace.performance_case.use_optimal_mixture_ratio {
                    workspace.performance_case.optimal_mixture_ratio_value
                } else {
                    workspace.performance_case.mixture_ratio
                };
                let mut mr_display = shown_mr;
                ui.add_enabled(
                    !workspace.performance_case.use_optimal_mixture_ratio,
                    egui::DragValue::new(&mut mr_display).speed(0.01),
                );
                if !workspace.performance_case.use_optimal_mixture_ratio {
                    workspace.performance_case.mixture_ratio = mr_display;
                }
                if workspace.performance_case.use_optimal_mixture_ratio {
                    ui.small(format!(
                        "Using {:.3}",
                        workspace.performance_case.optimal_mixture_ratio_value
                    ));
                }
            });
            ui.end_row();

            ui.label("Chamber pressure");
            if ui
                .text_edit_singleline(&mut workspace.performance_case.chamber_pressure_text)
                .changed()
            {
                if let Ok(value) = parse_quantity(
                    &workspace.performance_case.chamber_pressure_text,
                    Quantity::Pressure,
                ) {
                    workspace.performance_case.chamber_pressure_pa = value;
                }
            }
            ui.end_row();

            ui.label("Ambient pressure");
            if ui
                .text_edit_singleline(&mut workspace.performance_case.ambient_pressure_text)
                .changed()
            {
                if let Ok(value) = parse_quantity(
                    &workspace.performance_case.ambient_pressure_text,
                    Quantity::Pressure,
                ) {
                    workspace.performance_case.ambient_pressure_pa = value;
                }
            }
            ui.end_row();

            ui.label("Oxidizer temperature");
            if ui
                .text_edit_singleline(&mut workspace.performance_case.oxidizer_temperature_text)
                .changed()
            {
                if let Ok(value) = parse_quantity(
                    &workspace.performance_case.oxidizer_temperature_text,
                    Quantity::Temperature,
                ) {
                    workspace.performance_case.oxidizer_temperature_k = value;
                }
            }
            ui.end_row();

            ui.label("Fuel temperature");
            if ui
                .text_edit_singleline(&mut workspace.performance_case.fuel_temperature_text)
                .changed()
            {
                if let Ok(value) = parse_quantity(
                    &workspace.performance_case.fuel_temperature_text,
                    Quantity::Temperature,
                ) {
                    workspace.performance_case.fuel_temperature_k = value;
                }
            }
            ui.end_row();
        });

    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Combustor model:");
        let finite_selected = matches!(
            workspace.performance_case.combustor_model,
            tf_rpa::CombustorModel::FiniteArea { .. }
        );
        if ui.radio(!finite_selected, "Infinite Area").clicked() {
            workspace.performance_case.combustor_model = tf_rpa::CombustorModel::InfiniteArea;
        }
        if ui.radio(finite_selected, "Finite Area").clicked() {
            workspace.performance_case.combustor_model = tf_rpa::CombustorModel::FiniteArea {
                contraction_ratio: 2.5,
            };
        }
        if let tf_rpa::CombustorModel::FiniteArea { contraction_ratio } =
            &mut workspace.performance_case.combustor_model
        {
            ui.label("Contraction ratio:");
            ui.add(egui::DragValue::new(contraction_ratio).speed(0.05));
        }
    });

    ui.horizontal(|ui| {
        ui.label("Nozzle chemistry:");
        ui.selectable_value(
            &mut workspace.performance_case.nozzle_chemistry_model,
            tf_rpa::NozzleChemistryModel::ShiftingEquilibrium,
            "Shifting Equilibrium",
        );
        ui.selectable_value(
            &mut workspace.performance_case.nozzle_chemistry_model,
            tf_rpa::NozzleChemistryModel::FrozenAtChamber,
            "Frozen at Chamber",
        );
        ui.selectable_value(
            &mut workspace.performance_case.nozzle_chemistry_model,
            tf_rpa::NozzleChemistryModel::FrozenAtThroat,
            "Frozen at Throat",
        );
    });

    ui.horizontal(|ui| {
        ui.label("Nozzle constraint:");
        let is_expansion = matches!(
            workspace.performance_case.nozzle_constraint,
            NozzleConstraint::ExpansionRatio(_)
        );
        if ui.radio(is_expansion, "Expansion ratio").clicked() {
            workspace.performance_case.nozzle_constraint = NozzleConstraint::ExpansionRatio(40.0);
        }
        if ui.radio(!is_expansion, "Exit pressure").clicked() {
            workspace.performance_case.nozzle_constraint =
                NozzleConstraint::ExitPressurePa(50_000.0);
        }

        match &mut workspace.performance_case.nozzle_constraint {
            NozzleConstraint::ExpansionRatio(value) => {
                ui.label("epsilon:");
                ui.add(egui::DragValue::new(value).speed(0.2));
            }
            NozzleConstraint::ExitPressurePa(value) => {
                ui.label("P_exit [Pa]:");
                ui.add(egui::DragValue::new(value).speed(100.0));
            }
        }
    });

    ui.horizontal(|ui| {
        ui.label("Performance constraint:");

        let current = workspace.performance_case.performance_constraint.clone();

        if ui
            .radio(
                matches!(current, PerformanceConstraint::None),
                "None (solve for given conditions)",
            )
            .clicked()
        {
            workspace.performance_case.performance_constraint = PerformanceConstraint::None;
        }

        if ui
            .radio(
                matches!(current, PerformanceConstraint::Thrust { .. }),
                "Target thrust",
            )
            .clicked()
        {
            workspace.performance_case.performance_constraint = PerformanceConstraint::Thrust {
                target_n: 1_000_000.0,
            };
        }

        if ui
            .radio(
                matches!(current, PerformanceConstraint::MassFlow { .. }),
                "Target mass flow",
            )
            .clicked()
        {
            workspace.performance_case.performance_constraint = PerformanceConstraint::MassFlow {
                target_kg_per_s: 100.0,
            };
        }

        if ui
            .radio(
                matches!(current, PerformanceConstraint::ThroatDiameter { .. }),
                "Target throat diameter",
            )
            .clicked()
        {
            workspace.performance_case.performance_constraint =
                PerformanceConstraint::ThroatDiameter { target_m: 0.5 };
        }
    });

    match &mut workspace.performance_case.performance_constraint {
        PerformanceConstraint::None => {}
        PerformanceConstraint::Thrust { target_n } => {
            ui.horizontal(|ui| {
                ui.label("  Target thrust:");
                if ui
                    .text_edit_singleline(&mut workspace.performance_case.thrust_target_text)
                    .changed()
                {
                    if let Ok(value) = parse_quantity(
                        &workspace.performance_case.thrust_target_text,
                        Quantity::Force,
                    ) {
                        *target_n = value;
                    }
                }
            });
        }
        PerformanceConstraint::MassFlow { target_kg_per_s } => {
            ui.horizontal(|ui| {
                ui.label("  Target mass flow:");
                if ui
                    .text_edit_singleline(&mut workspace.performance_case.mass_flow_target_text)
                    .changed()
                {
                    if let Ok(value) = parse_quantity(
                        &workspace.performance_case.mass_flow_target_text,
                        Quantity::MassFlow,
                    ) {
                        *target_kg_per_s = value;
                    }
                }
            });
        }
        PerformanceConstraint::ThroatDiameter { target_m } => {
            ui.horizontal(|ui| {
                ui.label("  Target throat diameter:");
                if ui
                    .text_edit_singleline(&mut workspace.performance_case.diameter_target_text)
                    .changed()
                {
                    if let Ok(value) = parse_quantity(
                        &workspace.performance_case.diameter_target_text,
                        Quantity::Length,
                    ) {
                        *target_m = value;
                    }
                }
            });
        }
    }

    ui.separator();
    if ui.button("Solve Performance").clicked() {
        match solve_case(workspace) {
            Ok(msg) => {
                workspace.status = Some(msg);
                workspace.last_error = None;
            }
            Err(err) => {
                workspace.last_error = Some(err);
                workspace.status = None;
            }
        }
    }

    if let Some(status) = &workspace.status {
        ui.colored_label(egui::Color32::GREEN, status);
    }
    if let Some(err) = &workspace.last_error {
        ui.colored_label(egui::Color32::RED, err);
    }

    ui.separator();
    if let Some(result) = &workspace.last_result {
        ui.heading("Summary");
        ui.label(format!(
            "c* = {:.3} m/s",
            result.characteristic_velocity_m_per_s
        ));
        ui.label(format!("Cf,vac = {:.5}", result.thrust_coefficient_vac));
        ui.label(format!("Cf,amb = {:.5}", result.thrust_coefficient_amb));
        ui.label(format!("Isp,vac = {:.3} s", result.specific_impulse_vac_s));
        ui.label(format!("Isp,amb = {:.3} s", result.specific_impulse_amb_s));

        if let Some(geometry) = &workspace.last_geometry_result {
            let thrust_vac_n = result.thrust_coefficient_vac
                * result.chamber_pressure_pa
                * geometry.throat_area_m2;
            let thrust_amb_n = result.thrust_coefficient_amb
                * result.chamber_pressure_pa
                * geometry.throat_area_m2;
            ui.label(format!(
                "Thrust,vac = {:.1} N ({:.1} kN)",
                thrust_vac_n,
                thrust_vac_n / 1000.0
            ));
            ui.label(format!(
                "Thrust,amb = {:.1} N ({:.1} kN)",
                thrust_amb_n,
                thrust_amb_n / 1000.0
            ));
            ui.label(format!(
                "Throat area = {:.6} m^2 (diameter {:.4} m)",
                geometry.throat_area_m2, geometry.throat_diameter_m
            ));
        }

        ui.label(format!(
            "Chamber: T={:.2} K, gamma={:.5}, MW={:.4} kg/kmol",
            result.chamber.temperature_k.unwrap_or(f64::NAN),
            result.chamber.gamma.unwrap_or(f64::NAN),
            result
                .chamber
                .molecular_weight_kg_per_kmol
                .unwrap_or(f64::NAN),
        ));

        ui.separator();
        ui.heading("Assumptions");
        ui.label(format!(
            "Combustor: {:?}",
            result.assumptions.combustor_model
        ));
        ui.label(format!(
            "Nozzle chemistry: {:?}",
            result.assumptions.nozzle_chemistry_model
        ));
        ui.label(format!(
            "Nozzle constraint: {:?}",
            result.assumptions.nozzle_constraint
        ));
        ui.label(format!(
            "Ambient pressure: {:.2} Pa",
            result.ambient_pressure_pa
        ));

        ui.separator();
        ui.heading("Station summaries");
        ui.label(format!(
            "Chamber: {}",
            format_state_summary(&result.chamber)
        ));
        ui.label(format!("Throat:  {}", format_state_summary(&result.throat)));
        ui.label(format!("Exit:    {}", format_state_summary(&result.exit)));

        ui.separator();
        for note in &result.notes {
            ui.label(format!("* {note}"));
        }
    } else {
        ui.label("No result yet. Configure inputs and click Solve Performance.");
    }
}

fn solve_case(workspace: &mut RocketWorkspace) -> Result<String, String> {
    let backend = NativeCeaBackend::new();
    let solver = RocketAnalysisSolver::new(backend);
    if workspace.performance_case.use_optimal_mixture_ratio {
        let optimal = find_optimal_mixture_ratio(workspace, &solver)?;
        workspace.performance_case.optimal_mixture_ratio_value = optimal;
        workspace.performance_case.mixture_ratio = optimal;
    }
    let constraint = workspace.performance_case.performance_constraint.clone();

    match constraint {
        PerformanceConstraint::None => {
            let problem: RocketAnalysisProblem = workspace.performance_case.to_analysis_problem();
            let result = solver
                .solve(&problem)
                .map_err(|e| format!("Performance solve failed: {e}"))?;
            workspace.last_result = Some(result);
            Ok(format!(
                "Solved case '{}'",
                workspace.performance_case.case_name
            ))
        }
        PerformanceConstraint::Thrust { target_n } => {
            if target_n <= 0.0 {
                return Err("Target thrust must be greater than zero.".to_owned());
            }
            let throat_area_m2 = configured_throat_area_m2(workspace)?;
            let msg = solve_with_pressure_constraint(
                workspace,
                &solver,
                target_n,
                "Target thrust",
                |result| {
                    result.thrust_coefficient_vac * result.chamber_pressure_pa * throat_area_m2
                },
            )?;
            sync_geometry_from_constraint(workspace, target_n, ConstraintMetric::Thrust)?;
            Ok(msg)
        }
        PerformanceConstraint::MassFlow { target_kg_per_s } => {
            if target_kg_per_s <= 0.0 {
                return Err("Target mass flow must be greater than zero.".to_owned());
            }
            let throat_area_m2 = configured_throat_area_m2(workspace)?;
            let msg = solve_with_pressure_constraint(
                workspace,
                &solver,
                target_kg_per_s,
                "Target mass flow",
                |result| {
                    (result.chamber_pressure_pa * throat_area_m2)
                        / result.characteristic_velocity_m_per_s
                },
            )?;
            sync_geometry_from_constraint(workspace, target_kg_per_s, ConstraintMetric::MassFlow)?;
            Ok(msg)
        }
        PerformanceConstraint::ThroatDiameter { target_m } => {
            if target_m <= 0.0 {
                return Err("Target throat diameter must be greater than zero.".to_owned());
            }
            workspace.geometry.sizing_mode = GeometrySizingMode::GivenThroatDiameter;
            workspace.geometry.throat_input_value = target_m;

            let problem: RocketAnalysisProblem = workspace.performance_case.to_analysis_problem();
            let result = solver
                .solve(&problem)
                .map_err(|e| format!("Performance solve failed: {e}"))?;
            workspace.last_result = Some(result);
            let geometry_result = compute_geometry(&workspace.geometry_problem())
                .map_err(|e| format!("Geometry update failed after throat target solve: {e}"))?;
            workspace.last_geometry_result = Some(geometry_result);
            Ok(format!(
                "Solved case '{}' with throat diameter target set to {:.4} m",
                workspace.performance_case.case_name, target_m
            ))
        }
    }
}

fn find_optimal_mixture_ratio(
    workspace: &RocketWorkspace,
    solver: &RocketAnalysisSolver<NativeCeaBackend>,
) -> Result<f64, String> {
    // Sweep O/F and maximize a robust propulsion metric.
    // Prefer c* when valid, then fall back to Isp_vac / effective exhaust velocity.
    const MIN_MR: f64 = 0.5;
    const MAX_MR: f64 = 12.0;
    const POINTS: usize = 121;
    const G0: f64 = 9.80665;
    let mut best_mr = workspace
        .performance_case
        .mixture_ratio
        .clamp(MIN_MR, MAX_MR);
    let mut best_metric = f64::NEG_INFINITY;
    let mut valid_points = 0usize;
    let mut samples: Vec<(f64, f64)> = Vec::with_capacity(POINTS);
    let anchor_mr = workspace
        .performance_case
        .mixture_ratio
        .clamp(MIN_MR, MAX_MR);

    for i in 0..POINTS {
        let f = i as f64 / (POINTS - 1) as f64;
        let mr = MIN_MR + f * (MAX_MR - MIN_MR);
        let mut case = workspace.performance_case.clone();
        case.mixture_ratio = mr;
        let problem = case.to_analysis_problem();
        if let Ok(result) = solver.solve(&problem) {
            let metric = if result.characteristic_velocity_m_per_s.is_finite()
                && result.characteristic_velocity_m_per_s > 10.0
            {
                result.characteristic_velocity_m_per_s
            } else if result.specific_impulse_vac_s.is_finite()
                && result.specific_impulse_vac_s > 1.0
            {
                result.specific_impulse_vac_s * G0
            } else if result.effective_exhaust_velocity_vac_m_per_s.is_finite()
                && result.effective_exhaust_velocity_vac_m_per_s > 10.0
            {
                result.effective_exhaust_velocity_vac_m_per_s
            } else {
                continue;
            };
            valid_points += 1;
            samples.push((mr, metric));
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
    }
    if valid_points == 0 || !best_metric.is_finite() {
        return Err(
            "Could not determine optimal O/F from backend solves (no valid points in sweep)."
                .to_owned(),
        );
    }

    // If best is on a hard boundary but interior is nearly as good, choose an interior value.
    if (best_mr - MIN_MR).abs() < 1.0e-6 || (best_mr - MAX_MR).abs() < 1.0e-6 {
        let threshold = best_metric * 0.99;
        let mut best_interior: Option<(f64, f64)> = None;
        for (mr, metric) in samples
            .into_iter()
            .filter(|(mr, _)| *mr > MIN_MR + 0.25 && *mr < MAX_MR - 0.25)
        {
            if metric >= threshold {
                if let Some((_, prev_metric)) = best_interior {
                    if metric > prev_metric {
                        best_interior = Some((mr, metric));
                    }
                } else {
                    best_interior = Some((mr, metric));
                }
            }
        }
        if let Some((mr, metric)) = best_interior {
            best_mr = mr;
            best_metric = metric;
        }
    }

    if best_metric.is_finite() {
        Ok(best_mr)
    } else {
        Err("Could not determine optimal O/F from backend solves.".to_owned())
    }
}

#[derive(Clone, Copy)]
enum ConstraintMetric {
    Thrust,
    MassFlow,
}

fn sync_geometry_from_constraint(
    workspace: &mut RocketWorkspace,
    target: f64,
    metric: ConstraintMetric,
) -> Result<(), String> {
    let Some(result) = workspace.last_result.as_ref() else {
        return Ok(());
    };
    let pc = result.chamber_pressure_pa;
    if pc <= 0.0 || !pc.is_finite() {
        return Err("Invalid chamber pressure from solve when syncing geometry.".to_owned());
    }

    let throat_area_m2 = match metric {
        ConstraintMetric::Thrust => {
            let cf = result.thrust_coefficient_vac;
            if cf <= 0.0 || !cf.is_finite() {
                return Err("Invalid Cf_vac from solve when syncing geometry.".to_owned());
            }
            target / (cf * pc)
        }
        ConstraintMetric::MassFlow => {
            let c_star = result.characteristic_velocity_m_per_s;
            if c_star <= 0.0 || !c_star.is_finite() {
                return Err("Invalid c* from solve when syncing geometry.".to_owned());
            }
            target * c_star / pc
        }
    };

    if throat_area_m2 <= 0.0 || !throat_area_m2.is_finite() {
        return Err("Computed throat area was invalid while syncing geometry.".to_owned());
    }

    workspace.geometry.sizing_mode = GeometrySizingMode::GivenThroatArea;
    workspace.geometry.throat_input_value = throat_area_m2;
    let geometry_result = compute_geometry(&workspace.geometry_problem())
        .map_err(|e| format!("Geometry update failed after constrained solve: {e}"))?;
    workspace.last_geometry_result = Some(geometry_result);
    Ok(())
}

fn configured_throat_area_m2(workspace: &RocketWorkspace) -> Result<f64, String> {
    match workspace.geometry.sizing_mode {
        GeometrySizingMode::GivenThroatDiameter => {
            let d = workspace.geometry.throat_input_value;
            if d <= 0.0 {
                return Err(
                    "Geometry throat diameter must be > 0 for constrained solve.".to_owned(),
                );
            }
            Ok(std::f64::consts::PI * 0.25 * d * d)
        }
        GeometrySizingMode::GivenThroatArea => {
            let a = workspace.geometry.throat_input_value;
            if a <= 0.0 {
                return Err("Geometry throat area must be > 0 for constrained solve.".to_owned());
            }
            Ok(a)
        }
    }
}

fn solve_with_pressure_constraint<F>(
    workspace: &mut RocketWorkspace,
    solver: &RocketAnalysisSolver<NativeCeaBackend>,
    target: f64,
    constraint_name: &str,
    extract_metric: F,
) -> Result<String, String>
where
    F: Fn(&RocketAnalysisResult) -> f64,
{
    const MAX_ITERATIONS: usize = 20;
    const TOLERANCE: f64 = 0.02;
    const MIN_PRESSURE_PA: f64 = 1.0e4;
    const MAX_PRESSURE_PA: f64 = 1.0e8;

    if target <= 0.0 {
        return Err(format!("{constraint_name} target must be > 0."));
    }

    let current_pc = workspace
        .performance_case
        .chamber_pressure_pa
        .max(MIN_PRESSURE_PA);
    let mut pc_low = (current_pc / 100.0).max(MIN_PRESSURE_PA);
    let mut pc_high = (current_pc * 100.0).min(MAX_PRESSURE_PA);
    let mut best_result = None;
    let mut best_error = f64::INFINITY;

    for iteration in 0..MAX_ITERATIONS {
        let pc_mid = (pc_low + pc_high) / 2.0;
        workspace.performance_case.chamber_pressure_pa = pc_mid;

        let problem = workspace.performance_case.to_analysis_problem();
        let result = solver
            .solve(&problem)
            .map_err(|e| format!("Performance solve failed at iteration {}: {}", iteration, e))?;

        let metric = extract_metric(&result);
        if metric.is_nan() || metric.is_infinite() {
            return Err(format!("Invalid metric result at iteration {}", iteration));
        }

        let error = (metric - target) / target.max(1.0);
        if error.abs() < best_error {
            best_error = error.abs();
            best_result = Some(result);
        }

        if error.abs() < TOLERANCE {
            workspace.last_result = best_result;
            return Ok(format!(
                "Solved case '{}' to {} (error={:.1}%, {} iterations)",
                workspace.performance_case.case_name,
                constraint_name,
                error * 100.0,
                iteration + 1
            ));
        }

        if metric < target {
            pc_low = pc_mid;
        } else {
            pc_high = pc_mid;
        }

        if pc_high - pc_low < 100.0 {
            if best_error < 0.10 {
                workspace.last_result = best_result;
                return Ok(format!(
                    "Converged case '{}' to {} (error={:.1}%, {} iterations)",
                    workspace.performance_case.case_name,
                    constraint_name,
                    best_error * 100.0,
                    iteration + 1
                ));
            }
            workspace.last_result = best_result;
            return Err(format!(
                "Failed to converge to {} within acceptable error. Best error: {:.1}% after {} iterations. \
Consider reducing throat area/diameter for smaller targets or increasing pressure bounds.",
                constraint_name,
                best_error * 100.0,
                iteration + 1
            ));
        }
    }

    if let Some(result) = best_result {
        workspace.last_result = Some(result);
        Ok(format!(
            "Completed case '{}' with {} (error={:.1}%, {} iterations, tolerance not met)",
            workspace.performance_case.case_name,
            constraint_name,
            best_error * 100.0,
            MAX_ITERATIONS
        ))
    } else {
        Err(format!(
            "Failed to solve case '{}' after {} iterations",
            workspace.performance_case.case_name, MAX_ITERATIONS
        ))
    }
}
