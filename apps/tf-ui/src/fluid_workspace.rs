use tf_fluids::{EquilibriumState, FluidInputPair, Species};
use tf_project::schema::{FluidCaseDef, FluidInputPairDef, FluidWorkspaceDef};

/// Computation status for a state point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeStatus {
    /// Property computation successful
    Success,
    /// Property computation failed
    Failed,
    /// Currently computing (for async operations)
    #[allow(dead_code)]
    Computing,
    /// Not yet computed or inputs changed
    NotComputed,
}

/// Explicit state mode: how to interpret inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StateMode {
    /// Treat inputs as directly specified thermodynamic properties (P-T, P-H, etc.)
    #[default]
    Specified,
    /// Interpret inputs in an equilibrium/saturation-aware way (e.g., T with sat-liquid/vapor)
    Equilibrium,
}

impl StateMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Specified => "Specified",
            Self::Equilibrium => "Equilibrium",
        }
    }
}

/// For Equilibrium mode: how to specify saturation state.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SaturationMode {
    /// Specify temperature; quality will be requested if two-phase
    #[default]
    Temperature,
    /// Specify pressure; quality will be requested if two-phase
    Pressure,
    /// Directly specify quality (0.0 = sat liquid, 1.0 = sat vapor)
    Quality,
}

impl SaturationMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Temperature => "T (Saturation)",
            Self::Pressure => "P (Saturation)",
            Self::Quality => "Quality",
        }
    }
}

/// Single fluid state point (row) in the workspace.
#[derive(Debug, Clone)]
pub struct StatePoint {
    /// Unique identifier for this state point
    pub id: String,
    /// User-defined label for this state point (e.g., "Inlet", "State 1")
    pub label: String,
    /// Selected fluid species
    pub species: Species,
    /// Explicit state mode: Specified or Equilibrium
    pub state_mode: StateMode,
    /// For Equilibrium mode: saturation mode (T, P, or Q)
    pub saturation_mode: SaturationMode,
    /// Selected input pair (only used in Specified mode)
    pub input_pair: FluidInputPair,
    /// First input value (meaning depends on pair/mode)
    pub input_1: f64,
    /// Raw text input for first value (preserves user units)
    pub input_1_text: String,
    /// Second input value (meaning depends on pair/mode)
    pub input_2: f64,
    /// Raw text input for second value (preserves user units)
    pub input_2_text: String,
    /// Optional quality for two-phase disambiguation (0.0 = saturated liquid, 1.0 = saturated vapor)
    pub quality: Option<f64>,
    /// Text for quality input (preserves user input like "0.5" or "50%")
    #[allow(dead_code)]
    pub quality_text: String,
    /// Last computed result
    pub last_result: Option<EquilibriumState>,
    /// Computation status
    pub status: ComputeStatus,
    /// Error message if computation failed
    pub error_message: Option<String>,
    /// Whether disambiguation is needed (detected after compute attempt)
    pub needs_disambiguation: bool,
}

impl Default for StatePoint {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            label: "State 1".to_string(),
            species: Species::N2,
            state_mode: StateMode::Specified,
            saturation_mode: SaturationMode::Temperature,
            input_pair: FluidInputPair::PT,
            input_1: 101_325.0,
            input_1_text: "101325".to_string(),
            input_2: 300.0,
            input_2_text: "300".to_string(),
            quality: None,
            quality_text: String::new(),
            last_result: None,
            status: ComputeStatus::NotComputed,
            error_message: None,
            needs_disambiguation: false,
        }
    }
}

impl StatePoint {
    pub fn new_with_label(label: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            label,
            ..Default::default()
        }
    }

    pub fn clear_result(&mut self) {
        self.last_result = None;
        self.status = ComputeStatus::NotComputed;
        self.error_message = None;
        self.needs_disambiguation = false;
    }

    /// Check if inputs are complete (both values entered)
    pub fn inputs_complete(&self) -> bool {
        !self.input_1_text.trim().is_empty() && !self.input_2_text.trim().is_empty()
    }
}

/// Row-based fluid comparison workspace.
#[derive(Debug, Clone)]
pub struct FluidWorkspace {
    /// Collection of state points (rows)
    pub state_points: Vec<StatePoint>,
    /// Curves ready to be added to plotting workspace
    pub pending_curves: Vec<tf_project::schema::ArbitraryCurveSourceDef>,
}

impl Default for FluidWorkspace {
    fn default() -> Self {
        Self {
            state_points: vec![StatePoint::default()],
            pending_curves: Vec::new(),
        }
    }
}

impl FluidWorkspace {
    pub fn from_def(def: &FluidWorkspaceDef) -> Self {
        let state_points = if def.cases.is_empty() {
            vec![StatePoint::default()]
        } else {
            def.cases
                .iter()
                .enumerate()
                .map(|(i, case_def)| {
                    let state_mode = match case_def.state_mode.as_str() {
                        "Equilibrium" => StateMode::Equilibrium,
                        _ => StateMode::Specified,
                    };
                    let saturation_mode = match case_def.saturation_mode.as_str() {
                        "Pressure" => SaturationMode::Pressure,
                        "Quality" => SaturationMode::Quality,
                        _ => SaturationMode::Temperature,
                    };
                    StatePoint {
                        id: case_def.id.clone(),
                        label: format!("State {}", i + 1),
                        species: case_def.species.parse::<Species>().unwrap_or(Species::N2),
                        state_mode,
                        saturation_mode,
                        input_pair: input_pair_from_def(case_def.input_pair),
                        input_1: case_def.input_1,
                        input_1_text: case_def.input_1.to_string(),
                        input_2: case_def.input_2,
                        input_2_text: case_def.input_2.to_string(),
                        quality: case_def.quality,
                        quality_text: case_def
                            .quality
                            .map_or(String::new(), |q| format!("{:.1}%", q * 100.0)),
                        last_result: None,
                        status: ComputeStatus::NotComputed,
                        error_message: None,
                        needs_disambiguation: false,
                    }
                })
                .collect()
        };

        Self {
            state_points,
            pending_curves: Vec::new(),
        }
    }

    pub fn to_def(&self) -> FluidWorkspaceDef {
        FluidWorkspaceDef {
            cases: self
                .state_points
                .iter()
                .map(|state| {
                    let state_mode_str = match state.state_mode {
                        StateMode::Specified => "Specified",
                        StateMode::Equilibrium => "Equilibrium",
                    };
                    let saturation_mode_str = match state.saturation_mode {
                        SaturationMode::Temperature => "Temperature",
                        SaturationMode::Pressure => "Pressure",
                        SaturationMode::Quality => "Quality",
                    };
                    FluidCaseDef {
                        id: state.id.clone(),
                        species: state.species.key().to_string(),
                        state_mode: state_mode_str.to_string(),
                        saturation_mode: saturation_mode_str.to_string(),
                        input_pair: input_pair_to_def(state.input_pair),
                        input_1: state.input_1,
                        input_2: state.input_2,
                        quality: state.quality,
                    }
                })
                .collect(),
        }
    }

    pub fn add_state_point(&mut self) {
        let next_num = self.state_points.len() + 1;
        self.state_points
            .push(StatePoint::new_with_label(format!("State {}", next_num)));
    }

    pub fn remove_state_point(&mut self, state_id: &str) {
        self.state_points.retain(|s| s.id != state_id);
        if self.state_points.is_empty() {
            self.state_points.push(StatePoint::default());
        }
    }

    /// Legacy method for backward compatibility
    #[deprecated(note = "Use state_points field directly")]
    #[allow(dead_code)]
    pub fn cases(&self) -> &[StatePoint] {
        &self.state_points
    }

    /// Legacy method for backward compatibility
    #[deprecated(note = "Use add_state_point")]
    #[allow(dead_code)]
    pub fn add_case(&mut self) {
        self.add_state_point();
    }

    /// Legacy method for backward compatibility
    #[deprecated(note = "Use remove_state_point")]
    #[allow(dead_code)]
    pub fn remove_case(&mut self, case_id: &str) {
        self.remove_state_point(case_id);
    }
}

pub fn input_pair_from_def(def: FluidInputPairDef) -> FluidInputPair {
    match def {
        FluidInputPairDef::PT => FluidInputPair::PT,
        FluidInputPairDef::PH => FluidInputPair::PH,
        FluidInputPairDef::RhoH => FluidInputPair::RhoH,
        FluidInputPairDef::PS => FluidInputPair::PS,
    }
}

pub fn input_pair_to_def(pair: FluidInputPair) -> FluidInputPairDef {
    match pair {
        FluidInputPair::PT => FluidInputPairDef::PT,
        FluidInputPair::PH => FluidInputPairDef::PH,
        FluidInputPair::RhoH => FluidInputPairDef::RhoH,
        FluidInputPair::PS => FluidInputPairDef::PS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_roundtrip_def() {
        let mut workspace = FluidWorkspace::default();
        workspace.state_points[0].species = Species::NitrousOxide;
        workspace.state_points[0].input_pair = FluidInputPair::PS;
        workspace.state_points[0].input_1 = 202_650.0;
        workspace.state_points[0].input_2 = 7000.0;

        let def = workspace.to_def();
        let restored = FluidWorkspace::from_def(&def);

        assert_eq!(restored.state_points.len(), 1);
        assert_eq!(restored.state_points[0].species, Species::NitrousOxide);
        assert_eq!(restored.state_points[0].input_pair, FluidInputPair::PS);
        assert_eq!(restored.state_points[0].input_1, 202_650.0);
        assert_eq!(restored.state_points[0].input_2, 7000.0);
    }

    #[test]
    fn workspace_multi_case_roundtrip() {
        let mut workspace = FluidWorkspace::default();
        workspace.add_state_point();
        workspace.state_points[0].species = Species::N2;
        workspace.state_points[0].input_pair = FluidInputPair::PT;
        workspace.state_points[1].species = Species::NitrousOxide;
        workspace.state_points[1].input_pair = FluidInputPair::PH;
        workspace.state_points[1].quality = Some(0.5);

        let def = workspace.to_def();
        let restored = FluidWorkspace::from_def(&def);

        assert_eq!(restored.state_points.len(), 2);
        assert_eq!(restored.state_points[0].species, Species::N2);
        assert_eq!(restored.state_points[1].species, Species::NitrousOxide);
        assert_eq!(restored.state_points[1].quality, Some(0.5));
    }

    #[test]
    fn state_mode_specified_default() {
        let state = StatePoint::default();
        assert_eq!(state.state_mode, StateMode::Specified);
        assert_eq!(state.state_mode.label(), "Specified");
    }

    #[test]
    fn state_mode_equilibrium() {
        let state = StatePoint {
            state_mode: StateMode::Equilibrium,
            ..Default::default()
        };
        assert_eq!(state.state_mode, StateMode::Equilibrium);
        assert_eq!(state.state_mode.label(), "Equilibrium");
    }

    #[test]
    fn saturation_mode_temperature_default() {
        let state = StatePoint::default();
        assert_eq!(state.saturation_mode, SaturationMode::Temperature);
        assert_eq!(state.saturation_mode.label(), "T (Saturation)");
    }

    #[test]
    fn saturation_mode_variations() {
        let state_pressure = StatePoint {
            saturation_mode: SaturationMode::Pressure,
            ..Default::default()
        };
        assert_eq!(state_pressure.saturation_mode.label(), "P (Saturation)");

        let state_quality = StatePoint {
            saturation_mode: SaturationMode::Quality,
            ..Default::default()
        };
        assert_eq!(state_quality.saturation_mode.label(), "Quality");
    }

    #[test]
    fn state_mode_persistence_roundtrip() {
        let mut workspace = FluidWorkspace::default();
        workspace.state_points[0].state_mode = StateMode::Equilibrium;
        workspace.state_points[0].saturation_mode = SaturationMode::Pressure;

        let def = workspace.to_def();
        let restored = FluidWorkspace::from_def(&def);

        assert_eq!(restored.state_points[0].state_mode, StateMode::Equilibrium);
        assert_eq!(
            restored.state_points[0].saturation_mode,
            SaturationMode::Pressure
        );
    }

    #[test]
    fn quality_text_preservation() {
        let mut workspace = FluidWorkspace::default();
        workspace.state_points[0].quality_text = "0.5".to_string();
        workspace.state_points[0].quality = Some(0.5);

        let def = workspace.to_def();
        let restored = FluidWorkspace::from_def(&def);

        assert_eq!(restored.state_points[0].quality, Some(0.5));
    }

    #[test]
    fn pending_curves_initialized_empty() {
        let workspace = FluidWorkspace::default();
        assert_eq!(workspace.pending_curves.len(), 0);
    }

    #[test]
    fn add_state_point_increments_count() {
        let mut workspace = FluidWorkspace::default();
        assert_eq!(workspace.state_points.len(), 1);

        workspace.add_state_point();
        assert_eq!(workspace.state_points.len(), 2);

        workspace.add_state_point();
        assert_eq!(workspace.state_points.len(), 3);
    }

    #[test]
    fn remove_state_point_leaves_one_minimum() {
        let mut workspace = FluidWorkspace::default();
        let first_id = workspace.state_points[0].id.clone();

        workspace.remove_state_point(&first_id);
        assert_eq!(workspace.state_points.len(), 1);
    }

    #[test]
    fn multiple_state_modes_in_workspace() {
        let mut workspace = FluidWorkspace::default();
        workspace.add_state_point();
        workspace.add_state_point();

        workspace.state_points[0].state_mode = StateMode::Specified;
        workspace.state_points[1].state_mode = StateMode::Equilibrium;
        workspace.state_points[2].state_mode = StateMode::Specified;

        assert_eq!(workspace.state_points[0].state_mode, StateMode::Specified);
        assert_eq!(workspace.state_points[1].state_mode, StateMode::Equilibrium);
        assert_eq!(workspace.state_points[2].state_mode, StateMode::Specified);
    }
}
