use equations::{SolveMethod, compressible, eq};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsentropicInputKind {
    Mach,
    MachAngleRad,
    PressureRatio,
    TemperatureRatio,
    DensityRatio,
    AreaRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsentropicOutputKind {
    Mach,
    MachAngleRad,
    PressureRatio,
    TemperatureRatio,
    DensityRatio,
    AreaRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsentropicBranch {
    Subsonic,
    Supersonic,
}

impl IsentropicBranch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Subsonic => "subsonic",
            Self::Supersonic => "supersonic",
        }
    }
}

#[derive(Debug, Clone)]
pub struct IsentropicCalcRequest {
    pub gamma: f64,
    pub input_kind: IsentropicInputKind,
    pub input_value: f64,
    pub target_kind: IsentropicOutputKind,
    pub branch: Option<IsentropicBranch>,
}

#[derive(Debug, Clone)]
pub struct CalcStep {
    pub equation_path_id: String,
    pub solved_for: String,
    pub method: String,
    pub branch: Option<String>,
    pub inputs_used: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct IsentropicCalcResponse {
    pub value_si: f64,
    pub pivot_mach: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

impl IsentropicCalcResponse {
    pub fn path_text(&self) -> String {
        self.path
            .iter()
            .map(|s| match &s.branch {
                Some(b) => format!(
                    "{}:{} via {} [{}]",
                    s.equation_path_id, s.solved_for, s.method, b
                ),
                None => format!("{}:{} via {}", s.equation_path_id, s.solved_for, s.method),
            })
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

#[derive(Debug, Error)]
pub enum IsentropicCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: subsonic, supersonic)")]
    MissingBranch { kind: &'static str },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, IsentropicCalcError>;

#[derive(Debug, Clone)]
pub struct IsentropicCalculatorDevice {
    req: IsentropicCalcRequest,
}

pub fn isentropic_calc() -> IsentropicCalculatorDevice {
    IsentropicCalculatorDevice::new()
}

impl Default for IsentropicCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl IsentropicCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: IsentropicCalcRequest {
                gamma: 1.4,
                input_kind: IsentropicInputKind::Mach,
                input_value: 1.0,
                target_kind: IsentropicOutputKind::PressureRatio,
                branch: None,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn input(mut self, kind: IsentropicInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: IsentropicOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: IsentropicBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn solve(self) -> Result<IsentropicCalcResponse> {
        calc(self.req)
    }
}

pub fn calc(req: IsentropicCalcRequest) -> Result<IsentropicCalcResponse> {
    if !req.gamma.is_finite() || req.gamma <= 1.0 {
        return Err(IsentropicCalcError::InvalidGamma { value: req.gamma });
    }
    if !req.input_value.is_finite() {
        return Err(IsentropicCalcError::InvalidInputDomain {
            kind: input_kind_label(req.input_kind),
            reason: "must be finite".to_string(),
        });
    }

    let mut path = Vec::<CalcStep>::new();
    let pivot_mach = resolve_pivot_mach(&req, &mut path)?;
    if !(pivot_mach.is_finite() && pivot_mach > 0.0) {
        return Err(IsentropicCalcError::InvalidInputDomain {
            kind: "Mach",
            reason: "resolved pivot Mach must be finite and > 0".to_string(),
        });
    }

    let value_si = solve_target(req.gamma, req.target_kind, pivot_mach, req.branch, &mut path)?;
    Ok(IsentropicCalcResponse {
        value_si,
        pivot_mach,
        path,
        warnings: Vec::new(),
    })
}

fn resolve_pivot_mach(req: &IsentropicCalcRequest, path: &mut Vec<CalcStep>) -> Result<f64> {
    let gamma = req.gamma;
    let input = req.input_value;
    match req.input_kind {
        IsentropicInputKind::Mach => Ok(input),
        IsentropicInputKind::MachAngleRad => {
            let solved = eq
                .solve(compressible::mach_angle::equation())
                .target_m()
                .given_mu(input)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.mach_angle".to_string(),
                solved_for: "M".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("mu".to_string(), input)],
            });
            Ok(solved.value_si)
        }
        IsentropicInputKind::PressureRatio => {
            let solved = eq
                .solve(compressible::isentropic_pressure_ratio::equation())
                .target_m()
                .given_p_p0(input)
                .given_gamma(gamma)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_pressure_ratio".to_string(),
                solved_for: "M".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("p_p0".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicInputKind::TemperatureRatio => {
            let solved = eq
                .solve(compressible::isentropic_temperature_ratio::equation())
                .target_m()
                .given_t_t0(input)
                .given_gamma(gamma)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_temperature_ratio".to_string(),
                solved_for: "M".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("T_T0".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicInputKind::DensityRatio => {
            let solved = eq
                .solve(compressible::isentropic_density_ratio::equation())
                .target_m()
                .given_rho_rho0(input)
                .given_gamma(gamma)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_density_ratio".to_string(),
                solved_for: "M".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("rho_rho0".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicInputKind::AreaRatio => {
            let Some(branch) = req.branch else {
                return Err(IsentropicCalcError::MissingBranch { kind: "AreaRatio" });
            };
            let solved = eq
                .solve(compressible::area_mach::equation())
                .target_m()
                .branch(branch.as_str())
                .given_area_ratio(input)
                .given_gamma(gamma)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.area_mach".to_string(),
                solved_for: "M".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("area_ratio".to_string(), input), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
    }
}

fn solve_target(
    gamma: f64,
    target: IsentropicOutputKind,
    mach: f64,
    branch: Option<IsentropicBranch>,
    path: &mut Vec<CalcStep>,
) -> Result<f64> {
    match target {
        IsentropicOutputKind::Mach => Ok(mach),
        IsentropicOutputKind::MachAngleRad => {
            let solved = eq
                .solve(compressible::mach_angle::equation())
                .target_mu()
                .given_m(mach)
                .result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.mach_angle".to_string(),
                solved_for: "mu".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("M".to_string(), mach)],
            });
            Ok(solved.value_si)
        }
        IsentropicOutputKind::PressureRatio => {
            let solved = eq.solve_result(
                compressible::isentropic_pressure_ratio::equation(),
                "p_p0",
                [("M", mach), ("gamma", gamma)],
            )?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_pressure_ratio".to_string(),
                solved_for: "p_p0".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicOutputKind::TemperatureRatio => {
            let solved = eq.solve_result(
                compressible::isentropic_temperature_ratio::equation(),
                "T_T0",
                [("M", mach), ("gamma", gamma)],
            )?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_temperature_ratio".to_string(),
                solved_for: "T_T0".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicOutputKind::DensityRatio => {
            let solved = eq.solve_result(
                compressible::isentropic_density_ratio::equation(),
                "rho_rho0",
                [("M", mach), ("gamma", gamma)],
            )?;
            path.push(CalcStep {
                equation_path_id: "compressible.isentropic_density_ratio".to_string(),
                solved_for: "rho_rho0".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
            });
            Ok(solved.value_si)
        }
        IsentropicOutputKind::AreaRatio => {
            let mut builder = eq
                .solve(compressible::area_mach::equation())
                .target_area_ratio()
                .given_m(mach)
                .given_gamma(gamma);
            if let Some(b) = branch {
                builder = builder.branch(b.as_str());
            }
            let solved = builder.result()?;
            path.push(CalcStep {
                equation_path_id: "compressible.area_mach".to_string(),
                solved_for: "area_ratio".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
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

fn input_kind_label(kind: IsentropicInputKind) -> &'static str {
    match kind {
        IsentropicInputKind::Mach => "Mach",
        IsentropicInputKind::MachAngleRad => "MachAngleRad",
        IsentropicInputKind::PressureRatio => "PressureRatio",
        IsentropicInputKind::TemperatureRatio => "TemperatureRatio",
        IsentropicInputKind::DensityRatio => "DensityRatio",
        IsentropicInputKind::AreaRatio => "AreaRatio",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use equations::{compressible, eq};

    #[test]
    fn mach_angle_to_pressure_ratio_chain_works() {
        let gamma = 1.4;
        let m = 2.0_f64;
        let mu = (1.0 / m).asin();
        let out = calc(IsentropicCalcRequest {
            gamma,
            input_kind: IsentropicInputKind::MachAngleRad,
            input_value: mu,
            target_kind: IsentropicOutputKind::PressureRatio,
            branch: None,
        })
        .expect("isentropic calc");
        let expected = eq
            .solve(compressible::isentropic_pressure_ratio::equation())
            .target_p_p0()
            .given_m(m)
            .given_gamma(gamma)
            .value()
            .expect("direct pressure ratio");
        assert!((out.pivot_mach - m).abs() < 1e-10);
        assert!((out.value_si - expected).abs() < 1e-10);
        assert!(!out.path.is_empty());
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.mach_angle"),
            "expected registry-backed mach_angle step in path"
        );
    }

    #[test]
    fn pressure_ratio_to_area_ratio_chain_works() {
        let out = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::PressureRatio,
            input_value: 0.127_804_525_462_950_93, // M = 2.0 at gamma=1.4
            target_kind: IsentropicOutputKind::AreaRatio,
            branch: None,
        })
        .expect("isentropic calc");
        let expected = eq
            .solve(compressible::area_mach::equation())
            .target_area_ratio()
            .given_m(out.pivot_mach)
            .given_gamma(1.4)
            .value()
            .expect("area ratio from pivot");
        assert!(out.pivot_mach > 0.0);
        assert!((out.value_si - expected).abs() < 1e-10);
    }

    #[test]
    fn area_ratio_to_mach_requires_branch() {
        let err = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::AreaRatio,
            input_value: 2.0,
            target_kind: IsentropicOutputKind::Mach,
            branch: None,
        })
        .expect_err("missing branch should error");
        assert!(matches!(err, IsentropicCalcError::MissingBranch { .. }));
    }

    #[test]
    fn area_ratio_branch_selects_solution() {
        let sub = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::AreaRatio,
            input_value: 2.0,
            target_kind: IsentropicOutputKind::Mach,
            branch: Some(IsentropicBranch::Subsonic),
        })
        .expect("subsonic branch");
        let sup = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::AreaRatio,
            input_value: 2.0,
            target_kind: IsentropicOutputKind::Mach,
            branch: Some(IsentropicBranch::Supersonic),
        })
        .expect("supersonic branch");
        assert!(sub.value_si > 0.0 && sub.value_si < 1.0);
        assert!(sup.value_si > 1.0);
    }
}
