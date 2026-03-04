use equations::{compressible, eq};
use thiserror::Error;

use super::framework::{
    CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcSpec, method_label, path_text,
    run_pivot_calculation,
};
use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

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

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (mach, mach_angle_deg, prandtl_meyer_angle_deg, pressure_ratio, temperature_ratio, density_ratio, area_ratio)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Optional branch for double-valued inversions (subsonic/supersonic)",
    },
];

const M_GAMMA_BRANCH_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Optional branch",
    },
];

const FIXED_M_TO_P: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "pressure_ratio")];
const FIXED_MU_TO_P: &[(&str, &str)] = &[
    ("input_kind", "mach_angle_deg"),
    ("target_kind", "pressure_ratio"),
];
const FIXED_A_TO_M: &[(&str, &str)] = &[("input_kind", "area_ratio"), ("target_kind", "mach")];
const FIXED_NU_TO_M: &[(&str, &str)] = &[
    ("input_kind", "prandtl_meyer_angle_deg"),
    ("target_kind", "mach"),
];
const FIXED_M_TO_NU: &[(&str, &str)] = &[
    ("input_kind", "mach"),
    ("target_kind", "prandtl_meyer_angle_deg"),
];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc",
        python_name: "isentropic_calc",
        excel_name: "ENG_ISENTROPIC",
        op: "device.isentropic_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Isentropic calculator: input kind -> target kind through Mach pivot",
        rust_example: "eng::devices::isentropic_calc().solve()?",
        python_example: "engpy.devices.isentropic_calc(\"mach\", 2.0, \"pressure_ratio\", 1.4)",
        xloil_example: "=ENG_ISENTROPIC(\"mach\",2.0,\"pressure_ratio\",1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC(\"mach\",2.0,\"pressure_ratio\",1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.pivot",
        python_name: "isentropic_pivot_mach",
        excel_name: "ENG_ISENTROPIC_PIVOT_MACH",
        op: "device.isentropic_calc.pivot_mach",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Isentropic calculator helper: return resolved pivot Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_pivot_mach(\"area_ratio\", 2.0, \"mach\", 1.4, \"subsonic\")",
        xloil_example: "=ENG_ISENTROPIC_PIVOT_MACH(\"area_ratio\",2.0,\"mach\",1.4,\"subsonic\")",
        pyxll_example: "=ENG_ISENTROPIC_PIVOT_MACH(\"area_ratio\",2.0,\"mach\",1.4,\"subsonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.path_text",
        python_name: "isentropic_path_text",
        excel_name: "ENG_ISENTROPIC_PATH_TEXT",
        op: "device.isentropic_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Isentropic calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_path_text(\"mach_angle_deg\", 30.0, \"pressure_ratio\", 1.4)",
        xloil_example: "=ENG_ISENTROPIC_PATH_TEXT(\"mach_angle_deg\",30.0,\"pressure_ratio\",1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC_PATH_TEXT(\"mach_angle_deg\",30.0,\"pressure_ratio\",1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.from_m.to_p_p0",
        python_name: "isentropic_from_m_to_p_p0",
        excel_name: "ENG_ISENTROPIC_FROM_M_TO_P_P0",
        op: "device.isentropic_calc.value",
        fixed_args: FIXED_M_TO_P,
        args: M_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience isentropic path: Mach -> p/p0",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_from_m_to_p_p0(2.0, 1.4)",
        xloil_example: "=ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.from_mu_deg.to_p_p0",
        python_name: "isentropic_from_mu_deg_to_p_p0",
        excel_name: "ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0",
        op: "device.isentropic_calc.value",
        fixed_args: FIXED_MU_TO_P,
        args: M_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience isentropic path: mu(deg) -> p/p0",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_from_mu_deg_to_p_p0(30.0, 1.4)",
        xloil_example: "=ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30.0,1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30.0,1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.from_area_ratio.to_m",
        python_name: "isentropic_from_area_ratio_to_m",
        excel_name: "ENG_ISENTROPIC_FROM_A_ASTAR_TO_M",
        op: "device.isentropic_calc.value",
        fixed_args: FIXED_A_TO_M,
        args: M_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience isentropic path: A/A* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_from_area_ratio_to_m(2.0, 1.4, \"supersonic\")",
        xloil_example: "=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,\"supersonic\")",
        pyxll_example: "=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.from_nu_deg.to_m",
        python_name: "isentropic_from_nu_deg_to_m",
        excel_name: "ENG_ISENTROPIC_FROM_NU_DEG_TO_M",
        op: "device.isentropic_calc.value",
        fixed_args: FIXED_NU_TO_M,
        args: M_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience isentropic path: nu(deg) -> Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_from_nu_deg_to_m(26.3797608134, 1.4)",
        xloil_example: "=ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.isentropic_calc.from_m.to_nu_deg",
        python_name: "isentropic_from_m_to_nu_deg",
        excel_name: "ENG_ISENTROPIC_FROM_M_TO_NU_DEG",
        op: "device.isentropic_calc.value",
        fixed_args: FIXED_M_TO_NU,
        args: M_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience isentropic path: Mach -> nu(deg)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.isentropic_from_m_to_nu_deg(2.0, 1.4)",
        xloil_example: "=ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,\"\")",
        pyxll_example: "=ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,\"\")",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.isentropic_calc(\"mach_angle_deg\", 30.0, \"pressure_ratio\", 1.4)\nengpy.devices.isentropic_from_nu_deg_to_m(26.3797608134, 1.4)\nengpy.devices.isentropic_pivot_mach(\"area_ratio\", 2.0, \"mach\", 1.4, \"supersonic\")\nengpy.devices.isentropic_path_text(\"area_ratio\", 2.0, \"mach\", 1.4, \"subsonic\")\n```\n\n### Excel\n```excel\n=ENG_ISENTROPIC(\"mach_angle_deg\",30,\"pressure_ratio\",1.4,\"\")\n=ENG_ISENTROPIC_FROM_M_TO_P_P0(2.0,1.4,\"\")\n=ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0(30,1.4,\"\")\n=ENG_ISENTROPIC_FROM_NU_DEG_TO_M(26.3797608134,1.4,\"\")\n=ENG_ISENTROPIC_FROM_M_TO_NU_DEG(2.0,1.4,\"\")\n=ENG_ISENTROPIC_FROM_A_ASTAR_TO_M(2.0,1.4,\"supersonic\")\n=ENG_ISENTROPIC_PIVOT_MACH(\"area_ratio\",2.0,\"mach\",1.4,\"subsonic\")\n=ENG_ISENTROPIC_PATH_TEXT(\"mach\",2.0,\"pressure_ratio\",1.4,\"\")\n```\n\n**Excel arguments**\n- `value_kind_in`: `mach`, `mach_angle_deg`, `prandtl_meyer_angle_deg`, `pressure_ratio`, `temperature_ratio`, `density_ratio`, `area_ratio`\n- `value_in`: input value\n- `target_kind_out`: same enum family as input kind\n- `gamma`: specific heat ratio\n- `branch`: optional, required for double-valued inverse paths like `area_ratio -> mach`\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style compressible device that resolves a pivot Mach number from one supported isentropic input kind and then evaluates the requested target kind.\n\n### Supported input kinds\n- `mach`\n- `mach_angle_deg` (binding convenience; internally radians)\n- `prandtl_meyer_angle_deg` (binding convenience; internally radians)\n- `pressure_ratio` (`p/p0`)\n- `temperature_ratio` (`T/T0`)\n- `density_ratio` (`rho/rho0`)\n- `area_ratio` (`A/A*`, branch-sensitive)\n\n### Supported target kinds\n- `mach`\n- `mach_angle_deg`\n- `prandtl_meyer_angle_deg`\n- `pressure_ratio`\n- `temperature_ratio`\n- `density_ratio`\n- `area_ratio`\n\n### Branch behavior\n- `area_ratio -> mach` is double-valued and requires `subsonic` or `supersonic`.\n\n### Rust\n```rust\nuse eng::devices::{isentropic_calc, IsentropicInputKind, IsentropicOutputKind, IsentropicBranch};\nlet out = isentropic_calc()\n    .gamma(1.4)\n    .input(IsentropicInputKind::AreaRatio, 2.0)\n    .target(IsentropicOutputKind::Mach)\n    .branch(IsentropicBranch::Supersonic)\n    .solve()?;\nprintln!(\"M={}, value={}\", out.pivot_mach, out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: "isentropic_calc",
        name: DEVICE_SPEC.name,
        summary: DEVICE_SPEC.summary,
        supported_modes: &[
            "Input kinds: Mach, MachAngle, Prandtl-Meyer angle, p/p0, T/T0, rho/rho0, A/A*",
            "Branch-aware inversion for A/A*",
        ],
        outputs: &["value_si", "pivot_mach", "path diagnostics"],
        route: DEVICE_SPEC.route,
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "compressible.area_mach",
            "compressible.isentropic_pressure_ratio",
            "compressible.isentropic_temperature_ratio",
            "compressible.isentropic_density_ratio",
            "compressible.mach_angle",
            "compressible.prandtl_meyer",
        ],
        binding_functions: BINDING_FUNCTIONS,
    }
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
