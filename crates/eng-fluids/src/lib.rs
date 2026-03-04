use std::str::FromStr;

use eng_core::units::typed::{ExprInput, QuantityKind, UnitInput};
use eng_core::units::{
    Quantity, ensure_signature_matches_dimension, parse_equation_quantity_to_si, parse_quantity,
};
use tf_core::units::{k, pa};
use tf_fluids::{
    Composition, CoolPropModel, FluidCatalogEntry as TfFluidCatalogEntry, FluidModel, Species,
    StateInput,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidRef {
    pub key: &'static str,
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct FluidState {
    fluid: FluidRef,
    temperature_k: f64,
    pressure_pa: f64,
    input_pair: FluidInputPair,
    inputs: [FluidStateInput; 2],
    quality: Option<f64>,
    phase: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidInputPair {
    PT,
    PH,
    PS,
    RhoH,
    PQ,
    TQ,
}

impl FluidInputPair {
    pub fn label(self) -> &'static str {
        match self {
            Self::PT => "T,P",
            Self::PH => "P,h",
            Self::PS => "P,s",
            Self::RhoH => "rho,h",
            Self::PQ => "P,Q",
            Self::TQ => "T,Q",
        }
    }
}

pub const SUPPORTED_STATE_INPUT_PAIRS: &[FluidInputPair] = &[
    FluidInputPair::PT,
    FluidInputPair::PH,
    FluidInputPair::PS,
    FluidInputPair::RhoH,
    FluidInputPair::PQ,
    FluidInputPair::TQ,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidStateInputProperty {
    Temperature,
    Pressure,
    Density,
    SpecificEnthalpy,
    SpecificEntropy,
    Quality,
}

impl FluidStateInputProperty {
    pub fn key(self) -> &'static str {
        match self {
            Self::Temperature => "T",
            Self::Pressure => "P",
            Self::Density => "rho",
            Self::SpecificEnthalpy => "h",
            Self::SpecificEntropy => "s",
            Self::Quality => "Q",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Temperature => "temperature",
            Self::Pressure => "pressure",
            Self::Density => "density",
            Self::SpecificEnthalpy => "specific enthalpy",
            Self::SpecificEntropy => "specific entropy",
            Self::Quality => "quality",
        }
    }

    fn dimension_name(self) -> &'static str {
        match self {
            Self::Temperature => "temperature",
            Self::Pressure => "pressure",
            Self::Density => "density",
            Self::SpecificEnthalpy => "specific_enthalpy",
            Self::SpecificEntropy => "specific_entropy",
            Self::Quality => "dimensionless",
        }
    }

    fn quantity_kind(self) -> Option<QuantityKind> {
        match self {
            Self::Temperature => Some(QuantityKind::Temperature),
            Self::Pressure => Some(QuantityKind::Pressure),
            Self::Density => Some(QuantityKind::Density),
            Self::Quality => Some(QuantityKind::Dimensionless),
            Self::SpecificEnthalpy | Self::SpecificEntropy => None,
        }
    }

    fn quantity_parser_kind(self) -> Quantity {
        match self {
            Self::Temperature => Quantity::Temperature,
            Self::Pressure => Quantity::Pressure,
            Self::Density => Quantity::Density,
            Self::SpecificEnthalpy => Quantity::SpecificEnthalpy,
            Self::SpecificEntropy => Quantity::SpecificEntropy,
            Self::Quality => Quantity::Quality,
        }
    }
}

impl FromStr for FluidStateInputProperty {
    type Err = FluidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match normalize_key(s).as_str() {
            "t" | "temp" | "temperature" => Ok(Self::Temperature),
            "p" | "pressure" => Ok(Self::Pressure),
            "rho" | "density" => Ok(Self::Density),
            "h" | "enthalpy" | "specificenthalpy" => Ok(Self::SpecificEnthalpy),
            "s" | "entropy" | "specificentropy" => Ok(Self::SpecificEntropy),
            "q" | "quality" | "x" => Ok(Self::Quality),
            "u" | "internalenergy" => Err(FluidError::InvalidStateInputProperty {
                property: s.to_string(),
                note: "internal energy ('u') is intentionally distinct from enthalpy ('h'); this API currently supports h/s-based identities".to_string(),
            }),
            _ => Err(FluidError::InvalidStateInputProperty {
                property: s.to_string(),
                note: "supported properties: T, P, rho, h, s, Q".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FluidStateInput {
    property: FluidStateInputProperty,
    value_si: f64,
}

impl FluidStateInput {
    pub fn property(self) -> FluidStateInputProperty {
        self.property
    }

    pub fn value_si(self) -> f64 {
        self.value_si
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidProperty {
    Density,
    SpecificHeatCapacity,
    SpecificHeatCapacityCv,
    Gamma,
    SpeedOfSound,
    DynamicViscosity,
    ThermalConductivity,
    Temperature,
    Pressure,
    SpecificEnthalpy,
    SpecificEntropy,
    Quality,
}

impl FromStr for FluidProperty {
    type Err = FluidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match normalize_key(s).as_str() {
            "density" | "rho" => Ok(Self::Density),
            "specificheatcapacity" | "cp" => Ok(Self::SpecificHeatCapacity),
            "specificheatcapacitycv" | "cv" => Ok(Self::SpecificHeatCapacityCv),
            "gamma" | "heatcapacityratio" => Ok(Self::Gamma),
            "speedofsound" | "a" => Ok(Self::SpeedOfSound),
            "dynamicviscosity" | "mu" | "viscosity" => Ok(Self::DynamicViscosity),
            "thermalconductivity" | "k" => Ok(Self::ThermalConductivity),
            "temperature" | "t" => Ok(Self::Temperature),
            "pressure" | "p" => Ok(Self::Pressure),
            "specificenthalpy" | "enthalpy" | "h" => Ok(Self::SpecificEnthalpy),
            "specificentropy" | "entropy" | "s" => Ok(Self::SpecificEntropy),
            "quality" | "q" | "x" => Ok(Self::Quality),
            other => Err(FluidError::UnsupportedProperty(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FluidCatalogEntry {
    pub key: &'static str,
    pub display_name: &'static str,
    pub aliases: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct FluidDocsEntry {
    pub key: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub supported_state_inputs: Vec<String>,
    pub supported_properties: Vec<String>,
}

pub const SUPPORTED_PROPERTIES: &[&str] = &[
    "density",
    "specific_heat_capacity",
    "specific_heat_capacity_cv",
    "gamma",
    "speed_of_sound",
    "dynamic_viscosity",
    "thermal_conductivity",
    "temperature",
    "pressure",
    "specific_enthalpy",
    "specific_entropy",
    "quality",
];

#[derive(Debug, Clone)]
pub enum FluidInputValue {
    Si(f64),
    Typed(UnitInput),
    Expr(ExprInput),
    WithUnit(String),
}

pub trait IntoFluidInput {
    fn into_fluid_input(self) -> FluidInputValue;
}

impl IntoFluidInput for f64 {
    fn into_fluid_input(self) -> FluidInputValue {
        FluidInputValue::Si(self)
    }
}

impl IntoFluidInput for UnitInput {
    fn into_fluid_input(self) -> FluidInputValue {
        FluidInputValue::Typed(self)
    }
}

impl IntoFluidInput for ExprInput {
    fn into_fluid_input(self) -> FluidInputValue {
        FluidInputValue::Expr(self)
    }
}

impl IntoFluidInput for &str {
    fn into_fluid_input(self) -> FluidInputValue {
        FluidInputValue::WithUnit(self.to_string())
    }
}

impl IntoFluidInput for String {
    fn into_fluid_input(self) -> FluidInputValue {
        FluidInputValue::WithUnit(self)
    }
}

pub trait IntoFluidStateInputProperty {
    fn into_state_input_property(self) -> Result<FluidStateInputProperty>;
}

impl IntoFluidStateInputProperty for FluidStateInputProperty {
    fn into_state_input_property(self) -> Result<FluidStateInputProperty> {
        Ok(self)
    }
}

impl IntoFluidStateInputProperty for &str {
    fn into_state_input_property(self) -> Result<FluidStateInputProperty> {
        FluidStateInputProperty::from_str(self)
    }
}

impl IntoFluidStateInputProperty for String {
    fn into_state_input_property(self) -> Result<FluidStateInputProperty> {
        FluidStateInputProperty::from_str(&self)
    }
}

#[derive(Debug, Error)]
pub enum FluidError {
    #[error("unknown fluid '{0}'")]
    UnknownFluid(String),
    #[error("unsupported fluid property '{0}'")]
    UnsupportedProperty(String),
    #[error("invalid state input property '{property}': {note}")]
    InvalidStateInputProperty { property: String, note: String },
    #[error("state input property '{property}' was provided more than once")]
    DuplicateStateInputProperty { property: String },
    #[error("unsupported state input pair '{pair}'; supported pairs: {supported}")]
    UnsupportedStateInputPair { pair: String, supported: String },
    #[error("typed input kind mismatch for {property}: expected {expected}, got {got}")]
    TypedInputKindMismatch {
        property: String,
        expected: String,
        got: String,
    },
    #[error("expression input dimension mismatch for {property}: {message}")]
    ExprInputDimensionMismatch { property: String, message: String },
    #[error("value parse failed for {property} from '{input}': {message}")]
    InputParse {
        property: String,
        input: String,
        message: String,
    },
    #[error("backend could not construct state for fluid '{fluid}' with pair {pair}: {message}")]
    BackendState {
        fluid: String,
        pair: String,
        message: String,
    },
    #[error("backend property lookup failed for fluid '{fluid}' property '{property}': {message}")]
    BackendProperty {
        fluid: String,
        property: String,
        message: String,
    },
    #[error("property '{property}' is unavailable on this state: {reason}")]
    PropertyUnavailable { property: String, reason: String },
}

pub type Result<T> = std::result::Result<T, FluidError>;

#[derive(Debug, Clone)]
pub struct SaturationStates {
    pub liquid: FluidState,
    pub vapor: FluidState,
}

static FLUIDS: &[FluidRef] = &[
    FluidRef {
        key: "Air",
        display_name: "Air",
        aliases: &["atmosphere"],
    },
    FluidRef {
        key: "N2",
        display_name: "Nitrogen",
        aliases: &["nitrogen"],
    },
    FluidRef {
        key: "O2",
        display_name: "Oxygen",
        aliases: &["oxygen"],
    },
    FluidRef {
        key: "N2O",
        display_name: "Nitrous Oxide",
        aliases: &["nitrous oxide"],
    },
    FluidRef {
        key: "H2",
        display_name: "Hydrogen",
        aliases: &["hydrogen"],
    },
    FluidRef {
        key: "He",
        display_name: "Helium",
        aliases: &["helium"],
    },
    FluidRef {
        key: "Ar",
        display_name: "Argon",
        aliases: &["argon"],
    },
    FluidRef {
        key: "Ne",
        display_name: "Neon",
        aliases: &["neon"],
    },
    FluidRef {
        key: "Kr",
        display_name: "Krypton",
        aliases: &["krypton"],
    },
    FluidRef {
        key: "Xe",
        display_name: "Xenon",
        aliases: &["xenon"],
    },
    FluidRef {
        key: "CH4",
        display_name: "Methane",
        aliases: &["methane"],
    },
    FluidRef {
        key: "Ethane",
        display_name: "Ethane",
        aliases: &["c2h6"],
    },
    FluidRef {
        key: "Ethylene",
        display_name: "Ethylene",
        aliases: &["c2h4"],
    },
    FluidRef {
        key: "Propane",
        display_name: "Propane",
        aliases: &["c3h8", "n-propane"],
    },
    FluidRef {
        key: "Propylene",
        display_name: "Propylene",
        aliases: &["c3h6"],
    },
    FluidRef {
        key: "nButane",
        display_name: "n-Butane",
        aliases: &["butane", "n-butane"],
    },
    FluidRef {
        key: "Isobutane",
        display_name: "Isobutane",
        aliases: &["i-butane"],
    },
    FluidRef {
        key: "nPentane",
        display_name: "n-Pentane",
        aliases: &["pentane", "n-pentane"],
    },
    FluidRef {
        key: "Isopentane",
        display_name: "Isopentane",
        aliases: &["i-pentane"],
    },
    FluidRef {
        key: "nHexane",
        display_name: "n-Hexane",
        aliases: &["hexane", "n-hexane"],
    },
    FluidRef {
        key: "CO2",
        display_name: "Carbon Dioxide",
        aliases: &["carbon dioxide"],
    },
    FluidRef {
        key: "CO",
        display_name: "Carbon Monoxide",
        aliases: &["carbon monoxide"],
    },
    FluidRef {
        key: "H2O",
        display_name: "Water",
        aliases: &["water"],
    },
    FluidRef {
        key: "NH3",
        display_name: "Ammonia",
        aliases: &["ammonia"],
    },
    FluidRef {
        key: "SO2",
        display_name: "Sulfur Dioxide",
        aliases: &["sulfur dioxide"],
    },
    FluidRef {
        key: "R32",
        display_name: "R32",
        aliases: &[],
    },
    FluidRef {
        key: "R125",
        display_name: "R125",
        aliases: &[],
    },
    FluidRef {
        key: "R134a",
        display_name: "R134a",
        aliases: &[],
    },
    FluidRef {
        key: "R152a",
        display_name: "R152a",
        aliases: &[],
    },
    FluidRef {
        key: "R245fa",
        display_name: "R245fa",
        aliases: &[],
    },
    FluidRef {
        key: "R1234yf",
        display_name: "R1234yf",
        aliases: &[],
    },
];

pub fn catalog() -> &'static [FluidRef] {
    FLUIDS
}

pub fn docs_entries() -> Vec<FluidDocsEntry> {
    let mut out = Vec::new();
    for fluid in FLUIDS {
        out.push(FluidDocsEntry {
            key: fluid.key.to_string(),
            name: fluid.display_name.to_string(),
            aliases: fluid.aliases.iter().map(|a| (*a).to_string()).collect(),
            supported_state_inputs: SUPPORTED_STATE_INPUT_PAIRS
                .iter()
                .map(|p| p.label().to_string())
                .collect(),
            supported_properties: SUPPORTED_PROPERTIES
                .iter()
                .map(|p| (*p).to_string())
                .collect(),
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

pub fn find(query: &str) -> Option<FluidRef> {
    let q = query.trim().to_ascii_lowercase();
    FLUIDS
        .iter()
        .copied()
        .find(|f| f.key.eq_ignore_ascii_case(&q) || f.display_name.eq_ignore_ascii_case(&q))
        .or_else(|| {
            FLUIDS
                .iter()
                .copied()
                .find(|f| f.aliases.iter().any(|a| a.eq_ignore_ascii_case(&q)))
        })
}

pub fn catalog_entries_from_backend() -> Vec<FluidCatalogEntry> {
    tf_fluids::practical_coolprop_catalog()
        .iter()
        .map(|f: &TfFluidCatalogEntry| FluidCatalogEntry {
            key: f.canonical_id,
            display_name: f.display_name,
            aliases: f.aliases,
        })
        .collect()
}

impl FluidRef {
    pub fn state_tp<T, P>(self, temperature: T, pressure: P) -> Result<FluidState>
    where
        T: IntoFluidInput,
        P: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Temperature,
            temperature.into_fluid_input(),
            FluidStateInputProperty::Pressure,
            pressure.into_fluid_input(),
        )
    }

    pub fn state_ph<P, H>(self, pressure: P, enthalpy: H) -> Result<FluidState>
    where
        P: IntoFluidInput,
        H: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Pressure,
            pressure.into_fluid_input(),
            FluidStateInputProperty::SpecificEnthalpy,
            enthalpy.into_fluid_input(),
        )
    }

    pub fn state_ps<P, S>(self, pressure: P, entropy: S) -> Result<FluidState>
    where
        P: IntoFluidInput,
        S: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Pressure,
            pressure.into_fluid_input(),
            FluidStateInputProperty::SpecificEntropy,
            entropy.into_fluid_input(),
        )
    }

    pub fn state_rho_h<R, H>(self, density: R, enthalpy: H) -> Result<FluidState>
    where
        R: IntoFluidInput,
        H: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Density,
            density.into_fluid_input(),
            FluidStateInputProperty::SpecificEnthalpy,
            enthalpy.into_fluid_input(),
        )
    }

    pub fn state_rho_t<R, T>(self, _density: R, _temperature: T) -> Result<FluidState>
    where
        R: IntoFluidInput,
        T: IntoFluidInput,
    {
        Err(FluidError::UnsupportedStateInputPair {
            pair: "rho,T".to_string(),
            supported: supported_pair_labels(),
        })
    }

    pub fn state_pq<P, Q>(self, pressure: P, quality: Q) -> Result<FluidState>
    where
        P: IntoFluidInput,
        Q: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Pressure,
            pressure.into_fluid_input(),
            FluidStateInputProperty::Quality,
            quality.into_fluid_input(),
        )
    }

    pub fn state_tq<T, Q>(self, temperature: T, quality: Q) -> Result<FluidState>
    where
        T: IntoFluidInput,
        Q: IntoFluidInput,
    {
        self.build_pair_state(
            FluidStateInputProperty::Temperature,
            temperature.into_fluid_input(),
            FluidStateInputProperty::Quality,
            quality.into_fluid_input(),
        )
    }

    pub fn state<K1, V1, K2, V2>(
        self,
        prop1: K1,
        value1: V1,
        prop2: K2,
        value2: V2,
    ) -> Result<FluidState>
    where
        K1: IntoFluidStateInputProperty,
        K2: IntoFluidStateInputProperty,
        V1: IntoFluidInput,
        V2: IntoFluidInput,
    {
        self.build_pair_state(
            prop1.into_state_input_property()?,
            value1.into_fluid_input(),
            prop2.into_state_input_property()?,
            value2.into_fluid_input(),
        )
    }

    pub fn saturation_at_pressure<P>(self, pressure: P) -> Result<SaturationStates>
    where
        P: IntoFluidInput + Copy,
    {
        Ok(SaturationStates {
            liquid: self.state_pq(pressure, 0.0_f64)?,
            vapor: self.state_pq(pressure, 1.0_f64)?,
        })
    }

    pub fn saturation_at_temperature<T>(self, temperature: T) -> Result<SaturationStates>
    where
        T: IntoFluidInput + Copy,
    {
        Ok(SaturationStates {
            liquid: self.state_tq(temperature, 0.0_f64)?,
            vapor: self.state_tq(temperature, 1.0_f64)?,
        })
    }

    fn build_pair_state(
        self,
        prop1: FluidStateInputProperty,
        value1: FluidInputValue,
        prop2: FluidStateInputProperty,
        value2: FluidInputValue,
    ) -> Result<FluidState> {
        if prop1 == prop2 {
            return Err(FluidError::DuplicateStateInputProperty {
                property: prop1.name().to_string(),
            });
        }
        let input1 = FluidStateInput {
            property: prop1,
            value_si: parse_input_for_property(prop1, value1)?,
        };
        let input2 = FluidStateInput {
            property: prop2,
            value_si: parse_input_for_property(prop2, value2)?,
        };

        let (pair, state_input) = build_state_input(input1, input2)?;
        let model = CoolPropModel::new();
        let species = Species::from_str(self.key)
            .map_err(|_| FluidError::UnknownFluid(self.key.to_string()))?;
        let state = model
            .state(state_input, Composition::pure(species))
            .map_err(|e| FluidError::BackendState {
                fluid: self.key.to_string(),
                pair: pair.label().to_string(),
                message: e.to_string(),
            })?;

        Ok(FluidState {
            fluid: self,
            temperature_k: state.temperature().value,
            pressure_pa: state.pressure().value,
            input_pair: pair,
            inputs: [input1, input2],
            quality: [input1, input2]
                .iter()
                .find(|i| i.property == FluidStateInputProperty::Quality)
                .map(|i| i.value_si),
            phase: None,
        })
    }

    pub fn key(self) -> &'static str {
        self.key
    }
}

impl FluidState {
    pub fn fluid(&self) -> FluidRef {
        self.fluid
    }

    pub fn fluid_key(&self) -> &'static str {
        self.fluid.key
    }

    pub fn fluid_name(&self) -> &'static str {
        self.fluid.display_name
    }

    pub fn input_pair(&self) -> FluidInputPair {
        self.input_pair
    }

    pub fn input_pair_label(&self) -> &'static str {
        self.input_pair.label()
    }

    pub fn normalized_inputs(&self) -> [FluidStateInput; 2] {
        self.inputs
    }

    pub fn quality(&self) -> Option<f64> {
        self.quality
    }

    pub fn phase(&self) -> Option<&str> {
        self.phase.as_deref()
    }

    pub fn temperature_k(&self) -> f64 {
        self.temperature_k
    }

    pub fn pressure_pa(&self) -> f64 {
        self.pressure_pa
    }

    pub fn temperature(&self) -> f64 {
        self.temperature_k
    }

    pub fn pressure(&self) -> f64 {
        self.pressure_pa
    }

    pub fn t(&self) -> f64 {
        self.temperature_k
    }

    pub fn p(&self) -> f64 {
        self.pressure_pa
    }

    pub fn density(&self) -> Result<f64> {
        self.property(FluidProperty::Density)
    }

    pub fn rho(&self) -> Result<f64> {
        self.density()
    }

    pub fn specific_heat_capacity(&self) -> Result<f64> {
        self.property(FluidProperty::SpecificHeatCapacity)
    }

    pub fn cp(&self) -> Result<f64> {
        self.specific_heat_capacity()
    }

    pub fn specific_heat_capacity_cv(&self) -> Result<f64> {
        self.property(FluidProperty::SpecificHeatCapacityCv)
    }

    pub fn cv(&self) -> Result<f64> {
        self.specific_heat_capacity_cv()
    }

    pub fn gamma(&self) -> Result<f64> {
        self.property(FluidProperty::Gamma)
    }

    pub fn speed_of_sound(&self) -> Result<f64> {
        self.property(FluidProperty::SpeedOfSound)
    }

    pub fn a(&self) -> Result<f64> {
        self.speed_of_sound()
    }

    pub fn dynamic_viscosity(&self) -> Result<f64> {
        self.property(FluidProperty::DynamicViscosity)
    }

    pub fn mu(&self) -> Result<f64> {
        self.dynamic_viscosity()
    }

    pub fn thermal_conductivity(&self) -> Result<f64> {
        self.property(FluidProperty::ThermalConductivity)
    }

    pub fn k(&self) -> Result<f64> {
        self.thermal_conductivity()
    }

    pub fn specific_enthalpy(&self) -> Result<f64> {
        self.property(FluidProperty::SpecificEnthalpy)
    }

    pub fn h(&self) -> Result<f64> {
        self.specific_enthalpy()
    }

    pub fn specific_entropy(&self) -> Result<f64> {
        self.property(FluidProperty::SpecificEntropy)
    }

    pub fn s(&self) -> Result<f64> {
        self.specific_entropy()
    }

    pub fn property_by_name(&self, property: &str) -> Result<f64> {
        self.property(FluidProperty::from_str(property)?)
    }

    pub fn property(&self, property: FluidProperty) -> Result<f64> {
        match property {
            FluidProperty::Temperature => Ok(self.temperature_k),
            FluidProperty::Pressure => Ok(self.pressure_pa),
            FluidProperty::Quality => self.quality.ok_or_else(|| FluidError::PropertyUnavailable {
                property: "quality".to_string(),
                reason: "quality is only defined for states created with Q-based inputs"
                    .to_string(),
            }),
            FluidProperty::Density
            | FluidProperty::SpecificHeatCapacity
            | FluidProperty::SpecificHeatCapacityCv
            | FluidProperty::Gamma
            | FluidProperty::SpeedOfSound
            | FluidProperty::DynamicViscosity
            | FluidProperty::ThermalConductivity
            | FluidProperty::SpecificEnthalpy
            | FluidProperty::SpecificEntropy => {
                let model = CoolPropModel::new();
                let species = Species::from_str(self.fluid.key)
                    .map_err(|_| FluidError::UnknownFluid(self.fluid.key.to_string()))?;
                let state = model
                    .state(
                        StateInput::PT {
                            p: pa(self.pressure_pa),
                            t: k(self.temperature_k),
                        },
                        Composition::pure(species),
                    )
                    .map_err(|e| FluidError::BackendState {
                        fluid: self.fluid.key.to_string(),
                        pair: "T,P".to_string(),
                        message: e.to_string(),
                    })?;
                match property {
                    FluidProperty::Density => model.rho(&state).map(|x| x.value).map_err(|e| {
                        FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "density".to_string(),
                            message: e.to_string(),
                        }
                    }),
                    FluidProperty::SpecificHeatCapacity => {
                        model.cp(&state).map_err(|e| FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "specific_heat_capacity".to_string(),
                            message: e.to_string(),
                        })
                    }
                    FluidProperty::SpecificHeatCapacityCv => {
                        model.cv(&state).map_err(|e| FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "specific_heat_capacity_cv".to_string(),
                            message: e.to_string(),
                        })
                    }
                    FluidProperty::Gamma => {
                        model
                            .gamma(&state)
                            .map_err(|e| FluidError::BackendProperty {
                                fluid: self.fluid.key.to_string(),
                                property: "gamma".to_string(),
                                message: e.to_string(),
                            })
                    }
                    FluidProperty::SpeedOfSound => {
                        model
                            .a(&state)
                            .map(|x| x.value)
                            .map_err(|e| FluidError::BackendProperty {
                                fluid: self.fluid.key.to_string(),
                                property: "speed_of_sound".to_string(),
                                message: e.to_string(),
                            })
                    }
                    FluidProperty::DynamicViscosity => {
                        model
                            .dynamic_viscosity(&state)
                            .map_err(|e| FluidError::BackendProperty {
                                fluid: self.fluid.key.to_string(),
                                property: "dynamic_viscosity".to_string(),
                                message: e.to_string(),
                            })
                    }
                    FluidProperty::ThermalConductivity => model
                        .thermal_conductivity(&state)
                        .map_err(|e| FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "thermal_conductivity".to_string(),
                            message: e.to_string(),
                        }),
                    FluidProperty::SpecificEnthalpy => {
                        model.h(&state).map_err(|e| FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "specific_enthalpy".to_string(),
                            message: e.to_string(),
                        })
                    }
                    FluidProperty::SpecificEntropy => {
                        model.s(&state).map_err(|e| FluidError::BackendProperty {
                            fluid: self.fluid.key.to_string(),
                            property: "specific_entropy".to_string(),
                            message: e.to_string(),
                        })
                    }
                    FluidProperty::Temperature
                    | FluidProperty::Pressure
                    | FluidProperty::Quality => unreachable!(),
                }
            }
        }
    }
}

fn build_state_input(
    a: FluidStateInput,
    b: FluidStateInput,
) -> Result<(FluidInputPair, StateInput)> {
    let get = |p: FluidStateInputProperty| -> Option<f64> {
        if a.property == p {
            Some(a.value_si)
        } else if b.property == p {
            Some(b.value_si)
        } else {
            None
        }
    };

    if let (Some(p), Some(t)) = (
        get(FluidStateInputProperty::Pressure),
        get(FluidStateInputProperty::Temperature),
    ) {
        return Ok((FluidInputPair::PT, StateInput::PT { p: pa(p), t: k(t) }));
    }
    if let (Some(p), Some(h)) = (
        get(FluidStateInputProperty::Pressure),
        get(FluidStateInputProperty::SpecificEnthalpy),
    ) {
        return Ok((FluidInputPair::PH, StateInput::PH { p: pa(p), h }));
    }
    if let (Some(p), Some(s)) = (
        get(FluidStateInputProperty::Pressure),
        get(FluidStateInputProperty::SpecificEntropy),
    ) {
        return Ok((FluidInputPair::PS, StateInput::PS { p: pa(p), s }));
    }
    if let (Some(rho), Some(h)) = (
        get(FluidStateInputProperty::Density),
        get(FluidStateInputProperty::SpecificEnthalpy),
    ) {
        return Ok((FluidInputPair::RhoH, StateInput::RhoH { rho_kg_m3: rho, h }));
    }
    if let (Some(p), Some(q)) = (
        get(FluidStateInputProperty::Pressure),
        get(FluidStateInputProperty::Quality),
    ) {
        return Ok((
            FluidInputPair::PQ,
            StateInput::PxWithQuality {
                p: pa(p),
                quality: q,
            },
        ));
    }
    if let (Some(t), Some(q)) = (
        get(FluidStateInputProperty::Temperature),
        get(FluidStateInputProperty::Quality),
    ) {
        return Ok((
            FluidInputPair::TQ,
            StateInput::TxWithQuality {
                t: k(t),
                quality: q,
            },
        ));
    }

    Err(FluidError::UnsupportedStateInputPair {
        pair: format!("{},{}", a.property.key(), b.property.key()),
        supported: supported_pair_labels(),
    })
}

fn parse_input_for_property(
    property: FluidStateInputProperty,
    input: FluidInputValue,
) -> Result<f64> {
    match input {
        FluidInputValue::Si(v) => Ok(v),
        FluidInputValue::Typed(v) => {
            let Some(kind) = property.quantity_kind() else {
                return Err(FluidError::TypedInputKindMismatch {
                    property: property.name().to_string(),
                    expected: "SI numeric or unit-tagged string".to_string(),
                    got: format!("{:?}", v.kind),
                });
            };
            if v.kind != kind {
                return Err(FluidError::TypedInputKindMismatch {
                    property: property.name().to_string(),
                    expected: format!("{:?}", kind),
                    got: format!("{:?}", v.kind),
                });
            }
            Ok(v.value_si)
        }
        FluidInputValue::Expr(v) => {
            ensure_signature_matches_dimension(v.signature, property.dimension_name()).map_err(
                |e| FluidError::ExprInputDimensionMismatch {
                    property: property.name().to_string(),
                    message: e.to_string(),
                },
            )?;
            Ok(v.value_si)
        }
        FluidInputValue::WithUnit(text) => {
            parse_text_input(property, &text).map_err(|e| FluidError::InputParse {
                property: property.name().to_string(),
                input: text,
                message: e,
            })
        }
    }
}

fn parse_text_input(
    property: FluidStateInputProperty,
    text: &str,
) -> std::result::Result<f64, String> {
    if property == FluidStateInputProperty::Temperature {
        if let Ok(v) = parse_equation_quantity_to_si("temperature", text) {
            return Ok(v);
        }
    }
    if let Ok(v) = parse_equation_quantity_to_si(property.dimension_name(), text) {
        return Ok(v);
    }
    parse_quantity(text, property.quantity_parser_kind()).map_err(|e| e.to_string())
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn supported_pair_labels() -> String {
    SUPPORTED_STATE_INPUT_PAIRS
        .iter()
        .map(|p| p.label())
        .collect::<Vec<_>>()
        .join(", ")
}

macro_rules! fluid_fn {
    ($fn_name:ident, $key:literal) => {
        pub fn $fn_name() -> FluidRef {
            find($key).expect(concat!("generated fluid must exist: ", $key))
        }
    };
}

fluid_fn!(air, "Air");
fluid_fn!(nitrogen, "N2");
fluid_fn!(oxygen, "O2");
fluid_fn!(nitrous_oxide, "N2O");
fluid_fn!(hydrogen, "H2");
fluid_fn!(helium, "He");
fluid_fn!(argon, "Ar");
fluid_fn!(neon, "Ne");
fluid_fn!(krypton, "Kr");
fluid_fn!(xenon, "Xe");
fluid_fn!(methane, "CH4");
fluid_fn!(ethane, "Ethane");
fluid_fn!(ethylene, "Ethylene");
fluid_fn!(propane, "Propane");
fluid_fn!(propylene, "Propylene");
fluid_fn!(n_butane, "nButane");
fluid_fn!(isobutane, "Isobutane");
fluid_fn!(n_pentane, "nPentane");
fluid_fn!(isopentane, "Isopentane");
fluid_fn!(n_hexane, "nHexane");
fluid_fn!(carbon_dioxide, "CO2");
fluid_fn!(carbon_monoxide, "CO");
fluid_fn!(water, "H2O");
fluid_fn!(ammonia, "NH3");
fluid_fn!(sulfur_dioxide, "SO2");
fluid_fn!(r32, "R32");
fluid_fn!(r125, "R125");
fluid_fn!(r134a, "R134a");
fluid_fn!(r152a, "R152a");
fluid_fn!(r245fa, "R245fa");
fluid_fn!(r1234yf, "R1234yf");

#[cfg(test)]
mod tests {
    use super::*;
    use eng_core::units::typed::{density, pressure, temperature};

    #[test]
    fn explicit_tp_and_direct_accessors_work() {
        let state = water()
            .state_tp(temperature::k(300.0), pressure::bar(1.0))
            .unwrap();
        assert!(state.pressure() > 0.0);
        assert!(state.temperature() > 0.0);
        assert!(state.density().unwrap() > 0.0);
        assert!(state.dynamic_viscosity().unwrap() > 0.0);
    }

    #[test]
    fn generic_state_aliases_work() {
        let state = air()
            .state("T", "300 K", "pressure", pressure::bar(1.0))
            .unwrap();
        assert_eq!(state.input_pair(), FluidInputPair::PT);
        assert!(state.gamma().unwrap() > 1.0);
    }

    #[test]
    fn generic_state_distinguishes_h_and_u() {
        let err = air()
            .state("u", "400 kJ/kg", "P", "1 bar")
            .expect_err("u should be rejected");
        assert!(err.to_string().contains("internal energy"));
    }

    #[test]
    fn saturation_states_expose_quality() {
        let sat = water().saturation_at_pressure("1 bar").unwrap();
        assert_eq!(sat.liquid.quality(), Some(0.0));
        assert_eq!(sat.vapor.quality(), Some(1.0));
        assert!(sat.liquid.temperature() > 0.0);
    }

    #[test]
    fn unsupported_pair_reports_error() {
        let err = air()
            .state("rho", density::kg_per_m3(1.2), "T", "300 K")
            .expect_err("rho-t should not be supported");
        let msg = err.to_string();
        assert!(msg.contains("unsupported state input pair"));
        assert!(msg.contains("T,P"));
    }
}
