use crate::input_helper::UnitAwareInput;
use tf_project::schema::{
    RocketChannelGeometryDef, RocketCombustorModelDef, RocketCoolantFlowDirectionDef,
    RocketCoolantModelDef, RocketFilmCoolingDef, RocketGeometryDef, RocketGeometrySizingModeDef,
    RocketNozzleChemistryModelDef, RocketNozzleConstraintDef, RocketNozzleContourStyleDef,
    RocketPerformanceCaseDef, RocketPropellantWorkspaceDef, RocketStudyDef, RocketStudyMetricDef,
    RocketStudyVariableDef, RocketThermalDef, RocketThermalDesignSettingsDef,
    RocketThermalMaterialDef, RocketThermalModelDef, RocketThermalWorkspaceCoolingModeDef,
    RocketWallModelDef, RocketWorkspaceDef,
};
use tf_rpa::{
    ChannelGeometry, CombustorModel, CoolantFlowDirection, CoolantModel, CoolingMode,
    FilmCoolingModel, GeometrySizingMode, NozzleChemistryModel, NozzleConstraint,
    NozzleContourStyle, RocketAnalysisProblem, RocketAnalysisResult, RocketGeometryProblem,
    RocketGeometryResult, RocketThermalProblem, RocketThermalResult, StateSummary,
    StudyOutputMetric, StudyVariable, ThermalDesignSettings, ThermalModel, WallModel,
};

// Supported CEA species from NASA thermo.lib with common names
// These species are in the standard CEA thermodynamic database
pub const CEA_OXIDIZERS: &[&str] = &["O2", "F2", "N2O", "H2O2", "N2O4"];
pub const CEA_FUELS: &[&str] = &["H2", "CH4", "C2H6", "C3H8", "NH3", "N2H4", "RP-1"];

fn canonical_species_for_cea(name: &str) -> String {
    let key = name.trim().to_ascii_uppercase();
    match key.as_str() {
        // Common oxidizer aliases
        "LOX" | "LIQUID OXYGEN" | "OXYGEN" => "O2".to_owned(),
        "NTO" => "N2O4".to_owned(),
        "HTP" => "H2O2".to_owned(),
        // Common fuel aliases
        "LH2" | "GH2" | "HYDROGEN" => "H2".to_owned(),
        "LCH4" | "METHANE" => "CH4".to_owned(),
        "ETHANE" => "C2H6".to_owned(),
        "PROPANE" => "C3H8".to_owned(),
        "AMMONIA" => "NH3".to_owned(),
        "HYDRAZINE" => "N2H4".to_owned(),
        "RP1" => "RP-1".to_owned(),
        // Practical approximations for presets not in the current CEA species list
        "IPA" | "ISOPROPANOL" => "C3H8".to_owned(),
        "ETHANOL" => "C2H6".to_owned(),
        "MMH" | "UDMH" => "N2H4".to_owned(),
        _ => name.trim().to_owned(),
    }
}

/// Get human-readable name for a species chemical formula
pub fn species_display_name(formula: &str) -> &str {
    match formula {
        // Fuels
        "H2" => "Hydrogen",
        "CH4" => "Methane",
        "C2H6" => "Ethane",
        "C3H8" => "Propane",
        "NH3" => "Ammonia",
        "N2H4" => "Hydrazine",
        "RP-1" => "RP-1 (Jet Fuel)",

        // Oxidizers
        "O2" => "Oxygen",
        "F2" => "Fluorine",
        "N2O" => "Nitrous Oxide",
        "H2O2" => "Hydrogen Peroxide",
        "N2O4" => "Nitrogen Tetroxide",

        // Fallback
        _ => formula,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RocketSubtab {
    #[default]
    Performance,
    Geometry,
    Thermal,
    Propellants,
    Studies,
    Data,
}

impl RocketSubtab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Performance => "Performance",
            Self::Geometry => "Geometry",
            Self::Thermal => "Thermal",
            Self::Propellants => "Propellants",
            Self::Studies => "Studies",
            Self::Data => "Data",
        }
    }

    pub fn all() -> &'static [RocketSubtab] {
        const TABS: &[RocketSubtab] = &[
            RocketSubtab::Performance,
            RocketSubtab::Geometry,
            RocketSubtab::Thermal,
            RocketSubtab::Propellants,
            RocketSubtab::Studies,
            RocketSubtab::Data,
        ];
        TABS
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PerformanceConstraint {
    #[default]
    None,
    Thrust {
        target_n: f64,
    },
    MassFlow {
        target_kg_per_s: f64,
    },
    ThroatDiameter {
        target_m: f64,
    },
}

#[derive(Debug, Clone)]
pub struct RocketPerformanceCase {
    pub case_name: String,
    pub oxidizer_name: String,
    pub fuel_name: String,
    pub oxidizer_temperature_k: f64,
    pub fuel_temperature_k: f64,
    pub mixture_ratio: f64,
    pub use_optimal_mixture_ratio: bool,
    pub optimal_mixture_ratio_value: f64,
    pub chamber_pressure_pa: f64,
    pub ambient_pressure_pa: f64,
    pub combustor_model: CombustorModel,
    pub nozzle_chemistry_model: NozzleChemistryModel,
    pub nozzle_constraint: NozzleConstraint,
    pub performance_constraint: PerformanceConstraint,
    // Text fields for adaptive unit inputs
    pub chamber_pressure_text: String,
    pub ambient_pressure_text: String,
    pub oxidizer_temperature_text: String,
    pub fuel_temperature_text: String,
    pub thrust_target_text: String,
    pub mass_flow_target_text: String,
    pub diameter_target_text: String,
}

impl Default for RocketPerformanceCase {
    fn default() -> Self {
        Self {
            case_name: "LOX/RP-1 baseline".to_owned(),
            oxidizer_name: "O2".to_owned(),
            fuel_name: "RP-1".to_owned(),
            oxidizer_temperature_k: 90.0,
            fuel_temperature_k: 293.0,
            mixture_ratio: 2.6,
            use_optimal_mixture_ratio: false,
            optimal_mixture_ratio_value: 2.6,
            chamber_pressure_pa: 7.0e6,
            ambient_pressure_pa: 101_325.0,
            combustor_model: CombustorModel::InfiniteArea,
            nozzle_chemistry_model: NozzleChemistryModel::ShiftingEquilibrium,
            nozzle_constraint: NozzleConstraint::ExpansionRatio(40.0),
            performance_constraint: PerformanceConstraint::None,
            // Default text field values
            chamber_pressure_text: "7.0 MPa".to_owned(),
            ambient_pressure_text: "101325 Pa".to_owned(),
            oxidizer_temperature_text: "90 K".to_owned(),
            fuel_temperature_text: "293 K".to_owned(),
            thrust_target_text: "1000000 N".to_owned(),
            mass_flow_target_text: "10.0 kg/s".to_owned(),
            diameter_target_text: "0.5 m".to_owned(),
        }
    }
}

impl RocketPerformanceCase {
    pub fn to_analysis_problem(&self) -> RocketAnalysisProblem {
        RocketAnalysisProblem {
            oxidizer: tf_cea::Reactant {
                name: canonical_species_for_cea(&self.oxidizer_name),
                amount_moles: 1.0,
                temperature_k: Some(self.oxidizer_temperature_k),
            },
            fuel: tf_cea::Reactant {
                name: canonical_species_for_cea(&self.fuel_name),
                amount_moles: 1.0,
                temperature_k: Some(self.fuel_temperature_k),
            },
            chamber_pressure_pa: self.chamber_pressure_pa,
            mixture_ratio_oxidizer_to_fuel: self.mixture_ratio,
            nozzle_constraint: self.nozzle_constraint.clone(),
            combustor_model: self.combustor_model.clone(),
            nozzle_chemistry_model: self.nozzle_chemistry_model.clone(),
            ambient_pressure_pa: self.ambient_pressure_pa,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RocketStudyConfig {
    pub variable: StudyVariable,
    pub min: f64,
    pub max: f64,
    pub point_count: usize,
    pub selected_metrics: Vec<StudyOutputMetric>,
}

impl Default for RocketStudyConfig {
    fn default() -> Self {
        Self {
            variable: StudyVariable::ChamberPressurePa,
            min: 5.0e6,
            max: 10.0e6,
            point_count: 11,
            selected_metrics: vec![
                StudyOutputMetric::SpecificImpulseVacS,
                StudyOutputMetric::SpecificImpulseAmbS,
                StudyOutputMetric::ThrustCoefficientVac,
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropellantPreset {
    pub key: &'static str,
    pub display_name: &'static str,
    pub category: &'static str,
    pub oxidizer_name: &'static str,
    pub fuel_name: &'static str,
    pub notes: &'static str,
    pub recommended_mixture_ratio: Option<f64>,
    pub oxidizer_temperature_k: Option<f64>,
    pub fuel_temperature_k: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RocketPropellantConfig {
    pub selected_preset_key: String,
    pub search_query: String,
}

#[derive(Debug, Clone)]
pub struct RocketGeometryConfig {
    pub sizing_mode: GeometrySizingMode,
    pub nozzle_contour_style: NozzleContourStyle,
    pub throat_input_value: f64,
    pub chamber_contraction_ratio: f64,
    pub characteristic_length_m: f64,
    pub nozzle_half_angle_deg: f64,
    pub nozzle_truncation_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct RocketThermalConfig {
    pub model: ThermalModel,
    pub cooling_mode: CoolingMode,
    pub reference_recovery_temperature_k: f64,
    pub wall_temperature_k: f64,
    pub reference_gas_side_htc_w_m2_k: f64,
    pub wall: WallModel,
    pub coolant: CoolantModel,
    pub film: FilmCoolingModel,
    pub channels: ChannelGeometry,
    pub design: ThermalDesignSettings,
    pub selected_material_name: String,
    pub material_library: Vec<RocketThermalMaterial>,
    pub use_performance_mixture_ratio: bool,
    pub specified_mixture_ratio: f64,
    pub use_coolant_properties_from_fluids: bool,
    pub selected_coolant_species: String,
    pub use_engine_propellant_coolant: bool,
    pub coolant_fuel_multiplier: f64,
    pub coolant_oxidizer_multiplier: f64,
    pub include_film_in_propellant_coolant_mix: bool,
    pub coolant_property_override: bool,
    pub show_series_heat_flux: bool,
    pub show_series_htc: bool,
    pub show_series_recovery_temperature: bool,
    pub show_series_wall_temperature: bool,
    pub show_series_coolant_temperature: bool,
    pub show_series_coolant_pressure: bool,
    pub show_series_channel_height: bool,
    pub show_series_film_effectiveness: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RocketThermalMaterial {
    pub name: String,
    pub k_reference_w_m_k: f64,
    pub k_reference_temperature_k: f64,
    pub k_temp_coeff_per_k: f64,
    pub allowable_temperature_k: f64,
    pub density_kg_m3: f64,
    pub cp_j_kg_k: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RocketThermalPlotTab {
    #[default]
    Temperatures,
    HeatFlux,
    HeatTransferCoeff,
    Pressure,
    ChannelGeometry,
    FilmEffectiveness,
}

impl RocketThermalPlotTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Temperatures => "Temperatures",
            Self::HeatFlux => "Heat Flux",
            Self::HeatTransferCoeff => "HTC",
            Self::Pressure => "Pressure",
            Self::ChannelGeometry => "Channel Geometry",
            Self::FilmEffectiveness => "Film",
        }
    }

    pub fn all() -> &'static [Self] {
        const TABS: &[RocketThermalPlotTab] = &[
            RocketThermalPlotTab::Temperatures,
            RocketThermalPlotTab::HeatFlux,
            RocketThermalPlotTab::HeatTransferCoeff,
            RocketThermalPlotTab::Pressure,
            RocketThermalPlotTab::ChannelGeometry,
            RocketThermalPlotTab::FilmEffectiveness,
        ];
        TABS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFieldSource {
    NativeBackend,
    DerivedInRust,
    Estimated,
    Unavailable,
}

impl DataFieldSource {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NativeBackend => "Native backend",
            Self::DerivedInRust => "Derived in Rust",
            Self::Estimated => "Estimated",
            Self::Unavailable => "Unavailable",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RocketDataField {
    pub name: String,
    pub value: String,
    pub source: DataFieldSource,
}

#[derive(Debug, Clone)]
pub struct RocketDataPerformanceSummary {
    pub chamber_pressure_pa: f64,
    pub mixture_ratio: f64,
    pub ambient_pressure_pa: f64,
    pub oxidizer_temperature_k: f64,
    pub fuel_temperature_k: f64,
    pub combustor_model: String,
    pub nozzle_chemistry: String,
    pub nozzle_constraint: String,
}

#[derive(Debug, Clone)]
pub struct RocketDataGeometrySummary {
    pub sizing_mode: String,
    pub throat_input_label: String,
    pub contraction_ratio: f64,
    pub characteristic_length_m: f64,
    pub nozzle_half_angle_deg: f64,
}

#[derive(Debug, Clone)]
pub struct RocketDataThermalSummary {
    pub model: String,
    pub cooling_mode: String,
    pub recovery_temperature_k: f64,
    pub wall_temperature_k: f64,
    pub reference_htc_w_m2_k: f64,
}

#[derive(Debug, Clone)]
pub struct RocketDataSnapshot {
    pub case_name: String,
    pub backend_chain: String,
    pub status: Option<String>,
    pub last_error: Option<String>,
    pub oxidizer: String,
    pub fuel: String,
    pub selected_preset: Option<PropellantPresetSummary>,
    pub performance: RocketDataPerformanceSummary,
    pub geometry: RocketDataGeometrySummary,
    pub thermal: RocketDataThermalSummary,
    pub provenance_fields: Vec<RocketDataField>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PropellantPresetSummary {
    pub display_name: String,
    pub category: String,
    pub notes: String,
}

impl Default for RocketThermalConfig {
    fn default() -> Self {
        Self {
            model: ThermalModel::BartzLikeConvective,
            cooling_mode: CoolingMode::AdiabaticWall,
            reference_recovery_temperature_k: 3400.0,
            wall_temperature_k: 900.0,
            reference_gas_side_htc_w_m2_k: 25_000.0,
            wall: WallModel::default(),
            coolant: CoolantModel::default(),
            film: FilmCoolingModel::default(),
            channels: ChannelGeometry::default(),
            design: ThermalDesignSettings::default(),
            selected_material_name: "CuCrZr".to_owned(),
            material_library: vec![
                RocketThermalMaterial {
                    name: "CuCrZr".to_owned(),
                    k_reference_w_m_k: 320.0,
                    k_reference_temperature_k: 300.0,
                    k_temp_coeff_per_k: -0.0002,
                    allowable_temperature_k: 1100.0,
                    density_kg_m3: 8_900.0,
                    cp_j_kg_k: 385.0,
                },
                RocketThermalMaterial {
                    name: "Copper (OFHC)".to_owned(),
                    k_reference_w_m_k: 390.0,
                    k_reference_temperature_k: 300.0,
                    k_temp_coeff_per_k: -0.0004,
                    allowable_temperature_k: 1000.0,
                    density_kg_m3: 8_960.0,
                    cp_j_kg_k: 385.0,
                },
                RocketThermalMaterial {
                    name: "Inconel 718".to_owned(),
                    k_reference_w_m_k: 13.0,
                    k_reference_temperature_k: 300.0,
                    k_temp_coeff_per_k: 0.00035,
                    allowable_temperature_k: 1250.0,
                    density_kg_m3: 8_190.0,
                    cp_j_kg_k: 435.0,
                },
                RocketThermalMaterial {
                    name: "Stainless 304".to_owned(),
                    k_reference_w_m_k: 16.0,
                    k_reference_temperature_k: 300.0,
                    k_temp_coeff_per_k: 0.00025,
                    allowable_temperature_k: 1050.0,
                    density_kg_m3: 8_000.0,
                    cp_j_kg_k: 500.0,
                },
            ],
            use_coolant_properties_from_fluids: true,
            use_performance_mixture_ratio: true,
            specified_mixture_ratio: 2.6,
            selected_coolant_species: "N2".to_owned(),
            use_engine_propellant_coolant: true,
            coolant_fuel_multiplier: 1.0,
            coolant_oxidizer_multiplier: 0.0,
            include_film_in_propellant_coolant_mix: false,
            coolant_property_override: false,
            show_series_heat_flux: true,
            show_series_htc: true,
            show_series_recovery_temperature: false,
            show_series_wall_temperature: true,
            show_series_coolant_temperature: true,
            show_series_coolant_pressure: true,
            show_series_channel_height: true,
            show_series_film_effectiveness: true,
        }
    }
}

impl Default for RocketGeometryConfig {
    fn default() -> Self {
        Self {
            sizing_mode: GeometrySizingMode::GivenThroatDiameter,
            nozzle_contour_style: NozzleContourStyle::Conical,
            throat_input_value: 0.12,
            chamber_contraction_ratio: 3.0,
            characteristic_length_m: 1.2,
            nozzle_half_angle_deg: 15.0,
            nozzle_truncation_ratio: 1.0,
        }
    }
}

impl Default for RocketPropellantConfig {
    fn default() -> Self {
        Self {
            selected_preset_key: "lox-rp1".to_owned(),
            search_query: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RocketWorkspace {
    pub selected_subtab: RocketSubtab,
    pub performance_case: RocketPerformanceCase,
    pub studies: RocketStudyConfig,
    pub propellants: RocketPropellantConfig,
    pub geometry: RocketGeometryConfig,
    pub thermal: RocketThermalConfig,
    pub last_result: Option<RocketAnalysisResult>,
    pub last_study_result: Option<tf_rpa::RocketStudyResult>,
    pub last_geometry_result: Option<RocketGeometryResult>,
    pub last_thermal_result: Option<RocketThermalResult>,
    pub thermal_plot_tab: RocketThermalPlotTab,
    pub unit_inputs: UnitAwareInput,
    pub status: Option<String>,
    pub last_error: Option<String>,
}

impl RocketWorkspace {
    pub fn from_def(def: &RocketWorkspaceDef) -> Self {
        Self {
            selected_subtab: subtab_from_key(&def.selected_subtab),
            performance_case: case_from_def(&def.performance_case),
            studies: study_from_def(&def.study),
            propellants: propellant_from_def(&def.propellants),
            geometry: geometry_from_def(&def.geometry),
            thermal: thermal_from_def(&def.thermal),
            last_result: None,
            last_study_result: None,
            last_geometry_result: None,
            last_thermal_result: None,
            thermal_plot_tab: RocketThermalPlotTab::default(),
            unit_inputs: UnitAwareInput::default(),
            status: None,
            last_error: None,
        }
    }

    pub fn to_def(&self) -> RocketWorkspaceDef {
        RocketWorkspaceDef {
            selected_subtab: self.selected_subtab.label().to_owned(),
            performance_case: case_to_def(&self.performance_case),
            study: study_to_def(&self.studies),
            propellants: propellant_to_def(&self.propellants),
            geometry: geometry_to_def(&self.geometry),
            thermal: thermal_to_def(&self.thermal),
        }
    }

    pub fn geometry_problem(&self) -> RocketGeometryProblem {
        RocketGeometryProblem {
            base_problem: self.performance_case.to_analysis_problem(),
            sizing_mode: self.geometry.sizing_mode,
            throat_input_value: self.geometry.throat_input_value,
            chamber_contraction_ratio: self.geometry.chamber_contraction_ratio,
            characteristic_length_m: self.geometry.characteristic_length_m,
            nozzle_half_angle_deg: self.geometry.nozzle_half_angle_deg,
            nozzle_contour_style: self.geometry.nozzle_contour_style,
            nozzle_truncation_ratio: self.geometry.nozzle_truncation_ratio,
        }
    }

    pub fn thermal_problem(&self, geometry_result: RocketGeometryResult) -> RocketThermalProblem {
        let mut performance_problem = self.performance_case.to_analysis_problem();
        if !self.thermal.use_performance_mixture_ratio {
            performance_problem.mixture_ratio_oxidizer_to_fuel =
                self.thermal.specified_mixture_ratio;
        }
        RocketThermalProblem {
            performance_problem,
            geometry_problem: self.geometry_problem(),
            geometry_result,
            model: self.thermal.model,
            cooling_mode: self.thermal.cooling_mode,
            reference_recovery_temperature_k: self.thermal.reference_recovery_temperature_k,
            wall_temperature_k: self.thermal.wall_temperature_k,
            reference_gas_side_htc_w_m2_k: self.thermal.reference_gas_side_htc_w_m2_k,
            wall_model: self.thermal.wall.clone(),
            coolant_model: self.thermal.coolant.clone(),
            film_model: self.thermal.film.clone(),
            channel_geometry: self.thermal.channels.clone(),
            design_settings: self.thermal.design.clone(),
        }
    }

    pub fn data_snapshot(&self) -> RocketDataSnapshot {
        let nozzle_constraint = match self.performance_case.nozzle_constraint {
            NozzleConstraint::ExpansionRatio(v) => format!("Expansion ratio ε={v:.3}"),
            NozzleConstraint::ExitPressurePa(v) => format!("Exit pressure Pe={v:.1} Pa"),
        };

        let throat_input_label = match self.geometry.sizing_mode {
            GeometrySizingMode::GivenThroatDiameter => {
                format!(
                    "throat diameter = {:.5} m",
                    self.geometry.throat_input_value
                )
            }
            GeometrySizingMode::GivenThroatArea => {
                format!("throat area = {:.6} m²", self.geometry.throat_input_value)
            }
        };

        let mut provenance_fields = vec![
            RocketDataField {
                name: "Chamber temperature".to_owned(),
                value: self
                    .last_result
                    .as_ref()
                    .and_then(|r| r.chamber.temperature_k)
                    .map(|v| format!("{v:.2} K"))
                    .unwrap_or_else(|| "n/a".to_owned()),
                source: if self.last_result.is_some() {
                    DataFieldSource::NativeBackend
                } else {
                    DataFieldSource::Unavailable
                },
            },
            RocketDataField {
                name: "c*".to_owned(),
                value: self
                    .last_result
                    .as_ref()
                    .map(|r| format!("{:.3} m/s", r.characteristic_velocity_m_per_s))
                    .unwrap_or_else(|| "n/a".to_owned()),
                source: if self.last_result.is_some() {
                    DataFieldSource::NativeBackend
                } else {
                    DataFieldSource::Unavailable
                },
            },
            RocketDataField {
                name: "Isp,amb".to_owned(),
                value: self
                    .last_result
                    .as_ref()
                    .map(|r| format!("{:.3} s", r.specific_impulse_amb_s))
                    .unwrap_or_else(|| "n/a".to_owned()),
                source: if self.last_result.is_some() {
                    DataFieldSource::DerivedInRust
                } else {
                    DataFieldSource::Unavailable
                },
            },
            RocketDataField {
                name: "Exit diameter".to_owned(),
                value: self
                    .last_geometry_result
                    .as_ref()
                    .map(|r| format!("{:.5} m", r.exit_diameter_m))
                    .unwrap_or_else(|| "n/a".to_owned()),
                source: if self.last_geometry_result.is_some() {
                    DataFieldSource::DerivedInRust
                } else {
                    DataFieldSource::Unavailable
                },
            },
            RocketDataField {
                name: "Peak thermal heat flux".to_owned(),
                value: self
                    .last_thermal_result
                    .as_ref()
                    .map(|r| format!("{:.0} W/m² @ {}", r.peak_heat_flux_w_m2, r.peak_location))
                    .unwrap_or_else(|| "n/a".to_owned()),
                source: if self.last_thermal_result.is_some() {
                    DataFieldSource::Estimated
                } else {
                    DataFieldSource::Unavailable
                },
            },
        ];

        provenance_fields.push(RocketDataField {
            name: "Throat station thermodynamic detail".to_owned(),
            value: "placeholder summary only".to_owned(),
            source: DataFieldSource::Unavailable,
        });

        let mut warnings = Vec::new();
        if self.last_result.is_none() {
            warnings.push("No performance solve result yet; backend-backed performance fields are unavailable.".to_owned());
        }
        if self.last_geometry_result.is_none() {
            warnings.push("No geometry result yet; geometry-derived and thermal prerequisite fields are unavailable.".to_owned());
        }
        if self.last_thermal_result.is_none() {
            warnings.push("No thermal result yet; thermal estimates are unavailable.".to_owned());
        }
        if matches!(
            self.performance_case.nozzle_chemistry_model,
            NozzleChemistryModel::FrozenAtThroat
        ) {
            warnings.push(
                "Frozen-at-throat chemistry is currently unsupported by solver path and will fail in performance solves."
                    .to_owned(),
            );
        }
        if matches!(
            self.performance_case.nozzle_constraint,
            NozzleConstraint::ExitPressurePa(_)
        ) {
            warnings.push(
                "Exit-pressure constrained mode is currently unsupported in solver and geometry workflows."
                    .to_owned(),
            );
        }
        warnings.push(
            "Geometry outputs include first-pass estimated fields (chamber/nozzle lengths)."
                .to_owned(),
        );
        warnings.push("Thermal outputs are first-pass convective estimates; coolant-side resistance/radiation/CHT are deferred.".to_owned());

        RocketDataSnapshot {
            case_name: self.performance_case.case_name.clone(),
            backend_chain: "CEA backend (native/bridge) -> tf-rpa orchestration -> Rocket UI"
                .to_owned(),
            status: self.status.clone(),
            last_error: self.last_error.clone(),
            oxidizer: self.performance_case.oxidizer_name.clone(),
            fuel: self.performance_case.fuel_name.clone(),
            selected_preset: self
                .selected_propellant_preset()
                .map(|p| PropellantPresetSummary {
                    display_name: p.display_name.to_owned(),
                    category: p.category.to_owned(),
                    notes: p.notes.to_owned(),
                }),
            performance: RocketDataPerformanceSummary {
                chamber_pressure_pa: self.performance_case.chamber_pressure_pa,
                mixture_ratio: self.performance_case.mixture_ratio,
                ambient_pressure_pa: self.performance_case.ambient_pressure_pa,
                oxidizer_temperature_k: self.performance_case.oxidizer_temperature_k,
                fuel_temperature_k: self.performance_case.fuel_temperature_k,
                combustor_model: format!("{:?}", self.performance_case.combustor_model),
                nozzle_chemistry: format!("{:?}", self.performance_case.nozzle_chemistry_model),
                nozzle_constraint,
            },
            geometry: RocketDataGeometrySummary {
                sizing_mode: self.geometry.sizing_mode.label().to_owned(),
                throat_input_label,
                contraction_ratio: self.geometry.chamber_contraction_ratio,
                characteristic_length_m: self.geometry.characteristic_length_m,
                nozzle_half_angle_deg: self.geometry.nozzle_half_angle_deg,
            },
            thermal: RocketDataThermalSummary {
                model: self.thermal.model.label().to_owned(),
                cooling_mode: self.thermal.cooling_mode.label().to_owned(),
                recovery_temperature_k: self.thermal.reference_recovery_temperature_k,
                wall_temperature_k: self.thermal.wall_temperature_k,
                reference_htc_w_m2_k: self.thermal.reference_gas_side_htc_w_m2_k,
            },
            provenance_fields,
            warnings,
        }
    }

    pub fn selected_propellant_preset(&self) -> Option<&'static PropellantPreset> {
        propellant_presets()
            .iter()
            .find(|p| p.key == self.propellants.selected_preset_key)
    }

    pub fn filtered_propellant_presets(&self) -> Vec<&'static PropellantPreset> {
        let query = self.propellants.search_query.trim().to_ascii_lowercase();
        propellant_presets()
            .iter()
            .filter(|preset| {
                if query.is_empty() {
                    return true;
                }
                [
                    preset.display_name,
                    preset.category,
                    preset.oxidizer_name,
                    preset.fuel_name,
                    preset.notes,
                ]
                .iter()
                .any(|field| field.to_ascii_lowercase().contains(&query))
            })
            .collect()
    }

    pub fn apply_selected_propellant_to_performance(&mut self) -> Option<String> {
        let preset = self.selected_propellant_preset()?;
        self.performance_case.oxidizer_name = canonical_species_for_cea(preset.oxidizer_name);
        self.performance_case.fuel_name = canonical_species_for_cea(preset.fuel_name);
        if let Some(mr) = preset.recommended_mixture_ratio {
            self.performance_case.mixture_ratio = mr;
        }
        if let Some(t_ox) = preset.oxidizer_temperature_k {
            self.performance_case.oxidizer_temperature_k = t_ox;
        }
        if let Some(t_fuel) = preset.fuel_temperature_k {
            self.performance_case.fuel_temperature_k = t_fuel;
        }

        Some(format!(
            "Applied propellant preset '{}' to Performance case",
            preset.display_name
        ))
    }
}

pub fn propellant_presets() -> &'static [PropellantPreset] {
    const PRESETS: &[PropellantPreset] = &[
        PropellantPreset {
            key: "lox-rp1",
            display_name: "LOX / RP-1",
            category: "Cryogenic + Kerosene",
            oxidizer_name: "LOX",
            fuel_name: "RP-1",
            notes: "Common booster-class hydrocarbon pair.",
            recommended_mixture_ratio: Some(2.6),
            oxidizer_temperature_k: Some(90.0),
            fuel_temperature_k: Some(293.0),
        },
        PropellantPreset {
            key: "lox-ch4",
            display_name: "LOX / CH4",
            category: "Cryogenic Methalox",
            oxidizer_name: "LOX",
            fuel_name: "CH4",
            notes: "Reusable-oriented methane/oxygen pair.",
            recommended_mixture_ratio: Some(3.4),
            oxidizer_temperature_k: Some(90.0),
            fuel_temperature_k: Some(112.0),
        },
        PropellantPreset {
            key: "lox-lh2",
            display_name: "LOX / LH2",
            category: "Cryogenic Hydrolox",
            oxidizer_name: "LOX",
            fuel_name: "LH2",
            notes: "High-performance upper-stage pair.",
            recommended_mixture_ratio: Some(5.5),
            oxidizer_temperature_k: Some(90.0),
            fuel_temperature_k: Some(20.0),
        },
        PropellantPreset {
            key: "n2o-ipa",
            display_name: "N2O / IPA",
            category: "Nitrous-based",
            oxidizer_name: "N2O",
            fuel_name: "IPA",
            notes: "Pressure-fed small-engine friendly pair.",
            recommended_mixture_ratio: Some(3.5),
            oxidizer_temperature_k: Some(298.0),
            fuel_temperature_k: Some(298.0),
        },
        PropellantPreset {
            key: "n2o-ethanol",
            display_name: "N2O / Ethanol",
            category: "Nitrous-based",
            oxidizer_name: "N2O",
            fuel_name: "Ethanol",
            notes: "Experimental/education-friendly pair.",
            recommended_mixture_ratio: Some(4.0),
            oxidizer_temperature_k: Some(298.0),
            fuel_temperature_k: Some(298.0),
        },
        PropellantPreset {
            key: "mmh-nto",
            display_name: "MMH / NTO",
            category: "Storable Hypergolic",
            oxidizer_name: "NTO",
            fuel_name: "MMH",
            notes: "Classic hypergolic orbital pair.",
            recommended_mixture_ratio: Some(1.65),
            oxidizer_temperature_k: Some(298.0),
            fuel_temperature_k: Some(298.0),
        },
        PropellantPreset {
            key: "udmh-nto",
            display_name: "UDMH / NTO",
            category: "Storable Hypergolic",
            oxidizer_name: "NTO",
            fuel_name: "UDMH",
            notes: "Storable launch and orbital engines.",
            recommended_mixture_ratio: Some(2.1),
            oxidizer_temperature_k: Some(298.0),
            fuel_temperature_k: Some(298.0),
        },
        PropellantPreset {
            key: "htp-monoprop",
            display_name: "HTP Monoprop",
            category: "Monopropellant",
            oxidizer_name: "HTP",
            fuel_name: "HTP",
            notes: "Monoprop approximation in current two-stream model.",
            recommended_mixture_ratio: Some(1.0),
            oxidizer_temperature_k: Some(298.0),
            fuel_temperature_k: Some(298.0),
        },
    ];
    PRESETS
}

fn subtab_from_key(value: &str) -> RocketSubtab {
    match value {
        "Geometry" => RocketSubtab::Geometry,
        "Thermal" => RocketSubtab::Thermal,
        "Propellants" => RocketSubtab::Propellants,
        "Studies" => RocketSubtab::Studies,
        "Data" => RocketSubtab::Data,
        _ => RocketSubtab::Performance,
    }
}

fn case_from_def(def: &RocketPerformanceCaseDef) -> RocketPerformanceCase {
    RocketPerformanceCase {
        case_name: def.case_name.clone(),
        oxidizer_name: canonical_species_for_cea(&def.oxidizer_name),
        fuel_name: canonical_species_for_cea(&def.fuel_name),
        oxidizer_temperature_k: def.oxidizer_temperature_k,
        fuel_temperature_k: def.fuel_temperature_k,
        mixture_ratio: def.mixture_ratio,
        use_optimal_mixture_ratio: def.use_optimal_mixture_ratio,
        optimal_mixture_ratio_value: def.optimal_mixture_ratio_value,
        chamber_pressure_pa: def.chamber_pressure_pa,
        ambient_pressure_pa: def.ambient_pressure_pa,
        combustor_model: match def.combustor_model {
            RocketCombustorModelDef::InfiniteArea => CombustorModel::InfiniteArea,
            RocketCombustorModelDef::FiniteArea { contraction_ratio } => {
                CombustorModel::FiniteArea { contraction_ratio }
            }
        },
        nozzle_chemistry_model: match def.nozzle_chemistry_model {
            RocketNozzleChemistryModelDef::ShiftingEquilibrium => {
                NozzleChemistryModel::ShiftingEquilibrium
            }
            RocketNozzleChemistryModelDef::FrozenAtChamber => NozzleChemistryModel::FrozenAtChamber,
            RocketNozzleChemistryModelDef::FrozenAtThroat => NozzleChemistryModel::FrozenAtThroat,
        },
        nozzle_constraint: match def.nozzle_constraint {
            RocketNozzleConstraintDef::ExpansionRatio(value) => {
                NozzleConstraint::ExpansionRatio(value)
            }
            RocketNozzleConstraintDef::ExitPressurePa(value) => {
                NozzleConstraint::ExitPressurePa(value)
            }
        },
        performance_constraint: PerformanceConstraint::None, // Not persisted in schema yet
        // Initialize text fields with formatted values
        chamber_pressure_text: format!("{} Pa", def.chamber_pressure_pa),
        ambient_pressure_text: format!("{} Pa", def.ambient_pressure_pa),
        oxidizer_temperature_text: format!("{} K", def.oxidizer_temperature_k),
        fuel_temperature_text: format!("{} K", def.fuel_temperature_k),
        thrust_target_text: "1000000 N".to_owned(),
        mass_flow_target_text: "10.0 kg/s".to_owned(),
        diameter_target_text: "0.5 m".to_owned(),
    }
}

fn case_to_def(case: &RocketPerformanceCase) -> RocketPerformanceCaseDef {
    RocketPerformanceCaseDef {
        case_name: case.case_name.clone(),
        oxidizer_name: case.oxidizer_name.clone(),
        fuel_name: case.fuel_name.clone(),
        oxidizer_temperature_k: case.oxidizer_temperature_k,
        fuel_temperature_k: case.fuel_temperature_k,
        mixture_ratio: case.mixture_ratio,
        use_optimal_mixture_ratio: case.use_optimal_mixture_ratio,
        optimal_mixture_ratio_value: case.optimal_mixture_ratio_value,
        chamber_pressure_pa: case.chamber_pressure_pa,
        ambient_pressure_pa: case.ambient_pressure_pa,
        combustor_model: match case.combustor_model {
            CombustorModel::InfiniteArea => RocketCombustorModelDef::InfiniteArea,
            CombustorModel::FiniteArea { contraction_ratio } => {
                RocketCombustorModelDef::FiniteArea { contraction_ratio }
            }
        },
        nozzle_chemistry_model: match case.nozzle_chemistry_model {
            NozzleChemistryModel::ShiftingEquilibrium => {
                RocketNozzleChemistryModelDef::ShiftingEquilibrium
            }
            NozzleChemistryModel::FrozenAtChamber => RocketNozzleChemistryModelDef::FrozenAtChamber,
            NozzleChemistryModel::FrozenAtThroat => RocketNozzleChemistryModelDef::FrozenAtThroat,
        },
        nozzle_constraint: match case.nozzle_constraint {
            NozzleConstraint::ExpansionRatio(value) => {
                RocketNozzleConstraintDef::ExpansionRatio(value)
            }
            NozzleConstraint::ExitPressurePa(value) => {
                RocketNozzleConstraintDef::ExitPressurePa(value)
            }
        },
    }
}

fn study_from_def(def: &RocketStudyDef) -> RocketStudyConfig {
    RocketStudyConfig {
        variable: from_study_variable(def.variable),
        min: def.min,
        max: def.max,
        point_count: def.point_count,
        selected_metrics: def.metrics.iter().copied().map(from_study_metric).collect(),
    }
}

fn study_to_def(config: &RocketStudyConfig) -> RocketStudyDef {
    RocketStudyDef {
        variable: to_study_variable(config.variable),
        min: config.min,
        max: config.max,
        point_count: config.point_count,
        metrics: config
            .selected_metrics
            .iter()
            .copied()
            .map(to_study_metric)
            .collect(),
    }
}

fn propellant_from_def(def: &RocketPropellantWorkspaceDef) -> RocketPropellantConfig {
    RocketPropellantConfig {
        selected_preset_key: def.selected_preset_key.clone(),
        search_query: def.search_query.clone(),
    }
}

fn propellant_to_def(config: &RocketPropellantConfig) -> RocketPropellantWorkspaceDef {
    RocketPropellantWorkspaceDef {
        selected_preset_key: config.selected_preset_key.clone(),
        search_query: config.search_query.clone(),
    }
}

fn geometry_from_def(def: &RocketGeometryDef) -> RocketGeometryConfig {
    RocketGeometryConfig {
        sizing_mode: match def.sizing_mode {
            RocketGeometrySizingModeDef::GivenThroatDiameter => {
                GeometrySizingMode::GivenThroatDiameter
            }
            RocketGeometrySizingModeDef::GivenThroatArea => GeometrySizingMode::GivenThroatArea,
        },
        nozzle_contour_style: match def.nozzle_contour_style {
            RocketNozzleContourStyleDef::Conical => NozzleContourStyle::Conical,
            RocketNozzleContourStyleDef::BellParabolic => NozzleContourStyle::BellParabolic,
            RocketNozzleContourStyleDef::TruncatedIdeal => NozzleContourStyle::TruncatedIdeal,
        },
        throat_input_value: def.throat_input_value,
        chamber_contraction_ratio: def.chamber_contraction_ratio,
        characteristic_length_m: def.characteristic_length_m,
        nozzle_half_angle_deg: def.nozzle_half_angle_deg,
        nozzle_truncation_ratio: def.nozzle_truncation_ratio,
    }
}

fn geometry_to_def(config: &RocketGeometryConfig) -> RocketGeometryDef {
    RocketGeometryDef {
        sizing_mode: match config.sizing_mode {
            GeometrySizingMode::GivenThroatDiameter => {
                RocketGeometrySizingModeDef::GivenThroatDiameter
            }
            GeometrySizingMode::GivenThroatArea => RocketGeometrySizingModeDef::GivenThroatArea,
        },
        nozzle_contour_style: match config.nozzle_contour_style {
            NozzleContourStyle::Conical => RocketNozzleContourStyleDef::Conical,
            NozzleContourStyle::BellParabolic => RocketNozzleContourStyleDef::BellParabolic,
            NozzleContourStyle::TruncatedIdeal => RocketNozzleContourStyleDef::TruncatedIdeal,
        },
        throat_input_value: config.throat_input_value,
        chamber_contraction_ratio: config.chamber_contraction_ratio,
        characteristic_length_m: config.characteristic_length_m,
        nozzle_half_angle_deg: config.nozzle_half_angle_deg,
        nozzle_truncation_ratio: config.nozzle_truncation_ratio,
    }
}

fn thermal_from_def(def: &RocketThermalDef) -> RocketThermalConfig {
    RocketThermalConfig {
        model: match def.model {
            RocketThermalModelDef::BartzLikeConvective => ThermalModel::BartzLikeConvective,
        },
        cooling_mode: match def.cooling_mode {
            RocketThermalWorkspaceCoolingModeDef::AdiabaticWall => CoolingMode::AdiabaticWall,
            RocketThermalWorkspaceCoolingModeDef::Regenerative => CoolingMode::Regenerative,
            RocketThermalWorkspaceCoolingModeDef::Film => CoolingMode::Film,
            RocketThermalWorkspaceCoolingModeDef::RegenerativeFilm => CoolingMode::RegenerativeFilm,
        },
        reference_recovery_temperature_k: def.reference_recovery_temperature_k,
        wall_temperature_k: def.wall_temperature_k,
        reference_gas_side_htc_w_m2_k: def.reference_gas_side_htc_w_m2_k,
        wall: WallModel {
            material_name: def.wall.material_name.clone(),
            thermal_conductivity_w_m_k: def.wall.thermal_conductivity_w_m_k,
            thermal_conductivity_reference_temperature_k: def
                .wall
                .thermal_conductivity_reference_temperature_k,
            thermal_conductivity_temp_coeff_per_k: def.wall.thermal_conductivity_temp_coeff_per_k,
            gas_side_emissivity: def.wall.gas_side_emissivity,
            allowable_temperature_k: def.wall.allowable_temperature_k,
            density_kg_m3: def.wall.density_kg_m3,
            cp_j_kg_k: def.wall.cp_j_kg_k,
            thickness_m: def.wall.thickness_m,
        },
        coolant: CoolantModel {
            coolant_name: def.coolant.coolant_name.clone(),
            inlet_temperature_k: def.coolant.inlet_temperature_k,
            inlet_pressure_pa: def.coolant.inlet_pressure_pa,
            mass_flow_kg_s: def.coolant.mass_flow_kg_s,
            density_kg_m3: def.coolant.density_kg_m3,
            viscosity_pa_s: def.coolant.viscosity_pa_s,
            thermal_conductivity_w_m_k: def.coolant.thermal_conductivity_w_m_k,
            cp_j_kg_k: def.coolant.cp_j_kg_k,
        },
        film: FilmCoolingModel {
            film_mass_fraction: def.film.film_mass_fraction,
            effectiveness_start_fraction: def.film.effectiveness_start_fraction,
            effectiveness_end_fraction: def.film.effectiveness_end_fraction,
            max_effectiveness: def.film.max_effectiveness,
        },
        channels: ChannelGeometry {
            channel_count: def.channels.channel_count,
            width_m: def.channels.width_m,
            height_m: def.channels.height_m,
            rib_width_m: def.channels.rib_width_m,
            min_gap_m: def.channels.min_gap_m,
            roughness_m: def.channels.roughness_m,
            width_taper_end_factor: def.channels.width_taper_end_factor,
            height_taper_end_factor: def.channels.height_taper_end_factor,
            min_width_m: def.channels.min_width_m,
            max_width_m: def.channels.max_width_m,
            min_height_m: def.channels.min_height_m,
            max_height_m: def.channels.max_height_m,
        },
        design: ThermalDesignSettings {
            station_count: def.design.station_count,
            max_coolant_pressure_drop_pa: def.design.max_coolant_pressure_drop_pa,
            optimizer_max_iterations: def.design.optimizer_max_iterations,
            optimizer_local_adjustment_gain: def.design.optimizer_local_adjustment_gain,
            optimizer_global_area_gain: def.design.optimizer_global_area_gain,
            hold_channel_count_fixed: def.design.hold_channel_count_fixed,
            hold_channel_width_fixed: def.design.hold_channel_width_fixed,
            hold_channel_height_fixed: def.design.hold_channel_height_fixed,
            min_channel_count: def.design.min_channel_count,
            max_channel_count: def.design.max_channel_count,
            coolant_flow_direction: match def.design.coolant_flow_direction {
                RocketCoolantFlowDirectionDef::CoFlow => CoolantFlowDirection::CoFlow,
                RocketCoolantFlowDirectionDef::CounterFlow => CoolantFlowDirection::CounterFlow,
                RocketCoolantFlowDirectionDef::MidFeed => CoolantFlowDirection::MidFeed,
            },
            mid_feed_fraction: def.design.mid_feed_fraction,
            mid_feed_upstream_mass_fraction: def.design.mid_feed_upstream_mass_fraction,
            auto_balance_mid_feed_split: def.design.auto_balance_mid_feed_split,
        },
        selected_material_name: def.selected_material_name.clone(),
        material_library: def
            .material_library
            .iter()
            .map(|m| RocketThermalMaterial {
                name: m.name.clone(),
                k_reference_w_m_k: m.k_reference_w_m_k,
                k_reference_temperature_k: m.k_reference_temperature_k,
                k_temp_coeff_per_k: m.k_temp_coeff_per_k,
                allowable_temperature_k: m.allowable_temperature_k,
                density_kg_m3: m.density_kg_m3,
                cp_j_kg_k: m.cp_j_kg_k,
            })
            .collect(),
        use_performance_mixture_ratio: def.use_performance_mixture_ratio,
        specified_mixture_ratio: def.specified_mixture_ratio,
        use_coolant_properties_from_fluids: def.use_coolant_properties_from_fluids,
        selected_coolant_species: def.selected_coolant_species.clone(),
        use_engine_propellant_coolant: def.use_engine_propellant_coolant,
        coolant_fuel_multiplier: def.coolant_fuel_multiplier,
        coolant_oxidizer_multiplier: def.coolant_oxidizer_multiplier,
        include_film_in_propellant_coolant_mix: def.include_film_in_propellant_coolant_mix,
        coolant_property_override: def.coolant_property_override,
        show_series_heat_flux: def.show_series_heat_flux,
        show_series_htc: def.show_series_htc,
        show_series_recovery_temperature: def.show_series_recovery_temperature,
        show_series_wall_temperature: def.show_series_wall_temperature,
        show_series_coolant_temperature: def.show_series_coolant_temperature,
        show_series_coolant_pressure: def.show_series_coolant_pressure,
        show_series_channel_height: def.show_series_channel_height,
        show_series_film_effectiveness: def.show_series_film_effectiveness,
    }
}

fn thermal_to_def(config: &RocketThermalConfig) -> RocketThermalDef {
    RocketThermalDef {
        model: match config.model {
            ThermalModel::BartzLikeConvective => RocketThermalModelDef::BartzLikeConvective,
        },
        cooling_mode: match config.cooling_mode {
            CoolingMode::AdiabaticWall => RocketThermalWorkspaceCoolingModeDef::AdiabaticWall,
            CoolingMode::Regenerative => RocketThermalWorkspaceCoolingModeDef::Regenerative,
            CoolingMode::Film => RocketThermalWorkspaceCoolingModeDef::Film,
            CoolingMode::RegenerativeFilm => RocketThermalWorkspaceCoolingModeDef::RegenerativeFilm,
        },
        reference_recovery_temperature_k: config.reference_recovery_temperature_k,
        wall_temperature_k: config.wall_temperature_k,
        reference_gas_side_htc_w_m2_k: config.reference_gas_side_htc_w_m2_k,
        wall: RocketWallModelDef {
            material_name: config.wall.material_name.clone(),
            thermal_conductivity_w_m_k: config.wall.thermal_conductivity_w_m_k,
            thermal_conductivity_reference_temperature_k: config
                .wall
                .thermal_conductivity_reference_temperature_k,
            thermal_conductivity_temp_coeff_per_k: config
                .wall
                .thermal_conductivity_temp_coeff_per_k,
            gas_side_emissivity: config.wall.gas_side_emissivity,
            allowable_temperature_k: config.wall.allowable_temperature_k,
            density_kg_m3: config.wall.density_kg_m3,
            cp_j_kg_k: config.wall.cp_j_kg_k,
            thickness_m: config.wall.thickness_m,
        },
        coolant: RocketCoolantModelDef {
            coolant_name: config.coolant.coolant_name.clone(),
            inlet_temperature_k: config.coolant.inlet_temperature_k,
            inlet_pressure_pa: config.coolant.inlet_pressure_pa,
            mass_flow_kg_s: config.coolant.mass_flow_kg_s,
            density_kg_m3: config.coolant.density_kg_m3,
            viscosity_pa_s: config.coolant.viscosity_pa_s,
            thermal_conductivity_w_m_k: config.coolant.thermal_conductivity_w_m_k,
            cp_j_kg_k: config.coolant.cp_j_kg_k,
        },
        film: RocketFilmCoolingDef {
            film_mass_fraction: config.film.film_mass_fraction,
            effectiveness_start_fraction: config.film.effectiveness_start_fraction,
            effectiveness_end_fraction: config.film.effectiveness_end_fraction,
            max_effectiveness: config.film.max_effectiveness,
        },
        channels: RocketChannelGeometryDef {
            channel_count: config.channels.channel_count,
            width_m: config.channels.width_m,
            height_m: config.channels.height_m,
            rib_width_m: config.channels.rib_width_m,
            min_gap_m: config.channels.min_gap_m,
            roughness_m: config.channels.roughness_m,
            width_taper_end_factor: config.channels.width_taper_end_factor,
            height_taper_end_factor: config.channels.height_taper_end_factor,
            min_width_m: config.channels.min_width_m,
            max_width_m: config.channels.max_width_m,
            min_height_m: config.channels.min_height_m,
            max_height_m: config.channels.max_height_m,
        },
        design: RocketThermalDesignSettingsDef {
            station_count: config.design.station_count,
            max_coolant_pressure_drop_pa: config.design.max_coolant_pressure_drop_pa,
            optimizer_max_iterations: config.design.optimizer_max_iterations,
            optimizer_local_adjustment_gain: config.design.optimizer_local_adjustment_gain,
            optimizer_global_area_gain: config.design.optimizer_global_area_gain,
            hold_channel_count_fixed: config.design.hold_channel_count_fixed,
            hold_channel_width_fixed: config.design.hold_channel_width_fixed,
            hold_channel_height_fixed: config.design.hold_channel_height_fixed,
            min_channel_count: config.design.min_channel_count,
            max_channel_count: config.design.max_channel_count,
            coolant_flow_direction: match config.design.coolant_flow_direction {
                CoolantFlowDirection::CoFlow => RocketCoolantFlowDirectionDef::CoFlow,
                CoolantFlowDirection::CounterFlow => RocketCoolantFlowDirectionDef::CounterFlow,
                CoolantFlowDirection::MidFeed => RocketCoolantFlowDirectionDef::MidFeed,
            },
            mid_feed_fraction: config.design.mid_feed_fraction,
            mid_feed_upstream_mass_fraction: config.design.mid_feed_upstream_mass_fraction,
            auto_balance_mid_feed_split: config.design.auto_balance_mid_feed_split,
        },
        selected_material_name: config.selected_material_name.clone(),
        material_library: config
            .material_library
            .iter()
            .map(|m| RocketThermalMaterialDef {
                name: m.name.clone(),
                k_reference_w_m_k: m.k_reference_w_m_k,
                k_reference_temperature_k: m.k_reference_temperature_k,
                k_temp_coeff_per_k: m.k_temp_coeff_per_k,
                allowable_temperature_k: m.allowable_temperature_k,
                density_kg_m3: m.density_kg_m3,
                cp_j_kg_k: m.cp_j_kg_k,
            })
            .collect(),
        use_performance_mixture_ratio: config.use_performance_mixture_ratio,
        specified_mixture_ratio: config.specified_mixture_ratio,
        use_coolant_properties_from_fluids: config.use_coolant_properties_from_fluids,
        selected_coolant_species: config.selected_coolant_species.clone(),
        use_engine_propellant_coolant: config.use_engine_propellant_coolant,
        coolant_fuel_multiplier: config.coolant_fuel_multiplier,
        coolant_oxidizer_multiplier: config.coolant_oxidizer_multiplier,
        include_film_in_propellant_coolant_mix: config.include_film_in_propellant_coolant_mix,
        coolant_property_override: config.coolant_property_override,
        show_series_heat_flux: config.show_series_heat_flux,
        show_series_htc: config.show_series_htc,
        show_series_recovery_temperature: config.show_series_recovery_temperature,
        show_series_wall_temperature: config.show_series_wall_temperature,
        show_series_coolant_temperature: config.show_series_coolant_temperature,
        show_series_coolant_pressure: config.show_series_coolant_pressure,
        show_series_channel_height: config.show_series_channel_height,
        show_series_film_effectiveness: config.show_series_film_effectiveness,
    }
}

fn from_study_variable(v: RocketStudyVariableDef) -> StudyVariable {
    match v {
        RocketStudyVariableDef::ChamberPressurePa => StudyVariable::ChamberPressurePa,
        RocketStudyVariableDef::MixtureRatio => StudyVariable::MixtureRatio,
        RocketStudyVariableDef::AmbientPressurePa => StudyVariable::AmbientPressurePa,
        RocketStudyVariableDef::ExpansionRatio => StudyVariable::ExpansionRatio,
    }
}

fn to_study_variable(v: StudyVariable) -> RocketStudyVariableDef {
    match v {
        StudyVariable::ChamberPressurePa => RocketStudyVariableDef::ChamberPressurePa,
        StudyVariable::MixtureRatio => RocketStudyVariableDef::MixtureRatio,
        StudyVariable::AmbientPressurePa => RocketStudyVariableDef::AmbientPressurePa,
        StudyVariable::ExpansionRatio => RocketStudyVariableDef::ExpansionRatio,
    }
}

fn from_study_metric(v: RocketStudyMetricDef) -> StudyOutputMetric {
    match v {
        RocketStudyMetricDef::ChamberTemperatureK => StudyOutputMetric::ChamberTemperatureK,
        RocketStudyMetricDef::ChamberGamma => StudyOutputMetric::ChamberGamma,
        RocketStudyMetricDef::ChamberMolecularWeightKgPerKmol => {
            StudyOutputMetric::ChamberMolecularWeightKgPerKmol
        }
        RocketStudyMetricDef::CharacteristicVelocityMPerS => {
            StudyOutputMetric::CharacteristicVelocityMPerS
        }
        RocketStudyMetricDef::ThrustCoefficientVac => StudyOutputMetric::ThrustCoefficientVac,
        RocketStudyMetricDef::SpecificImpulseVacS => StudyOutputMetric::SpecificImpulseVacS,
        RocketStudyMetricDef::SpecificImpulseAmbS => StudyOutputMetric::SpecificImpulseAmbS,
        RocketStudyMetricDef::EffectiveExhaustVelocityVacMPerS => {
            StudyOutputMetric::EffectiveExhaustVelocityVacMPerS
        }
        RocketStudyMetricDef::EffectiveExhaustVelocityAmbMPerS => {
            StudyOutputMetric::EffectiveExhaustVelocityAmbMPerS
        }
        RocketStudyMetricDef::ChamberToAmbientPressureRatio => {
            StudyOutputMetric::ChamberToAmbientPressureRatio
        }
    }
}

fn to_study_metric(v: StudyOutputMetric) -> RocketStudyMetricDef {
    match v {
        StudyOutputMetric::ChamberTemperatureK => RocketStudyMetricDef::ChamberTemperatureK,
        StudyOutputMetric::ChamberGamma => RocketStudyMetricDef::ChamberGamma,
        StudyOutputMetric::ChamberMolecularWeightKgPerKmol => {
            RocketStudyMetricDef::ChamberMolecularWeightKgPerKmol
        }
        StudyOutputMetric::CharacteristicVelocityMPerS => {
            RocketStudyMetricDef::CharacteristicVelocityMPerS
        }
        StudyOutputMetric::ThrustCoefficientVac => RocketStudyMetricDef::ThrustCoefficientVac,
        StudyOutputMetric::SpecificImpulseVacS => RocketStudyMetricDef::SpecificImpulseVacS,
        StudyOutputMetric::SpecificImpulseAmbS => RocketStudyMetricDef::SpecificImpulseAmbS,
        StudyOutputMetric::EffectiveExhaustVelocityVacMPerS => {
            RocketStudyMetricDef::EffectiveExhaustVelocityVacMPerS
        }
        StudyOutputMetric::EffectiveExhaustVelocityAmbMPerS => {
            RocketStudyMetricDef::EffectiveExhaustVelocityAmbMPerS
        }
        StudyOutputMetric::ChamberToAmbientPressureRatio => {
            RocketStudyMetricDef::ChamberToAmbientPressureRatio
        }
    }
}

pub fn format_state_summary(state: &StateSummary) -> String {
    let t = state
        .temperature_k
        .map(|v| format!("T={v:.2} K"))
        .unwrap_or_else(|| "T=n/a".to_owned());
    let p = state
        .pressure_pa
        .map(|v| format!("P={v:.2} Pa"))
        .unwrap_or_else(|| "P=n/a".to_owned());
    let g = state
        .gamma
        .map(|v| format!("γ={v:.5}"))
        .unwrap_or_else(|| "γ=n/a".to_owned());
    let mw = state
        .molecular_weight_kg_per_kmol
        .map(|v| format!("MW={v:.4} kg/kmol"))
        .unwrap_or_else(|| "MW=n/a".to_owned());
    format!("{t} | {p} | {g} | {mw}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_roundtrip_preserves_assumptions_study_propellant_and_geometry_config() {
        let workspace = RocketWorkspace {
            selected_subtab: RocketSubtab::Studies,
            performance_case: RocketPerformanceCase {
                nozzle_chemistry_model: NozzleChemistryModel::FrozenAtChamber,
                nozzle_constraint: NozzleConstraint::ExitPressurePa(50_000.0),
                combustor_model: CombustorModel::FiniteArea {
                    contraction_ratio: 2.5,
                },
                ..RocketPerformanceCase::default()
            },
            studies: RocketStudyConfig {
                variable: StudyVariable::AmbientPressurePa,
                min: 50_000.0,
                max: 150_000.0,
                point_count: 7,
                selected_metrics: vec![
                    StudyOutputMetric::SpecificImpulseAmbS,
                    StudyOutputMetric::ChamberToAmbientPressureRatio,
                ],
            },
            propellants: RocketPropellantConfig {
                selected_preset_key: "lox-ch4".to_owned(),
                search_query: "meth".to_owned(),
            },
            geometry: RocketGeometryConfig {
                sizing_mode: GeometrySizingMode::GivenThroatArea,
                nozzle_contour_style: NozzleContourStyle::BellParabolic,
                throat_input_value: 0.018,
                chamber_contraction_ratio: 2.8,
                characteristic_length_m: 1.05,
                nozzle_half_angle_deg: 12.0,
                nozzle_truncation_ratio: 0.95,
            },
            thermal: RocketThermalConfig {
                model: ThermalModel::BartzLikeConvective,
                cooling_mode: CoolingMode::Regenerative,
                reference_recovery_temperature_k: 3350.0,
                wall_temperature_k: 850.0,
                reference_gas_side_htc_w_m2_k: 22_000.0,
                show_series_heat_flux: true,
                show_series_htc: false,
                show_series_recovery_temperature: true,
                ..RocketThermalConfig::default()
            },
            ..RocketWorkspace::default()
        };

        let def = workspace.to_def();
        let restored = RocketWorkspace::from_def(&def);
        assert_eq!(restored.selected_subtab, RocketSubtab::Studies);
        assert_eq!(
            restored.performance_case.nozzle_chemistry_model,
            NozzleChemistryModel::FrozenAtChamber
        );
        assert_eq!(
            restored.performance_case.nozzle_constraint,
            NozzleConstraint::ExitPressurePa(50_000.0)
        );
        assert_eq!(
            restored.performance_case.combustor_model,
            CombustorModel::FiniteArea {
                contraction_ratio: 2.5
            }
        );
        assert_eq!(restored.studies.variable, StudyVariable::AmbientPressurePa);
        assert_eq!(restored.studies.point_count, 7);
        assert_eq!(restored.studies.selected_metrics.len(), 2);
        assert_eq!(restored.propellants.selected_preset_key, "lox-ch4");
        assert_eq!(restored.propellants.search_query, "meth");
        assert_eq!(
            restored.geometry.sizing_mode,
            GeometrySizingMode::GivenThroatArea
        );
        assert_eq!(
            restored.geometry.nozzle_contour_style,
            NozzleContourStyle::BellParabolic
        );
        assert!((restored.geometry.throat_input_value - 0.018).abs() < 1.0e-12);
        assert!((restored.geometry.nozzle_truncation_ratio - 0.95).abs() < 1.0e-12);
        assert_eq!(restored.thermal.cooling_mode, CoolingMode::Regenerative);
        assert!((restored.thermal.wall_temperature_k - 850.0).abs() < 1.0e-12);
        assert!(!restored.thermal.show_series_htc);
    }

    #[test]
    fn propellant_search_and_apply_to_performance_works() {
        let mut workspace = RocketWorkspace::default();
        workspace.propellants.search_query = "nitrous".to_owned();
        let filtered = workspace.filtered_propellant_presets();
        assert!(!filtered.is_empty());

        workspace.propellants.selected_preset_key = "n2o-ipa".to_owned();
        let message = workspace
            .apply_selected_propellant_to_performance()
            .expect("preset");
        assert!(message.contains("Applied"));
        assert_eq!(workspace.performance_case.oxidizer_name, "N2O");
        assert_eq!(workspace.performance_case.fuel_name, "C3H8");
        assert!(workspace.performance_case.mixture_ratio > 0.0);
    }

    #[test]
    fn data_snapshot_classifies_provenance_and_warnings() {
        let mut workspace = RocketWorkspace::default();
        workspace.performance_case.nozzle_chemistry_model = NozzleChemistryModel::FrozenAtThroat;
        workspace.performance_case.nozzle_constraint = NozzleConstraint::ExitPressurePa(60_000.0);

        let snapshot = workspace.data_snapshot();
        assert!(
            snapshot
                .warnings
                .iter()
                .any(|w| w.contains("Frozen-at-throat chemistry"))
        );
        assert!(
            snapshot
                .warnings
                .iter()
                .any(|w| w.contains("Exit-pressure constrained mode"))
        );
        assert!(
            snapshot
                .provenance_fields
                .iter()
                .any(|f| matches!(f.source, DataFieldSource::Unavailable))
        );
        assert!(
            snapshot
                .provenance_fields
                .iter()
                .any(|f| f.name.contains("Peak thermal heat flux"))
        );
    }
}
