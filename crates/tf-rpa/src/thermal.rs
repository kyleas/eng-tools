use serde::{Deserialize, Serialize};

use crate::{RocketAnalysisProblem, RocketGeometryProblem, RocketGeometryResult, RpaError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ThermalModel {
    #[default]
    BartzLikeConvective,
}

impl ThermalModel {
    pub fn label(&self) -> &'static str {
        "Bartz-like convective estimate"
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CoolingMode {
    #[default]
    AdiabaticWall,
    Regenerative,
    Film,
    RegenerativeFilm,
}

impl CoolingMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::AdiabaticWall => "Adiabatic wall (no active cooling)",
            Self::Regenerative => "Regenerative cooling",
            Self::Film => "Film cooling",
            Self::RegenerativeFilm => "Regenerative + film cooling",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WallModel {
    pub material_name: String,
    pub thermal_conductivity_w_m_k: f64,
    pub thermal_conductivity_reference_temperature_k: f64,
    pub thermal_conductivity_temp_coeff_per_k: f64,
    pub gas_side_emissivity: f64,
    pub allowable_temperature_k: f64,
    pub density_kg_m3: f64,
    pub cp_j_kg_k: f64,
    pub thickness_m: f64,
}

impl Default for WallModel {
    fn default() -> Self {
        Self {
            material_name: "CuCrZr (representative)".to_owned(),
            thermal_conductivity_w_m_k: 320.0,
            thermal_conductivity_reference_temperature_k: 300.0,
            thermal_conductivity_temp_coeff_per_k: -0.0002,
            gas_side_emissivity: 0.8,
            allowable_temperature_k: 1100.0,
            density_kg_m3: 8_900.0,
            cp_j_kg_k: 385.0,
            thickness_m: 0.0025,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoolantModel {
    pub coolant_name: String,
    pub inlet_temperature_k: f64,
    pub inlet_pressure_pa: f64,
    pub mass_flow_kg_s: f64,
    pub density_kg_m3: f64,
    pub viscosity_pa_s: f64,
    pub thermal_conductivity_w_m_k: f64,
    pub cp_j_kg_k: f64,
}

impl Default for CoolantModel {
    fn default() -> Self {
        Self {
            coolant_name: "RP-1 (bulk property placeholder)".to_owned(),
            inlet_temperature_k: 300.0,
            inlet_pressure_pa: 8_000_000.0,
            mass_flow_kg_s: 8.0,
            density_kg_m3: 780.0,
            viscosity_pa_s: 0.0015,
            thermal_conductivity_w_m_k: 0.13,
            cp_j_kg_k: 2_200.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilmCoolingModel {
    pub film_mass_fraction: f64,
    pub effectiveness_start_fraction: f64,
    pub effectiveness_end_fraction: f64,
    pub max_effectiveness: f64,
}

impl Default for FilmCoolingModel {
    fn default() -> Self {
        Self {
            film_mass_fraction: 0.03,
            effectiveness_start_fraction: 0.0,
            effectiveness_end_fraction: 0.55,
            max_effectiveness: 0.35,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelGeometry {
    pub channel_count: usize,
    pub width_m: f64,
    pub height_m: f64,
    pub rib_width_m: f64,
    pub min_gap_m: f64,
    pub roughness_m: f64,
    pub width_taper_end_factor: f64,
    pub height_taper_end_factor: f64,
    pub min_width_m: f64,
    pub max_width_m: f64,
    pub min_height_m: f64,
    pub max_height_m: f64,
}

impl Default for ChannelGeometry {
    fn default() -> Self {
        Self {
            channel_count: 160,
            width_m: 0.0012,
            height_m: 0.0022,
            rib_width_m: 0.0008,
            min_gap_m: 0.0002,
            roughness_m: 4.0e-6,
            width_taper_end_factor: 1.15,
            height_taper_end_factor: 0.92,
            min_width_m: 0.0006,
            max_width_m: 0.0035,
            min_height_m: 0.0010,
            max_height_m: 0.0050,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CoolantFlowDirection {
    #[default]
    CoFlow,
    CounterFlow,
    MidFeed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalDesignSettings {
    pub station_count: usize,
    pub max_coolant_pressure_drop_pa: f64,
    pub optimizer_max_iterations: usize,
    pub optimizer_local_adjustment_gain: f64,
    pub optimizer_global_area_gain: f64,
    pub hold_channel_count_fixed: bool,
    pub hold_channel_width_fixed: bool,
    pub hold_channel_height_fixed: bool,
    pub min_channel_count: usize,
    pub max_channel_count: usize,
    pub coolant_flow_direction: CoolantFlowDirection,
    pub mid_feed_fraction: f64,
    pub mid_feed_upstream_mass_fraction: f64,
    pub auto_balance_mid_feed_split: bool,
}

impl Default for ThermalDesignSettings {
    fn default() -> Self {
        Self {
            station_count: 61,
            max_coolant_pressure_drop_pa: 1_500_000.0,
            optimizer_max_iterations: 40,
            optimizer_local_adjustment_gain: 0.07,
            optimizer_global_area_gain: 0.04,
            hold_channel_count_fixed: true,
            hold_channel_width_fixed: false,
            hold_channel_height_fixed: false,
            min_channel_count: 32,
            max_channel_count: 2000,
            coolant_flow_direction: CoolantFlowDirection::CoFlow,
            mid_feed_fraction: 0.45,
            mid_feed_upstream_mass_fraction: 0.5,
            auto_balance_mid_feed_split: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketThermalProblem {
    pub performance_problem: RocketAnalysisProblem,
    pub geometry_problem: RocketGeometryProblem,
    pub geometry_result: RocketGeometryResult,
    pub model: ThermalModel,
    pub cooling_mode: CoolingMode,
    pub reference_recovery_temperature_k: f64,
    pub wall_temperature_k: f64,
    pub reference_gas_side_htc_w_m2_k: f64,
    pub wall_model: WallModel,
    pub coolant_model: CoolantModel,
    pub film_model: FilmCoolingModel,
    pub channel_geometry: ChannelGeometry,
    pub design_settings: ThermalDesignSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EquationSource {
    Correlation,
    Derived,
    Constraint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquationTrace {
    pub name: String,
    pub equation: String,
    pub source: EquationSource,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelAssumption {
    pub id: String,
    pub category: String,
    pub statement: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalDesignStation {
    pub station_index: usize,
    pub axial_position_m: f64,
    pub x_fraction: f64,
    pub gas_side_htc_w_m2_k: f64,
    pub adiabatic_wall_temperature_k: f64,
    pub film_effectiveness: f64,
    pub net_gas_heat_flux_w_m2: f64,
    pub coolant_side_htc_w_m2_k: f64,
    pub wall_hot_side_temperature_k: f64,
    pub wall_cold_side_temperature_k: f64,
    pub coolant_bulk_temperature_k: f64,
    pub coolant_pressure_pa: f64,
    pub local_pressure_drop_pa: f64,
    pub channel_width_m: f64,
    pub channel_height_m: f64,
    pub channel_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PressureDropResult {
    pub total_pressure_drop_pa: f64,
    pub coolant_inlet_pressure_pa: f64,
    pub coolant_outlet_pressure_pa: f64,
    pub min_coolant_pressure_pa: f64,
    pub limit_pressure_drop_pa: f64,
    pub within_limit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizerIteration {
    pub iteration: usize,
    pub peak_wall_temperature_k: f64,
    pub pressure_drop_pa: f64,
    pub min_coolant_pressure_pa: f64,
    pub channel_count: usize,
    pub width_bound_hits: usize,
    pub height_bound_hits: usize,
    pub objective_value: f64,
    pub accepted: bool,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalDesignResult {
    pub stations: Vec<ThermalDesignStation>,
    pub peak_wall_temperature_k: f64,
    pub peak_wall_temperature_station: usize,
    pub coolant_outlet_temperature_k: f64,
    pub pressure_drop: PressureDropResult,
    pub objective_value: f64,
    pub final_channel_count: usize,
    pub feasible_channel_count_max: usize,
    pub channel_width_bound_hits: usize,
    pub channel_height_bound_hits: usize,
    pub unmet_constraints: Vec<String>,
    pub optimizer_iterations: Vec<OptimizerIteration>,
    pub assumptions: Vec<ModelAssumption>,
    pub equation_traces: Vec<EquationTrace>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalStation {
    pub name: &'static str,
    pub axial_position_m: f64,
    pub heat_flux_w_m2: f64,
    pub htc_w_m2_k: f64,
    pub adiabatic_wall_temperature_k: f64,
    pub area_m2_estimate: f64,
    pub heat_load_w_estimate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalProfileSample {
    pub axial_position_m: f64,
    pub heat_flux_w_m2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThermalAssumptions {
    pub model: ThermalModel,
    pub cooling_mode: CoolingMode,
    pub reference_recovery_temperature_k: f64,
    pub wall_temperature_k: f64,
    pub reference_gas_side_htc_w_m2_k: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketThermalResult {
    pub chamber: ThermalStation,
    pub throat: ThermalStation,
    pub exit: ThermalStation,
    pub peak_heat_flux_w_m2: f64,
    pub peak_location: &'static str,
    pub total_heat_load_w_estimate: f64,
    pub profile_samples: Vec<ThermalProfileSample>,
    pub assumptions: ThermalAssumptions,
    pub notes: Vec<String>,
    pub design: ThermalDesignResult,
}

// Implementation follows in next patch chunk.
pub fn compute_thermal(problem: &RocketThermalProblem) -> Result<RocketThermalResult, RpaError> {
    validate(problem)?;
    let design = compute_thermal_design(problem)?;
    let (chamber, throat, exit, peak, peak_loc, total_q, samples) =
        map_design_to_legacy(problem, &design);
    Ok(RocketThermalResult {
        chamber,
        throat,
        exit,
        peak_heat_flux_w_m2: peak,
        peak_location: peak_loc,
        total_heat_load_w_estimate: total_q,
        profile_samples: samples,
        assumptions: ThermalAssumptions {
            model: problem.model,
            cooling_mode: problem.cooling_mode,
            reference_recovery_temperature_k: problem.reference_recovery_temperature_k,
            wall_temperature_k: problem.wall_temperature_k,
            reference_gas_side_htc_w_m2_k: problem.reference_gas_side_htc_w_m2_k,
        },
        notes: vec![
            "Thermal design uses station-by-station gas/wall/coolant coupling.".to_owned(),
            "Channel optimizer targets lower peak wall temperature under pressure-drop limit."
                .to_owned(),
        ],
        design,
    })
}

#[derive(Clone)]
struct ChannelStationGeometry {
    width_m: f64,
    height_m: f64,
}

struct DesignEvaluation {
    stations: Vec<ThermalDesignStation>,
    peak_wall_temperature_k: f64,
    peak_wall_temperature_station: usize,
    coolant_outlet_temperature_k: f64,
    pressure_drop: PressureDropResult,
    width_bound_hits: usize,
    height_bound_hits: usize,
}

pub fn compute_thermal_design(
    problem: &RocketThermalProblem,
) -> Result<ThermalDesignResult, RpaError> {
    validate(problem)?;
    let mut channels = initialize_channel_profiles(problem);
    let mut channel_count = problem.channel_geometry.channel_count.clamp(
        1,
        max_feasible_channel_count(problem, &channels)
            .min(problem.design_settings.max_channel_count),
    );
    let mut history = Vec::new();
    let mut best = evaluate_design(problem, &channels, channel_count)?;
    let mut best_obj = objective(
        best.peak_wall_temperature_k,
        problem.wall_model.allowable_temperature_k,
        best.pressure_drop.total_pressure_drop_pa,
        problem.design_settings.max_coolant_pressure_drop_pa,
        best.pressure_drop.min_coolant_pressure_pa,
        problem.coolant_model.inlet_pressure_pa,
    );

    for iteration in 0..problem.design_settings.optimizer_max_iterations {
        let mut candidate = channels.clone();
        let mut candidate_count = channel_count;
        let action;
        if best.pressure_drop.total_pressure_drop_pa
            > problem.design_settings.max_coolant_pressure_drop_pa
        {
            for g in &mut candidate {
                if !problem.design_settings.hold_channel_width_fixed {
                    g.width_m = (g.width_m
                        * (1.0 + problem.design_settings.optimizer_global_area_gain))
                        .clamp(
                            problem.channel_geometry.min_width_m,
                            problem.channel_geometry.max_width_m,
                        );
                }
                if !problem.design_settings.hold_channel_height_fixed {
                    g.height_m = (g.height_m
                        * (1.0 + problem.design_settings.optimizer_global_area_gain))
                        .clamp(
                            problem.channel_geometry.min_height_m,
                            problem.channel_geometry.max_height_m,
                        );
                }
            }
            if !problem.design_settings.hold_channel_count_fixed {
                candidate_count = (candidate_count + 1).clamp(
                    problem.design_settings.min_channel_count,
                    problem.design_settings.max_channel_count,
                );
            }
            action = format!("global area increase, count={candidate_count}");
        } else {
            let peak_i = best.peak_wall_temperature_station;
            let spread = ((candidate.len() as f64) * 0.08).max(2.0);
            for (i, g) in candidate.iter_mut().enumerate() {
                let dx = (i as f64 - peak_i as f64).abs() / spread;
                let influence = (-dx * dx).exp();
                let scale =
                    1.0 + problem.design_settings.optimizer_local_adjustment_gain * influence;
                if !problem.design_settings.hold_channel_width_fixed {
                    g.width_m = (g.width_m * scale).clamp(
                        problem.channel_geometry.min_width_m,
                        problem.channel_geometry.max_width_m,
                    );
                }
                if !problem.design_settings.hold_channel_height_fixed {
                    g.height_m = (g.height_m * scale).clamp(
                        problem.channel_geometry.min_height_m,
                        problem.channel_geometry.max_height_m,
                    );
                }
            }
            if !problem.design_settings.hold_channel_count_fixed
                && best.peak_wall_temperature_k > problem.wall_model.allowable_temperature_k
            {
                candidate_count = (candidate_count + 1).clamp(
                    problem.design_settings.min_channel_count,
                    problem.design_settings.max_channel_count,
                );
            }
            action = format!(
                "local expansion near station {}, count={candidate_count}",
                peak_i
            );
        }
        let feasible_max = max_feasible_channel_count(problem, &candidate)
            .min(problem.design_settings.max_channel_count);
        let bounded_min = problem
            .design_settings
            .min_channel_count
            .min(feasible_max)
            .max(1);
        candidate_count = candidate_count.clamp(bounded_min, feasible_max.max(1));
        let eval = evaluate_design(problem, &candidate, candidate_count)?;
        let obj = objective(
            eval.peak_wall_temperature_k,
            problem.wall_model.allowable_temperature_k,
            eval.pressure_drop.total_pressure_drop_pa,
            problem.design_settings.max_coolant_pressure_drop_pa,
            eval.pressure_drop.min_coolant_pressure_pa,
            problem.coolant_model.inlet_pressure_pa,
        );
        let accepted = obj <= best_obj;
        history.push(OptimizerIteration {
            iteration,
            peak_wall_temperature_k: eval.peak_wall_temperature_k,
            pressure_drop_pa: eval.pressure_drop.total_pressure_drop_pa,
            min_coolant_pressure_pa: eval.pressure_drop.min_coolant_pressure_pa,
            channel_count: candidate_count,
            width_bound_hits: eval.width_bound_hits,
            height_bound_hits: eval.height_bound_hits,
            objective_value: obj,
            accepted,
            action,
        });
        if accepted {
            channels = candidate;
            channel_count = candidate_count;
            best = eval;
            best_obj = obj;
        }
    }

    let final_channel_count = channel_count;
    let feasible_channel_count_max = max_feasible_channel_count(problem, &channels);
    let mut unmet_constraints = Vec::new();
    if !best.pressure_drop.within_limit {
        unmet_constraints.push(format!(
            "coolant pressure drop exceeds limit ({:.0} Pa > {:.0} Pa)",
            best.pressure_drop.total_pressure_drop_pa, best.pressure_drop.limit_pressure_drop_pa
        ));
    }
    if best.pressure_drop.min_coolant_pressure_pa < 0.0 {
        unmet_constraints.push(format!(
            "coolant static pressure became negative (min {:.0} Pa)",
            best.pressure_drop.min_coolant_pressure_pa
        ));
    }
    if best.peak_wall_temperature_k > problem.wall_model.allowable_temperature_k {
        unmet_constraints.push(format!(
            "peak wall temperature exceeds allowable ({:.1} K > {:.1} K)",
            best.peak_wall_temperature_k, problem.wall_model.allowable_temperature_k
        ));
    }
    if final_channel_count > feasible_channel_count_max {
        unmet_constraints.push(format!(
            "final channel count {} exceeds feasible maximum {} for current min gap",
            final_channel_count, feasible_channel_count_max
        ));
    }

    let mut notes = vec![
        "1D station thermal model with explicit simplifications.".to_owned(),
        "Film model is effectiveness-based and not injector-resolved.".to_owned(),
    ];
    for c in &unmet_constraints {
        if c.contains("pressure drop") {
            notes.push("Hint: increase channel hydraulic area (width/height/count), reduce coolant mass flow, or raise pressure-drop limit.".to_owned());
        } else if c.contains("negative") {
            notes.push("Hint: increase coolant inlet pressure or reduce hydraulic losses (larger channels / fewer restrictions).".to_owned());
        } else if c.contains("temperature exceeds allowable") {
            notes.push("Hint: increase cooling effectiveness (more flow/area, stronger film, lower gas-side heat load assumptions).".to_owned());
        } else if c.contains("channel count") {
            notes.push("Hint: reduce min gap or channel width, or adjust geometry to increase local available circumference.".to_owned());
        }
    }

    Ok(ThermalDesignResult {
        stations: best.stations,
        peak_wall_temperature_k: best.peak_wall_temperature_k,
        peak_wall_temperature_station: best.peak_wall_temperature_station,
        coolant_outlet_temperature_k: best.coolant_outlet_temperature_k,
        pressure_drop: best.pressure_drop,
        objective_value: best_obj,
        final_channel_count,
        feasible_channel_count_max,
        channel_width_bound_hits: best.width_bound_hits,
        channel_height_bound_hits: best.height_bound_hits,
        unmet_constraints,
        optimizer_iterations: history,
        assumptions: build_assumptions(problem),
        equation_traces: build_equation_trace(),
        notes,
    })
}

fn initialize_channel_profiles(problem: &RocketThermalProblem) -> Vec<ChannelStationGeometry> {
    let n = problem.design_settings.station_count.max(5);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let f = i as f64 / (n - 1) as f64;
        out.push(ChannelStationGeometry {
            width_m: problem.channel_geometry.width_m
                * (1.0 + f * (problem.channel_geometry.width_taper_end_factor - 1.0)),
            height_m: problem.channel_geometry.height_m
                * (1.0 + f * (problem.channel_geometry.height_taper_end_factor - 1.0)),
        });
    }
    out
}

fn evaluate_design(
    problem: &RocketThermalProblem,
    channels: &[ChannelStationGeometry],
    channel_count: usize,
) -> Result<DesignEvaluation, RpaError> {
    let total_l = problem.geometry_result.chamber_length_m_estimate
        + problem.geometry_result.canonical_model.converging_length_m
        + problem.geometry_result.nozzle_length_m_estimate;
    let n = channels.len();
    let dx = total_l / (n.saturating_sub(1)) as f64;

    check_channel_feasibility(problem, channels, channel_count)?;

    let mut coolant_t = vec![problem.coolant_model.inlet_temperature_k; n];
    let mut coolant_p = vec![problem.coolant_model.inlet_pressure_pa; n];
    let mut local_dp = vec![0.0; n];
    let mut coolant_h = vec![0.0; n];
    let mut q_vec = vec![0.0; n];
    let mut t_hot_vec = vec![problem.wall_temperature_k; n];
    let mut t_cold_vec = vec![problem.wall_temperature_k; n];

    let mut peak_t = f64::NEG_INFINITY;
    let mut peak_i = 0usize;
    let throat_x = problem.geometry_result.canonical_model.throat_axial_m;
    let throat_span = (0.18 * total_l).max(1.0e-4);
    let mut stations = Vec::with_capacity(n);
    let mut width_bound_hits = 0usize;
    let mut height_bound_hits = 0usize;

    let regen_active = matches!(
        problem.cooling_mode,
        CoolingMode::Regenerative | CoolingMode::RegenerativeFilm
    );

    let mut taw_eff_vec = vec![problem.reference_recovery_temperature_k; n];
    let mut h_g_vec = vec![problem.reference_gas_side_htc_w_m2_k; n];
    let mut eta_film_vec = vec![0.0; n];

    for (i, _g) in channels.iter().enumerate() {
        let x = i as f64 * dx;
        let xf = if total_l > 0.0 { x / total_l } else { 0.0 };
        let throat_factor = (-(((x - throat_x).abs() / throat_span).powi(2))).exp();
        let taw = problem.reference_recovery_temperature_k * (0.80 + 0.20 * throat_factor);
        let h_g = problem.reference_gas_side_htc_w_m2_k * (0.55 + 0.45 * throat_factor);
        let eta_film = film_effectiveness(problem, xf);
        taw_eff_vec[i] = taw - eta_film * (taw - problem.coolant_model.inlet_temperature_k);
        h_g_vec[i] = h_g;
        eta_film_vec[i] = eta_film;
    }

    let mut evaluate_path = |indices: &[usize], mass_flow_kg_s: f64| -> (f64, f64) {
        let mut t = problem.coolant_model.inlet_temperature_k;
        let mut p = problem.coolant_model.inlet_pressure_pa;
        let mut branch_dp = 0.0;
        let count = channel_count as f64;
        let mut prev_hot = problem.wall_temperature_k;

        for &i in indices {
            let g = &channels[i];
            let area_total = (g.width_m * g.height_m * count).max(1.0e-12);
            let perimeter = 2.0 * (g.width_m + g.height_m);
            let dh = (4.0 * g.width_m * g.height_m / perimeter).max(1.0e-8);
            let vel = mass_flow_kg_s / (problem.coolant_model.density_kg_m3 * area_total);
            let re = (problem.coolant_model.density_kg_m3 * vel * dh
                / problem.coolant_model.viscosity_pa_s)
                .max(1.0);
            let pr = (problem.coolant_model.cp_j_kg_k * problem.coolant_model.viscosity_pa_s
                / problem.coolant_model.thermal_conductivity_w_m_k)
                .max(0.01);

            let (h_c, q, t_hot, t_cold, dp) = if regen_active {
                let nu = if re < 2300.0 {
                    3.66
                } else {
                    0.023 * re.powf(0.8) * pr.powf(0.4)
                };
                let h_c = (nu * problem.coolant_model.thermal_conductivity_w_m_k / dh).max(20.0);
                let eps = problem.wall_model.gas_side_emissivity.clamp(0.0, 1.0);
                let sigma = 5.670_374_419e-8_f64;
                let h_rad = eps
                    * sigma
                    * (taw_eff_vec[i] + prev_hot)
                    * (taw_eff_vec[i] * taw_eff_vec[i] + prev_hot * prev_hot);
                let h_g_total = h_g_vec[i] + h_rad;
                let wall_mid_t = 0.5 * (taw_eff_vec[i] + t);
                let k_wall = effective_wall_conductivity(problem, wall_mid_t);
                let r_total = 1.0 / h_g_total + problem.wall_model.thickness_m / k_wall + 1.0 / h_c;
                let q = ((taw_eff_vec[i] - t) / r_total).max(0.0);
                let t_hot = taw_eff_vec[i] - q / h_g_total;
                let t_cold = t_hot - q * problem.wall_model.thickness_m / k_wall;
                let wall_area = count * g.width_m * dx;
                t += q * wall_area / (mass_flow_kg_s * problem.coolant_model.cp_j_kg_k);
                let f = if re < 2300.0 {
                    64.0 / re
                } else {
                    0.3164 / re.powf(0.25)
                };
                let dp = f * (dx / dh) * 0.5 * problem.coolant_model.density_kg_m3 * vel * vel;
                p -= dp;
                (h_c, q, t_hot, t_cold, dp)
            } else {
                (0.0, 0.0, taw_eff_vec[i], taw_eff_vec[i], 0.0)
            };
            branch_dp += dp;
            prev_hot = t_hot;
            coolant_t[i] = t;
            coolant_p[i] = p;
            local_dp[i] = dp;
            coolant_h[i] = h_c;
            q_vec[i] = q;
            t_hot_vec[i] = t_hot;
            t_cold_vec[i] = t_cold;
        }
        (t, branch_dp)
    };

    let (coolant_outlet_temperature_k, total_pressure_drop_pa, coolant_outlet_pressure_pa) =
        if regen_active {
            match problem.design_settings.coolant_flow_direction {
                CoolantFlowDirection::CoFlow => {
                    let order: Vec<usize> = (0..n).collect();
                    let (t_out, dp) = evaluate_path(&order, problem.coolant_model.mass_flow_kg_s);
                    (t_out, dp, coolant_p[n - 1])
                }
                CoolantFlowDirection::CounterFlow => {
                    let order: Vec<usize> = (0..n).rev().collect();
                    let (t_out, dp) = evaluate_path(&order, problem.coolant_model.mass_flow_kg_s);
                    (t_out, dp, coolant_p[0])
                }
                CoolantFlowDirection::MidFeed => {
                    let mid = ((problem.design_settings.mid_feed_fraction.clamp(0.0, 1.0)
                        * (n.saturating_sub(1) as f64))
                        .round()) as usize;
                    let up_order: Vec<usize> = (0..=mid).rev().collect();
                    let down_order: Vec<usize> = (mid..n).collect();

                    let split = if problem.design_settings.auto_balance_mid_feed_split {
                        let r_up = hydraulic_branch_resistance(
                            problem,
                            channels,
                            channel_count,
                            &up_order,
                            dx,
                        )
                        .max(1.0e-12);
                        let r_down = hydraulic_branch_resistance(
                            problem,
                            channels,
                            channel_count,
                            &down_order,
                            dx,
                        )
                        .max(1.0e-12);
                        let w_up = 1.0 / r_up.sqrt();
                        w_up / (w_up + 1.0 / r_down.sqrt())
                    } else {
                        problem
                            .design_settings
                            .mid_feed_upstream_mass_fraction
                            .clamp(0.0, 1.0)
                    };
                    let m_up = problem.coolant_model.mass_flow_kg_s * split;
                    let m_down = problem.coolant_model.mass_flow_kg_s - m_up;
                    let (t_up, dp_up) = evaluate_path(&up_order, m_up.max(1.0e-9));
                    let (t_down, dp_down) = evaluate_path(&down_order, m_down.max(1.0e-9));
                    let t_out = split * t_up + (1.0 - split) * t_down;
                    let p_out = coolant_p[0].min(coolant_p[n - 1]);
                    (t_out, dp_up.max(dp_down), p_out)
                }
            }
        } else {
            for i in 0..n {
                coolant_t[i] = problem.coolant_model.inlet_temperature_k;
                coolant_p[i] = problem.coolant_model.inlet_pressure_pa;
                local_dp[i] = 0.0;
                coolant_h[i] = 0.0;
                q_vec[i] = 0.0;
                t_hot_vec[i] = taw_eff_vec[i];
                t_cold_vec[i] = taw_eff_vec[i];
            }
            (
                problem.coolant_model.inlet_temperature_k,
                0.0,
                problem.coolant_model.inlet_pressure_pa,
            )
        };

    for (i, g) in channels.iter().enumerate() {
        let x = i as f64 * dx;
        let xf = if total_l > 0.0 { x / total_l } else { 0.0 };
        if (g.width_m - problem.channel_geometry.min_width_m).abs() <= 1.0e-12
            || (g.width_m - problem.channel_geometry.max_width_m).abs() <= 1.0e-12
        {
            width_bound_hits += 1;
        }
        if (g.height_m - problem.channel_geometry.min_height_m).abs() <= 1.0e-12
            || (g.height_m - problem.channel_geometry.max_height_m).abs() <= 1.0e-12
        {
            height_bound_hits += 1;
        }
        if t_hot_vec[i] > peak_t {
            peak_t = t_hot_vec[i];
            peak_i = i;
        }
        stations.push(ThermalDesignStation {
            station_index: i,
            axial_position_m: x,
            x_fraction: xf,
            gas_side_htc_w_m2_k: h_g_vec[i],
            adiabatic_wall_temperature_k: taw_eff_vec[i],
            film_effectiveness: eta_film_vec[i],
            net_gas_heat_flux_w_m2: q_vec[i],
            coolant_side_htc_w_m2_k: coolant_h[i],
            wall_hot_side_temperature_k: t_hot_vec[i],
            wall_cold_side_temperature_k: t_cold_vec[i],
            coolant_bulk_temperature_k: coolant_t[i],
            coolant_pressure_pa: coolant_p[i],
            local_pressure_drop_pa: local_dp[i],
            channel_width_m: g.width_m,
            channel_height_m: g.height_m,
            channel_count,
        });
    }

    Ok(DesignEvaluation {
        stations,
        peak_wall_temperature_k: peak_t,
        peak_wall_temperature_station: peak_i,
        coolant_outlet_temperature_k,
        pressure_drop: PressureDropResult {
            total_pressure_drop_pa,
            coolant_inlet_pressure_pa: problem.coolant_model.inlet_pressure_pa,
            coolant_outlet_pressure_pa,
            min_coolant_pressure_pa: coolant_p.iter().copied().fold(f64::INFINITY, f64::min),
            limit_pressure_drop_pa: problem.design_settings.max_coolant_pressure_drop_pa,
            within_limit: total_pressure_drop_pa
                <= problem.design_settings.max_coolant_pressure_drop_pa,
        },
        width_bound_hits,
        height_bound_hits,
    })
}

fn station_radius_m(problem: &RocketThermalProblem, x: f64) -> f64 {
    let contour = &problem.geometry_result.canonical_model.wall_contour_upper;
    if contour.is_empty() {
        return 0.5 * problem.geometry_result.throat_diameter_m;
    }
    if x <= contour[0][0] {
        return contour[0][1].max(1.0e-8);
    }
    for window in contour.windows(2) {
        let a = window[0];
        let b = window[1];
        if x <= b[0] {
            let dx = (b[0] - a[0]).max(1.0e-12);
            let t = ((x - a[0]) / dx).clamp(0.0, 1.0);
            return (a[1] + t * (b[1] - a[1])).max(1.0e-8);
        }
    }
    contour
        .last()
        .map(|p| p[1].max(1.0e-8))
        .unwrap_or(0.5 * problem.geometry_result.throat_diameter_m)
}

fn max_feasible_channel_count(
    problem: &RocketThermalProblem,
    channels: &[ChannelStationGeometry],
) -> usize {
    let n = channels.len().max(2);
    let total_l = problem
        .geometry_result
        .canonical_model
        .exit_axial_m
        .max(1.0e-9);
    let dx = total_l / (n - 1) as f64;
    let mut max_count = usize::MAX;
    for (i, g) in channels.iter().enumerate() {
        let x = i as f64 * dx;
        let r = station_radius_m(problem, x);
        let circumference = 2.0 * std::f64::consts::PI * r;
        let pitch_min = (g.width_m + problem.channel_geometry.min_gap_m).max(1.0e-9);
        let local_max = (circumference / pitch_min).floor() as usize;
        max_count = max_count.min(local_max.max(1));
    }
    max_count
        .min(problem.design_settings.max_channel_count)
        .max(1)
}

fn check_channel_feasibility(
    problem: &RocketThermalProblem,
    channels: &[ChannelStationGeometry],
    channel_count: usize,
) -> Result<(), RpaError> {
    let feasible_max = max_feasible_channel_count(problem, channels);
    if channel_count > feasible_max {
        return Err(RpaError::InvalidInput(format!(
            "channel_count={} is not feasible with min gap {:.6} m; maximum feasible count is {}",
            channel_count, problem.channel_geometry.min_gap_m, feasible_max
        )));
    }
    Ok(())
}

fn hydraulic_branch_resistance(
    problem: &RocketThermalProblem,
    channels: &[ChannelStationGeometry],
    channel_count: usize,
    indices: &[usize],
    dx: f64,
) -> f64 {
    if indices.is_empty() {
        return 1.0e-9;
    }
    let mut r_total = 0.0;
    let count = channel_count as f64;
    let rho = problem.coolant_model.density_kg_m3;
    let mu = problem.coolant_model.viscosity_pa_s;
    for &i in indices {
        let g = &channels[i];
        let area_total = (g.width_m * g.height_m * count).max(1.0e-12);
        let perimeter = 2.0 * (g.width_m + g.height_m);
        let dh = (4.0 * g.width_m * g.height_m / perimeter).max(1.0e-8);
        // Linearized branch resistance around unit mass-flow for split allocation.
        let vel = 1.0 / (rho * area_total);
        let re = (rho * vel * dh / mu).max(1.0);
        let f = if re < 2300.0 {
            64.0 / re
        } else {
            0.3164 / re.powf(0.25)
        };
        let r_i = f * (dx / dh) * 0.5 * rho * vel * vel;
        r_total += r_i.max(1.0e-12);
    }
    r_total
}

fn film_effectiveness(problem: &RocketThermalProblem, xf: f64) -> f64 {
    if !matches!(
        problem.cooling_mode,
        CoolingMode::Film | CoolingMode::RegenerativeFilm
    ) {
        return 0.0;
    }
    let start = problem
        .film_model
        .effectiveness_start_fraction
        .clamp(0.0, 1.0);
    let end = problem
        .film_model
        .effectiveness_end_fraction
        .clamp(start + 1.0e-6, 1.0);
    if xf < start || xf > end {
        return 0.0;
    }
    let f = (xf - start) / (end - start);
    let shape = (1.0 - f).powf(0.5);
    (problem.film_model.max_effectiveness * problem.film_model.film_mass_fraction / 0.05 * shape)
        .clamp(0.0, problem.film_model.max_effectiveness)
}

fn objective(
    peak_t: f64,
    allowable_t: f64,
    dp: f64,
    max_dp: f64,
    min_pressure_pa: f64,
    inlet_pressure_pa: f64,
) -> f64 {
    let mut penalty = if dp > max_dp {
        4_000.0 * ((dp - max_dp) / max_dp.max(1.0))
    } else {
        0.0
    };
    if peak_t > allowable_t {
        penalty += 4_000.0 * ((peak_t - allowable_t) / allowable_t.max(1.0));
    }
    if min_pressure_pa < 0.0 {
        penalty += 8_000.0 * ((-min_pressure_pa) / inlet_pressure_pa.max(1.0));
    }
    peak_t + penalty
}

#[allow(clippy::type_complexity)]
fn map_design_to_legacy(
    problem: &RocketThermalProblem,
    design: &ThermalDesignResult,
) -> (
    ThermalStation,
    ThermalStation,
    ThermalStation,
    f64,
    &'static str,
    f64,
    Vec<ThermalProfileSample>,
) {
    let station_at = |target_x: f64| -> &ThermalDesignStation {
        let mut best = &design.stations[0];
        let mut best_err = (best.axial_position_m - target_x).abs();
        for s in &design.stations {
            let err = (s.axial_position_m - target_x).abs();
            if err < best_err {
                best_err = err;
                best = s;
            }
        }
        best
    };
    let chamber_s = station_at(0.0);
    let throat_s = station_at(problem.geometry_result.canonical_model.throat_axial_m);
    let exit_s = station_at(problem.geometry_result.canonical_model.exit_axial_m);

    let chamber_area = std::f64::consts::PI
        * problem.geometry_result.chamber_diameter_m_estimate
        * problem.geometry_result.chamber_length_m_estimate;
    let throat_area = std::f64::consts::PI
        * problem.geometry_result.throat_diameter_m
        * (0.08 * problem.geometry_result.throat_diameter_m).max(0.005);
    let nozzle_area = std::f64::consts::PI
        * 0.5
        * (problem.geometry_result.throat_diameter_m + problem.geometry_result.exit_diameter_m)
        * problem.geometry_result.nozzle_length_m_estimate;

    let chamber = ThermalStation {
        name: "Chamber",
        axial_position_m: chamber_s.axial_position_m,
        heat_flux_w_m2: chamber_s.net_gas_heat_flux_w_m2,
        htc_w_m2_k: chamber_s.gas_side_htc_w_m2_k,
        adiabatic_wall_temperature_k: chamber_s.adiabatic_wall_temperature_k,
        area_m2_estimate: chamber_area,
        heat_load_w_estimate: chamber_s.net_gas_heat_flux_w_m2 * chamber_area,
    };
    let throat = ThermalStation {
        name: "Throat",
        axial_position_m: throat_s.axial_position_m,
        heat_flux_w_m2: throat_s.net_gas_heat_flux_w_m2,
        htc_w_m2_k: throat_s.gas_side_htc_w_m2_k,
        adiabatic_wall_temperature_k: throat_s.adiabatic_wall_temperature_k,
        area_m2_estimate: throat_area,
        heat_load_w_estimate: throat_s.net_gas_heat_flux_w_m2 * throat_area,
    };
    let exit = ThermalStation {
        name: "Exit",
        axial_position_m: exit_s.axial_position_m,
        heat_flux_w_m2: exit_s.net_gas_heat_flux_w_m2,
        htc_w_m2_k: exit_s.gas_side_htc_w_m2_k,
        adiabatic_wall_temperature_k: exit_s.adiabatic_wall_temperature_k,
        area_m2_estimate: nozzle_area,
        heat_load_w_estimate: exit_s.net_gas_heat_flux_w_m2 * nozzle_area,
    };

    let (peak_location, peak_heat_flux) = if throat.heat_flux_w_m2 >= chamber.heat_flux_w_m2
        && throat.heat_flux_w_m2 >= exit.heat_flux_w_m2
    {
        ("Throat", throat.heat_flux_w_m2)
    } else if chamber.heat_flux_w_m2 >= exit.heat_flux_w_m2 {
        ("Chamber", chamber.heat_flux_w_m2)
    } else {
        ("Exit", exit.heat_flux_w_m2)
    };
    let total_heat =
        chamber.heat_load_w_estimate + throat.heat_load_w_estimate + exit.heat_load_w_estimate;
    let samples = design
        .stations
        .iter()
        .map(|s| ThermalProfileSample {
            axial_position_m: s.axial_position_m,
            heat_flux_w_m2: s.net_gas_heat_flux_w_m2,
        })
        .collect();
    (
        chamber,
        throat,
        exit,
        peak_heat_flux,
        peak_location,
        total_heat,
        samples,
    )
}

fn build_assumptions(problem: &RocketThermalProblem) -> Vec<ModelAssumption> {
    vec![
        ModelAssumption {
            id: "gas-side".to_owned(),
            category: "Gas-side HTC".to_owned(),
            statement: "Gas-side HTC uses throat-weighted Bartz-like scaling.".to_owned(),
            impact: "First-order only; detailed boundary-layer effects are deferred.".to_owned(),
        },
        ModelAssumption {
            id: "wall-1d".to_owned(),
            category: "Wall conduction".to_owned(),
            statement:
                "Wall conduction uses a 1D through-thickness resistance with temperature-adjusted k(T)."
                    .to_owned(),
            impact: "No circumferential gradients or local stress/strain coupling.".to_owned(),
        },
        ModelAssumption {
            id: "film".to_owned(),
            category: "Film cooling".to_owned(),
            statement: "Film cooling modifies adiabatic-wall temperature via effectiveness."
                .to_owned(),
            impact: "Injection/mixing fidelity is intentionally simplified.".to_owned(),
        },
        ModelAssumption {
            id: "radiation".to_owned(),
            category: "Gas-side radiation".to_owned(),
            statement: format!(
                "Gas-side radiation is modeled as an effective h_rad with emissivity {:.2}.",
                problem.wall_model.gas_side_emissivity
            ),
            impact: "Gray-gas approximation; wavelength and species-radiation details are deferred."
                .to_owned(),
        },
        ModelAssumption {
            id: "coolant".to_owned(),
            category: "Coolant properties".to_owned(),
            statement: format!(
                "Coolant '{}' uses bulk-constant rho, mu, k, cp inputs.",
                problem.coolant_model.coolant_name
            ),
            impact: "Property variation with temperature and pressure is not captured.".to_owned(),
        },
    ]
}

fn effective_wall_conductivity(problem: &RocketThermalProblem, wall_temperature_k: f64) -> f64 {
    let k_ref = problem.wall_model.thermal_conductivity_w_m_k;
    let t_ref = problem
        .wall_model
        .thermal_conductivity_reference_temperature_k;
    let alpha = problem.wall_model.thermal_conductivity_temp_coeff_per_k;
    let factor = 1.0 + alpha * (wall_temperature_k - t_ref);
    (k_ref * factor).max(1.0)
}

fn build_equation_trace() -> Vec<EquationTrace> {
    vec![
        EquationTrace {
            name: "Coolant Reynolds".to_owned(),
            equation: "Re = rho*u*Dh/mu".to_owned(),
            source: EquationSource::Derived,
            notes: "Per-station channel flow state.".to_owned(),
        },
        EquationTrace {
            name: "Coolant Nusselt".to_owned(),
            equation: "Nu = 0.023 Re^0.8 Pr^0.4 (turb) ; Nu=3.66 (lam)".to_owned(),
            source: EquationSource::Correlation,
            notes: "Dittus-Boelter style approximation.".to_owned(),
        },
        EquationTrace {
            name: "Thermal circuit".to_owned(),
            equation: "q''=(Taw_eff-Tcool)/(1/hg+t/k+1/hc)".to_owned(),
            source: EquationSource::Derived,
            notes: "Gas convection + wall conduction + coolant convection.".to_owned(),
        },
        EquationTrace {
            name: "Gas-side radiation".to_owned(),
            equation: "h_rad=eps*sigma*(Taw+Tw)*(Taw^2+Tw^2)".to_owned(),
            source: EquationSource::Derived,
            notes: "Linearized radiative exchange added to gas-side h.".to_owned(),
        },
        EquationTrace {
            name: "Pressure drop".to_owned(),
            equation: "dP=f*(dx/Dh)*(rho*u^2/2)".to_owned(),
            source: EquationSource::Correlation,
            notes: "Darcy friction per station.".to_owned(),
        },
        EquationTrace {
            name: "Design objective".to_owned(),
            equation: "min max(Twall_hot), constrain dP<=dPmax".to_owned(),
            source: EquationSource::Constraint,
            notes: "Penalty-based constraint handling.".to_owned(),
        },
    ]
}

fn validate(problem: &RocketThermalProblem) -> Result<(), RpaError> {
    let checks = [
        (
            problem.reference_recovery_temperature_k,
            "reference recovery temperature",
        ),
        (problem.wall_temperature_k, "wall temperature"),
        (problem.reference_gas_side_htc_w_m2_k, "gas-side htc"),
        (
            problem.wall_model.thermal_conductivity_w_m_k,
            "wall thermal conductivity",
        ),
        (
            problem.wall_model.allowable_temperature_k,
            "wall allowable temperature",
        ),
        (problem.wall_model.thickness_m, "wall thickness"),
        (
            problem.wall_model.gas_side_emissivity,
            "gas-side emissivity",
        ),
        (problem.coolant_model.mass_flow_kg_s, "coolant mass flow"),
        (problem.coolant_model.density_kg_m3, "coolant density"),
        (problem.coolant_model.viscosity_pa_s, "coolant viscosity"),
        (
            problem.coolant_model.thermal_conductivity_w_m_k,
            "coolant conductivity",
        ),
        (problem.coolant_model.cp_j_kg_k, "coolant cp"),
        (
            problem.design_settings.max_coolant_pressure_drop_pa,
            "max coolant pressure drop",
        ),
    ];
    for (v, name) in checks {
        if !v.is_finite() || v <= 0.0 {
            return Err(RpaError::InvalidInput(format!(
                "{name} must be finite and > 0"
            )));
        }
    }
    if problem.design_settings.station_count < 5 {
        return Err(RpaError::InvalidInput(
            "station_count must be >= 5".to_owned(),
        ));
    }
    if problem.channel_geometry.channel_count == 0 {
        return Err(RpaError::InvalidInput(
            "channel_count must be > 0".to_owned(),
        ));
    }
    if problem.wall_temperature_k >= problem.reference_recovery_temperature_k {
        return Err(RpaError::InvalidInput(
            "wall temperature must be below recovery temperature".to_owned(),
        ));
    }
    if problem.wall_model.gas_side_emissivity > 1.0 {
        return Err(RpaError::InvalidInput(
            "gas-side emissivity must be <= 1".to_owned(),
        ));
    }
    if problem.channel_geometry.min_gap_m <= 0.0 || !problem.channel_geometry.min_gap_m.is_finite()
    {
        return Err(RpaError::InvalidInput(
            "channel min gap must be finite and > 0".to_owned(),
        ));
    }
    if problem.design_settings.min_channel_count == 0
        || problem.design_settings.max_channel_count < problem.design_settings.min_channel_count
    {
        return Err(RpaError::InvalidInput(
            "channel count bounds are invalid".to_owned(),
        ));
    }
    if !problem.design_settings.mid_feed_fraction.is_finite()
        || !(0.0..=1.0).contains(&problem.design_settings.mid_feed_fraction)
    {
        return Err(RpaError::InvalidInput(
            "mid-feed fraction must be within [0, 1]".to_owned(),
        ));
    }
    if !problem
        .design_settings
        .mid_feed_upstream_mass_fraction
        .is_finite()
        || !(0.0..=1.0).contains(&problem.design_settings.mid_feed_upstream_mass_fraction)
    {
        return Err(RpaError::InvalidInput(
            "mid-feed upstream split must be within [0, 1]".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tf_cea::Reactant;

    use crate::{
        CombustorModel, GeometrySizingMode, NozzleChemistryModel, NozzleConstraint,
        NozzleContourStyle, compute_geometry,
    };

    use super::*;

    fn make_problem() -> RocketThermalProblem {
        let perf = RocketAnalysisProblem {
            oxidizer: Reactant {
                name: "O2".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(90.0),
            },
            fuel: Reactant {
                name: "RP-1".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(293.0),
            },
            chamber_pressure_pa: 7.0e6,
            mixture_ratio_oxidizer_to_fuel: 2.6,
            nozzle_constraint: NozzleConstraint::ExpansionRatio(40.0),
            combustor_model: CombustorModel::InfiniteArea,
            nozzle_chemistry_model: NozzleChemistryModel::ShiftingEquilibrium,
            ambient_pressure_pa: 101_325.0,
        };
        let geom_problem = RocketGeometryProblem {
            base_problem: perf.clone(),
            sizing_mode: GeometrySizingMode::GivenThroatDiameter,
            throat_input_value: 0.1,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_contour_style: NozzleContourStyle::BellParabolic,
            nozzle_truncation_ratio: 0.95,
        };
        let geom_result = compute_geometry(&geom_problem).expect("geometry");
        RocketThermalProblem {
            performance_problem: perf,
            geometry_problem: geom_problem,
            geometry_result: geom_result,
            model: ThermalModel::BartzLikeConvective,
            cooling_mode: CoolingMode::RegenerativeFilm,
            reference_recovery_temperature_k: 3400.0,
            wall_temperature_k: 900.0,
            reference_gas_side_htc_w_m2_k: 25_000.0,
            wall_model: WallModel::default(),
            coolant_model: CoolantModel::default(),
            film_model: FilmCoolingModel::default(),
            channel_geometry: ChannelGeometry::default(),
            design_settings: ThermalDesignSettings::default(),
        }
    }

    #[test]
    fn computes_design_outputs() {
        let result = compute_thermal(&make_problem()).expect("thermal");
        assert!(!result.design.stations.is_empty());
        assert!(result.design.peak_wall_temperature_k > 0.0);
        assert!(result.design.pressure_drop.total_pressure_drop_pa > 0.0);
    }

    #[test]
    fn tighter_pressure_drop_limit_reduces_dp() {
        let mut loose = make_problem();
        loose.design_settings.max_coolant_pressure_drop_pa = 2_500_000.0;
        let a = compute_thermal_design(&loose).expect("loose");
        let mut tight = make_problem();
        tight.design_settings.max_coolant_pressure_drop_pa = 600_000.0;
        let b = compute_thermal_design(&tight).expect("tight");
        assert!(b.pressure_drop.total_pressure_drop_pa <= a.pressure_drop.total_pressure_drop_pa);
    }

    #[test]
    fn adiabatic_mode_runs_hotter_than_regenerative() {
        let mut adiabatic = make_problem();
        adiabatic.cooling_mode = CoolingMode::AdiabaticWall;
        let hot = compute_thermal_design(&adiabatic).expect("adiabatic");

        let mut regen = make_problem();
        regen.cooling_mode = CoolingMode::Regenerative;
        let cool = compute_thermal_design(&regen).expect("regenerative");

        assert!(hot.peak_wall_temperature_k > cool.peak_wall_temperature_k);
        assert_eq!(hot.pressure_drop.total_pressure_drop_pa, 0.0);
    }
}
