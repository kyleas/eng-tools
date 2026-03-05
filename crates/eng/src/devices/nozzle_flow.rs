use equations::{compressible, eq};
use thiserror::Error;

use super::framework::{
    CalcStep, CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcSpec, method_label, path_text,
    run_pivot_calculation,
};
use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

const NOZZLE_INPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "area_ratio",
        label: "A/A*",
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
];

const NOZZLE_OUTPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "area_ratio",
        label: "A/A*",
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
        key: "p",
        label: "Static pressure p (requires p0)",
    },
    CalculatorKindSpec {
        key: "t",
        label: "Static temperature T (requires t0)",
    },
    CalculatorKindSpec {
        key: "rho",
        label: "Static density rho (requires rho0)",
    },
];

pub const DEVICE_SPEC: CalculatorDeviceSpec = CalculatorDeviceSpec {
    key: "nozzle_flow_calc",
    name: "Nozzle Flow Calculator",
    summary: "Calculator-style quasi-1D nozzle device: solve isentropic nozzle input kinds to target kinds through Mach pivot orchestration.",
    route: "devices/nozzle_flow_calc.md",
    pivot_label: "Mach",
    input_kinds: NOZZLE_INPUT_KIND_SPECS,
    output_kinds: NOZZLE_OUTPUT_KIND_SPECS,
    branches: &["subsonic", "supersonic"],
};

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (mach, area_ratio, pressure_ratio, temperature_ratio, density_ratio, p, t, rho)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "p0",
        description: "Optional stagnation pressure reference for static p output",
    },
    DeviceBindingArgSpec {
        name: "t0",
        description: "Optional stagnation temperature reference for static T output",
    },
    DeviceBindingArgSpec {
        name: "rho0",
        description: "Optional stagnation density reference for static rho output",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Subsonic/supersonic branch for area_ratio -> mach inversion",
    },
];

const M_GAMMA_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Mach input value",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
];

const AREA_GAMMA_BRANCH_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Area ratio input value (A/A*)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Subsonic or supersonic branch",
    },
];

const FIXED_M_TO_A_ASTAR: &[(&str, &str)] =
    &[("input_kind", "mach"), ("target_kind", "area_ratio")];
const FIXED_A_ASTAR_TO_M: &[(&str, &str)] =
    &[("input_kind", "area_ratio"), ("target_kind", "mach")];
const FIXED_M_TO_P_P0: &[(&str, &str)] =
    &[("input_kind", "mach"), ("target_kind", "pressure_ratio")];
const FIXED_M_TO_T_T0: &[(&str, &str)] =
    &[("input_kind", "mach"), ("target_kind", "temperature_ratio")];
const FIXED_M_TO_RHO_RHO0: &[(&str, &str)] =
    &[("input_kind", "mach"), ("target_kind", "density_ratio")];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc",
        python_name: "nozzle_flow_calc",
        excel_name: "ENG_NOZZLE_FLOW",
        op: "device.nozzle_flow_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Nozzle-flow calculator: input kind -> target kind through Mach pivot",
        rust_example: "eng::devices::nozzle_flow_calc().solve()?",
        python_example: "engpy.devices.nozzle_flow_calc(\"mach\", 2.0, \"pressure_ratio\", 1.4, None, None, None, \"\")",
        xloil_example: "=ENG_NOZZLE_FLOW(\"mach\",2.0,\"pressure_ratio\",1.4,NA(),NA(),NA(),\"\")",
        pyxll_example: "=ENG_NOZZLE_FLOW(\"mach\",2.0,\"pressure_ratio\",1.4,NA(),NA(),NA(),\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.path_text",
        python_name: "nozzle_flow_path_text",
        excel_name: "ENG_NOZZLE_FLOW_PATH_TEXT",
        op: "device.nozzle_flow_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Nozzle-flow calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_path_text(\"area_ratio\", 2.0, \"mach\", 1.4, None, None, None, \"supersonic\")",
        xloil_example: "=ENG_NOZZLE_FLOW_PATH_TEXT(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"supersonic\")",
        pyxll_example: "=ENG_NOZZLE_FLOW_PATH_TEXT(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.pivot",
        python_name: "nozzle_flow_pivot_mach",
        excel_name: "ENG_NOZZLE_FLOW_PIVOT_MACH",
        op: "device.nozzle_flow_calc.pivot_mach",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Nozzle-flow calculator helper: return resolved pivot Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_pivot_mach(\"area_ratio\", 2.0, \"mach\", 1.4, None, None, None, \"subsonic\")",
        xloil_example: "=ENG_NOZZLE_FLOW_PIVOT_MACH(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"subsonic\")",
        pyxll_example: "=ENG_NOZZLE_FLOW_PIVOT_MACH(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"subsonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.from_m.to_a_astar",
        python_name: "nozzle_flow_from_m_to_a_astar",
        excel_name: "ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR",
        op: "device.nozzle_flow_calc.value",
        fixed_args: FIXED_M_TO_A_ASTAR,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience nozzle-flow path: Mach -> A/A*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_from_m_to_a_astar(2.0, 1.4)",
        xloil_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)",
        pyxll_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.from_a_astar.to_m",
        python_name: "nozzle_flow_from_a_astar_to_m",
        excel_name: "ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M",
        op: "device.nozzle_flow_calc.value",
        fixed_args: FIXED_A_ASTAR_TO_M,
        args: AREA_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience nozzle-flow path: A/A* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_from_a_astar_to_m(2.0, 1.4, \"supersonic\")",
        xloil_example: "=ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,\"supersonic\")",
        pyxll_example: "=ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.from_m.to_p_p0",
        python_name: "nozzle_flow_from_m_to_p_p0",
        excel_name: "ENG_NOZZLE_FLOW_FROM_M_TO_P_P0",
        op: "device.nozzle_flow_calc.value",
        fixed_args: FIXED_M_TO_P_P0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience nozzle-flow path: Mach -> p/p0",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_from_m_to_p_p0(2.0, 1.4)",
        xloil_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)",
        pyxll_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.from_m.to_t_t0",
        python_name: "nozzle_flow_from_m_to_t_t0",
        excel_name: "ENG_NOZZLE_FLOW_FROM_M_TO_T_T0",
        op: "device.nozzle_flow_calc.value",
        fixed_args: FIXED_M_TO_T_T0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience nozzle-flow path: Mach -> T/T0",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_from_m_to_t_t0(2.0, 1.4)",
        xloil_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)",
        pyxll_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.nozzle_flow_calc.from_m.to_rho_rho0",
        python_name: "nozzle_flow_from_m_to_rho_rho0",
        excel_name: "ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0",
        op: "device.nozzle_flow_calc.value",
        fixed_args: FIXED_M_TO_RHO_RHO0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience nozzle-flow path: Mach -> rho/rho0",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.nozzle_flow_from_m_to_rho_rho0(2.0, 1.4)",
        xloil_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)",
        pyxll_example: "=ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.nozzle_flow_calc(\"area_ratio\", 2.0, \"mach\", 1.4, None, None, None, \"supersonic\")\nengpy.devices.nozzle_flow_from_m_to_a_astar(2.0, 1.4)\nengpy.devices.nozzle_flow_from_m_to_p_p0(2.0, 1.4)\nengpy.devices.nozzle_flow_path_text(\"mach\", 2.0, \"p\", 1.4, 2.0e6, None, None, \"\")\n```\n\n### Excel\n```excel\n=ENG_NOZZLE_FLOW(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"supersonic\")\n=ENG_NOZZLE_FLOW_FROM_M_TO_A_ASTAR(2.0,1.4)\n=ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M(2.0,1.4,\"subsonic\")\n=ENG_NOZZLE_FLOW_FROM_M_TO_P_P0(2.0,1.4)\n=ENG_NOZZLE_FLOW_FROM_M_TO_T_T0(2.0,1.4)\n=ENG_NOZZLE_FLOW_FROM_M_TO_RHO_RHO0(2.0,1.4)\n=ENG_NOZZLE_FLOW(\"mach\",2.0,\"p\",1.4,2000000,NA(),NA(),\"\")\n=ENG_NOZZLE_FLOW_PATH_TEXT(\"area_ratio\",2.0,\"mach\",1.4,NA(),NA(),NA(),\"supersonic\")\n```\n\n**Excel arguments**\n- `input_kind`: `mach`, `area_ratio`, `pressure_ratio`, `temperature_ratio`, `density_ratio`\n- `input_value`: input value\n- `target_kind`: `mach`, `area_ratio`, `pressure_ratio`, `temperature_ratio`, `density_ratio`, `p`, `t`, `rho`\n- `gamma`: specific heat ratio\n- `p0`, `t0`, `rho0`: optional stagnation references required for static outputs `p`, `t`, `rho`\n- `branch`: required for `area_ratio -> mach`\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style quasi-1D nozzle-flow device that resolves a pivot Mach number from one supported input kind and then evaluates the requested target kind.\n\n### Supported input kinds\n- `mach`\n- `area_ratio` (`A/A*`, branch-sensitive inverse)\n- `pressure_ratio` (`p/p0`)\n- `temperature_ratio` (`T/T0`)\n- `density_ratio` (`rho/rho0`)\n\n### Supported target kinds\n- `mach`\n- `area_ratio`\n- `pressure_ratio`\n- `temperature_ratio`\n- `density_ratio`\n- `p` (requires `p0`)\n- `t` (requires `t0`)\n- `rho` (requires `rho0`)\n\n### Branch behavior\n- `area_ratio -> mach` is double-valued and requires `subsonic` or `supersonic`.\n\n### Rust\n```rust\nuse eng::devices::{nozzle_flow_calc, NozzleFlowBranch, NozzleFlowInputKind, NozzleFlowOutputKind};\nlet out = nozzle_flow_calc()\n    .gamma(1.4)\n    .input(NozzleFlowInputKind::AreaRatio, 2.0)\n    .target(NozzleFlowOutputKind::Mach)\n    .branch(NozzleFlowBranch::Supersonic)\n    .solve()?;\nprintln!(\"M={}, value={}\", out.pivot_mach, out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: DEVICE_SPEC.key,
        name: DEVICE_SPEC.name,
        summary: DEVICE_SPEC.summary,
        supported_modes: &[
            "Input kinds: Mach, A/A*, p/p0, T/T0, rho/rho0",
            "Branch-aware inversion for A/A* -> Mach",
            "Optional stagnation-reference scaling for static p/T/rho outputs",
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
        ],
        binding_functions: BINDING_FUNCTIONS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NozzleFlowInputKind {
    Mach,
    AreaRatio,
    PressureRatio,
    TemperatureRatio,
    DensityRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NozzleFlowOutputKind {
    Mach,
    AreaRatio,
    PressureRatio,
    TemperatureRatio,
    DensityRatio,
    StaticPressure,
    StaticTemperature,
    StaticDensity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NozzleFlowBranch {
    Subsonic,
    Supersonic,
}

impl NozzleFlowBranch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Subsonic => "subsonic",
            Self::Supersonic => "supersonic",
        }
    }
}

pub fn parse_input_kind(raw: &str) -> Option<NozzleFlowInputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(NozzleFlowInputKind::Mach),
        "area_ratio" | "a_astar" | "a/a*" => Some(NozzleFlowInputKind::AreaRatio),
        "pressure_ratio" | "p_p0" | "p/p0" => Some(NozzleFlowInputKind::PressureRatio),
        "temperature_ratio" | "t_t0" | "t/t0" => Some(NozzleFlowInputKind::TemperatureRatio),
        "density_ratio" | "rho_rho0" | "rho/rho0" => Some(NozzleFlowInputKind::DensityRatio),
        _ => None,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<NozzleFlowOutputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(NozzleFlowOutputKind::Mach),
        "area_ratio" | "a_astar" | "a/a*" => Some(NozzleFlowOutputKind::AreaRatio),
        "pressure_ratio" | "p_p0" | "p/p0" => Some(NozzleFlowOutputKind::PressureRatio),
        "temperature_ratio" | "t_t0" | "t/t0" => Some(NozzleFlowOutputKind::TemperatureRatio),
        "density_ratio" | "rho_rho0" | "rho/rho0" => Some(NozzleFlowOutputKind::DensityRatio),
        "p" | "static_pressure" => Some(NozzleFlowOutputKind::StaticPressure),
        "t" | "static_temperature" => Some(NozzleFlowOutputKind::StaticTemperature),
        "rho" | "static_density" => Some(NozzleFlowOutputKind::StaticDensity),
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<NozzleFlowBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "subsonic" => Some(NozzleFlowBranch::Subsonic),
        "supersonic" => Some(NozzleFlowBranch::Supersonic),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct NozzleFlowCalcRequest {
    pub gamma: f64,
    pub input_kind: NozzleFlowInputKind,
    pub input_value: f64,
    pub target_kind: NozzleFlowOutputKind,
    pub branch: Option<NozzleFlowBranch>,
    pub p0: Option<f64>,
    pub t0: Option<f64>,
    pub rho0: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct NozzleFlowCalcResponse {
    pub value_si: f64,
    pub pivot_mach: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

impl NozzleFlowCalcResponse {
    pub fn path_text(&self) -> String {
        path_text(&self.path)
    }
}

#[derive(Debug, Error)]
pub enum NozzleFlowCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: subsonic, supersonic)")]
    MissingBranch { kind: &'static str },
    #[error("target '{target}' requires stagnation reference '{reference}'")]
    MissingReference {
        target: &'static str,
        reference: &'static str,
    },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, NozzleFlowCalcError>;

#[derive(Debug, Clone)]
pub struct NozzleFlowCalculatorDevice {
    req: NozzleFlowCalcRequest,
}

pub fn nozzle_flow_calc() -> NozzleFlowCalculatorDevice {
    NozzleFlowCalculatorDevice::new()
}

impl Default for NozzleFlowCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl NozzleFlowCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: NozzleFlowCalcRequest {
                gamma: 1.4,
                input_kind: NozzleFlowInputKind::Mach,
                input_value: 1.0,
                target_kind: NozzleFlowOutputKind::PressureRatio,
                branch: None,
                p0: None,
                t0: None,
                rho0: None,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn input(mut self, kind: NozzleFlowInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: NozzleFlowOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: NozzleFlowBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn stagnation_pressure(mut self, p0: f64) -> Self {
        self.req.p0 = Some(p0);
        self
    }

    pub fn stagnation_temperature(mut self, t0: f64) -> Self {
        self.req.t0 = Some(t0);
        self
    }

    pub fn stagnation_density(mut self, rho0: f64) -> Self {
        self.req.rho0 = Some(rho0);
        self
    }

    pub fn solve(self) -> Result<NozzleFlowCalcResponse> {
        calc(self.req)
    }
}

struct NozzleFlowRuntime;

impl PivotCalcSpec for NozzleFlowRuntime {
    type Request = NozzleFlowCalcRequest;
    type Error = NozzleFlowCalcError;

    fn validate_request(&self, req: &Self::Request) -> Result<()> {
        if !req.gamma.is_finite() || req.gamma <= 1.0 {
            return Err(NozzleFlowCalcError::InvalidGamma { value: req.gamma });
        }
        if !req.input_value.is_finite() {
            return Err(NozzleFlowCalcError::InvalidInputDomain {
                kind: input_kind_label(req.input_kind),
                reason: "must be finite".to_string(),
            });
        }
        validate_optional_ref(req.p0, "p0", req.target_kind)?;
        validate_optional_ref(req.t0, "t0", req.target_kind)?;
        validate_optional_ref(req.rho0, "rho0", req.target_kind)?;
        Ok(())
    }

    fn resolve_pivot(&self, req: &Self::Request, path: &mut Vec<CalcStep>) -> Result<f64> {
        let gamma = req.gamma;
        let input = req.input_value;
        match req.input_kind {
            NozzleFlowInputKind::Mach => Ok(input),
            NozzleFlowInputKind::AreaRatio => {
                let Some(branch) = req.branch else {
                    return Err(NozzleFlowCalcError::MissingBranch { kind: "area_ratio" });
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
            NozzleFlowInputKind::PressureRatio => {
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
            NozzleFlowInputKind::TemperatureRatio => {
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
            NozzleFlowInputKind::DensityRatio => {
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
        }
    }

    fn validate_pivot(&self, pivot_value: f64) -> Result<()> {
        if pivot_value.is_finite() && pivot_value > 0.0 {
            Ok(())
        } else {
            Err(NozzleFlowCalcError::InvalidInputDomain {
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
            NozzleFlowOutputKind::Mach => Ok(mach),
            NozzleFlowOutputKind::AreaRatio => {
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
            NozzleFlowOutputKind::PressureRatio => {
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
            NozzleFlowOutputKind::TemperatureRatio => {
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
            NozzleFlowOutputKind::DensityRatio => {
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
            NozzleFlowOutputKind::StaticPressure => {
                let Some(p0) = req.p0 else {
                    return Err(NozzleFlowCalcError::MissingReference {
                        target: "p",
                        reference: "p0",
                    });
                };
                let ratio = self.solve_target(
                    &NozzleFlowCalcRequest {
                        target_kind: NozzleFlowOutputKind::PressureRatio,
                        ..req.clone()
                    },
                    mach,
                    path,
                )?;
                let value = ratio * p0;
                path.push(CalcStep {
                    equation_path_id: "devices.nozzle_flow_calc.reference_scale".to_string(),
                    solved_for: "p".to_string(),
                    method: "explicit".to_string(),
                    branch: None,
                    inputs_used: vec![("p_p0".to_string(), ratio), ("p0".to_string(), p0)],
                });
                Ok(value)
            }
            NozzleFlowOutputKind::StaticTemperature => {
                let Some(t0) = req.t0 else {
                    return Err(NozzleFlowCalcError::MissingReference {
                        target: "t",
                        reference: "t0",
                    });
                };
                let ratio = self.solve_target(
                    &NozzleFlowCalcRequest {
                        target_kind: NozzleFlowOutputKind::TemperatureRatio,
                        ..req.clone()
                    },
                    mach,
                    path,
                )?;
                let value = ratio * t0;
                path.push(CalcStep {
                    equation_path_id: "devices.nozzle_flow_calc.reference_scale".to_string(),
                    solved_for: "t".to_string(),
                    method: "explicit".to_string(),
                    branch: None,
                    inputs_used: vec![("T_T0".to_string(), ratio), ("t0".to_string(), t0)],
                });
                Ok(value)
            }
            NozzleFlowOutputKind::StaticDensity => {
                let Some(rho0) = req.rho0 else {
                    return Err(NozzleFlowCalcError::MissingReference {
                        target: "rho",
                        reference: "rho0",
                    });
                };
                let ratio = self.solve_target(
                    &NozzleFlowCalcRequest {
                        target_kind: NozzleFlowOutputKind::DensityRatio,
                        ..req.clone()
                    },
                    mach,
                    path,
                )?;
                let value = ratio * rho0;
                path.push(CalcStep {
                    equation_path_id: "devices.nozzle_flow_calc.reference_scale".to_string(),
                    solved_for: "rho".to_string(),
                    method: "explicit".to_string(),
                    branch: None,
                    inputs_used: vec![("rho_rho0".to_string(), ratio), ("rho0".to_string(), rho0)],
                });
                Ok(value)
            }
        }
    }
}

pub fn calc(req: NozzleFlowCalcRequest) -> Result<NozzleFlowCalcResponse> {
    let out = run_pivot_calculation(&NozzleFlowRuntime, req)?;
    Ok(NozzleFlowCalcResponse {
        value_si: out.value_si,
        pivot_mach: out.pivot_value,
        path: out.path,
        warnings: out.warnings,
    })
}

fn validate_optional_ref(
    value: Option<f64>,
    name: &'static str,
    target_kind: NozzleFlowOutputKind,
) -> Result<()> {
    if let Some(v) = value {
        if !v.is_finite() || v <= 0.0 {
            return Err(NozzleFlowCalcError::InvalidInputDomain {
                kind: name,
                reason: "must be finite and > 0 when provided".to_string(),
            });
        }
    }
    if value.is_none()
        && matches!(
            (name, target_kind),
            ("p0", NozzleFlowOutputKind::StaticPressure)
                | ("t0", NozzleFlowOutputKind::StaticTemperature)
                | ("rho0", NozzleFlowOutputKind::StaticDensity)
        )
    {
        return Err(NozzleFlowCalcError::MissingReference {
            target: match target_kind {
                NozzleFlowOutputKind::StaticPressure => "p",
                NozzleFlowOutputKind::StaticTemperature => "t",
                NozzleFlowOutputKind::StaticDensity => "rho",
                _ => unreachable!(),
            },
            reference: name,
        });
    }
    Ok(())
}

fn input_kind_label(kind: NozzleFlowInputKind) -> &'static str {
    match kind {
        NozzleFlowInputKind::Mach => "mach",
        NozzleFlowInputKind::AreaRatio => "area_ratio",
        NozzleFlowInputKind::PressureRatio => "pressure_ratio",
        NozzleFlowInputKind::TemperatureRatio => "temperature_ratio",
        NozzleFlowInputKind::DensityRatio => "density_ratio",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mach_to_area_ratio_works() {
        let out = nozzle_flow_calc()
            .gamma(1.4)
            .input(NozzleFlowInputKind::Mach, 2.0)
            .target(NozzleFlowOutputKind::AreaRatio)
            .solve()
            .expect("M->A/A* should solve");
        assert!((out.value_si - 1.6875).abs() < 1e-6);
        assert!((out.pivot_mach - 2.0).abs() < 1e-12);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.area_mach")
        );
    }

    #[test]
    fn area_ratio_requires_branch_for_inverse() {
        let err = nozzle_flow_calc()
            .gamma(1.4)
            .input(NozzleFlowInputKind::AreaRatio, 2.0)
            .target(NozzleFlowOutputKind::Mach)
            .solve()
            .expect_err("A/A* -> M should require branch");
        assert!(matches!(err, NozzleFlowCalcError::MissingBranch { .. }));
    }

    #[test]
    fn static_pressure_requires_p0() {
        let err = nozzle_flow_calc()
            .gamma(1.4)
            .input(NozzleFlowInputKind::Mach, 2.0)
            .target(NozzleFlowOutputKind::StaticPressure)
            .solve()
            .expect_err("static pressure output without p0 should fail");
        assert!(matches!(err, NozzleFlowCalcError::MissingReference { .. }));
    }

    #[test]
    fn static_pressure_computes_from_ratio_and_reference() {
        let out = nozzle_flow_calc()
            .gamma(1.4)
            .input(NozzleFlowInputKind::Mach, 2.0)
            .target(NozzleFlowOutputKind::StaticPressure)
            .stagnation_pressure(2.0e6)
            .solve()
            .expect("static pressure output should solve");
        let expected_ratio = 0.12780452546295096;
        assert!((out.value_si - expected_ratio * 2.0e6).abs() < 1e-6);
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.isentropic_pressure_ratio")
        );
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "devices.nozzle_flow_calc.reference_scale")
        );
    }

    #[test]
    fn path_text_mentions_area_mach() {
        let out = nozzle_flow_calc()
            .gamma(1.4)
            .input(NozzleFlowInputKind::AreaRatio, 2.0)
            .target(NozzleFlowOutputKind::PressureRatio)
            .branch(NozzleFlowBranch::Supersonic)
            .solve()
            .expect("A/A* supersonic path should solve");
        let text = out.path_text();
        assert!(text.contains("compressible.area_mach"));
        assert!(text.contains("compressible.isentropic_pressure_ratio"));
    }
}
