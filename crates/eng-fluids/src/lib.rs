use std::str::FromStr;

use eng_core::units::parse_equation_quantity_to_si;
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
}

impl FromStr for FluidProperty {
    type Err = FluidError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "density" | "rho" => Ok(Self::Density),
            "specific_heat_capacity" | "cp" => Ok(Self::SpecificHeatCapacity),
            "specific_heat_capacity_cv" | "cv" => Ok(Self::SpecificHeatCapacityCv),
            "gamma" | "heat_capacity_ratio" => Ok(Self::Gamma),
            "speed_of_sound" | "a" => Ok(Self::SpeedOfSound),
            "dynamic_viscosity" | "mu" | "viscosity" => Ok(Self::DynamicViscosity),
            "thermal_conductivity" | "k" => Ok(Self::ThermalConductivity),
            "temperature" | "t" => Ok(Self::Temperature),
            "pressure" | "p" => Ok(Self::Pressure),
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
];

#[derive(Debug, Clone)]
pub enum FluidInputValue {
    Si(f64),
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

#[derive(Debug, Error)]
pub enum FluidError {
    #[error("unknown fluid '{0}'")]
    UnknownFluid(String),
    #[error("unsupported fluid property '{0}'")]
    UnsupportedProperty(String),
    #[error("unit parse failed for {dimension}: {message}")]
    UnitParse { dimension: String, message: String },
    #[error("backend error: {0}")]
    Backend(String),
}

pub type Result<T> = std::result::Result<T, FluidError>;

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
            supported_state_inputs: vec!["T,P".to_string()],
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
        let t_k = parse_input(temperature.into_fluid_input(), "temperature")?;
        let p_pa = parse_input(pressure.into_fluid_input(), "pressure")?;
        Ok(FluidState {
            fluid: self,
            temperature_k: t_k,
            pressure_pa: p_pa,
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

    pub fn temperature_k(&self) -> f64 {
        self.temperature_k
    }

    pub fn pressure_pa(&self) -> f64 {
        self.pressure_pa
    }

    pub fn property(&self, property: FluidProperty) -> Result<f64> {
        match property {
            FluidProperty::Temperature => Ok(self.temperature_k),
            FluidProperty::Pressure => Ok(self.pressure_pa),
            FluidProperty::Density
            | FluidProperty::SpecificHeatCapacity
            | FluidProperty::SpecificHeatCapacityCv
            | FluidProperty::Gamma
            | FluidProperty::SpeedOfSound
            | FluidProperty::DynamicViscosity
            | FluidProperty::ThermalConductivity => {
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
                    .map_err(|e| FluidError::Backend(e.to_string()))?;
                match property {
                    FluidProperty::Density => model
                        .rho(&state)
                        .map(|x| x.value)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::SpecificHeatCapacity => model
                        .cp(&state)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::SpecificHeatCapacityCv => model
                        .cv(&state)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::Gamma => model
                        .gamma(&state)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::SpeedOfSound => model
                        .a(&state)
                        .map(|x| x.value)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::DynamicViscosity => model
                        .dynamic_viscosity(&state)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::ThermalConductivity => model
                        .thermal_conductivity(&state)
                        .map_err(|e| FluidError::Backend(e.to_string())),
                    FluidProperty::Temperature | FluidProperty::Pressure => unreachable!(),
                }
            }
        }
    }
}

fn parse_input(input: FluidInputValue, dimension: &str) -> Result<f64> {
    match input {
        FluidInputValue::Si(v) => Ok(v),
        FluidInputValue::WithUnit(text) => {
            parse_equation_quantity_to_si(dimension, &text).map_err(|e| FluidError::UnitParse {
                dimension: dimension.to_string(),
                message: e.to_string(),
            })
        }
    }
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
