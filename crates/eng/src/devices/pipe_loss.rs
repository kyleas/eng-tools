use eng_core::units::typed::QuantityKind;
use eng_core::units::{ensure_signature_matches_dimension, parse_equation_quantity_to_si};
use eng_fluids::FluidState;
use equations::{IntoSolveInput, SolveInputValue, eq, fluids};
use thiserror::Error;

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
