use eng_core::units::typed::QuantityKind;
use eng_core::units::{ensure_signature_matches_dimension, parse_equation_quantity_to_si};
use eng_fluids::FluidState;
use equations::{IntoSolveInput, SolveInputValue, eq, fluids};
use thiserror::Error;

use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

#[derive(Debug, Clone, PartialEq)]
pub enum PipeFrictionModel {
    Fixed(f64),
    Colebrook,
}

#[derive(Debug, Clone)]
pub struct PipeLossResult {
    pub delta_p_pa: f64,
    pub friction_factor: f64,
    pub reynolds_number: Option<f64>,
    pub model_used: PipeFrictionModel,
}

impl PipeLossResult {
    pub fn delta_p(&self) -> f64 {
        self.delta_p_pa
    }

    pub fn reynolds_number(&self) -> Option<f64> {
        self.reynolds_number
    }

    pub fn friction_factor(&self) -> f64 {
        self.friction_factor
    }
}

#[derive(Debug, Error)]
pub enum PipeLossError {
    #[error("missing required input '{input}' for mode '{mode}'. {hint}")]
    MissingInput {
        input: &'static str,
        mode: &'static str,
        hint: &'static str,
    },
    #[error("invalid friction factor '{value}' (must be positive and finite)")]
    InvalidFrictionFactor { value: f64 },
    #[error("invalid value for '{input}': {message}")]
    InvalidInput {
        input: &'static str,
        message: String,
    },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
    #[error(transparent)]
    Fluid(#[from] eng_fluids::FluidError),
}

pub type Result<T> = std::result::Result<T, PipeLossError>;

#[derive(Debug, Clone)]
pub struct PipeLossDevice {
    friction_model: PipeFrictionModel,
    fluid: Option<FluidState>,
    rho: Option<SolveInputValue>,
    mu: Option<SolveInputValue>,
    v: Option<SolveInputValue>,
    d: Option<SolveInputValue>,
    l: Option<SolveInputValue>,
    eps: Option<SolveInputValue>,
}

pub fn pipe_loss() -> PipeLossDevice {
    PipeLossDevice::new()
}

const PIPE_LOSS_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "friction_model",
        description: "Colebrook or Fixed",
    },
    DeviceBindingArgSpec {
        name: "fixed_f",
        description: "Required when friction_model=Fixed",
    },
    DeviceBindingArgSpec {
        name: "rho",
        description: "Density input (optional with fluid context)",
    },
    DeviceBindingArgSpec {
        name: "mu",
        description: "Viscosity input (required for Colebrook without fluid context)",
    },
    DeviceBindingArgSpec {
        name: "v",
        description: "Velocity",
    },
    DeviceBindingArgSpec {
        name: "d",
        description: "Diameter",
    },
    DeviceBindingArgSpec {
        name: "l",
        description: "Length",
    },
    DeviceBindingArgSpec {
        name: "eps",
        description: "Roughness (Colebrook)",
    },
    DeviceBindingArgSpec {
        name: "fluid",
        description: "Optional fluid key (e.g. H2O)",
    },
    DeviceBindingArgSpec {
        name: "in1_key",
        description: "Fluid state input key 1",
    },
    DeviceBindingArgSpec {
        name: "in1_value",
        description: "Fluid state input value 1",
    },
    DeviceBindingArgSpec {
        name: "in2_key",
        description: "Fluid state input key 2",
    },
    DeviceBindingArgSpec {
        name: "in2_value",
        description: "Fluid state input value 2",
    },
];

const PIPE_LOSS_BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[DeviceBindingFunctionSpec {
    id: "device.pipe_loss.solve_delta_p",
    python_name: "pipe_loss_solve_delta_p",
    excel_name: "ENG_PIPE_LOSS_DELTA_P",
    op: "device.pipe_loss.solve_delta_p",
    fixed_args: &[],
    args: PIPE_LOSS_ARGS,
    returns: "f64",
    help: "Solve pipe pressure drop using Fixed/Colebrook friction model",
    rust_example: "eng::devices::pipe_loss().solve_delta_p()?",
    python_example: "engpy.devices.pipe_loss_solve_delta_p(...)",
    xloil_example: "=ENG_PIPE_LOSS_DELTA_P(...)",
    pyxll_example: "=ENG_PIPE_LOSS_DELTA_P(...)",
}];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\ndp = engpy.devices.pipe_loss_solve_delta_p(friction_model=\"Colebrook\", v=\"3 m/s\", d=\"0.1 m\", l=\"10 m\", eps=\"0.00015 in\", fluid=\"H2O\", in1_key=\"T\", in1_value=\"300 K\", in2_key=\"P\", in2_value=\"1 atm\")\nengpy.helpers.format_value(dp, \"Pa\", \"psia\")\n```\n\n### Excel\n```excel\n=ENG_PIPE_LOSS_DELTA_P(\"Colebrook\",,\"\",,\"3 m/s\",\"0.1 m\",\"10 m\",\"0.00015 in\",\"H2O\",\"T\",\"300 K\",\"P\",\"1 atm\")\n=ENG_FORMAT(ENG_PIPE_LOSS_DELTA_P(\"Colebrook\",,\"\",,\"3 m/s\",\"0.1 m\",\"10 m\",\"0.00015 in\",\"H2O\",\"T\",\"300 K\",\"P\",\"1 atm\"),\"Pa\",\"psia\")\n=ENG_META(\"device\",\"pipe_loss\",\"supported_modes\")\n```\n\n**Excel arguments**\n- `friction_model`: `Colebrook` or `Fixed`\n- `fixed_f`: fixed Darcy friction factor when model is `Fixed`\n- `density` / `viscosity` / `velocity` / `diameter` / `length` / `roughness`: direct engineering inputs\n- `fluid`, `in1_key`, `in1_value`, `in2_key`, `in2_value`: optional fluid-state context pair\n";

const OVERVIEW_MD: &str = "## Overview\n\nComposes Reynolds, friction-factor model, and Darcy-Weisbach pressure drop for a practical pipe-loss solve surface.\n\n### Modes\n- `Fixed`: direct friction factor\n- `Colebrook`: computes Reynolds + friction factor from roughness and viscosity\n\n### Rust\n```rust\nuse eng::devices::{pipe_loss, PipeFrictionModel};\nlet dp = pipe_loss()\n    .friction_model(PipeFrictionModel::Colebrook)\n    .given_rho(\"1000 kg/m^3\")\n    .given_mu(\"1 cP\")\n    .given_v(\"3 m/s\")\n    .given_d(\"0.1 m\")\n    .given_l(\"10 m\")\n    .given_eps(\"0.00015 in\")\n    .solve_delta_p()?;\nprintln!(\"delta_p = {dp} Pa\");\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: "pipe_loss",
        name: "Pipe Pressure Drop",
        summary: "Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss.",
        supported_modes: &["Fixed friction factor", "Colebrook"],
        outputs: &["delta_p (Pa)", "friction_factor", "reynolds_number"],
        route: "devices/pipe_loss.md",
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "fluids.reynolds_number",
            "fluids.colebrook",
            "fluids.darcy_weisbach_pressure_drop",
        ],
        binding_functions: PIPE_LOSS_BINDING_FUNCTIONS,
    }
}

impl Default for PipeLossDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl PipeLossDevice {
    pub fn new() -> Self {
        Self {
            friction_model: PipeFrictionModel::Colebrook,
            fluid: None,
            rho: None,
            mu: None,
            v: None,
            d: None,
            l: None,
            eps: None,
        }
    }

    pub fn friction_model(mut self, model: PipeFrictionModel) -> Self {
        self.friction_model = model;
        self
    }

    pub fn fluid(mut self, fluid: FluidState) -> Self {
        self.fluid = Some(fluid);
        self
    }

    pub fn given_rho<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.rho = Some(value.into_solve_input());
        self
    }

    pub fn given_mu<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.mu = Some(value.into_solve_input());
        self
    }

    pub fn given_v<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.v = Some(value.into_solve_input());
        self
    }

    pub fn given_d<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.d = Some(value.into_solve_input());
        self
    }

    pub fn given_l<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.l = Some(value.into_solve_input());
        self
    }

    pub fn given_eps<V: IntoSolveInput>(mut self, value: V) -> Self {
        self.eps = Some(value.into_solve_input());
        self
    }

    pub fn solve_delta_p(self) -> Result<f64> {
        Ok(self.solve()?.delta_p_pa)
    }

    pub fn solve(self) -> Result<PipeLossResult> {
        let rho = self.resolve_rho()?;
        let v = resolve_input(
            self.v.as_ref(),
            "velocity",
            "v",
            &self.friction_model_mode_name(),
        )?;
        let d = resolve_input(
            self.d.as_ref(),
            "length",
            "d",
            &self.friction_model_mode_name(),
        )?;
        let l = resolve_input(
            self.l.as_ref(),
            "length",
            "l",
            &self.friction_model_mode_name(),
        )?;

        if !(d > 0.0 && l >= 0.0 && rho > 0.0) {
            return Err(PipeLossError::InvalidInput {
                input: "rho/d/l",
                message: "expected rho>0, d>0, l>=0".to_string(),
            });
        }

        let (friction_factor, reynolds_number) = match self.friction_model.clone() {
            PipeFrictionModel::Fixed(f) => {
                if !(f.is_finite() && f > 0.0) {
                    return Err(PipeLossError::InvalidFrictionFactor { value: f });
                }
                (f, None)
            }
            PipeFrictionModel::Colebrook => {
                let mu = self.resolve_mu()?;
                let eps = resolve_input(self.eps.as_ref(), "length", "eps", "colebrook")?;
                if !(mu > 0.0 && eps >= 0.0) {
                    return Err(PipeLossError::InvalidInput {
                        input: "mu/eps",
                        message: "expected mu>0, eps>=0".to_string(),
                    });
                }
                let re = eq
                    .solve(fluids::reynolds_number::equation())
                    .target_re()
                    .given_rho(rho)
                    .given_v(v)
                    .given_d(d)
                    .given_mu(mu)
                    .value()?;
                let eps_d = eps / d;
                let f = eq
                    .solve(fluids::colebrook::equation())
                    .target_f()
                    .given_re(re)
                    .given_eps_d(eps_d)
                    .value()?;
                (f, Some(re))
            }
        };

        let delta_p = eq
            .solve(fluids::darcy_weisbach_pressure_drop::equation())
            .target_delta_p()
            .given_f(friction_factor)
            .given_l(l)
            .given_d(d)
            .given_rho(rho)
            .given_v(v)
            .value()?;

        Ok(PipeLossResult {
            delta_p_pa: delta_p,
            friction_factor,
            reynolds_number,
            model_used: self.friction_model,
        })
    }

    fn resolve_rho(&self) -> Result<f64> {
        if let Some(v) = self.rho.as_ref() {
            return resolve_input(Some(v), "density", "rho", &self.friction_model_mode_name());
        }
        if let Some(fluid) = self.fluid.as_ref() {
            return Ok(fluid.rho()?);
        }
        Err(PipeLossError::MissingInput {
            input: "rho",
            mode: self.friction_model_mode_name(),
            hint: "provide `given_rho(...)` or `.fluid(...)`",
        })
    }

    fn resolve_mu(&self) -> Result<f64> {
        if let Some(v) = self.mu.as_ref() {
            return resolve_input(Some(v), "viscosity", "mu", "colebrook");
        }
        if let Some(fluid) = self.fluid.as_ref() {
            return Ok(fluid.mu()?);
        }
        Err(PipeLossError::MissingInput {
            input: "mu",
            mode: "colebrook",
            hint: "provide `given_mu(...)` or `.fluid(...)`",
        })
    }

    fn friction_model_mode_name(&self) -> &'static str {
        match self.friction_model {
            PipeFrictionModel::Fixed(_) => "fixed_f",
            PipeFrictionModel::Colebrook => "colebrook",
        }
    }
}

fn resolve_input(
    value: Option<&SolveInputValue>,
    dimension: &str,
    input: &'static str,
    mode: &'static str,
) -> Result<f64> {
    let value = value.ok_or(PipeLossError::MissingInput {
        input,
        mode,
        hint: "set required device input",
    })?;
    solve_input_to_si(value, dimension, input)
}

fn solve_input_to_si(value: &SolveInputValue, dimension: &str, input: &'static str) -> Result<f64> {
    match value {
        SolveInputValue::Si(v) => Ok(*v),
        SolveInputValue::WithUnit(text) => {
            parse_equation_quantity_to_si(dimension, text).map_err(|e| {
                PipeLossError::InvalidInput {
                    input,
                    message: format!("{e} (input='{text}')"),
                }
            })
        }
        SolveInputValue::Typed(q) => {
            ensure_kind_matches_dimension(q.kind, dimension).map_err(|msg| {
                PipeLossError::InvalidInput {
                    input,
                    message: msg,
                }
            })?;
            Ok(q.value_si)
        }
        SolveInputValue::Expr(q) => {
            ensure_signature_matches_dimension(q.signature, dimension).map_err(|e| {
                PipeLossError::InvalidInput {
                    input,
                    message: format!("{e}"),
                }
            })?;
            Ok(q.value_si)
        }
    }
}

fn ensure_kind_matches_dimension(
    kind: QuantityKind,
    dimension: &str,
) -> std::result::Result<(), String> {
    let dim = dimension.trim().to_ascii_lowercase().replace(' ', "_");
    let ok = match dim.as_str() {
        "pressure" | "stress" => kind == QuantityKind::Pressure,
        "length" | "diameter" | "distance" | "roughness" => kind == QuantityKind::Length,
        "area" => kind == QuantityKind::Area,
        "density" => kind == QuantityKind::Density,
        "viscosity" | "dynamic_viscosity" => kind == QuantityKind::DynamicViscosity,
        "force" => kind == QuantityKind::Force,
        "moment" => kind == QuantityKind::Moment,
        "temperature" => kind == QuantityKind::Temperature,
        "thermal_conductivity" => kind == QuantityKind::ThermalConductivity,
        "heat_transfer_coefficient" => kind == QuantityKind::HeatTransferCoefficient,
        "mass_flow_rate" => kind == QuantityKind::MassFlowRate,
        "volumetric_flow_rate" => kind == QuantityKind::VolumetricFlowRate,
        "dimensionless" | "ratio" | "friction_factor" | "mach" => {
            kind == QuantityKind::Dimensionless
        }
        _ => true,
    };
    if ok {
        Ok(())
    } else {
        Err(format!(
            "typed quantity kind '{:?}' is not valid for dimension '{}'",
            kind, dimension
        ))
    }
}
