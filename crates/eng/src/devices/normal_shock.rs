use equations::{SolveMethod, compressible, eq};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalShockInputKind {
    M1,
    M2,
    PressureRatio,
    DensityRatio,
    TemperatureRatio,
    StagnationPressureRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalShockOutputKind {
    M1,
    M2,
    PressureRatio,
    DensityRatio,
    TemperatureRatio,
    StagnationPressureRatio,
}

#[derive(Debug, Clone)]
pub struct NormalShockCalcRequest {
    pub gamma: f64,
    pub input_kind: NormalShockInputKind,
    pub input_value: f64,
    pub target_kind: NormalShockOutputKind,
}

#[derive(Debug, Clone)]
pub struct NormalShockCalcStep {
    pub equation_path_id: String,
    pub solved_for: String,
    pub method: String,
    pub inputs_used: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct NormalShockCalcResponse {
    pub value_si: f64,
    pub pivot_m1: f64,
    pub path: Vec<NormalShockCalcStep>,
    pub warnings: Vec<String>,
}

impl NormalShockCalcResponse {
    pub fn path_text(&self) -> String {
        self.path
            .iter()
            .map(|s| format!("{}:{} via {}", s.equation_path_id, s.solved_for, s.method))
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

#[derive(Debug, Error)]
pub enum NormalShockCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, NormalShockCalcError>;

#[derive(Debug, Clone)]
pub struct NormalShockCalculatorDevice {
    req: NormalShockCalcRequest,
}

pub fn normal_shock_calc() -> NormalShockCalculatorDevice {
    NormalShockCalculatorDevice::new()
}

impl Default for NormalShockCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl NormalShockCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: NormalShockCalcRequest {
                gamma: 1.4,
                input_kind: NormalShockInputKind::M1,
                input_value: 2.0,
                target_kind: NormalShockOutputKind::M2,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn input(mut self, kind: NormalShockInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: NormalShockOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn solve(self) -> Result<NormalShockCalcResponse> {
        calc(self.req)
    }
}

pub fn calc(req: NormalShockCalcRequest) -> Result<NormalShockCalcResponse> {
    if !req.gamma.is_finite() || req.gamma <= 1.0 {
        return Err(NormalShockCalcError::InvalidGamma { value: req.gamma });
    }
    if !req.input_value.is_finite() {
        return Err(NormalShockCalcError::InvalidInputDomain {
            kind: input_kind_label(req.input_kind),
            reason: "must be finite".to_string(),
        });
    }

    validate_input_domain(req.input_kind, req.input_value)?;

    let mut path = Vec::<NormalShockCalcStep>::new();
    let pivot_m1 = resolve_pivot_m1(&req, &mut path)?;
    if !(pivot_m1.is_finite() && pivot_m1 >= 1.0) {
        return Err(NormalShockCalcError::InvalidInputDomain {
            kind: "M1",
            reason: "resolved pivot M1 must be finite and >= 1".to_string(),
        });
    }

    let value_si = solve_target(req.gamma, req.target_kind, pivot_m1, &mut path)?;
    Ok(NormalShockCalcResponse {
        value_si,
        pivot_m1,
        path,
        warnings: Vec::new(),
    })
}

fn validate_input_domain(kind: NormalShockInputKind, value: f64) -> Result<()> {
    let invalid = match kind {
        NormalShockInputKind::M1 => value < 1.0,
        NormalShockInputKind::M2 => !(value > 0.0 && value <= 1.0),
        NormalShockInputKind::PressureRatio
        | NormalShockInputKind::DensityRatio
        | NormalShockInputKind::TemperatureRatio => value < 1.0,
        NormalShockInputKind::StagnationPressureRatio => !(value > 0.0 && value <= 1.0),
    };
    if invalid {
        return Err(NormalShockCalcError::InvalidInputDomain {
            kind: input_kind_label(kind),
            reason: "value is outside physical normal-shock domain".to_string(),
        });
    }
    Ok(())
}

fn resolve_pivot_m1(
    req: &NormalShockCalcRequest,
    path: &mut Vec<NormalShockCalcStep>,
) -> Result<f64> {
    let gamma = req.gamma;
    let input = req.input_value;
    match req.input_kind {
        NormalShockInputKind::M1 => Ok(input),
        NormalShockInputKind::M2 => {
            let solved = eq
                .solve(compressible::normal_shock_m2::equation())
                .target_m1()
                .given_m2(input)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_m2".to_string(),
                solved_for: "M1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M2".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockInputKind::PressureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_pressure_ratio::equation())
                .target_m1()
                .given_p2_p1(input)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_pressure_ratio".to_string(),
                solved_for: "M1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("p2_p1".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockInputKind::DensityRatio => {
            let solved = eq
                .solve(compressible::normal_shock_density_ratio::equation())
                .target_m1()
                .given_rho2_rho1(input)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_density_ratio".to_string(),
                solved_for: "M1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![
                    ("rho2_rho1".to_string(), input),
                    ("gamma".to_string(), gamma),
                ],
            });
            Ok(solved.value_si)
        }
        NormalShockInputKind::TemperatureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_temperature_ratio::equation())
                .target_m1()
                .given_t2_t1(input)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_temperature_ratio".to_string(),
                solved_for: "M1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("T2_T1".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockInputKind::StagnationPressureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_stagnation_pressure_ratio::equation())
                .target_m1()
                .given_p02_p01(input)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_stagnation_pressure_ratio".to_string(),
                solved_for: "M1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("p02_p01".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
    }
}

fn solve_target(
    gamma: f64,
    target: NormalShockOutputKind,
    m1: f64,
    path: &mut Vec<NormalShockCalcStep>,
) -> Result<f64> {
    match target {
        NormalShockOutputKind::M1 => Ok(m1),
        NormalShockOutputKind::M2 => {
            let solved = eq
                .solve(compressible::normal_shock_m2::equation())
                .target_m2()
                .given_m1(m1)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_m2".to_string(),
                solved_for: "M2".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M1".to_string(), m1), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockOutputKind::PressureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_pressure_ratio::equation())
                .target_p2_p1()
                .given_m1(m1)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_pressure_ratio".to_string(),
                solved_for: "p2_p1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M1".to_string(), m1), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockOutputKind::DensityRatio => {
            let solved = eq
                .solve(compressible::normal_shock_density_ratio::equation())
                .target_rho2_rho1()
                .given_m1(m1)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_density_ratio".to_string(),
                solved_for: "rho2_rho1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M1".to_string(), m1), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockOutputKind::TemperatureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_temperature_ratio::equation())
                .target_t2_t1()
                .given_m1(m1)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_temperature_ratio".to_string(),
                solved_for: "T2_T1".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M1".to_string(), m1), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        NormalShockOutputKind::StagnationPressureRatio => {
            let solved = eq
                .solve(compressible::normal_shock_stagnation_pressure_ratio::equation())
                .target_p02_p01()
                .given_m1(m1)
                .given_gamma(gamma)
                .result()?;
            path.push(NormalShockCalcStep {
                equation_path_id: "compressible.normal_shock_stagnation_pressure_ratio".to_string(),
                solved_for: "p02_p01".to_string(),
                method: method_label(solved.method),
                inputs_used: vec![("M1".to_string(), m1), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
    }
}

fn method_label(method: SolveMethod) -> String {
    match method {
        SolveMethod::Auto => "auto",
        SolveMethod::Explicit => "explicit",
        SolveMethod::Numerical => "numerical",
    }
    .to_string()
}

fn input_kind_label(kind: NormalShockInputKind) -> &'static str {
    match kind {
        NormalShockInputKind::M1 => "M1",
        NormalShockInputKind::M2 => "M2",
        NormalShockInputKind::PressureRatio => "p2_p1",
        NormalShockInputKind::DensityRatio => "rho2_rho1",
        NormalShockInputKind::TemperatureRatio => "T2_T1",
        NormalShockInputKind::StagnationPressureRatio => "p02_p01",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m1_to_m2_works() {
        let out = calc(NormalShockCalcRequest {
            gamma: 1.4,
            input_kind: NormalShockInputKind::M1,
            input_value: 2.0,
            target_kind: NormalShockOutputKind::M2,
        })
        .expect("normal shock calc");
        assert!((out.value_si - 0.577_350_269_189_625_7).abs() < 1e-10);
    }

    #[test]
    fn m1_to_pressure_ratio_works() {
        let out = calc(NormalShockCalcRequest {
            gamma: 1.4,
            input_kind: NormalShockInputKind::M1,
            input_value: 2.0,
            target_kind: NormalShockOutputKind::PressureRatio,
        })
        .expect("normal shock calc");
        assert!((out.value_si - 4.5).abs() < 1e-10);
    }

    #[test]
    fn pressure_ratio_to_m1_to_temperature_ratio_chain_works() {
        let out = calc(NormalShockCalcRequest {
            gamma: 1.4,
            input_kind: NormalShockInputKind::PressureRatio,
            input_value: 4.5,
            target_kind: NormalShockOutputKind::TemperatureRatio,
        })
        .expect("normal shock calc");
        assert!((out.pivot_m1 - 2.0).abs() < 1e-8);
        assert!((out.value_si - 1.687_499_999_999_999_8).abs() < 1e-8);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.normal_shock_pressure_ratio")
        );
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.normal_shock_temperature_ratio")
        );
    }

    #[test]
    fn p02_p01_to_m1_to_m2_chain_works() {
        let out = calc(NormalShockCalcRequest {
            gamma: 1.4,
            input_kind: NormalShockInputKind::StagnationPressureRatio,
            input_value: 0.720_873_861_484_745_5,
            target_kind: NormalShockOutputKind::M2,
        })
        .expect("normal shock calc");
        assert!((out.pivot_m1 - 2.0).abs() < 1e-8);
        assert!((out.value_si - 0.577_350_269_189_625_7).abs() < 1e-8);
    }

    #[test]
    fn invalid_domain_values_are_rejected() {
        let err = calc(NormalShockCalcRequest {
            gamma: 1.4,
            input_kind: NormalShockInputKind::M1,
            input_value: 0.8,
            target_kind: NormalShockOutputKind::M2,
        })
        .expect_err("subsonic M1 should fail");
        assert!(
            err.to_string()
                .contains("outside physical normal-shock domain")
        );
    }
}
