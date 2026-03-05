use equations::{SolveMethod, compressible, eq};
use thiserror::Error;

use crate::solve::numeric::find_roots_by_scan_bisection;

use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObliqueShockInputKind {
    BetaRad,
    ThetaRad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObliqueShockOutputKind {
    ThetaRad,
    BetaRad,
    Mn1,
    Mn2,
    M2,
    PressureRatio,
    DensityRatio,
    TemperatureRatio,
    StagnationPressureRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObliqueShockBranch {
    Weak,
    Strong,
}

impl ObliqueShockBranch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Weak => "weak",
            Self::Strong => "strong",
        }
    }
}

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "m1",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (beta_deg or theta_deg)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value in degrees",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (theta_deg, beta_deg, mn1, mn2, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Weak/strong branch required for theta->beta inversion paths",
    },
];

const M1_BETA_GAMMA_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "m1",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Shock angle beta in degrees",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
];

const M1_THETA_GAMMA_BRANCH_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "m1",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Flow deflection theta in degrees",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Weak or strong branch",
    },
];

const FIXED_M1_BETA_TO_THETA: &[(&str, &str)] =
    &[("input_kind", "beta_deg"), ("target_kind", "theta_deg")];
const FIXED_M1_THETA_TO_BETA: &[(&str, &str)] =
    &[("input_kind", "theta_deg"), ("target_kind", "beta_deg")];
const FIXED_M1_THETA_TO_P2P1: &[(&str, &str)] =
    &[("input_kind", "theta_deg"), ("target_kind", "p2_p1")];
const FIXED_M1_BETA_TO_M2: &[(&str, &str)] = &[("input_kind", "beta_deg"), ("target_kind", "m2")];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc",
        python_name: "oblique_shock_calc",
        excel_name: "ENG_OBLIQUE_SHOCK",
        op: "device.oblique_shock_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Oblique shock calculator: (M1 + beta/theta) -> target with weak/strong branch support",
        rust_example: "eng::devices::oblique_shock_calc().solve()?",
        python_example: "engpy.devices.oblique_shock_calc(2.0, \"theta_deg\", 10.0, \"beta_deg\", 1.4, \"weak\")",
        xloil_example: "=ENG_OBLIQUE_SHOCK(2.0,\"theta_deg\",10.0,\"beta_deg\",1.4,\"weak\")",
        pyxll_example: "=ENG_OBLIQUE_SHOCK(2.0,\"theta_deg\",10.0,\"beta_deg\",1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc.path_text",
        python_name: "oblique_shock_path_text",
        excel_name: "ENG_OBLIQUE_SHOCK_PATH_TEXT",
        op: "device.oblique_shock_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Oblique shock calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.oblique_shock_path_text(2.0, \"theta_deg\", 10.0, \"p2_p1\", 1.4, \"weak\")",
        xloil_example: "=ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,\"theta_deg\",10.0,\"p2_p1\",1.4,\"weak\")",
        pyxll_example: "=ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,\"theta_deg\",10.0,\"p2_p1\",1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc.from_m1_beta.to_theta",
        python_name: "oblique_shock_from_m1_beta_to_theta",
        excel_name: "ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA",
        op: "device.oblique_shock_calc.value",
        fixed_args: FIXED_M1_BETA_TO_THETA,
        args: M1_BETA_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience oblique-shock path: (M1, beta_deg) -> theta_deg",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.oblique_shock_from_m1_beta_to_theta(2.0, 40.0, 1.4)",
        xloil_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)",
        pyxll_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc.from_m1_theta.to_beta",
        python_name: "oblique_shock_from_m1_theta_to_beta",
        excel_name: "ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA",
        op: "device.oblique_shock_calc.value",
        fixed_args: FIXED_M1_THETA_TO_BETA,
        args: M1_THETA_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience oblique-shock path: (M1, theta_deg, branch) -> beta_deg",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.oblique_shock_from_m1_theta_to_beta(2.0, 10.0, 1.4, \"weak\")",
        xloil_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,\"weak\")",
        pyxll_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc.from_m1_theta.to_p2_p1",
        python_name: "oblique_shock_from_m1_theta_to_p2_p1",
        excel_name: "ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1",
        op: "device.oblique_shock_calc.value",
        fixed_args: FIXED_M1_THETA_TO_P2P1,
        args: M1_THETA_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience oblique-shock path: (M1, theta_deg, branch) -> p2/p1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.oblique_shock_from_m1_theta_to_p2_p1(2.0, 10.0, 1.4, \"weak\")",
        xloil_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,\"weak\")",
        pyxll_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.oblique_shock_calc.from_m1_beta.to_m2",
        python_name: "oblique_shock_from_m1_beta_to_m2",
        excel_name: "ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2",
        op: "device.oblique_shock_calc.value",
        fixed_args: FIXED_M1_BETA_TO_M2,
        args: M1_BETA_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience oblique-shock path: (M1, beta_deg) -> M2",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.oblique_shock_from_m1_beta_to_m2(2.0, 40.0, 1.4)",
        xloil_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)",
        pyxll_example: "=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.oblique_shock_calc(2.0, \"theta_deg\", 10.0, \"beta_deg\", 1.4, \"weak\")\nengpy.devices.oblique_shock_from_m1_beta_to_theta(2.0, 40.0, 1.4)\nengpy.devices.oblique_shock_from_m1_theta_to_beta(2.0, 10.0, 1.4, \"strong\")\nengpy.devices.oblique_shock_from_m1_theta_to_p2_p1(2.0, 10.0, 1.4, \"weak\")\nengpy.devices.oblique_shock_path_text(2.0, \"theta_deg\", 10.0, \"m2\", 1.4, \"weak\")\n```\n\n### Excel\n```excel\n=ENG_OBLIQUE_SHOCK(2.0,\"theta_deg\",10.0,\"beta_deg\",1.4,\"weak\")\n=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_THETA(2.0,40.0,1.4)\n=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_BETA(2.0,10.0,1.4,\"strong\")\n=ENG_OBLIQUE_SHOCK_FROM_M1_THETA_TO_P2_P1(2.0,10.0,1.4,\"weak\")\n=ENG_OBLIQUE_SHOCK_FROM_M1_BETA_TO_M2(2.0,40.0,1.4)\n=ENG_OBLIQUE_SHOCK_PATH_TEXT(2.0,\"theta_deg\",10.0,\"m2\",1.4,\"weak\")\n```\n\n**Excel arguments**\n- `m1`: upstream Mach number\n- `value_kind_in`: `beta_deg` or `theta_deg`\n- `value_in`: angle input in degrees\n- `target_kind_out`: `theta_deg`, `beta_deg`, `mn1`, `mn2`, `m2`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`\n- `gamma`: specific heat ratio\n- `branch`: `weak`/`strong` (required for `theta_deg` input paths)\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style compressible device for oblique shocks. It supports the two practical input pairs:\n- `(M1, beta)`\n- `(M1, theta)`\n\nThe solver resolves the shock geometry and normal component (`Mn1`) then reuses normal-shock equations for downstream ratios and `Mn2`.\n\n### Supported outputs\n- `theta_deg`\n- `beta_deg`\n- `mn1`\n- `mn2`\n- `m2`\n- `p2_p1`\n- `rho2_rho1`\n- `t2_t1`\n- `p02_p01`\n\n### Branch behavior\n- `M1 + theta -> beta` is double-valued and requires explicit `weak` or `strong` branch selection.\n\n### Rust\n```rust\nuse eng::devices::{oblique_shock_calc, ObliqueShockInputKind, ObliqueShockOutputKind, ObliqueShockBranch};\nlet out = oblique_shock_calc()\n    .gamma(1.4)\n    .m1(2.0)\n    .input(ObliqueShockInputKind::ThetaRad, 10f64.to_radians())\n    .target(ObliqueShockOutputKind::PressureRatio)\n    .branch(ObliqueShockBranch::Weak)\n    .solve()?;\nprintln!(\"beta={} deg, p2/p1={}\", out.beta_rad.to_degrees(), out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: "oblique_shock_calc",
        name: "Oblique Shock Calculator",
        summary: "Calculator-style compressible device: solve oblique-shock input pairs (M1+beta / M1+theta) to target outputs with explicit weak/strong branch handling.",
        supported_modes: &[
            "Input pairs: (M1, beta), (M1, theta)",
            "Branch-aware inversion for (M1, theta) -> beta",
        ],
        outputs: &[
            "value_si",
            "beta_rad",
            "theta_rad",
            "mn1",
            "mn2",
            "m2",
            "path diagnostics",
        ],
        route: "devices/oblique_shock_calc.md",
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "compressible.oblique_shock_theta_beta_m",
            "compressible.oblique_shock_mn1",
            "compressible.oblique_shock_m2",
            "compressible.normal_shock_m2",
            "compressible.normal_shock_pressure_ratio",
            "compressible.normal_shock_density_ratio",
            "compressible.normal_shock_temperature_ratio",
            "compressible.normal_shock_stagnation_pressure_ratio",
        ],
        binding_functions: BINDING_FUNCTIONS,
    }
}

#[derive(Debug, Clone)]
pub struct ObliqueShockCalcRequest {
    pub gamma: f64,
    pub m1: f64,
    pub input_kind: ObliqueShockInputKind,
    pub input_value: f64,
    pub target_kind: ObliqueShockOutputKind,
    pub branch: Option<ObliqueShockBranch>,
}

#[derive(Debug, Clone)]
pub struct ObliqueShockCalcStep {
    pub equation_path_id: String,
    pub solved_for: String,
    pub method: String,
    pub branch: Option<String>,
    pub inputs_used: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct ObliqueShockCalcResponse {
    pub value_si: f64,
    pub beta_rad: f64,
    pub theta_rad: f64,
    pub mn1: f64,
    pub mn2: f64,
    pub m2: f64,
    pub path: Vec<ObliqueShockCalcStep>,
    pub warnings: Vec<String>,
}

impl ObliqueShockCalcResponse {
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
pub enum ObliqueShockCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid upstream Mach M1 '{value}' (must be finite and > 1.0)")]
    InvalidM1 { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: weak, strong)")]
    MissingBranch { kind: &'static str },
    #[error("oblique shock solution failed: {reason}")]
    NoAttachedSolution { reason: String },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, ObliqueShockCalcError>;

#[derive(Debug, Clone)]
pub struct ObliqueShockCalculatorDevice {
    req: ObliqueShockCalcRequest,
}

pub fn oblique_shock_calc() -> ObliqueShockCalculatorDevice {
    ObliqueShockCalculatorDevice::new()
}

impl Default for ObliqueShockCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl ObliqueShockCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: ObliqueShockCalcRequest {
                gamma: 1.4,
                m1: 2.0,
                input_kind: ObliqueShockInputKind::BetaRad,
                input_value: 40.0_f64.to_radians(),
                target_kind: ObliqueShockOutputKind::ThetaRad,
                branch: None,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn m1(mut self, m1: f64) -> Self {
        self.req.m1 = m1;
        self
    }

    pub fn input(mut self, kind: ObliqueShockInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: ObliqueShockOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: ObliqueShockBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn solve(self) -> Result<ObliqueShockCalcResponse> {
        calc(self.req)
    }
}

#[derive(Debug, Clone, Copy)]
struct ObliqueState {
    beta: f64,
    theta: f64,
    mn1: f64,
    mn2: f64,
    m2: f64,
    p2_p1: f64,
    rho2_rho1: f64,
    t2_t1: f64,
    p02_p01: f64,
}

pub fn calc(req: ObliqueShockCalcRequest) -> Result<ObliqueShockCalcResponse> {
    if !req.gamma.is_finite() || req.gamma <= 1.0 {
        return Err(ObliqueShockCalcError::InvalidGamma { value: req.gamma });
    }
    if !req.m1.is_finite() || req.m1 <= 1.0 {
        return Err(ObliqueShockCalcError::InvalidM1 { value: req.m1 });
    }
    if !req.input_value.is_finite() {
        return Err(ObliqueShockCalcError::InvalidInputDomain {
            kind: input_kind_label(req.input_kind),
            reason: "must be finite".to_string(),
        });
    }

    let mut path = Vec::<ObliqueShockCalcStep>::new();
    let state = resolve_oblique_state(&req, &mut path)?;

    let value_si = match req.target_kind {
        ObliqueShockOutputKind::ThetaRad => state.theta,
        ObliqueShockOutputKind::BetaRad => state.beta,
        ObliqueShockOutputKind::Mn1 => state.mn1,
        ObliqueShockOutputKind::Mn2 => state.mn2,
        ObliqueShockOutputKind::M2 => state.m2,
        ObliqueShockOutputKind::PressureRatio => state.p2_p1,
        ObliqueShockOutputKind::DensityRatio => state.rho2_rho1,
        ObliqueShockOutputKind::TemperatureRatio => state.t2_t1,
        ObliqueShockOutputKind::StagnationPressureRatio => state.p02_p01,
    };

    Ok(ObliqueShockCalcResponse {
        value_si,
        beta_rad: state.beta,
        theta_rad: state.theta,
        mn1: state.mn1,
        mn2: state.mn2,
        m2: state.m2,
        path,
        warnings: Vec::new(),
    })
}

fn resolve_oblique_state(
    req: &ObliqueShockCalcRequest,
    path: &mut Vec<ObliqueShockCalcStep>,
) -> Result<ObliqueState> {
    let gamma = req.gamma;
    let m1 = req.m1;

    let (beta, theta) = match req.input_kind {
        ObliqueShockInputKind::BetaRad => {
            validate_beta_domain(m1, req.input_value)?;
            let solved = eq.solve_result(
                compressible::oblique_shock_theta_beta_m::equation(),
                "theta",
                [("m1", m1), ("beta", req.input_value), ("gamma", gamma)],
            )?;
            path.push(ObliqueShockCalcStep {
                equation_path_id: "compressible.oblique_shock_theta_beta_m".to_string(),
                solved_for: "theta".to_string(),
                method: method_label(solved.method),
                branch: solved.branch,
                inputs_used: vec![
                    ("m1".to_string(), m1),
                    ("beta".to_string(), req.input_value),
                    ("gamma".to_string(), gamma),
                ],
            });
            (req.input_value, solved.value_si)
        }
        ObliqueShockInputKind::ThetaRad => {
            validate_theta_domain(req.input_value)?;
            let Some(branch) = req.branch else {
                return Err(ObliqueShockCalcError::MissingBranch { kind: "theta" });
            };
            let beta = solve_beta_from_theta(m1, gamma, req.input_value, branch)?;
            path.push(ObliqueShockCalcStep {
                equation_path_id: "compressible.oblique_shock_theta_beta_m".to_string(),
                solved_for: "beta".to_string(),
                method: "numerical".to_string(),
                branch: Some(branch.as_str().to_string()),
                inputs_used: vec![
                    ("m1".to_string(), m1),
                    ("theta".to_string(), req.input_value),
                    ("gamma".to_string(), gamma),
                ],
            });
            (beta, req.input_value)
        }
    };

    let mn1 = solve_scalar(
        compressible::oblique_shock_mn1::equation(),
        "mn1",
        [("m1", m1), ("beta", beta)],
        "compressible.oblique_shock_mn1",
        path,
    )?;

    let mn2 = solve_scalar(
        compressible::normal_shock_m2::equation(),
        "M2",
        [("M1", mn1), ("gamma", gamma)],
        "compressible.normal_shock_m2",
        path,
    )?;

    let p2_p1 = solve_scalar(
        compressible::normal_shock_pressure_ratio::equation(),
        "p2_p1",
        [("M1", mn1), ("gamma", gamma)],
        "compressible.normal_shock_pressure_ratio",
        path,
    )?;

    let rho2_rho1 = solve_scalar(
        compressible::normal_shock_density_ratio::equation(),
        "rho2_rho1",
        [("M1", mn1), ("gamma", gamma)],
        "compressible.normal_shock_density_ratio",
        path,
    )?;

    let t2_t1 = solve_scalar(
        compressible::normal_shock_temperature_ratio::equation(),
        "T2_T1",
        [("M1", mn1), ("gamma", gamma)],
        "compressible.normal_shock_temperature_ratio",
        path,
    )?;

    let p02_p01 = solve_scalar(
        compressible::normal_shock_stagnation_pressure_ratio::equation(),
        "p02_p01",
        [("M1", mn1), ("gamma", gamma)],
        "compressible.normal_shock_stagnation_pressure_ratio",
        path,
    )?;

    let m2 = solve_scalar(
        compressible::oblique_shock_m2::equation(),
        "m2",
        [("mn2", mn2), ("beta", beta), ("theta", theta)],
        "compressible.oblique_shock_m2",
        path,
    )?;

    Ok(ObliqueState {
        beta,
        theta,
        mn1,
        mn2,
        m2,
        p2_p1,
        rho2_rho1,
        t2_t1,
        p02_p01,
    })
}

fn solve_beta_from_theta(
    m1: f64,
    gamma: f64,
    theta: f64,
    branch: ObliqueShockBranch,
) -> Result<f64> {
    fn theta_at_beta(m1: f64, gamma: f64, beta: f64) -> Result<f64> {
        Ok(eq
            .solve_result(
                compressible::oblique_shock_theta_beta_m::equation(),
                "theta",
                [("m1", m1), ("beta", beta), ("gamma", gamma)],
            )?
            .value_si)
    }

    let beta_min = (1.0 / m1).asin() + 1e-6;
    let beta_max = std::f64::consts::FRAC_PI_2 - 1e-6;
    let (roots, _) = find_roots_by_scan_bisection(beta_min, beta_max, 600, 1e-12, 1e-7, |beta| {
        Ok::<f64, ObliqueShockCalcError>(theta_at_beta(m1, gamma, beta)? - theta)
    })
    .map_err(|err| ObliqueShockCalcError::NoAttachedSolution {
        reason: format!("theta-beta solve failed: {err:?}"),
    })?;
    if roots.is_empty() {
        return Err(ObliqueShockCalcError::NoAttachedSolution {
            reason: "no attached oblique-shock beta solution for given M1/theta".to_string(),
        });
    }
    match branch {
        ObliqueShockBranch::Weak => Ok(roots[0]),
        ObliqueShockBranch::Strong => Ok(*roots.last().expect("non-empty roots")),
    }
}

fn solve_scalar<E, const N: usize>(
    equation: E,
    target: &str,
    inputs: [(&str, f64); N],
    equation_path_id: &str,
    path: &mut Vec<ObliqueShockCalcStep>,
) -> Result<f64>
where
    E: equations::IntoEquationId + Copy,
{
    let solved = eq.solve_result(equation, target, inputs)?;
    path.push(ObliqueShockCalcStep {
        equation_path_id: equation_path_id.to_string(),
        solved_for: target.to_string(),
        method: method_label(solved.method),
        branch: solved.branch,
        inputs_used: inputs
            .iter()
            .map(|(k, v)| ((*k).to_string(), *v))
            .collect::<Vec<_>>(),
    });
    Ok(solved.value_si)
}

fn method_label(method: SolveMethod) -> String {
    match method {
        SolveMethod::Auto => "auto",
        SolveMethod::Explicit => "explicit",
        SolveMethod::Numerical => "numerical",
    }
    .to_string()
}

fn validate_theta_domain(theta: f64) -> Result<()> {
    if theta <= 0.0 || theta >= std::f64::consts::FRAC_PI_2 {
        return Err(ObliqueShockCalcError::InvalidInputDomain {
            kind: "theta",
            reason: "theta must be in (0, pi/2) radians".to_string(),
        });
    }
    Ok(())
}

fn validate_beta_domain(m1: f64, beta: f64) -> Result<()> {
    let mu = (1.0 / m1).asin();
    if beta <= mu || beta >= std::f64::consts::FRAC_PI_2 {
        return Err(ObliqueShockCalcError::InvalidInputDomain {
            kind: "beta",
            reason: format!(
                "beta must be in (mu, pi/2) where mu=asin(1/M1)={mu:.12} rad for M1={m1}"
            ),
        });
    }
    Ok(())
}

fn input_kind_label(kind: ObliqueShockInputKind) -> &'static str {
    match kind {
        ObliqueShockInputKind::BetaRad => "beta",
        ObliqueShockInputKind::ThetaRad => "theta",
    }
}

pub fn parse_input_kind(raw: &str, value: f64) -> Option<ObliqueShockInputKind> {
    let key = raw.trim().to_ascii_lowercase();
    match key.as_str() {
        "beta" | "beta_rad" | "shock_angle" => Some(ObliqueShockInputKind::BetaRad),
        "beta_deg" => Some(ObliqueShockInputKind::BetaRad),
        "theta" | "theta_rad" | "deflection_angle" => Some(ObliqueShockInputKind::ThetaRad),
        "theta_deg" => Some(ObliqueShockInputKind::ThetaRad),
        _ => {
            let _ = value;
            None
        }
    }
}

pub fn input_value_to_si(kind_raw: &str, value: f64) -> f64 {
    match kind_raw.trim().to_ascii_lowercase().as_str() {
        "beta_deg" | "theta_deg" => value.to_radians(),
        _ => value,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<(ObliqueShockOutputKind, bool)> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "theta" | "theta_rad" => Some((ObliqueShockOutputKind::ThetaRad, false)),
        "theta_deg" => Some((ObliqueShockOutputKind::ThetaRad, true)),
        "beta" | "beta_rad" => Some((ObliqueShockOutputKind::BetaRad, false)),
        "beta_deg" => Some((ObliqueShockOutputKind::BetaRad, true)),
        "mn1" => Some((ObliqueShockOutputKind::Mn1, false)),
        "mn2" => Some((ObliqueShockOutputKind::Mn2, false)),
        "m2" => Some((ObliqueShockOutputKind::M2, false)),
        "pressure_ratio" | "p2_p1" | "p2/p1" => {
            Some((ObliqueShockOutputKind::PressureRatio, false))
        }
        "density_ratio" | "rho2_rho1" | "rho2/rho1" => {
            Some((ObliqueShockOutputKind::DensityRatio, false))
        }
        "temperature_ratio" | "t2_t1" | "t2/t1" => {
            Some((ObliqueShockOutputKind::TemperatureRatio, false))
        }
        "stagnation_pressure_ratio" | "p02_p01" | "p02/p01" => {
            Some((ObliqueShockOutputKind::StagnationPressureRatio, false))
        }
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<ObliqueShockBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "weak" => Some(ObliqueShockBranch::Weak),
        "strong" => Some(ObliqueShockBranch::Strong),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m1_beta_to_theta_works() {
        let out = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::BetaRad,
            input_value: 40.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::ThetaRad,
            branch: None,
        })
        .expect("oblique shock calc");
        assert!((out.value_si.to_degrees() - 10.622_909_624_949_55).abs() < 1e-8);
        assert!(out.path.iter().any(|s| {
            s.equation_path_id == "compressible.oblique_shock_theta_beta_m"
                && s.solved_for == "theta"
        }));
    }

    #[test]
    fn m1_theta_weak_to_beta_works() {
        let out = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::ThetaRad,
            input_value: 10.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::BetaRad,
            branch: Some(ObliqueShockBranch::Weak),
        })
        .expect("oblique shock calc");
        let beta_deg = out.value_si.to_degrees();
        assert!(
            (beta_deg - 39.313_931_844_818_825).abs() < 1e-7,
            "expected weak beta ~39.3139 deg, got {beta_deg}"
        );
    }

    #[test]
    fn m1_theta_strong_to_beta_works() {
        let out = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::ThetaRad,
            input_value: 10.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::BetaRad,
            branch: Some(ObliqueShockBranch::Strong),
        })
        .expect("oblique shock calc");
        let beta_deg = out.value_si.to_degrees();
        assert!(
            (beta_deg - 83.700_080_375_747_26).abs() < 1e-7,
            "expected strong beta ~83.7001 deg, got {beta_deg}"
        );
    }

    #[test]
    fn m1_theta_weak_to_pressure_ratio_works() {
        let out = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::ThetaRad,
            input_value: 10.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::PressureRatio,
            branch: Some(ObliqueShockBranch::Weak),
        })
        .expect("oblique shock calc");
        assert!((out.value_si - 1.706_578_604_000_033).abs() < 1e-8);
    }

    #[test]
    fn m1_beta_to_m2_works() {
        let out = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::BetaRad,
            input_value: 40.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::M2,
            branch: None,
        })
        .expect("oblique shock calc");
        assert!((out.value_si - 1.617_318_834_026_265_4).abs() < 1e-10);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.oblique_shock_m2")
        );
    }

    #[test]
    fn theta_requires_branch() {
        let err = calc(ObliqueShockCalcRequest {
            gamma: 1.4,
            m1: 2.0,
            input_kind: ObliqueShockInputKind::ThetaRad,
            input_value: 10.0_f64.to_radians(),
            target_kind: ObliqueShockOutputKind::BetaRad,
            branch: None,
        })
        .expect_err("missing branch should fail");
        assert!(matches!(err, ObliqueShockCalcError::MissingBranch { .. }));
    }
}
