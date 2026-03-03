//! Project schema definitions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub version: u32,
    pub name: String,
    #[serde(default)]
    pub systems: Vec<SystemDef>,
    #[serde(default)]
    pub modules: Vec<ModuleDef>,
    #[serde(default)]
    pub layouts: Vec<LayoutDef>,
    #[serde(default)]
    pub runs: RunLibraryDef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plotting_workspace: Option<PlottingWorkspaceDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fluid_workspace: Option<FluidWorkspaceDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rocket_workspace: Option<RocketWorkspaceDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemDef {
    pub id: String,
    pub name: String,
    pub fluid: FluidDef,
    #[serde(default)]
    pub nodes: Vec<NodeDef>,
    #[serde(default)]
    pub components: Vec<ComponentDef>,
    #[serde(default)]
    pub boundaries: Vec<BoundaryDef>,
    #[serde(default)]
    pub schedules: Vec<ScheduleDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<ControlSystemDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlSystemDef {
    #[serde(default)]
    pub blocks: Vec<ControlBlockDef>,
    #[serde(default)]
    pub connections: Vec<ControlConnectionDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlBlockDef {
    pub id: String,
    pub name: String,
    pub kind: ControlBlockKindDef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ControlBlockKindDef {
    Constant {
        value: f64,
    },
    MeasuredVariable {
        reference: MeasuredVariableDef,
    },
    PIController {
        kp: f64,
        ti_s: f64,
        out_min: f64,
        out_max: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        integral_limit: Option<f64>,
        sample_period_s: f64,
    },
    PIDController {
        kp: f64,
        ti_s: f64,
        td_s: f64,
        td_filter_s: f64,
        out_min: f64,
        out_max: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        integral_limit: Option<f64>,
        sample_period_s: f64,
    },
    FirstOrderActuator {
        tau_s: f64,
        rate_limit_per_s: f64,
        #[serde(default = "default_actuator_initial_position")]
        initial_position: f64,
    },
    ActuatorCommand {
        component_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MeasuredVariableDef {
    NodePressure {
        node_id: String,
    },
    NodeTemperature {
        node_id: String,
    },
    EdgeMassFlow {
        component_id: String,
    },
    PressureDrop {
        from_node_id: String,
        to_node_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlConnectionDef {
    pub from_block_id: String,
    pub to_block_id: String,
    pub to_input: String,
}

fn default_actuator_initial_position() -> f64 {
    0.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FluidDef {
    pub composition: CompositionDef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum CompositionDef {
    Pure { species: String },
    Mixture { fractions: Vec<(String, f64)> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeDef {
    pub id: String,
    pub name: String,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum NodeKind {
    Junction,
    ControlVolume {
        volume_m3: f64,
        #[serde(default)]
        initial: InitialCvDef,
    },
    Atmosphere {
        pressure_pa: f64,
        temperature_k: f64,
    },
}

/// Control volume initial condition specification.
///
/// Supports explicit mode-based initialization (preferred) and backward-compatible
/// optional-field syntax (for migration).
///
/// Explicit modes (preferred):
/// ```yaml
/// initial:
///   mode: PT       # or PH, mT, mH
///   p_pa: 3500000.0
///   t_k: 300.0
/// ```
///
/// Backward-compatible syntax (deprecated; requires validation):
/// ```yaml
/// initial:
///   p_pa: 300000.0
///   t_k: 300.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InitialCvDef {
    /// Explicit initialization mode. If present, only relevant fields for that mode are used.
    /// If absent, the system will attempt to infer the mode from provided optional fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>, // "PT", "PH", "mT", "mH"

    // Mode-specific parameters (all optional for backward compat)
    pub p_pa: Option<f64>,
    pub t_k: Option<f64>,
    pub h_j_per_kg: Option<f64>,
    pub m_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentDef {
    pub id: String,
    pub name: String,
    pub kind: ComponentKind,
    pub from_node_id: String,
    pub to_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ComponentKind {
    Orifice {
        cd: f64,
        area_m2: f64,
        treat_as_gas: bool,
    },
    Valve {
        cd: f64,
        area_max_m2: f64,
        position: f64,
        law: ValveLawDef,
        treat_as_gas: bool,
    },
    Pipe {
        length_m: f64,
        diameter_m: f64,
        roughness_m: f64,
        k_minor: f64,
        mu_pa_s: f64,
    },
    Pump {
        cd: f64,
        area_m2: f64,
        delta_p_pa: f64,
        eta: f64,
        treat_as_liquid: bool,
    },
    Turbine {
        cd: f64,
        area_m2: f64,
        eta: f64,
        treat_as_gas: bool,
    },
    LineVolume {
        volume_m3: f64,
        #[serde(default)]
        cd: f64,
        #[serde(default)]
        area_m2: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ValveLawDef {
    Linear,
    Quadratic,
    QuickOpening,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundaryDef {
    pub node_id: String,
    pub pressure_pa: Option<f64>,
    pub temperature_k: Option<f64>,
    pub enthalpy_j_per_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleDef {
    pub id: String,
    pub name: String,
    pub events: Vec<EventDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventDef {
    pub time_s: f64,
    pub action: ActionDef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ActionDef {
    SetValvePosition { component_id: String, position: f64 },
    SetBoundaryPressure { node_id: String, pressure_pa: f64 },
    SetBoundaryTemperature { node_id: String, temperature_k: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDef {
    pub id: String,
    pub name: String,
    pub interface: ModuleInterfaceDef,
    pub template_system_id: Option<String>,
    #[serde(default)]
    pub exposed_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleInterfaceDef {
    #[serde(default)]
    pub inputs: Vec<PortDef>,
    #[serde(default)]
    pub outputs: Vec<PortDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortDef {
    pub name: String,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlBlockLayout {
    pub block_id: String,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub label_offset_x: f32,
    #[serde(default)]
    pub label_offset_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalConnectionRoute {
    #[serde(default)]
    pub from_block_id: String,
    #[serde(default)]
    pub to_block_id: String,
    #[serde(default)]
    pub to_input: String,
    #[serde(default)]
    pub points: Vec<RoutePointDef>,
    #[serde(default)]
    pub label_offset_x: f32,
    #[serde(default)]
    pub label_offset_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayoutDef {
    pub system_id: String,
    #[serde(default)]
    pub nodes: Vec<NodeLayout>,
    #[serde(default)]
    pub edges: Vec<EdgeLayout>,
    #[serde(default)]
    pub control_blocks: Vec<ControlBlockLayout>,
    #[serde(default)]
    pub signal_connections: Vec<SignalConnectionRoute>,
    #[serde(default)]
    pub overlay: OverlaySettingsDef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeLayout {
    pub node_id: String,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub label_offset_x: f32,
    #[serde(default)]
    pub label_offset_y: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlay: Option<NodeOverlayDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NodeOverlayDef {
    #[serde(default)]
    pub show_pressure: bool,
    #[serde(default)]
    pub show_temperature: bool,
    #[serde(default)]
    pub show_enthalpy: bool,
    #[serde(default)]
    pub show_density: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeLayout {
    pub component_id: String,
    #[serde(default)]
    pub points: Vec<RoutePointDef>,
    pub label_offset_x: f32,
    pub label_offset_y: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_x: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_y: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutePointDef {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OverlaySettingsDef {
    pub show_pressure: bool,
    pub show_temperature: bool,
    pub show_enthalpy: bool,
    pub show_density: bool,
    pub show_mass_flow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RunLibraryDef {
    #[serde(default)]
    pub runs: Vec<RunMetadataDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunMetadataDef {
    pub run_id: String,
    pub system_id: String,
    pub timestamp: String,
    pub run_type: RunTypeDef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum RunTypeDef {
    Steady,
    Transient { dt_s: f64, t_end_s: f64 },
}

/// Persistent plotting workspace configuration.
/// Stores all plot panels, their positions, sizes, and series selections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PlottingWorkspaceDef {
    #[serde(default)]
    pub panels: Vec<PlotPanelDef>,
    #[serde(default)]
    pub templates: Vec<PlotTemplateDef>,
    /// Width of the plot workspace area in pixels
    #[serde(default)]
    pub workspace_width: f32,
    /// Height of the plot workspace area in pixels
    #[serde(default)]
    pub workspace_height: f32,
}

/// A single plot panel in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlotPanelDef {
    /// Unique identifier for the panel
    pub id: String,
    /// User-defined name/title for the plot
    pub title: String,
    /// X position in the workspace (in pixels)
    pub x: f32,
    /// Y position in the workspace (in pixels)
    pub y: f32,
    /// Width of the panel (in pixels)
    pub width: f32,
    /// Height of the panel (in pixels)
    pub height: f32,
    /// Which run this plot is displaying data from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Series selection for this plot
    pub series_selection: PlotSeriesSelectionDef,
}

/// Generic arbitrary curve source  (valve characteristics, actuator responses, fluid property sweeps).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ArbitraryCurveSourceDef {
    ValveCharacteristic {
        component_id: String,
        characteristic: ValveCharacteristicKindDef,
        #[serde(default = "default_curve_sample_count")]
        sample_count: usize,
    },
    ActuatorResponse {
        tau_s: f64,
        rate_limit_per_s: f64,
        #[serde(default)]
        initial_position: f64,
        #[serde(default = "default_step_command")]
        command: f64,
        #[serde(default = "default_response_duration")]
        duration_s: f64,
        #[serde(default = "default_curve_sample_count")]
        sample_count: usize,
    },
    FluidPropertySweep {
        x_property: String,
        y_property: String,
        parameters: FluidSweepParametersDef,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValveCharacteristicKindDef {
    EffectiveArea,
    OpeningFactor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FluidSweepParametersDef {
    /// Independent variable being swept (e.g., "Temperature", "Pressure")
    pub sweep_variable: String,
    /// Start value with unit (e.g., "300K", "1bar")
    pub start_value: String,
    /// End value with unit (e.g., "400K", "10bar")
    pub end_value: String,
    /// Number of points to generate in the sweep
    #[serde(default = "default_sweep_points")]
    pub num_points: usize,
    /// Spacing type: "Linear" or "Logarithmic"
    #[serde(default = "default_sweep_type")]
    pub sweep_type: String,
    /// Fixed fluid species for the sweep (e.g., "N2", "H2O")
    pub species: String,
    /// Secondary fixed property (e.g., if sweeping temperature, might fix pressure)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_property: Option<FixedPropertyDef>,
}

impl Default for FluidSweepParametersDef {
    fn default() -> Self {
        Self {
            sweep_variable: "Temperature".to_string(),
            start_value: "300K".to_string(),
            end_value: "400K".to_string(),
            num_points: default_sweep_points(),
            sweep_type: default_sweep_type(),
            species: "N2".to_string(),
            fixed_property: Some(FixedPropertyDef {
                property_name: "Pressure".to_string(),
                value: "101325Pa".to_string(),
            }),
        }
    }
}

/// Fixed property definition for sweeps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixedPropertyDef {
    /// Property name (e.g., "Pressure", "Temperature")
    pub property_name: String,
    /// Value with unit (e.g., "101325Pa", "300K")
    pub value: String,
}

fn default_sweep_points() -> usize {
    50
}

fn default_sweep_type() -> String {
    "Linear".to_string()
}

fn default_curve_sample_count() -> usize {
    100
}

fn default_step_command() -> f64 {
    1.0
}

fn default_response_duration() -> f64 {
    5.0
}

/// Specifies which series are shown in a plot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PlotSeriesSelectionDef {
    #[serde(default)]
    pub node_ids_and_variables: Vec<NodePlotVariableDef>,
    #[serde(default)]
    pub component_ids_and_variables: Vec<ComponentPlotVariableDef>,
    #[serde(default)]
    pub control_ids: Vec<String>,
    /// Arbitrary curve sources (valve characteristics, actuator responses, etc.)
    #[serde(default)]
    pub arbitrary_curves: Vec<ArbitraryCurveSourceDef>,
}

/// A node series to plot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodePlotVariableDef {
    pub node_id: String,
    pub variable: String, // "Pressure", "Temperature", "Enthalpy", "Density"
}

/// A component series to plot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentPlotVariableDef {
    pub component_id: String,
    pub variable: String, // "MassFlow", "PressureDrop"
}

/// A reusable plot template/preset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlotTemplateDef {
    /// Unique identifier for the template
    pub id: String,
    /// User-defined name for the template
    pub name: String,
    /// Optional description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Series selection that this template defines
    pub series_selection: PlotSeriesSelectionDef,
    /// Default width for plots created from this template
    #[serde(default)]
    pub default_width: f32,
    /// Default height for plots created from this template
    #[serde(default)]
    pub default_height: f32,
}

/// Persistent fluid workspace configuration for multi-column fluid comparison and analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FluidWorkspaceDef {
    /// Collection of fluid cases for comparison
    #[serde(default)]
    pub cases: Vec<FluidCaseDef>,
}

impl Default for FluidWorkspaceDef {
    fn default() -> Self {
        Self {
            cases: vec![FluidCaseDef::default()],
        }
    }
}

/// Single fluid case definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FluidCaseDef {
    /// Unique identifier for this case
    pub id: String,
    /// Selected fluid species key (e.g. "N2", "H2O").
    pub species: String,
    /// Explicit state mode: "Specified" or "Equilibrium"
    #[serde(default = "default_state_mode")]
    pub state_mode: String,
    /// For Equilibrium mode: saturation interpretation: "Temperature", "Pressure", or "Quality"
    #[serde(default = "default_saturation_mode")]
    pub saturation_mode: String,
    /// Selected input pair (only used in Specified mode).
    pub input_pair: FluidInputPairDef,
    /// First input value (meaning depends on pair/mode).
    pub input_1: f64,
    /// Second input value (meaning depends on pair/mode).
    pub input_2: f64,
    /// Optional quality for two-phase disambiguation (0.0 = sat. liquid, 1.0 = sat. vapor)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<f64>,
}

fn default_state_mode() -> String {
    "Specified".to_string()
}

fn default_saturation_mode() -> String {
    "Temperature".to_string()
}

impl Default for FluidCaseDef {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            species: "N2".to_string(),
            state_mode: default_state_mode(),
            saturation_mode: default_saturation_mode(),
            input_pair: FluidInputPairDef::PT,
            input_1: 101_325.0,
            input_2: 300.0,
            quality: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FluidInputPairDef {
    #[default]
    PT,
    PH,
    RhoH,
    PS,
}

/// Persistent rocket workspace configuration for the Rocket app surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketWorkspaceDef {
    /// Selected Rocket subtab label (e.g. "Performance", "Geometry")
    #[serde(default = "default_rocket_subtab")]
    pub selected_subtab: String,
    /// Current Performance case inputs
    #[serde(default)]
    pub performance_case: RocketPerformanceCaseDef,
    /// Study configuration for Rocket > Studies
    #[serde(default)]
    pub study: RocketStudyDef,
    /// Propellant workspace selection state
    #[serde(default)]
    pub propellants: RocketPropellantWorkspaceDef,
    /// Geometry subtab sizing configuration
    #[serde(default)]
    pub geometry: RocketGeometryDef,
    /// Thermal subtab configuration
    #[serde(default)]
    pub thermal: RocketThermalDef,
}

fn default_rocket_subtab() -> String {
    "Performance".to_string()
}

impl Default for RocketWorkspaceDef {
    fn default() -> Self {
        Self {
            selected_subtab: default_rocket_subtab(),
            performance_case: RocketPerformanceCaseDef::default(),
            study: RocketStudyDef::default(),
            propellants: RocketPropellantWorkspaceDef::default(),
            geometry: RocketGeometryDef::default(),
            thermal: RocketThermalDef::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketPerformanceCaseDef {
    #[serde(default = "default_rocket_case_name")]
    pub case_name: String,
    #[serde(default = "default_rocket_oxidizer_name")]
    pub oxidizer_name: String,
    #[serde(default = "default_rocket_fuel_name")]
    pub fuel_name: String,
    #[serde(default = "default_rocket_oxidizer_temperature_k")]
    pub oxidizer_temperature_k: f64,
    #[serde(default = "default_rocket_fuel_temperature_k")]
    pub fuel_temperature_k: f64,
    #[serde(default = "default_rocket_mixture_ratio")]
    pub mixture_ratio: f64,
    #[serde(default)]
    pub use_optimal_mixture_ratio: bool,
    #[serde(default = "default_rocket_mixture_ratio")]
    pub optimal_mixture_ratio_value: f64,
    #[serde(default = "default_rocket_chamber_pressure_pa")]
    pub chamber_pressure_pa: f64,
    #[serde(default = "default_rocket_ambient_pressure_pa")]
    pub ambient_pressure_pa: f64,
    #[serde(default)]
    pub combustor_model: RocketCombustorModelDef,
    #[serde(default)]
    pub nozzle_chemistry_model: RocketNozzleChemistryModelDef,
    #[serde(default)]
    pub nozzle_constraint: RocketNozzleConstraintDef,
}

impl Default for RocketPerformanceCaseDef {
    fn default() -> Self {
        Self {
            case_name: default_rocket_case_name(),
            oxidizer_name: default_rocket_oxidizer_name(),
            fuel_name: default_rocket_fuel_name(),
            oxidizer_temperature_k: default_rocket_oxidizer_temperature_k(),
            fuel_temperature_k: default_rocket_fuel_temperature_k(),
            mixture_ratio: default_rocket_mixture_ratio(),
            use_optimal_mixture_ratio: false,
            optimal_mixture_ratio_value: default_rocket_mixture_ratio(),
            chamber_pressure_pa: default_rocket_chamber_pressure_pa(),
            ambient_pressure_pa: default_rocket_ambient_pressure_pa(),
            combustor_model: RocketCombustorModelDef::default(),
            nozzle_chemistry_model: RocketNozzleChemistryModelDef::default(),
            nozzle_constraint: RocketNozzleConstraintDef::default(),
        }
    }
}

fn default_rocket_case_name() -> String {
    "LOX/RP-1 baseline".to_string()
}
fn default_rocket_oxidizer_name() -> String {
    "LOX".to_string()
}
fn default_rocket_fuel_name() -> String {
    "RP-1".to_string()
}
fn default_rocket_oxidizer_temperature_k() -> f64 {
    90.0
}
fn default_rocket_fuel_temperature_k() -> f64 {
    293.0
}
fn default_rocket_mixture_ratio() -> f64 {
    2.6
}
fn default_rocket_chamber_pressure_pa() -> f64 {
    7_000_000.0
}
fn default_rocket_ambient_pressure_pa() -> f64 {
    101_325.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RocketCombustorModelDef {
    #[default]
    InfiniteArea,
    FiniteArea {
        contraction_ratio: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketNozzleChemistryModelDef {
    #[default]
    ShiftingEquilibrium,
    FrozenAtChamber,
    FrozenAtThroat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum RocketNozzleConstraintDef {
    ExpansionRatio(f64),
    ExitPressurePa(f64),
}

impl Default for RocketNozzleConstraintDef {
    fn default() -> Self {
        Self::ExpansionRatio(40.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketStudyDef {
    #[serde(default)]
    pub variable: RocketStudyVariableDef,
    #[serde(default = "default_rocket_study_min")]
    pub min: f64,
    #[serde(default = "default_rocket_study_max")]
    pub max: f64,
    #[serde(default = "default_rocket_study_point_count")]
    pub point_count: usize,
    #[serde(default = "default_rocket_study_metrics")]
    pub metrics: Vec<RocketStudyMetricDef>,
}

impl Default for RocketStudyDef {
    fn default() -> Self {
        Self {
            variable: RocketStudyVariableDef::default(),
            min: default_rocket_study_min(),
            max: default_rocket_study_max(),
            point_count: default_rocket_study_point_count(),
            metrics: default_rocket_study_metrics(),
        }
    }
}

fn default_rocket_study_min() -> f64 {
    5_000_000.0
}

fn default_rocket_study_max() -> f64 {
    10_000_000.0
}

fn default_rocket_study_point_count() -> usize {
    11
}

fn default_rocket_study_metrics() -> Vec<RocketStudyMetricDef> {
    vec![
        RocketStudyMetricDef::SpecificImpulseVacS,
        RocketStudyMetricDef::SpecificImpulseAmbS,
        RocketStudyMetricDef::ThrustCoefficientVac,
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketStudyVariableDef {
    #[default]
    ChamberPressurePa,
    MixtureRatio,
    AmbientPressurePa,
    ExpansionRatio,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RocketStudyMetricDef {
    ChamberTemperatureK,
    ChamberGamma,
    ChamberMolecularWeightKgPerKmol,
    CharacteristicVelocityMPerS,
    ThrustCoefficientVac,
    SpecificImpulseVacS,
    SpecificImpulseAmbS,
    EffectiveExhaustVelocityVacMPerS,
    EffectiveExhaustVelocityAmbMPerS,
    ChamberToAmbientPressureRatio,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketGeometryDef {
    #[serde(default)]
    pub sizing_mode: RocketGeometrySizingModeDef,
    #[serde(default)]
    pub nozzle_contour_style: RocketNozzleContourStyleDef,
    #[serde(default = "default_rocket_geometry_throat_input_value")]
    pub throat_input_value: f64,
    #[serde(default = "default_rocket_geometry_contraction_ratio")]
    pub chamber_contraction_ratio: f64,
    #[serde(default = "default_rocket_geometry_characteristic_length_m")]
    pub characteristic_length_m: f64,
    #[serde(default = "default_rocket_geometry_nozzle_half_angle_deg")]
    pub nozzle_half_angle_deg: f64,
    #[serde(default = "default_rocket_geometry_nozzle_truncation_ratio")]
    pub nozzle_truncation_ratio: f64,
}

impl Default for RocketGeometryDef {
    fn default() -> Self {
        Self {
            sizing_mode: RocketGeometrySizingModeDef::default(),
            nozzle_contour_style: RocketNozzleContourStyleDef::default(),
            throat_input_value: default_rocket_geometry_throat_input_value(),
            chamber_contraction_ratio: default_rocket_geometry_contraction_ratio(),
            characteristic_length_m: default_rocket_geometry_characteristic_length_m(),
            nozzle_half_angle_deg: default_rocket_geometry_nozzle_half_angle_deg(),
            nozzle_truncation_ratio: default_rocket_geometry_nozzle_truncation_ratio(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketGeometrySizingModeDef {
    #[default]
    GivenThroatDiameter,
    GivenThroatArea,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketNozzleContourStyleDef {
    #[default]
    Conical,
    BellParabolic,
    TruncatedIdeal,
}

fn default_rocket_geometry_throat_input_value() -> f64 {
    0.12
}

fn default_rocket_geometry_contraction_ratio() -> f64 {
    3.0
}

fn default_rocket_geometry_characteristic_length_m() -> f64 {
    1.2
}

fn default_rocket_geometry_nozzle_half_angle_deg() -> f64 {
    15.0
}

fn default_rocket_geometry_nozzle_truncation_ratio() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketThermalDef {
    #[serde(default)]
    pub model: RocketThermalModelDef,
    #[serde(default)]
    pub cooling_mode: RocketThermalWorkspaceCoolingModeDef,
    #[serde(default = "default_rocket_thermal_recovery_temperature_k")]
    pub reference_recovery_temperature_k: f64,
    #[serde(default = "default_rocket_thermal_wall_temperature_k")]
    pub wall_temperature_k: f64,
    #[serde(default = "default_rocket_thermal_reference_htc")]
    pub reference_gas_side_htc_w_m2_k: f64,
    #[serde(default = "default_rocket_selected_material_name")]
    pub selected_material_name: String,
    #[serde(default = "default_rocket_material_library")]
    pub material_library: Vec<RocketThermalMaterialDef>,
    #[serde(default = "default_true")]
    pub use_performance_mixture_ratio: bool,
    #[serde(default = "default_rocket_mixture_ratio")]
    pub specified_mixture_ratio: f64,
    #[serde(default = "default_true")]
    pub use_coolant_properties_from_fluids: bool,
    #[serde(default = "default_rocket_selected_coolant_species")]
    pub selected_coolant_species: String,
    #[serde(default = "default_true")]
    pub use_engine_propellant_coolant: bool,
    #[serde(default = "default_rocket_coolant_fuel_multiplier")]
    pub coolant_fuel_multiplier: f64,
    #[serde(default = "default_rocket_coolant_oxidizer_multiplier")]
    pub coolant_oxidizer_multiplier: f64,
    #[serde(default)]
    pub include_film_in_propellant_coolant_mix: bool,
    #[serde(default)]
    pub coolant_property_override: bool,
    #[serde(default = "default_true")]
    pub show_series_heat_flux: bool,
    #[serde(default = "default_true")]
    pub show_series_htc: bool,
    #[serde(default)]
    pub show_series_recovery_temperature: bool,
    #[serde(default)]
    pub wall: RocketWallModelDef,
    #[serde(default)]
    pub coolant: RocketCoolantModelDef,
    #[serde(default)]
    pub film: RocketFilmCoolingDef,
    #[serde(default)]
    pub channels: RocketChannelGeometryDef,
    #[serde(default)]
    pub design: RocketThermalDesignSettingsDef,
    #[serde(default = "default_true")]
    pub show_series_wall_temperature: bool,
    #[serde(default = "default_true")]
    pub show_series_coolant_temperature: bool,
    #[serde(default = "default_true")]
    pub show_series_coolant_pressure: bool,
    #[serde(default = "default_true")]
    pub show_series_channel_height: bool,
    #[serde(default = "default_true")]
    pub show_series_film_effectiveness: bool,
}

impl Default for RocketThermalDef {
    fn default() -> Self {
        Self {
            model: RocketThermalModelDef::default(),
            cooling_mode: RocketThermalWorkspaceCoolingModeDef::default(),
            reference_recovery_temperature_k: default_rocket_thermal_recovery_temperature_k(),
            wall_temperature_k: default_rocket_thermal_wall_temperature_k(),
            reference_gas_side_htc_w_m2_k: default_rocket_thermal_reference_htc(),
            selected_material_name: default_rocket_selected_material_name(),
            material_library: default_rocket_material_library(),
            use_performance_mixture_ratio: true,
            specified_mixture_ratio: default_rocket_mixture_ratio(),
            use_coolant_properties_from_fluids: true,
            selected_coolant_species: default_rocket_selected_coolant_species(),
            use_engine_propellant_coolant: true,
            coolant_fuel_multiplier: default_rocket_coolant_fuel_multiplier(),
            coolant_oxidizer_multiplier: default_rocket_coolant_oxidizer_multiplier(),
            include_film_in_propellant_coolant_mix: false,
            coolant_property_override: false,
            show_series_heat_flux: true,
            show_series_htc: true,
            show_series_recovery_temperature: false,
            wall: RocketWallModelDef::default(),
            coolant: RocketCoolantModelDef::default(),
            film: RocketFilmCoolingDef::default(),
            channels: RocketChannelGeometryDef::default(),
            design: RocketThermalDesignSettingsDef::default(),
            show_series_wall_temperature: true,
            show_series_coolant_temperature: true,
            show_series_coolant_pressure: true,
            show_series_channel_height: true,
            show_series_film_effectiveness: true,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketThermalModelDef {
    #[default]
    BartzLikeConvective,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketThermalWorkspaceCoolingModeDef {
    #[default]
    AdiabaticWall,
    Regenerative,
    Film,
    RegenerativeFilm,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RocketCoolantFlowDirectionDef {
    #[default]
    CoFlow,
    CounterFlow,
    MidFeed,
}

fn default_rocket_thermal_recovery_temperature_k() -> f64 {
    3400.0
}

fn default_rocket_thermal_wall_temperature_k() -> f64 {
    900.0
}

fn default_rocket_thermal_reference_htc() -> f64 {
    25_000.0
}

fn default_rocket_selected_material_name() -> String {
    "CuCrZr".to_string()
}

fn default_rocket_selected_coolant_species() -> String {
    "N2".to_string()
}
fn default_rocket_coolant_fuel_multiplier() -> f64 {
    1.0
}
fn default_rocket_coolant_oxidizer_multiplier() -> f64 {
    0.0
}

fn default_rocket_material_library() -> Vec<RocketThermalMaterialDef> {
    vec![
        RocketThermalMaterialDef {
            name: "CuCrZr".to_string(),
            k_reference_w_m_k: 320.0,
            k_reference_temperature_k: 300.0,
            k_temp_coeff_per_k: -0.0002,
            allowable_temperature_k: 1100.0,
            density_kg_m3: 8900.0,
            cp_j_kg_k: 385.0,
        },
        RocketThermalMaterialDef {
            name: "Copper (OFHC)".to_string(),
            k_reference_w_m_k: 390.0,
            k_reference_temperature_k: 300.0,
            k_temp_coeff_per_k: -0.0004,
            allowable_temperature_k: 1000.0,
            density_kg_m3: 8960.0,
            cp_j_kg_k: 385.0,
        },
        RocketThermalMaterialDef {
            name: "Inconel 718".to_string(),
            k_reference_w_m_k: 13.0,
            k_reference_temperature_k: 300.0,
            k_temp_coeff_per_k: 0.00035,
            allowable_temperature_k: 1250.0,
            density_kg_m3: 8190.0,
            cp_j_kg_k: 435.0,
        },
        RocketThermalMaterialDef {
            name: "Stainless 304".to_string(),
            k_reference_w_m_k: 16.0,
            k_reference_temperature_k: 300.0,
            k_temp_coeff_per_k: 0.00025,
            allowable_temperature_k: 1050.0,
            density_kg_m3: 8000.0,
            cp_j_kg_k: 500.0,
        },
        RocketThermalMaterialDef {
            name: "Niobium C-103".to_string(),
            k_reference_w_m_k: 54.0,
            k_reference_temperature_k: 300.0,
            k_temp_coeff_per_k: -0.0001,
            allowable_temperature_k: 1650.0,
            density_kg_m3: 8850.0,
            cp_j_kg_k: 270.0,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketWallModelDef {
    #[serde(default = "default_rocket_wall_material_name")]
    pub material_name: String,
    #[serde(default = "default_rocket_wall_k")]
    pub thermal_conductivity_w_m_k: f64,
    #[serde(default = "default_rocket_wall_k_ref_t")]
    pub thermal_conductivity_reference_temperature_k: f64,
    #[serde(default = "default_rocket_wall_k_temp_coeff")]
    pub thermal_conductivity_temp_coeff_per_k: f64,
    #[serde(default = "default_rocket_wall_emissivity")]
    pub gas_side_emissivity: f64,
    #[serde(default = "default_rocket_wall_allowable_t")]
    pub allowable_temperature_k: f64,
    #[serde(default = "default_rocket_wall_density")]
    pub density_kg_m3: f64,
    #[serde(default = "default_rocket_wall_cp")]
    pub cp_j_kg_k: f64,
    #[serde(default = "default_rocket_wall_thickness")]
    pub thickness_m: f64,
}

impl Default for RocketWallModelDef {
    fn default() -> Self {
        Self {
            material_name: default_rocket_wall_material_name(),
            thermal_conductivity_w_m_k: default_rocket_wall_k(),
            thermal_conductivity_reference_temperature_k: default_rocket_wall_k_ref_t(),
            thermal_conductivity_temp_coeff_per_k: default_rocket_wall_k_temp_coeff(),
            gas_side_emissivity: default_rocket_wall_emissivity(),
            allowable_temperature_k: default_rocket_wall_allowable_t(),
            density_kg_m3: default_rocket_wall_density(),
            cp_j_kg_k: default_rocket_wall_cp(),
            thickness_m: default_rocket_wall_thickness(),
        }
    }
}

fn default_rocket_wall_material_name() -> String {
    "CuCrZr (representative)".to_string()
}
fn default_rocket_wall_k() -> f64 {
    320.0
}
fn default_rocket_wall_k_ref_t() -> f64 {
    300.0
}
fn default_rocket_wall_k_temp_coeff() -> f64 {
    -0.0002
}
fn default_rocket_wall_emissivity() -> f64 {
    0.8
}
fn default_rocket_wall_allowable_t() -> f64 {
    1100.0
}
fn default_rocket_wall_density() -> f64 {
    8_900.0
}
fn default_rocket_wall_cp() -> f64 {
    385.0
}
fn default_rocket_wall_thickness() -> f64 {
    0.0025
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketThermalMaterialDef {
    #[serde(default = "default_rocket_wall_material_name")]
    pub name: String,
    #[serde(default = "default_rocket_wall_k")]
    pub k_reference_w_m_k: f64,
    #[serde(default = "default_rocket_wall_k_ref_t")]
    pub k_reference_temperature_k: f64,
    #[serde(default = "default_rocket_wall_k_temp_coeff")]
    pub k_temp_coeff_per_k: f64,
    #[serde(default = "default_rocket_wall_allowable_t")]
    pub allowable_temperature_k: f64,
    #[serde(default = "default_rocket_wall_density")]
    pub density_kg_m3: f64,
    #[serde(default = "default_rocket_wall_cp")]
    pub cp_j_kg_k: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketCoolantModelDef {
    #[serde(default = "default_rocket_coolant_name")]
    pub coolant_name: String,
    #[serde(default = "default_rocket_coolant_inlet_t")]
    pub inlet_temperature_k: f64,
    #[serde(default = "default_rocket_coolant_inlet_p")]
    pub inlet_pressure_pa: f64,
    #[serde(default = "default_rocket_coolant_mdot")]
    pub mass_flow_kg_s: f64,
    #[serde(default = "default_rocket_coolant_rho")]
    pub density_kg_m3: f64,
    #[serde(default = "default_rocket_coolant_mu")]
    pub viscosity_pa_s: f64,
    #[serde(default = "default_rocket_coolant_k")]
    pub thermal_conductivity_w_m_k: f64,
    #[serde(default = "default_rocket_coolant_cp")]
    pub cp_j_kg_k: f64,
}

impl Default for RocketCoolantModelDef {
    fn default() -> Self {
        Self {
            coolant_name: default_rocket_coolant_name(),
            inlet_temperature_k: default_rocket_coolant_inlet_t(),
            inlet_pressure_pa: default_rocket_coolant_inlet_p(),
            mass_flow_kg_s: default_rocket_coolant_mdot(),
            density_kg_m3: default_rocket_coolant_rho(),
            viscosity_pa_s: default_rocket_coolant_mu(),
            thermal_conductivity_w_m_k: default_rocket_coolant_k(),
            cp_j_kg_k: default_rocket_coolant_cp(),
        }
    }
}

fn default_rocket_coolant_name() -> String {
    "RP-1 (bulk property placeholder)".to_string()
}
fn default_rocket_coolant_inlet_t() -> f64 {
    300.0
}
fn default_rocket_coolant_inlet_p() -> f64 {
    8_000_000.0
}
fn default_rocket_coolant_mdot() -> f64 {
    8.0
}
fn default_rocket_coolant_rho() -> f64 {
    780.0
}
fn default_rocket_coolant_mu() -> f64 {
    0.0015
}
fn default_rocket_coolant_k() -> f64 {
    0.13
}
fn default_rocket_coolant_cp() -> f64 {
    2_200.0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketFilmCoolingDef {
    #[serde(default = "default_rocket_film_mass_fraction")]
    pub film_mass_fraction: f64,
    #[serde(default = "default_rocket_film_start")]
    pub effectiveness_start_fraction: f64,
    #[serde(default = "default_rocket_film_end")]
    pub effectiveness_end_fraction: f64,
    #[serde(default = "default_rocket_film_max_eff")]
    pub max_effectiveness: f64,
}

impl Default for RocketFilmCoolingDef {
    fn default() -> Self {
        Self {
            film_mass_fraction: default_rocket_film_mass_fraction(),
            effectiveness_start_fraction: default_rocket_film_start(),
            effectiveness_end_fraction: default_rocket_film_end(),
            max_effectiveness: default_rocket_film_max_eff(),
        }
    }
}

fn default_rocket_film_mass_fraction() -> f64 {
    0.03
}
fn default_rocket_film_start() -> f64 {
    0.0
}
fn default_rocket_film_end() -> f64 {
    0.55
}
fn default_rocket_film_max_eff() -> f64 {
    0.35
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketChannelGeometryDef {
    #[serde(default = "default_rocket_channel_count")]
    pub channel_count: usize,
    #[serde(default = "default_rocket_channel_width")]
    pub width_m: f64,
    #[serde(default = "default_rocket_channel_height")]
    pub height_m: f64,
    #[serde(default = "default_rocket_channel_rib")]
    pub rib_width_m: f64,
    #[serde(default = "default_rocket_channel_min_gap")]
    pub min_gap_m: f64,
    #[serde(default = "default_rocket_channel_roughness")]
    pub roughness_m: f64,
    #[serde(default = "default_rocket_channel_width_taper")]
    pub width_taper_end_factor: f64,
    #[serde(default = "default_rocket_channel_height_taper")]
    pub height_taper_end_factor: f64,
    #[serde(default = "default_rocket_channel_min_width")]
    pub min_width_m: f64,
    #[serde(default = "default_rocket_channel_max_width")]
    pub max_width_m: f64,
    #[serde(default = "default_rocket_channel_min_height")]
    pub min_height_m: f64,
    #[serde(default = "default_rocket_channel_max_height")]
    pub max_height_m: f64,
}

impl Default for RocketChannelGeometryDef {
    fn default() -> Self {
        Self {
            channel_count: default_rocket_channel_count(),
            width_m: default_rocket_channel_width(),
            height_m: default_rocket_channel_height(),
            rib_width_m: default_rocket_channel_rib(),
            min_gap_m: default_rocket_channel_min_gap(),
            roughness_m: default_rocket_channel_roughness(),
            width_taper_end_factor: default_rocket_channel_width_taper(),
            height_taper_end_factor: default_rocket_channel_height_taper(),
            min_width_m: default_rocket_channel_min_width(),
            max_width_m: default_rocket_channel_max_width(),
            min_height_m: default_rocket_channel_min_height(),
            max_height_m: default_rocket_channel_max_height(),
        }
    }
}

fn default_rocket_channel_count() -> usize {
    160
}
fn default_rocket_channel_width() -> f64 {
    0.0012
}
fn default_rocket_channel_height() -> f64 {
    0.0022
}
fn default_rocket_channel_rib() -> f64 {
    0.0008
}
fn default_rocket_channel_min_gap() -> f64 {
    0.0002
}
fn default_rocket_channel_roughness() -> f64 {
    4.0e-6
}
fn default_rocket_channel_width_taper() -> f64 {
    1.15
}
fn default_rocket_channel_height_taper() -> f64 {
    0.92
}
fn default_rocket_channel_min_width() -> f64 {
    0.0006
}
fn default_rocket_channel_max_width() -> f64 {
    0.0035
}
fn default_rocket_channel_min_height() -> f64 {
    0.0010
}
fn default_rocket_channel_max_height() -> f64 {
    0.0050
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketThermalDesignSettingsDef {
    #[serde(default = "default_rocket_thermal_station_count")]
    pub station_count: usize,
    #[serde(default = "default_rocket_thermal_max_dp")]
    pub max_coolant_pressure_drop_pa: f64,
    #[serde(default = "default_rocket_thermal_opt_iters")]
    pub optimizer_max_iterations: usize,
    #[serde(default = "default_rocket_thermal_local_gain")]
    pub optimizer_local_adjustment_gain: f64,
    #[serde(default = "default_rocket_thermal_global_gain")]
    pub optimizer_global_area_gain: f64,
    #[serde(default = "default_true")]
    pub hold_channel_count_fixed: bool,
    #[serde(default)]
    pub hold_channel_width_fixed: bool,
    #[serde(default)]
    pub hold_channel_height_fixed: bool,
    #[serde(default = "default_rocket_thermal_min_channel_count")]
    pub min_channel_count: usize,
    #[serde(default = "default_rocket_thermal_max_channel_count")]
    pub max_channel_count: usize,
    #[serde(default)]
    pub coolant_flow_direction: RocketCoolantFlowDirectionDef,
    #[serde(default = "default_rocket_thermal_mid_feed_fraction")]
    pub mid_feed_fraction: f64,
    #[serde(default = "default_rocket_thermal_mid_feed_upstream_split")]
    pub mid_feed_upstream_mass_fraction: f64,
    #[serde(default = "default_true")]
    pub auto_balance_mid_feed_split: bool,
}

impl Default for RocketThermalDesignSettingsDef {
    fn default() -> Self {
        Self {
            station_count: default_rocket_thermal_station_count(),
            max_coolant_pressure_drop_pa: default_rocket_thermal_max_dp(),
            optimizer_max_iterations: default_rocket_thermal_opt_iters(),
            optimizer_local_adjustment_gain: default_rocket_thermal_local_gain(),
            optimizer_global_area_gain: default_rocket_thermal_global_gain(),
            hold_channel_count_fixed: true,
            hold_channel_width_fixed: false,
            hold_channel_height_fixed: false,
            min_channel_count: default_rocket_thermal_min_channel_count(),
            max_channel_count: default_rocket_thermal_max_channel_count(),
            coolant_flow_direction: RocketCoolantFlowDirectionDef::default(),
            mid_feed_fraction: default_rocket_thermal_mid_feed_fraction(),
            mid_feed_upstream_mass_fraction: default_rocket_thermal_mid_feed_upstream_split(),
            auto_balance_mid_feed_split: true,
        }
    }
}

fn default_rocket_thermal_station_count() -> usize {
    61
}
fn default_rocket_thermal_max_dp() -> f64 {
    1_500_000.0
}
fn default_rocket_thermal_opt_iters() -> usize {
    40
}
fn default_rocket_thermal_local_gain() -> f64 {
    0.07
}
fn default_rocket_thermal_global_gain() -> f64 {
    0.04
}
fn default_rocket_thermal_min_channel_count() -> usize {
    32
}
fn default_rocket_thermal_max_channel_count() -> usize {
    2000
}
fn default_rocket_thermal_mid_feed_fraction() -> f64 {
    0.45
}
fn default_rocket_thermal_mid_feed_upstream_split() -> f64 {
    0.5
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RocketPropellantWorkspaceDef {
    #[serde(default = "default_rocket_propellant_preset_key")]
    pub selected_preset_key: String,
    #[serde(default)]
    pub search_query: String,
}

impl Default for RocketPropellantWorkspaceDef {
    fn default() -> Self {
        Self {
            selected_preset_key: default_rocket_propellant_preset_key(),
            search_query: String::new(),
        }
    }
}

fn default_rocket_propellant_preset_key() -> String {
    "lox-rp1".to_string()
}
