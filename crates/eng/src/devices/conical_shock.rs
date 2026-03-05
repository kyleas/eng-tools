use equations::{SolveMethod, compressible, eq};
use thiserror::Error;

use crate::solve::{
    numeric::bisect_by_sign_change,
    ode::{OdeSolveError, rk4_step_2},
};

use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicalShockInputKind {
    ConeAngleRad,
    WaveAngleRad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicalShockOutputKind {
    WaveAngleRad,
    ConeAngleRad,
    ShockTurnAngleRad,
    ConeSurfaceMach,
    PressureRatio,
    DensityRatio,
    TemperatureRatio,
    StagnationPressureRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConicalShockBranch {
    Weak,
    Strong,
}

impl ConicalShockBranch {
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
        description: "Input kind (cone_angle_deg or wave_angle_deg)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input angle value in degrees",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (wave_angle_deg, cone_angle_deg, shock_turn_angle_deg, mc, p2_p1, rho2_rho1, t2_t1, p02_p01)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Weak/strong branch for cone-angle inversion paths",
    },
];

const M1_ANGLE_GAMMA_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "m1",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input angle in degrees",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
];

const M1_CONE_GAMMA_BRANCH_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "m1",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Cone angle in degrees",
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

const FIXED_M1_CONE_TO_WAVE: &[(&str, &str)] = &[
    ("input_kind", "cone_angle_deg"),
    ("target_kind", "wave_angle_deg"),
];
const FIXED_M1_CONE_TO_P2P1: &[(&str, &str)] =
    &[("input_kind", "cone_angle_deg"), ("target_kind", "p2_p1")];
const FIXED_M1_CONE_TO_MC: &[(&str, &str)] =
    &[("input_kind", "cone_angle_deg"), ("target_kind", "mc")];
const FIXED_M1_WAVE_TO_CONE: &[(&str, &str)] = &[
    ("input_kind", "wave_angle_deg"),
    ("target_kind", "cone_angle_deg"),
];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc",
        python_name: "conical_shock_calc",
        excel_name: "ENG_CONICAL_SHOCK",
        op: "device.conical_shock_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Conical shock calculator: (M1 + cone/wave angle) -> target with Taylor-Maccoll integration",
        rust_example: "eng::devices::conical_shock_calc().solve()?",
        python_example: "engpy.devices.conical_shock_calc(2.2, \"cone_angle_deg\", 12.0, \"wave_angle_deg\", 1.4, \"weak\")",
        xloil_example: "=ENG_CONICAL_SHOCK(2.2,\"cone_angle_deg\",12.0,\"wave_angle_deg\",1.4,\"weak\")",
        pyxll_example: "=ENG_CONICAL_SHOCK(2.2,\"cone_angle_deg\",12.0,\"wave_angle_deg\",1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc.path_text",
        python_name: "conical_shock_path_text",
        excel_name: "ENG_CONICAL_SHOCK_PATH_TEXT",
        op: "device.conical_shock_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Conical shock calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.conical_shock_path_text(2.2, \"cone_angle_deg\", 12.0, \"p2_p1\", 1.4, \"weak\")",
        xloil_example: "=ENG_CONICAL_SHOCK_PATH_TEXT(2.2,\"cone_angle_deg\",12.0,\"p2_p1\",1.4,\"weak\")",
        pyxll_example: "=ENG_CONICAL_SHOCK_PATH_TEXT(2.2,\"cone_angle_deg\",12.0,\"p2_p1\",1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc.from_m1_cone.to_wave",
        python_name: "conical_shock_from_m1_cone_deg_to_wave_deg",
        excel_name: "ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_WAVE_DEG",
        op: "device.conical_shock_calc.value",
        fixed_args: FIXED_M1_CONE_TO_WAVE,
        args: M1_CONE_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience conical-shock path: (M1, cone_angle_deg, branch) -> wave_angle_deg",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.conical_shock_from_m1_cone_deg_to_wave_deg(2.2, 12.0, 1.4, \"weak\")",
        xloil_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_WAVE_DEG(2.2,12.0,1.4,\"weak\")",
        pyxll_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_WAVE_DEG(2.2,12.0,1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc.from_m1_cone.to_p2_p1",
        python_name: "conical_shock_from_m1_cone_deg_to_p2_p1",
        excel_name: "ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_P2_P1",
        op: "device.conical_shock_calc.value",
        fixed_args: FIXED_M1_CONE_TO_P2P1,
        args: M1_CONE_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience conical-shock path: (M1, cone_angle_deg, branch) -> p2/p1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.conical_shock_from_m1_cone_deg_to_p2_p1(2.2, 12.0, 1.4, \"weak\")",
        xloil_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_P2_P1(2.2,12.0,1.4,\"weak\")",
        pyxll_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_P2_P1(2.2,12.0,1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc.from_m1_cone.to_mc",
        python_name: "conical_shock_from_m1_cone_deg_to_mc",
        excel_name: "ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_MC",
        op: "device.conical_shock_calc.value",
        fixed_args: FIXED_M1_CONE_TO_MC,
        args: M1_CONE_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience conical-shock path: (M1, cone_angle_deg, branch) -> cone-surface Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.conical_shock_from_m1_cone_deg_to_mc(2.2, 12.0, 1.4, \"weak\")",
        xloil_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_MC(2.2,12.0,1.4,\"weak\")",
        pyxll_example: "=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_MC(2.2,12.0,1.4,\"weak\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.conical_shock_calc.from_m1_wave.to_cone",
        python_name: "conical_shock_from_m1_wave_deg_to_cone_deg",
        excel_name: "ENG_CONICAL_SHOCK_FROM_M1_WAVE_DEG_TO_CONE_DEG",
        op: "device.conical_shock_calc.value",
        fixed_args: FIXED_M1_WAVE_TO_CONE,
        args: M1_ANGLE_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience conical-shock path: (M1, wave_angle_deg) -> cone_angle_deg",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.conical_shock_from_m1_wave_deg_to_cone_deg(2.2, 36.0, 1.4)",
        xloil_example: "=ENG_CONICAL_SHOCK_FROM_M1_WAVE_DEG_TO_CONE_DEG(2.2,36.0,1.4)",
        pyxll_example: "=ENG_CONICAL_SHOCK_FROM_M1_WAVE_DEG_TO_CONE_DEG(2.2,36.0,1.4)",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.conical_shock_calc(2.2, \"cone_angle_deg\", 12.0, \"wave_angle_deg\", 1.4, \"weak\")\nengpy.devices.conical_shock_from_m1_cone_deg_to_wave_deg(2.2, 12.0, 1.4, \"weak\")\nengpy.devices.conical_shock_from_m1_cone_deg_to_p2_p1(2.2, 12.0, 1.4, \"weak\")\nengpy.devices.conical_shock_from_m1_cone_deg_to_mc(2.2, 12.0, 1.4, \"weak\")\nengpy.devices.conical_shock_from_m1_wave_deg_to_cone_deg(2.2, 36.0, 1.4)\nengpy.devices.conical_shock_path_text(2.2, \"cone_angle_deg\", 12.0, \"mc\", 1.4, \"weak\")\n```\n\n### Excel\n```excel\n=ENG_CONICAL_SHOCK(2.2,\"cone_angle_deg\",12.0,\"wave_angle_deg\",1.4,\"weak\")\n=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_WAVE_DEG(2.2,12.0,1.4,\"weak\")\n=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_P2_P1(2.2,12.0,1.4,\"weak\")\n=ENG_CONICAL_SHOCK_FROM_M1_CONE_DEG_TO_MC(2.2,12.0,1.4,\"weak\")\n=ENG_CONICAL_SHOCK_FROM_M1_WAVE_DEG_TO_CONE_DEG(2.2,36.0,1.4)\n=ENG_CONICAL_SHOCK_PATH_TEXT(2.2,\"cone_angle_deg\",12.0,\"mc\",1.4,\"weak\")\n```\n\n**Excel arguments**\n- `m1`: upstream Mach number\n- `value_kind_in`: `cone_angle_deg` or `wave_angle_deg`\n- `value_in`: input angle in degrees\n- `target_kind_out`: `wave_angle_deg`, `cone_angle_deg`, `shock_turn_angle_deg`, `mc`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`\n- `gamma`: specific heat ratio\n- `branch`: `weak`/`strong` for cone-angle inversion paths\n";

const OVERVIEW_MD: &str = "## Overview\n\nA production conical-shock calculator that combines compressible jump relations with Taylor-Maccoll integration for axisymmetric cone flow.\n\n### Supported input forms\n- `(M1, cone_angle_deg)`\n- `(M1, wave_angle_deg)`\n\n### Supported outputs\n- `wave_angle_deg`\n- `cone_angle_deg`\n- `shock_turn_angle_deg`\n- `mc` (cone-surface Mach)\n- `p2_p1`\n- `rho2_rho1`\n- `t2_t1`\n- `p02_p01`\n\n### Branch behavior\n- `M1 + cone_angle -> wave_angle` paths are branch-sensitive. Use explicit `weak` or `strong` branch selection.\n\n### Domain behavior\n- Returns structured errors for invalid Mach/angles, detached/no-solution regimes, and numerical convergence failures.\n\n### Rust\n```rust\nuse eng::devices::{conical_shock_calc, ConicalShockInputKind, ConicalShockOutputKind, ConicalShockBranch};\nlet out = conical_shock_calc()\n    .gamma(1.4)\n    .m1(2.2)\n    .input(ConicalShockInputKind::ConeAngleRad, 12f64.to_radians())\n    .target(ConicalShockOutputKind::WaveAngleRad)\n    .branch(ConicalShockBranch::Weak)\n    .solve()?;\nprintln!(\"wave={} deg, Mc={}\", out.wave_angle_rad.to_degrees(), out.cone_surface_mach);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: "conical_shock_calc",
        name: "Conical Shock Calculator",
        summary: "Calculator-style compressible device for Taylor-Maccoll conical-shock workflows with explicit weak/strong branch handling.",
        supported_modes: &[
            "Input forms: (M1, cone_angle), (M1, wave_angle)",
            "Taylor-Maccoll integrated conical-flow state resolution",
        ],
        outputs: &[
            "value_si",
            "wave_angle_rad",
            "cone_angle_rad",
            "shock_turn_angle_rad",
            "cone_surface_mach",
            "path diagnostics",
        ],
        route: "devices/conical_shock_calc.md",
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "compressible.oblique_shock_theta_beta_m",
            "compressible.oblique_shock_mn1",
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
pub struct ConicalShockCalcRequest {
    pub gamma: f64,
    pub m1: f64,
    pub input_kind: ConicalShockInputKind,
    pub input_value: f64,
    pub target_kind: ConicalShockOutputKind,
    pub branch: Option<ConicalShockBranch>,
}

#[derive(Debug, Clone)]
pub struct ConicalShockCalcStep {
    pub equation_path_id: String,
    pub solved_for: String,
    pub method: String,
    pub branch: Option<String>,
    pub inputs_used: Vec<(String, f64)>,
}

#[derive(Debug, Clone)]
pub struct ConicalShockCalcResponse {
    pub value_si: f64,
    pub wave_angle_rad: f64,
    pub cone_angle_rad: f64,
    pub shock_turn_angle_rad: f64,
    pub cone_surface_mach: f64,
    pub p2_p1: f64,
    pub rho2_rho1: f64,
    pub t2_t1: f64,
    pub p02_p01: f64,
    pub path: Vec<ConicalShockCalcStep>,
    pub warnings: Vec<String>,
}

impl ConicalShockCalcResponse {
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
pub enum ConicalShockCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid upstream Mach M1 '{value}' (must be finite and > 1.0)")]
    InvalidM1 { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: weak, strong)")]
    MissingBranch { kind: &'static str },
    #[error("detached or unattached conical-shock solution: {reason}")]
    NoAttachedSolution { reason: String },
    #[error("Taylor-Maccoll integration failed: {reason}")]
    NumericalFailure { reason: String },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, ConicalShockCalcError>;

#[derive(Debug, Clone)]
pub struct ConicalShockCalculatorDevice {
    req: ConicalShockCalcRequest,
}

pub fn conical_shock_calc() -> ConicalShockCalculatorDevice {
    ConicalShockCalculatorDevice::new()
}

impl Default for ConicalShockCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl ConicalShockCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: ConicalShockCalcRequest {
                gamma: 1.4,
                m1: 2.2,
                input_kind: ConicalShockInputKind::ConeAngleRad,
                input_value: 12.0_f64.to_radians(),
                target_kind: ConicalShockOutputKind::WaveAngleRad,
                branch: Some(ConicalShockBranch::Weak),
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

    pub fn input(mut self, kind: ConicalShockInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: ConicalShockOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: ConicalShockBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn solve(self) -> Result<ConicalShockCalcResponse> {
        calc(self.req)
    }
}

#[derive(Debug, Clone, Copy)]
struct ConicalState {
    wave_angle_rad: f64,
    cone_angle_rad: f64,
    shock_turn_angle_rad: f64,
    cone_surface_mach: f64,
    p2_p1: f64,
    rho2_rho1: f64,
    t2_t1: f64,
    p02_p01: f64,
}

#[derive(Debug, Clone, Copy)]
struct ShockJumpState {
    shock_turn_angle_rad: f64,
    m2: f64,
    p2_p1: f64,
    rho2_rho1: f64,
    t2_t1: f64,
    p02_p01: f64,
}

#[derive(Debug, Clone, Copy)]
struct TaylorMaccollResult {
    cone_angle_rad: f64,
    cone_surface_mach: f64,
    vtheta_cone: f64,
}

pub fn calc(req: ConicalShockCalcRequest) -> Result<ConicalShockCalcResponse> {
    validate_request(&req)?;
    let mut path = Vec::<ConicalShockCalcStep>::new();
    let state = resolve_state(&req, &mut path)?;

    let value_si = match req.target_kind {
        ConicalShockOutputKind::WaveAngleRad => state.wave_angle_rad,
        ConicalShockOutputKind::ConeAngleRad => state.cone_angle_rad,
        ConicalShockOutputKind::ShockTurnAngleRad => state.shock_turn_angle_rad,
        ConicalShockOutputKind::ConeSurfaceMach => state.cone_surface_mach,
        ConicalShockOutputKind::PressureRatio => state.p2_p1,
        ConicalShockOutputKind::DensityRatio => state.rho2_rho1,
        ConicalShockOutputKind::TemperatureRatio => state.t2_t1,
        ConicalShockOutputKind::StagnationPressureRatio => state.p02_p01,
    };

    Ok(ConicalShockCalcResponse {
        value_si,
        wave_angle_rad: state.wave_angle_rad,
        cone_angle_rad: state.cone_angle_rad,
        shock_turn_angle_rad: state.shock_turn_angle_rad,
        cone_surface_mach: state.cone_surface_mach,
        p2_p1: state.p2_p1,
        rho2_rho1: state.rho2_rho1,
        t2_t1: state.t2_t1,
        p02_p01: state.p02_p01,
        path,
        warnings: Vec::new(),
    })
}

fn validate_request(req: &ConicalShockCalcRequest) -> Result<()> {
    if !req.gamma.is_finite() || req.gamma <= 1.0 {
        return Err(ConicalShockCalcError::InvalidGamma { value: req.gamma });
    }
    if !req.m1.is_finite() || req.m1 <= 1.0 {
        return Err(ConicalShockCalcError::InvalidM1 { value: req.m1 });
    }
    if !req.input_value.is_finite() {
        return Err(ConicalShockCalcError::InvalidInputDomain {
            kind: input_kind_label(req.input_kind),
            reason: "must be finite".to_string(),
        });
    }
    match req.input_kind {
        ConicalShockInputKind::ConeAngleRad => {
            if req.input_value <= 0.0 || req.input_value >= std::f64::consts::FRAC_PI_2 {
                return Err(ConicalShockCalcError::InvalidInputDomain {
                    kind: "cone_angle",
                    reason: "must be in (0, pi/2) radians".to_string(),
                });
            }
            if req.branch.is_none() {
                return Err(ConicalShockCalcError::MissingBranch { kind: "cone_angle" });
            }
        }
        ConicalShockInputKind::WaveAngleRad => validate_wave_domain(req.m1, req.input_value)?,
    }
    Ok(())
}

fn resolve_state(
    req: &ConicalShockCalcRequest,
    path: &mut Vec<ConicalShockCalcStep>,
) -> Result<ConicalState> {
    let gamma = req.gamma;
    let m1 = req.m1;

    let (wave, cone, tm_result) = match req.input_kind {
        ConicalShockInputKind::WaveAngleRad => {
            let wave = req.input_value;
            let jump = solve_shock_jump(m1, wave, gamma, path)?;
            let tm = integrate_taylor_maccoll_from_wave(
                m1,
                wave,
                jump.shock_turn_angle_rad,
                jump.m2,
                gamma,
            )?;
            path.push(ConicalShockCalcStep {
                equation_path_id: "compressible.conical_shock_taylor_maccoll".to_string(),
                solved_for: "cone_angle".to_string(),
                method: "numerical".to_string(),
                branch: None,
                inputs_used: vec![
                    ("m1".to_string(), m1),
                    ("wave_angle".to_string(), wave),
                    ("gamma".to_string(), gamma),
                ],
            });
            (wave, tm.cone_angle_rad, tm)
        }
        ConicalShockInputKind::ConeAngleRad => {
            let branch = req.branch.expect("branch required by validate_request");
            let cone = req.input_value;
            let wave = solve_wave_from_cone(m1, cone, gamma, branch)?;
            path.push(ConicalShockCalcStep {
                equation_path_id: "compressible.conical_shock_taylor_maccoll".to_string(),
                solved_for: "wave_angle".to_string(),
                method: "numerical".to_string(),
                branch: Some(branch.as_str().to_string()),
                inputs_used: vec![
                    ("m1".to_string(), m1),
                    ("cone_angle".to_string(), cone),
                    ("gamma".to_string(), gamma),
                ],
            });
            let jump = solve_shock_jump(m1, wave, gamma, path)?;
            let tm = integrate_taylor_maccoll_to_cone(
                m1,
                wave,
                cone,
                jump.shock_turn_angle_rad,
                jump.m2,
                gamma,
            )?;
            (wave, cone, tm)
        }
    };

    let jump = solve_shock_jump(m1, wave, gamma, path)?;

    Ok(ConicalState {
        wave_angle_rad: wave,
        cone_angle_rad: cone,
        shock_turn_angle_rad: jump.shock_turn_angle_rad,
        cone_surface_mach: tm_result.cone_surface_mach,
        p2_p1: jump.p2_p1,
        rho2_rho1: jump.rho2_rho1,
        t2_t1: jump.t2_t1,
        p02_p01: jump.p02_p01,
    })
}

fn solve_shock_jump(
    m1: f64,
    wave: f64,
    gamma: f64,
    path: &mut Vec<ConicalShockCalcStep>,
) -> Result<ShockJumpState> {
    let shock_turn = solve_scalar(
        compressible::oblique_shock_theta_beta_m::equation(),
        "theta",
        [("m1", m1), ("beta", wave), ("gamma", gamma)],
        "compressible.oblique_shock_theta_beta_m",
        path,
    )?;

    let mn1 = solve_scalar(
        compressible::oblique_shock_mn1::equation(),
        "mn1",
        [("m1", m1), ("beta", wave)],
        "compressible.oblique_shock_mn1",
        path,
    )?;

    let m2 = solve_scalar(
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

    Ok(ShockJumpState {
        shock_turn_angle_rad: shock_turn,
        m2,
        p2_p1,
        rho2_rho1,
        t2_t1,
        p02_p01,
    })
}

fn integrate_taylor_maccoll_from_wave(
    _m1: f64,
    wave: f64,
    shock_turn: f64,
    m2: f64,
    gamma: f64,
) -> Result<TaylorMaccollResult> {
    let vp2 = velocity_prime_from_mach(m2, gamma)?;
    let mut theta = wave;
    let mut vr = vp2 * (wave - shock_turn).cos();
    let mut vt = -vp2 * (wave - shock_turn).sin();
    let step_mag: f64 = 2.0e-4;
    let theta_min = 1.0e-5;
    let max_steps = 200_000usize;

    for _ in 0..max_steps {
        if theta <= theta_min {
            break;
        }
        let h = -step_mag.min(theta - theta_min);
        let prev_theta = theta;
        let prev_vr = vr;
        let prev_vt = vt;
        let (next_vr, next_vt) = rk4_step_2(theta, vr, vt, h, |x, y1, y2| {
            taylor_maccoll_rhs(x, y1, y2, gamma)
        })
        .map_err(map_ode_error)?;
        theta += h;
        vr = next_vr;
        vt = next_vt;

        if prev_vt <= 0.0 && vt >= 0.0 {
            let frac = if (vt - prev_vt).abs() < 1e-14 {
                0.5
            } else {
                (-prev_vt / (vt - prev_vt)).clamp(0.0, 1.0)
            };
            let cone = prev_theta + frac * (theta - prev_theta);
            let vr_zero = prev_vr + frac * (vr - prev_vr);
            let vp_cone = vr_zero.abs().clamp(1e-10, 1.0 - 1e-10);
            let mc = mach_from_velocity_prime(vp_cone, gamma)?;
            return Ok(TaylorMaccollResult {
                cone_angle_rad: cone,
                cone_surface_mach: mc,
                vtheta_cone: 0.0,
            });
        }
    }

    Err(ConicalShockCalcError::NoAttachedSolution {
        reason: "Taylor-Maccoll integration did not intersect cone-wall boundary (v_theta=0)"
            .to_string(),
    })
}

fn integrate_taylor_maccoll_to_cone(
    _m1: f64,
    wave: f64,
    cone: f64,
    shock_turn: f64,
    m2: f64,
    gamma: f64,
) -> Result<TaylorMaccollResult> {
    if cone >= wave {
        return Err(ConicalShockCalcError::NoAttachedSolution {
            reason: "cone angle must be below wave angle for attached conical shock".to_string(),
        });
    }
    let vp2 = velocity_prime_from_mach(m2, gamma)?;
    let mut theta = wave;
    let mut vr = vp2 * (wave - shock_turn).cos();
    let mut vt = -vp2 * (wave - shock_turn).sin();
    let step_mag: f64 = 2.0e-4;
    let max_steps = 200_000usize;

    for _ in 0..max_steps {
        if theta <= cone {
            break;
        }
        let h = -step_mag.min(theta - cone);
        let (next_vr, next_vt) = rk4_step_2(theta, vr, vt, h, |x, y1, y2| {
            taylor_maccoll_rhs(x, y1, y2, gamma)
        })
        .map_err(map_ode_error)?;
        theta += h;
        vr = next_vr;
        vt = next_vt;
    }

    let vp_cone = (vr * vr + vt * vt).sqrt().clamp(1e-10, 1.0 - 1e-10);
    let mc = mach_from_velocity_prime(vp_cone, gamma)?;
    Ok(TaylorMaccollResult {
        cone_angle_rad: cone,
        cone_surface_mach: mc,
        vtheta_cone: vt,
    })
}

fn solve_wave_from_cone(m1: f64, cone: f64, gamma: f64, branch: ConicalShockBranch) -> Result<f64> {
    let mu = (1.0 / m1).asin();
    let beta_min = (mu + 1e-5).max(cone + 1e-5);
    let beta_max = std::f64::consts::FRAC_PI_2 - 1e-5;
    if beta_min >= beta_max {
        return Err(ConicalShockCalcError::NoAttachedSolution {
            reason: "cone angle is too large for attached conical shock at this Mach number"
                .to_string(),
        });
    }

    let residual = |beta: f64| -> Result<f64> {
        let jump = simple_shock_jump(m1, beta, gamma)?;
        let tm = integrate_taylor_maccoll_to_cone(
            m1,
            beta,
            cone,
            jump.shock_turn_angle_rad,
            jump.m2,
            gamma,
        )?;
        Ok(tm.vtheta_cone)
    };

    let mut roots = Vec::<f64>::new();
    let steps = 220usize;
    let mut prev_beta = beta_min;
    let mut prev_r = residual(prev_beta)?;
    for i in 1..=steps {
        let frac = (i as f64) / (steps as f64);
        let beta = beta_min + (beta_max - beta_min) * frac;
        let r = residual(beta)?;
        if prev_r.abs() < 1e-8 {
            roots.push(prev_beta);
        } else if r.abs() < 1e-8 {
            roots.push(beta);
        } else if prev_r * r < 0.0 {
            let out = bisect_by_sign_change(prev_beta, beta, 1e-12, 120, |x| residual(x)).map_err(
                |err| ConicalShockCalcError::NoAttachedSolution {
                    reason: format!("beta bracket solve failed: {err:?}"),
                },
            )?;
            roots.push(out.root);
        }
        prev_beta = beta;
        prev_r = r;
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    roots.dedup_by(|a, b| (*a - *b).abs() < 1e-7);
    if roots.is_empty() {
        return Err(ConicalShockCalcError::NoAttachedSolution {
            reason: "no attached conical-shock wave angle solves the requested cone angle"
                .to_string(),
        });
    }
    match branch {
        ConicalShockBranch::Weak => Ok(roots[0]),
        ConicalShockBranch::Strong => Ok(*roots.last().expect("non-empty roots")),
    }
}

fn simple_shock_jump(m1: f64, wave: f64, gamma: f64) -> Result<ShockJumpState> {
    let shock_turn = eq
        .solve_result(
            compressible::oblique_shock_theta_beta_m::equation(),
            "theta",
            [("m1", m1), ("beta", wave), ("gamma", gamma)],
        )?
        .value_si;
    let mn1 = eq
        .solve_result(
            compressible::oblique_shock_mn1::equation(),
            "mn1",
            [("m1", m1), ("beta", wave)],
        )?
        .value_si;
    let m2 = eq
        .solve_result(
            compressible::normal_shock_m2::equation(),
            "M2",
            [("M1", mn1), ("gamma", gamma)],
        )?
        .value_si;
    let p2_p1 = eq
        .solve_result(
            compressible::normal_shock_pressure_ratio::equation(),
            "p2_p1",
            [("M1", mn1), ("gamma", gamma)],
        )?
        .value_si;
    let rho2_rho1 = eq
        .solve_result(
            compressible::normal_shock_density_ratio::equation(),
            "rho2_rho1",
            [("M1", mn1), ("gamma", gamma)],
        )?
        .value_si;
    let t2_t1 = eq
        .solve_result(
            compressible::normal_shock_temperature_ratio::equation(),
            "T2_T1",
            [("M1", mn1), ("gamma", gamma)],
        )?
        .value_si;
    let p02_p01 = eq
        .solve_result(
            compressible::normal_shock_stagnation_pressure_ratio::equation(),
            "p02_p01",
            [("M1", mn1), ("gamma", gamma)],
        )?
        .value_si;
    Ok(ShockJumpState {
        shock_turn_angle_rad: shock_turn,
        m2,
        p2_p1,
        rho2_rho1,
        t2_t1,
        p02_p01,
    })
}

fn taylor_maccoll_rhs(theta: f64, vr: f64, vt: f64, gamma: f64) -> Result<(f64, f64)> {
    if theta <= 0.0 || theta >= std::f64::consts::PI {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "theta out of valid integration range".to_string(),
        });
    }
    let tan_theta = theta.tan();
    if tan_theta.abs() < 1e-14 {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "tan(theta) underflow in Taylor-Maccoll RHS".to_string(),
        });
    }
    let gm1 = gamma - 1.0;
    let speed_term = 1.0 - vr * vr - vt * vt;
    let numer = vt * vt * vr - 0.5 * gm1 * speed_term * (2.0 * vr + vt / tan_theta);
    let denom = 0.5 * gm1 * speed_term - vt * vt;
    if denom.abs() < 1e-14 {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "Taylor-Maccoll denominator approached zero".to_string(),
        });
    }
    Ok((vt, numer / denom))
}

fn map_ode_error(err: OdeSolveError<ConicalShockCalcError>) -> ConicalShockCalcError {
    match err {
        OdeSolveError::Rhs(source) => source,
        OdeSolveError::NonFiniteState { .. } => ConicalShockCalcError::NumericalFailure {
            reason: "non-finite state in Taylor-Maccoll integration".to_string(),
        },
    }
}

fn velocity_prime_from_mach(m: f64, gamma: f64) -> Result<f64> {
    if !m.is_finite() || m <= 0.0 {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "invalid Mach in velocity-prime conversion".to_string(),
        });
    }
    Ok((1.0 + 2.0 / ((gamma - 1.0) * m * m)).sqrt().recip())
}

fn mach_from_velocity_prime(vp: f64, gamma: f64) -> Result<f64> {
    if !vp.is_finite() || vp <= 0.0 || vp >= 1.0 {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "velocity-prime must be in (0, 1)".to_string(),
        });
    }
    let m2 = 2.0 * vp * vp / ((gamma - 1.0) * (1.0 - vp * vp));
    if !m2.is_finite() || m2 <= 0.0 {
        return Err(ConicalShockCalcError::NumericalFailure {
            reason: "invalid Mach^2 recovered from velocity-prime".to_string(),
        });
    }
    Ok(m2.sqrt())
}

fn solve_scalar<E, const N: usize>(
    equation: E,
    target: &str,
    inputs: [(&str, f64); N],
    equation_path_id: &str,
    path: &mut Vec<ConicalShockCalcStep>,
) -> Result<f64>
where
    E: equations::IntoEquationId + Copy,
{
    let solved = eq.solve_result(equation, target, inputs)?;
    path.push(ConicalShockCalcStep {
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

fn validate_wave_domain(m1: f64, wave: f64) -> Result<()> {
    let mu = (1.0 / m1).asin();
    if wave <= mu || wave >= std::f64::consts::FRAC_PI_2 {
        return Err(ConicalShockCalcError::InvalidInputDomain {
            kind: "wave_angle",
            reason: format!(
                "wave angle must be in (mu, pi/2) where mu=asin(1/M1)={mu:.12} rad for M1={m1}"
            ),
        });
    }
    Ok(())
}

fn input_kind_label(kind: ConicalShockInputKind) -> &'static str {
    match kind {
        ConicalShockInputKind::ConeAngleRad => "cone_angle",
        ConicalShockInputKind::WaveAngleRad => "wave_angle",
    }
}

pub fn parse_input_kind(raw: &str, _value: f64) -> Option<ConicalShockInputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "cone_angle" | "cone_angle_rad" | "cone_deg" | "cone_angle_deg" => {
            Some(ConicalShockInputKind::ConeAngleRad)
        }
        "wave_angle" | "wave_angle_rad" | "beta" | "beta_deg" | "wave_deg" | "wave_angle_deg" => {
            Some(ConicalShockInputKind::WaveAngleRad)
        }
        _ => None,
    }
}

pub fn input_value_to_si(kind_raw: &str, value: f64) -> f64 {
    match kind_raw.trim().to_ascii_lowercase().as_str() {
        "cone_deg" | "cone_angle_deg" | "wave_deg" | "wave_angle_deg" | "beta_deg" => {
            value.to_radians()
        }
        _ => value,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<(ConicalShockOutputKind, bool)> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "wave_angle" | "wave_angle_rad" | "beta" | "beta_rad" => {
            Some((ConicalShockOutputKind::WaveAngleRad, false))
        }
        "wave_angle_deg" | "beta_deg" => Some((ConicalShockOutputKind::WaveAngleRad, true)),
        "cone_angle" | "cone_angle_rad" => Some((ConicalShockOutputKind::ConeAngleRad, false)),
        "cone_angle_deg" => Some((ConicalShockOutputKind::ConeAngleRad, true)),
        "shock_turn_angle" | "shock_turn_angle_rad" | "theta" | "theta_rad" => {
            Some((ConicalShockOutputKind::ShockTurnAngleRad, false))
        }
        "shock_turn_angle_deg" | "theta_deg" => {
            Some((ConicalShockOutputKind::ShockTurnAngleRad, true))
        }
        "mc" | "cone_surface_mach" => Some((ConicalShockOutputKind::ConeSurfaceMach, false)),
        "pressure_ratio" | "p2_p1" | "p2/p1" => {
            Some((ConicalShockOutputKind::PressureRatio, false))
        }
        "density_ratio" | "rho2_rho1" | "rho2/rho1" => {
            Some((ConicalShockOutputKind::DensityRatio, false))
        }
        "temperature_ratio" | "t2_t1" | "t2/t1" => {
            Some((ConicalShockOutputKind::TemperatureRatio, false))
        }
        "stagnation_pressure_ratio" | "p02_p01" | "p02/p01" => {
            Some((ConicalShockOutputKind::StagnationPressureRatio, false))
        }
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<ConicalShockBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "weak" => Some(ConicalShockBranch::Weak),
        "strong" => Some(ConicalShockBranch::Strong),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m1_wave_to_cone_roundtrip_weak() {
        let forward = calc(ConicalShockCalcRequest {
            gamma: 1.4,
            m1: 2.2,
            input_kind: ConicalShockInputKind::WaveAngleRad,
            input_value: 38.0_f64.to_radians(),
            target_kind: ConicalShockOutputKind::ConeAngleRad,
            branch: None,
        })
        .expect("wave->cone should solve");
        assert!(forward.value_si > 0.0);
        assert!(forward.value_si < forward.wave_angle_rad);

        let backward = calc(ConicalShockCalcRequest {
            gamma: 1.4,
            m1: 2.2,
            input_kind: ConicalShockInputKind::ConeAngleRad,
            input_value: forward.value_si,
            target_kind: ConicalShockOutputKind::WaveAngleRad,
            branch: Some(ConicalShockBranch::Weak),
        })
        .expect("cone->wave weak should solve");
        assert!((backward.value_si - 38.0_f64.to_radians()).abs() < 2.5e-2);
    }

    #[test]
    fn cone_input_requires_branch() {
        let err = calc(ConicalShockCalcRequest {
            gamma: 1.4,
            m1: 2.2,
            input_kind: ConicalShockInputKind::ConeAngleRad,
            input_value: 12.0_f64.to_radians(),
            target_kind: ConicalShockOutputKind::WaveAngleRad,
            branch: None,
        })
        .expect_err("branch should be required");
        assert!(matches!(err, ConicalShockCalcError::MissingBranch { .. }));
    }

    #[test]
    fn m1_cone_to_mc_and_ratios_work() {
        let out = calc(ConicalShockCalcRequest {
            gamma: 1.4,
            m1: 2.2,
            input_kind: ConicalShockInputKind::ConeAngleRad,
            input_value: 12.0_f64.to_radians(),
            target_kind: ConicalShockOutputKind::ConeSurfaceMach,
            branch: Some(ConicalShockBranch::Weak),
        })
        .expect("conical calc");
        assert!(out.value_si > 0.1);
        assert!(out.p2_p1 > 1.0);
        assert!(out.rho2_rho1 > 1.0);
        assert!(out.t2_t1 > 1.0);
        assert!(
            out.path
                .iter()
                .any(|s| { s.equation_path_id == "compressible.oblique_shock_theta_beta_m" })
        );
        assert!(
            out.path
                .iter()
                .any(|s| { s.equation_path_id == "compressible.conical_shock_taylor_maccoll" })
        );
    }

    #[test]
    fn invalid_wave_domain_is_rejected() {
        let err = calc(ConicalShockCalcRequest {
            gamma: 1.4,
            m1: 2.2,
            input_kind: ConicalShockInputKind::WaveAngleRad,
            input_value: 5.0_f64.to_radians(),
            target_kind: ConicalShockOutputKind::ConeAngleRad,
            branch: None,
        })
        .expect_err("invalid wave angle should fail");
        assert!(matches!(
            err,
            ConicalShockCalcError::InvalidInputDomain { .. }
        ));
    }
}
