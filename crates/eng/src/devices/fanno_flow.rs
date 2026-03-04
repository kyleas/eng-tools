use equations::{compressible, eq};
use thiserror::Error;

use super::framework::{
    CalcStep, CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcSpec, method_label, path_text,
    run_pivot_calculation,
};
use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

const FANNO_INPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "t_tstar",
        label: "T/T*",
    },
    CalculatorKindSpec {
        key: "p_pstar",
        label: "p/p*",
    },
    CalculatorKindSpec {
        key: "rho_rhostar",
        label: "rho/rho*",
    },
    CalculatorKindSpec {
        key: "p0_p0star",
        label: "p0/p0*",
    },
    CalculatorKindSpec {
        key: "four_flstar_d",
        label: "4fL*/D",
    },
];

const FANNO_OUTPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
    CalculatorKindSpec {
        key: "mach",
        label: "Mach",
    },
    CalculatorKindSpec {
        key: "t_tstar",
        label: "T/T*",
    },
    CalculatorKindSpec {
        key: "p_pstar",
        label: "p/p*",
    },
    CalculatorKindSpec {
        key: "rho_rhostar",
        label: "rho/rho*",
    },
    CalculatorKindSpec {
        key: "p0_p0star",
        label: "p0/p0*",
    },
    CalculatorKindSpec {
        key: "v_vstar",
        label: "V/V*",
    },
    CalculatorKindSpec {
        key: "four_flstar_d",
        label: "4fL*/D",
    },
];

pub const DEVICE_SPEC: CalculatorDeviceSpec = CalculatorDeviceSpec {
    key: "fanno_flow_calc",
    name: "Fanno Flow Calculator",
    summary: "Calculator-style compressible device: solve Fanno star-reference input kinds to target kinds through Mach pivot orchestration.",
    route: "devices/fanno_flow_calc.md",
    pivot_label: "Mach",
    input_kinds: FANNO_INPUT_KIND_SPECS,
    output_kinds: FANNO_OUTPUT_KIND_SPECS,
    branches: &["subsonic", "supersonic"],
};

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, four_flstar_d)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (mach, t_tstar, p_pstar, rho_rhostar, p0_p0star, v_vstar, four_flstar_d)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Subsonic/supersonic branch for inverse paths",
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

const RATIO_GAMMA_BRANCH_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input ratio value",
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

const FIXED_M_TO_T: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "t_tstar")];
const FIXED_M_TO_P: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "p_pstar")];
const FIXED_M_TO_RHO: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "rho_rhostar")];
const FIXED_M_TO_P0: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "p0_p0star")];
const FIXED_M_TO_V: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "v_vstar")];
const FIXED_M_TO_4FL: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "four_flstar_d")];
const FIXED_4FL_TO_M: &[(&str, &str)] = &[("input_kind", "four_flstar_d"), ("target_kind", "mach")];
const FIXED_P0_TO_M: &[(&str, &str)] = &[("input_kind", "p0_p0star"), ("target_kind", "mach")];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc",
        python_name: "fanno_flow_calc",
        excel_name: "ENG_FANNO_FLOW",
        op: "device.fanno_flow_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Fanno-flow calculator: input kind -> target kind through Mach pivot",
        rust_example: "eng::devices::fanno_flow_calc().solve()?",
        python_example: "engpy.devices.fanno_flow_calc(\"mach\", 2.0, \"p_pstar\", 1.4)",
        xloil_example: "=ENG_FANNO_FLOW(\"mach\",2.0,\"p_pstar\",1.4,\"\")",
        pyxll_example: "=ENG_FANNO_FLOW(\"mach\",2.0,\"p_pstar\",1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.path_text",
        python_name: "fanno_flow_path_text",
        excel_name: "ENG_FANNO_FLOW_PATH_TEXT",
        op: "device.fanno_flow_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Fanno-flow calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_path_text(\"p0_p0star\", 1.33984375, \"mach\", 1.4, \"subsonic\")",
        xloil_example: "=ENG_FANNO_FLOW_PATH_TEXT(\"p0_p0star\",1.33984375,\"mach\",1.4,\"subsonic\")",
        pyxll_example: "=ENG_FANNO_FLOW_PATH_TEXT(\"p0_p0star\",1.33984375,\"mach\",1.4,\"subsonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.pivot",
        python_name: "fanno_flow_pivot_mach",
        excel_name: "ENG_FANNO_FLOW_PIVOT_MACH",
        op: "device.fanno_flow_calc.pivot_mach",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Fanno-flow calculator helper: return resolved pivot Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_pivot_mach(\"four_flstar_d\", 0.3049965026, \"mach\", 1.4, \"supersonic\")",
        xloil_example: "=ENG_FANNO_FLOW_PIVOT_MACH(\"four_flstar_d\",0.3049965026,\"mach\",1.4,\"supersonic\")",
        pyxll_example: "=ENG_FANNO_FLOW_PIVOT_MACH(\"four_flstar_d\",0.3049965026,\"mach\",1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_t_tstar",
        python_name: "fanno_flow_from_m_to_t_tstar",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_T,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> T/T*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_t_tstar(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_p_pstar",
        python_name: "fanno_flow_from_m_to_p_pstar",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_P,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> p/p*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_p_pstar(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_rho_rhostar",
        python_name: "fanno_flow_from_m_to_rho_rhostar",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_RHO,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> rho/rho*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_rho_rhostar(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_p0_p0star",
        python_name: "fanno_flow_from_m_to_p0_p0star",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_P0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> p0/p0*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_p0_p0star(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_v_vstar",
        python_name: "fanno_flow_from_m_to_v_vstar",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_V,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> V/V*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_v_vstar(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_m.to_4flstar_d",
        python_name: "fanno_flow_from_m_to_4flstar_d",
        excel_name: "ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_M_TO_4FL,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: Mach -> 4fL*/D",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_m_to_4flstar_d(2.0, 1.4)",
        xloil_example: "=ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_4flstar_d.to_m",
        python_name: "fanno_flow_from_4flstar_d_to_m",
        excel_name: "ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_4FL_TO_M,
        args: RATIO_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: 4fL*/D -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_4flstar_d_to_m(0.3049965026, 1.4, \"supersonic\")",
        xloil_example: "=ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,\"supersonic\")",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.fanno_flow_calc.from_p0_p0star.to_m",
        python_name: "fanno_flow_from_p0_p0star_to_m",
        excel_name: "ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M",
        op: "device.fanno_flow_calc.value",
        fixed_args: FIXED_P0_TO_M,
        args: RATIO_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience Fanno path: p0/p0* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.fanno_flow_from_p0_p0star_to_m(1.33984375, 1.4, \"subsonic\")",
        xloil_example: "=ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,\"subsonic\")",
        pyxll_example: "=ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,\"subsonic\")",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.fanno_flow_calc(\"mach\", 2.0, \"p_pstar\", 1.4)\nengpy.devices.fanno_flow_from_m_to_p0_p0star(2.0, 1.4)\nengpy.devices.fanno_flow_from_4flstar_d_to_m(0.3049965026, 1.4, \"supersonic\")\nengpy.devices.fanno_flow_path_text(\"p0_p0star\", 1.33984375, \"mach\", 1.4, \"subsonic\")\n```\n\n### Excel\n```excel\n=ENG_FANNO_FLOW(\"mach\",2.0,\"p_pstar\",1.4,\"\")\n=ENG_FANNO_FLOW_FROM_M_TO_T_TSTAR(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_M_TO_P_PSTAR(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_M_TO_P0_P0STAR(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_M_TO_V_VSTAR(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_M_TO_4FLSTAR_D(2.0,1.4)\n=ENG_FANNO_FLOW_FROM_4FLSTAR_D_TO_M(0.3049965026,1.4,\"supersonic\")\n=ENG_FANNO_FLOW_FROM_P0_P0STAR_TO_M(1.33984375,1.4,\"subsonic\")\n=ENG_FANNO_FLOW_PATH_TEXT(\"four_flstar_d\",0.3049965026,\"mach\",1.4,\"supersonic\")\n```\n\n**Excel arguments**\n- `value_kind_in`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `p0_p0star`, `four_flstar_d`\n- `value_in`: input value\n- `target_kind_out`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `p0_p0star`, `v_vstar`, `four_flstar_d`\n- `gamma`: specific heat ratio\n- `branch`: required for inverse paths (`subsonic`/`supersonic`)\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style compressible device that resolves a pivot Mach number from one supported Fanno input kind and then evaluates the requested target kind.\n\n### Supported input kinds\n- `mach`\n- `t_tstar` (`T/T*`)\n- `p_pstar` (`p/p*`)\n- `rho_rhostar` (`rho/rho*`)\n- `p0_p0star` (`p0/p0*`)\n- `four_flstar_d` (`4fL*/D`)\n\n### Supported target kinds\n- `mach`\n- `t_tstar`\n- `p_pstar`\n- `rho_rhostar`\n- `p0_p0star`\n- `v_vstar`\n- `four_flstar_d`\n\n### Branch behavior\n- Inverse paths (`ratio -> mach`) are branch-sensitive and require `subsonic` or `supersonic`.\n\n### Rust\n```rust\nuse eng::devices::{fanno_flow_calc, FannoFlowBranch, FannoFlowInputKind, FannoFlowOutputKind};\nlet out = fanno_flow_calc()\n    .gamma(1.4)\n    .input(FannoFlowInputKind::FrictionParameter, 0.3049965025814798)\n    .target(FannoFlowOutputKind::Mach)\n    .branch(FannoFlowBranch::Supersonic)\n    .solve()?;\nprintln!(\"M={}, value={}\", out.pivot_mach, out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: DEVICE_SPEC.key,
        name: DEVICE_SPEC.name,
        summary: DEVICE_SPEC.summary,
        supported_modes: &[
            "Input kinds: Mach, T/T*, p/p*, rho/rho*, p0/p0*, 4fL*/D",
            "Branch-aware inversion for ratio -> Mach",
        ],
        outputs: &["value_si", "pivot_mach", "path diagnostics"],
        route: DEVICE_SPEC.route,
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "compressible.fanno_temperature_ratio",
            "compressible.fanno_pressure_ratio",
            "compressible.fanno_density_ratio",
            "compressible.fanno_stagnation_pressure_ratio",
            "compressible.fanno_velocity_ratio",
            "compressible.fanno_friction_parameter",
        ],
        binding_functions: BINDING_FUNCTIONS,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FannoFlowInputKind {
    Mach,
    TemperatureRatio,
    PressureRatio,
    DensityRatio,
    StagnationPressureRatio,
    FrictionParameter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FannoFlowOutputKind {
    Mach,
    TemperatureRatio,
    PressureRatio,
    DensityRatio,
    StagnationPressureRatio,
    VelocityRatio,
    FrictionParameter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FannoFlowBranch {
    Subsonic,
    Supersonic,
}

impl FannoFlowBranch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Subsonic => "subsonic",
            Self::Supersonic => "supersonic",
        }
    }
}

pub fn parse_input_kind(raw: &str) -> Option<FannoFlowInputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(FannoFlowInputKind::Mach),
        "t_tstar" | "t/t*" | "temperature_ratio" => Some(FannoFlowInputKind::TemperatureRatio),
        "p_pstar" | "p/p*" | "pressure_ratio" => Some(FannoFlowInputKind::PressureRatio),
        "rho_rhostar" | "rho/rho*" | "density_ratio" => Some(FannoFlowInputKind::DensityRatio),
        "p0_p0star" | "p0/p0*" | "stagnation_pressure_ratio" => {
            Some(FannoFlowInputKind::StagnationPressureRatio)
        }
        "four_flstar_d" | "4flstar_d" | "4fl*/d" | "4fl_star_d" | "friction_parameter" => {
            Some(FannoFlowInputKind::FrictionParameter)
        }
        _ => None,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<FannoFlowOutputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(FannoFlowOutputKind::Mach),
        "t_tstar" | "t/t*" | "temperature_ratio" => Some(FannoFlowOutputKind::TemperatureRatio),
        "p_pstar" | "p/p*" | "pressure_ratio" => Some(FannoFlowOutputKind::PressureRatio),
        "rho_rhostar" | "rho/rho*" | "density_ratio" => Some(FannoFlowOutputKind::DensityRatio),
        "p0_p0star" | "p0/p0*" | "stagnation_pressure_ratio" => {
            Some(FannoFlowOutputKind::StagnationPressureRatio)
        }
        "v_vstar" | "v/v*" | "velocity_ratio" => Some(FannoFlowOutputKind::VelocityRatio),
        "four_flstar_d" | "4flstar_d" | "4fl*/d" | "4fl_star_d" | "friction_parameter" => {
            Some(FannoFlowOutputKind::FrictionParameter)
        }
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<FannoFlowBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "subsonic" => Some(FannoFlowBranch::Subsonic),
        "supersonic" => Some(FannoFlowBranch::Supersonic),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct FannoFlowCalcRequest {
    pub gamma: f64,
    pub input_kind: FannoFlowInputKind,
    pub input_value: f64,
    pub target_kind: FannoFlowOutputKind,
    pub branch: Option<FannoFlowBranch>,
}

#[derive(Debug, Clone)]
pub struct FannoFlowCalcResponse {
    pub value_si: f64,
    pub pivot_mach: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

impl FannoFlowCalcResponse {
    pub fn path_text(&self) -> String {
        path_text(&self.path)
    }
}

#[derive(Debug, Error)]
pub enum FannoFlowCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: subsonic, supersonic)")]
    MissingBranch { kind: &'static str },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, FannoFlowCalcError>;

#[derive(Debug, Clone)]
pub struct FannoFlowCalculatorDevice {
    req: FannoFlowCalcRequest,
}

pub fn fanno_flow_calc() -> FannoFlowCalculatorDevice {
    FannoFlowCalculatorDevice::new()
}

impl Default for FannoFlowCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl FannoFlowCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: FannoFlowCalcRequest {
                gamma: 1.4,
                input_kind: FannoFlowInputKind::Mach,
                input_value: 1.0,
                target_kind: FannoFlowOutputKind::PressureRatio,
                branch: None,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn input(mut self, kind: FannoFlowInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: FannoFlowOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: FannoFlowBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn solve(self) -> Result<FannoFlowCalcResponse> {
        calc(self.req)
    }
}
struct FannoFlowRuntime;

impl PivotCalcSpec for FannoFlowRuntime {
    type Request = FannoFlowCalcRequest;
    type Error = FannoFlowCalcError;

    fn validate_request(&self, req: &Self::Request) -> Result<()> {
        if !req.gamma.is_finite() || req.gamma <= 1.0 {
            return Err(FannoFlowCalcError::InvalidGamma { value: req.gamma });
        }
        if !req.input_value.is_finite() {
            return Err(FannoFlowCalcError::InvalidInputDomain {
                kind: input_kind_label(req.input_kind),
                reason: "must be finite".to_string(),
            });
        }
        match req.input_kind {
            FannoFlowInputKind::Mach if req.input_value <= 0.0 => {
                return Err(FannoFlowCalcError::InvalidInputDomain {
                    kind: "Mach",
                    reason: "must be > 0".to_string(),
                });
            }
            FannoFlowInputKind::FrictionParameter if req.input_value < 0.0 => {
                return Err(FannoFlowCalcError::InvalidInputDomain {
                    kind: "FrictionParameter",
                    reason: "must be >= 0".to_string(),
                });
            }
            FannoFlowInputKind::Mach => {}
            _ if req.input_value <= 0.0 => {
                return Err(FannoFlowCalcError::InvalidInputDomain {
                    kind: input_kind_label(req.input_kind),
                    reason: "must be > 0".to_string(),
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_pivot(&self, req: &Self::Request, path: &mut Vec<CalcStep>) -> Result<f64> {
        let gamma = req.gamma;
        let input = req.input_value;

        if matches!(req.input_kind, FannoFlowInputKind::Mach) {
            return Ok(input);
        }

        let Some(branch) = req.branch else {
            return Err(FannoFlowCalcError::MissingBranch {
                kind: input_kind_label(req.input_kind),
            });
        };

        match req.input_kind {
            FannoFlowInputKind::TemperatureRatio => {
                let solved = eq
                    .solve(compressible::fanno_temperature_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_t_tstar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_temperature_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("t_tstar".to_string(), input), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowInputKind::PressureRatio => {
                let solved = eq
                    .solve(compressible::fanno_pressure_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_p_pstar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_pressure_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("p_pstar".to_string(), input), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowInputKind::DensityRatio => {
                let solved = eq
                    .solve(compressible::fanno_density_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_rho_rhostar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_density_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![
                        ("rho_rhostar".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
                });
                Ok(solved.value_si)
            }
            FannoFlowInputKind::StagnationPressureRatio => {
                let solved = eq
                    .solve(compressible::fanno_stagnation_pressure_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_p0_p0star(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_stagnation_pressure_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![
                        ("p0_p0star".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
                });
                Ok(solved.value_si)
            }
            FannoFlowInputKind::FrictionParameter => {
                let solved = eq
                    .solve(compressible::fanno_friction_parameter::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_four_flstar_d(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_friction_parameter".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![
                        ("four_flstar_d".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
                });
                Ok(solved.value_si)
            }
            FannoFlowInputKind::Mach => unreachable!(),
        }
    }

    fn validate_pivot(&self, pivot_value: f64) -> Result<()> {
        if pivot_value.is_finite() && pivot_value > 0.0 {
            Ok(())
        } else {
            Err(FannoFlowCalcError::InvalidInputDomain {
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
            FannoFlowOutputKind::Mach => Ok(mach),
            FannoFlowOutputKind::TemperatureRatio => {
                let solved = eq.solve_result(
                    compressible::fanno_temperature_ratio::equation(),
                    "t_tstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_temperature_ratio".to_string(),
                    solved_for: "t_tstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowOutputKind::PressureRatio => {
                let solved = eq.solve_result(
                    compressible::fanno_pressure_ratio::equation(),
                    "p_pstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_pressure_ratio".to_string(),
                    solved_for: "p_pstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowOutputKind::DensityRatio => {
                let solved = eq.solve_result(
                    compressible::fanno_density_ratio::equation(),
                    "rho_rhostar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_density_ratio".to_string(),
                    solved_for: "rho_rhostar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowOutputKind::StagnationPressureRatio => {
                let solved = eq.solve_result(
                    compressible::fanno_stagnation_pressure_ratio::equation(),
                    "p0_p0star",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_stagnation_pressure_ratio".to_string(),
                    solved_for: "p0_p0star".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowOutputKind::VelocityRatio => {
                let solved = eq.solve_result(
                    compressible::fanno_velocity_ratio::equation(),
                    "v_vstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_velocity_ratio".to_string(),
                    solved_for: "v_vstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            FannoFlowOutputKind::FrictionParameter => {
                let solved = eq.solve_result(
                    compressible::fanno_friction_parameter::equation(),
                    "four_flstar_d",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.fanno_friction_parameter".to_string(),
                    solved_for: "four_flstar_d".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
        }
    }
}

pub fn calc(req: FannoFlowCalcRequest) -> Result<FannoFlowCalcResponse> {
    let out = run_pivot_calculation(&FannoFlowRuntime, req)?;
    Ok(FannoFlowCalcResponse {
        value_si: out.value_si,
        pivot_mach: out.pivot_value,
        path: out.path,
        warnings: out.warnings,
    })
}

fn input_kind_label(kind: FannoFlowInputKind) -> &'static str {
    match kind {
        FannoFlowInputKind::Mach => "Mach",
        FannoFlowInputKind::TemperatureRatio => "TemperatureRatio",
        FannoFlowInputKind::PressureRatio => "PressureRatio",
        FannoFlowInputKind::DensityRatio => "DensityRatio",
        FannoFlowInputKind::StagnationPressureRatio => "StagnationPressureRatio",
        FannoFlowInputKind::FrictionParameter => "FrictionParameter",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use equations::{compressible, eq};

    #[test]
    fn m_to_t_tstar_works() {
        let out = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::Mach,
            input_value: 2.0,
            target_kind: FannoFlowOutputKind::TemperatureRatio,
            branch: None,
        })
        .expect("fanno calc");
        let expected = eq
            .solve(compressible::fanno_temperature_ratio::equation())
            .target_t_tstar()
            .given_m(2.0)
            .given_gamma(1.4)
            .value()
            .expect("direct T/T*");
        assert!((out.value_si - expected).abs() < 1e-10);
        assert!((out.pivot_mach - 2.0).abs() < 1e-12);
    }

    #[test]
    fn m_to_p_pstar_works() {
        let out = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::Mach,
            input_value: 2.0,
            target_kind: FannoFlowOutputKind::PressureRatio,
            branch: None,
        })
        .expect("fanno calc");
        assert!((out.value_si - 0.408248290463863).abs() < 1e-10);
    }

    #[test]
    fn m_to_p0_p0star_works() {
        let out = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::Mach,
            input_value: 0.5,
            target_kind: FannoFlowOutputKind::StagnationPressureRatio,
            branch: None,
        })
        .expect("fanno calc");
        assert!((out.value_si - 1.3398437500000004).abs() < 1e-10);
    }

    #[test]
    fn four_flstar_d_to_m_supports_branches() {
        let input = 0.3049965025814798;
        let sub = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::FrictionParameter,
            input_value: input,
            target_kind: FannoFlowOutputKind::Mach,
            branch: Some(FannoFlowBranch::Subsonic),
        })
        .expect("sub branch");
        let sup = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::FrictionParameter,
            input_value: input,
            target_kind: FannoFlowOutputKind::Mach,
            branch: Some(FannoFlowBranch::Supersonic),
        })
        .expect("sup branch");

        assert!(sub.value_si < 1.0);
        assert!(sup.value_si > 1.0);

        let expected_sup = eq
            .solve(compressible::fanno_friction_parameter::equation())
            .target_m()
            .branch("supersonic")
            .given_four_flstar_d(input)
            .given_gamma(1.4)
            .value()
            .expect("sup root");
        assert!((sup.value_si - expected_sup).abs() < 1e-10);
    }

    #[test]
    fn missing_branch_errors_on_inverse_paths() {
        let err = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::StagnationPressureRatio,
            input_value: 1.33984375,
            target_kind: FannoFlowOutputKind::Mach,
            branch: None,
        })
        .expect_err("missing branch should error");
        assert!(matches!(err, FannoFlowCalcError::MissingBranch { .. }));
    }

    #[test]
    fn path_contains_registry_equation_ids() {
        let out = calc(FannoFlowCalcRequest {
            gamma: 1.4,
            input_kind: FannoFlowInputKind::PressureRatio,
            input_value: 0.408248290463863,
            target_kind: FannoFlowOutputKind::VelocityRatio,
            branch: Some(FannoFlowBranch::Supersonic),
        })
        .expect("fanno calc");

        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.fanno_pressure_ratio")
        );
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.fanno_velocity_ratio")
        );
    }
}
