use equations::{compressible, eq};
use thiserror::Error;

use super::framework::{
    CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcSpec, method_label, path_text,
    run_pivot_calculation,
};

pub type CalcStep = super::framework::CalcStep;

const ISENTROPIC_INPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "mach_angle_deg",
        label: "Mach angle (deg)",
    },
    CalculatorKindSpec {
        key: "prandtl_meyer_angle_deg",
        label: "Prandtl-Meyer angle (deg)",
    },
    CalculatorKindSpec {
        key: "pressure_ratio",
        label: "p/p0",
    },
    CalculatorKindSpec {
        key: "temperature_ratio",
        label: "T/T0",
    },
    CalculatorKindSpec {
        key: "density_ratio",
        label: "rho/rho0",
    },
    CalculatorKindSpec {
        key: "area_ratio",
        label: "A/A*",
    },
];

const ISENTROPIC_OUTPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "mach_angle_deg",
        label: "Mach angle (deg)",
    },
    CalculatorKindSpec {
        key: "prandtl_meyer_angle_deg",
        label: "Prandtl-Meyer angle (deg)",
    },
    CalculatorKindSpec {
        key: "pressure_ratio",
        label: "p/p0",
    },
    CalculatorKindSpec {
        key: "temperature_ratio",
        label: "T/T0",
    },
    CalculatorKindSpec {
        key: "density_ratio",
        label: "rho/rho0",
    },
    CalculatorKindSpec {
        key: "area_ratio",
        label: "A/A*",
    },
];

pub const DEVICE_SPEC: CalculatorDeviceSpec = CalculatorDeviceSpec {
    key: "isentropic_calc",
    name: "Isentropic Calculator",
    summary: "Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration.",
    route: "devices/isentropic_calc.md",
    pivot_label: "Mach",
    input_kinds: ISENTROPIC_INPUT_KIND_SPECS,
    output_kinds: ISENTROPIC_OUTPUT_KIND_SPECS,
    branches: &["subsonic", "supersonic"],
};

pub fn supported_input_kinds_text() -> String {
    DEVICE_SPEC
        .input_kinds
        .iter()
        .map(|k| k.label)
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsentropicInputKind {
    Mach,
    MachAngleRad,
    PrandtlMeyerAngleRad,
    PressureRatio,
    TemperatureRatio,
    DensityRatio,
    AreaRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsentropicOutputKind {
    Mach,
    MachAngleRad,
    PrandtlMeyerAngleRad,
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

pub fn parse_input_kind(raw: &str, value: f64) -> Option<(IsentropicInputKind, f64)> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some((IsentropicInputKind::Mach, value)),
        "mach_angle" | "mach_angle_rad" | "mu" | "mu_rad" => {
            Some((IsentropicInputKind::MachAngleRad, value))
        }
        "mach_angle_deg" | "mu_deg" => {
            Some((IsentropicInputKind::MachAngleRad, value.to_radians()))
        }
        "prandtl_meyer_angle" | "prandtl_meyer_angle_rad" | "prandtl_meyer" | "nu" | "nu_rad" => {
            Some((IsentropicInputKind::PrandtlMeyerAngleRad, value))
        }
        "prandtl_meyer_angle_deg" | "prandtl_meyer_deg" | "nu_deg" => Some((
            IsentropicInputKind::PrandtlMeyerAngleRad,
            value.to_radians(),
        )),
        "pressure_ratio" | "p_p0" | "p/p0" => Some((IsentropicInputKind::PressureRatio, value)),
        "temperature_ratio" | "t_t0" | "t/t0" => {
            Some((IsentropicInputKind::TemperatureRatio, value))
        }
        "density_ratio" | "rho_rho0" | "rho/rho0" => {
            Some((IsentropicInputKind::DensityRatio, value))
        }
        "area_ratio" | "a_astar" | "a/a*" => Some((IsentropicInputKind::AreaRatio, value)),
        _ => None,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<(IsentropicOutputKind, bool)> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some((IsentropicOutputKind::Mach, false)),
        "mach_angle" | "mach_angle_rad" | "mu" | "mu_rad" => {
            Some((IsentropicOutputKind::MachAngleRad, false))
        }
        "mach_angle_deg" | "mu_deg" => Some((IsentropicOutputKind::MachAngleRad, true)),
        "prandtl_meyer_angle" | "prandtl_meyer_angle_rad" | "prandtl_meyer" | "nu" | "nu_rad" => {
            Some((IsentropicOutputKind::PrandtlMeyerAngleRad, false))
        }
        "prandtl_meyer_angle_deg" | "prandtl_meyer_deg" | "nu_deg" => {
            Some((IsentropicOutputKind::PrandtlMeyerAngleRad, true))
        }
        "pressure_ratio" | "p_p0" | "p/p0" => Some((IsentropicOutputKind::PressureRatio, false)),
        "temperature_ratio" | "t_t0" | "t/t0" => {
            Some((IsentropicOutputKind::TemperatureRatio, false))
        }
        "density_ratio" | "rho_rho0" | "rho/rho0" => {
            Some((IsentropicOutputKind::DensityRatio, false))
        }
        "area_ratio" | "a_astar" | "a/a*" => Some((IsentropicOutputKind::AreaRatio, false)),
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<IsentropicBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "subsonic" => Some(IsentropicBranch::Subsonic),
        "supersonic" => Some(IsentropicBranch::Supersonic),
        _ => None,
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
pub struct IsentropicCalcResponse {
    pub value_si: f64,
    pub pivot_mach: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

impl IsentropicCalcResponse {
    pub fn path_text(&self) -> String {
        path_text(&self.path)
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

struct IsentropicRuntime;

impl PivotCalcSpec for IsentropicRuntime {
    type Request = IsentropicCalcRequest;
    type Error = IsentropicCalcError;

    fn validate_request(&self, req: &Self::Request) -> Result<()> {
        if !req.gamma.is_finite() || req.gamma <= 1.0 {
            return Err(IsentropicCalcError::InvalidGamma { value: req.gamma });
        }
        if !req.input_value.is_finite() {
            return Err(IsentropicCalcError::InvalidInputDomain {
                kind: input_kind_label(req.input_kind),
                reason: "must be finite".to_string(),
            });
        }
        if matches!(req.input_kind, IsentropicInputKind::PrandtlMeyerAngleRad)
            && req.input_value < 0.0
        {
            return Err(IsentropicCalcError::InvalidInputDomain {
                kind: input_kind_label(req.input_kind),
                reason: "must be >= 0 rad".to_string(),
            });
        }
        Ok(())
    }

    fn resolve_pivot(&self, req: &Self::Request, path: &mut Vec<CalcStep>) -> Result<f64> {
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
            IsentropicInputKind::PrandtlMeyerAngleRad => {
                let solved = eq
                    .solve(compressible::prandtl_meyer::equation())
                    .target_m()
                    .given_nu(input)
                    .given_gamma(gamma)
                    .result()
                    .map_err(|source| {
                        let nu_max_hint = eq
                            .solve(compressible::prandtl_meyer::equation())
                            .target_nu()
                            .given_m(100.0)
                            .given_gamma(gamma)
                            .value()
                            .unwrap_or(2.276_853_163_690_696);
                        IsentropicCalcError::InvalidInputDomain {
                            kind: "PrandtlMeyerAngleRad",
                            reason: format!(
                                "expected 0 <= nu < ~{nu_max_hint:.12} rad for gamma={gamma}; solver detail: {source}"
                            ),
                        }
                    })?;
                path.push(CalcStep {
                    equation_path_id: "compressible.prandtl_meyer".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("nu".to_string(), input), ("gamma".to_string(), gamma)],
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
                    inputs_used: vec![
                        ("rho_rho0".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
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
                    inputs_used: vec![
                        ("area_ratio".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
                });
                Ok(solved.value_si)
            }
        }
    }

    fn validate_pivot(&self, pivot_value: f64) -> Result<()> {
        if pivot_value.is_finite() && pivot_value > 0.0 {
            Ok(())
        } else {
            Err(IsentropicCalcError::InvalidInputDomain {
                kind: "Mach",
                reason: "resolved pivot Mach must be finite and > 0".to_string(),
            })
        }
    }

    fn solve_target(
        &self,
        req: &Self::Request,
        mach: f64,
        path: &mut Vec<CalcStep>,
    ) -> Result<f64> {
        let gamma = req.gamma;
        match req.target_kind {
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
            IsentropicOutputKind::PrandtlMeyerAngleRad => {
                let solved = eq
                    .solve(compressible::prandtl_meyer::equation())
                    .target_nu()
                    .given_m(mach)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.prandtl_meyer".to_string(),
                    solved_for: "nu".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
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
                if let Some(b) = req.branch {
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
}

pub fn calc(req: IsentropicCalcRequest) -> Result<IsentropicCalcResponse> {
    let out = run_pivot_calculation(&IsentropicRuntime, req)?;
    Ok(IsentropicCalcResponse {
        value_si: out.value_si,
        pivot_mach: out.pivot_value,
        path: out.path,
        warnings: out.warnings,
    })
}

fn input_kind_label(kind: IsentropicInputKind) -> &'static str {
    match kind {
        IsentropicInputKind::Mach => "Mach",
        IsentropicInputKind::MachAngleRad => "MachAngleRad",
        IsentropicInputKind::PrandtlMeyerAngleRad => "PrandtlMeyerAngleRad",
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
            input_value: 0.127_804_525_462_950_93,
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

    #[test]
    fn prandtl_meyer_angle_to_pressure_ratio_chain_works() {
        let gamma = 1.4;
        let m = 3.0_f64;
        let nu = eq
            .solve(compressible::prandtl_meyer::equation())
            .target_nu()
            .given_m(m)
            .given_gamma(gamma)
            .value()
            .expect("nu from M");
        let out = calc(IsentropicCalcRequest {
            gamma,
            input_kind: IsentropicInputKind::PrandtlMeyerAngleRad,
            input_value: nu,
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
        assert!((out.pivot_mach - m).abs() < 1e-8);
        assert!((out.value_si - expected).abs() < 1e-8);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.prandtl_meyer"),
            "expected registry-backed prandtl_meyer step in path"
        );
    }

    #[test]
    fn invalid_prandtl_meyer_angle_domain_is_rejected() {
        let err = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::PrandtlMeyerAngleRad,
            input_value: 10.0,
            target_kind: IsentropicOutputKind::Mach,
            branch: None,
        })
        .expect_err("nu out of domain should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("PrandtlMeyerAngleRad") && msg.contains("expected 0 <="),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn mach_to_prandtl_meyer_angle_target_works() {
        let out = calc(IsentropicCalcRequest {
            gamma: 1.4,
            input_kind: IsentropicInputKind::Mach,
            input_value: 2.0,
            target_kind: IsentropicOutputKind::PrandtlMeyerAngleRad,
            branch: None,
        })
        .expect("mach -> nu");
        assert!((out.value_si - 0.460_413_682_082_694_73).abs() < 1e-10);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.prandtl_meyer"),
            "expected prandtl_meyer in path"
        );
    }
}
