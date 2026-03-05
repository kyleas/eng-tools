use equations::{compressible, eq};
use thiserror::Error;

use super::framework::{
    CalcStep, CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcSpec, method_label, path_text,
    run_pivot_calculation,
};
use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

const RAYLEIGH_INPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
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
        key: "t0_t0star",
        label: "T0/T0*",
    },
    CalculatorKindSpec {
        key: "p0_p0star",
        label: "p0/p0*",
    },
    CalculatorKindSpec {
        key: "v_vstar",
        label: "V/V*",
    },
];

const RAYLEIGH_OUTPUT_KIND_SPECS: &[CalculatorKindSpec] = &[
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
        key: "t0_t0star",
        label: "T0/T0*",
    },
    CalculatorKindSpec {
        key: "p0_p0star",
        label: "p0/p0*",
    },
    CalculatorKindSpec {
        key: "v_vstar",
        label: "V/V*",
    },
];

pub const DEVICE_SPEC: CalculatorDeviceSpec = CalculatorDeviceSpec {
    key: "rayleigh_calc",
    name: "Rayleigh Flow Calculator",
    summary: "Calculator-style compressible device: solve Rayleigh star-reference input kinds to target kinds through Mach pivot orchestration.",
    route: "devices/rayleigh_calc.md",
    pivot_label: "Mach",
    input_kinds: RAYLEIGH_INPUT_KIND_SPECS,
    output_kinds: RAYLEIGH_OUTPUT_KIND_SPECS,
    branches: &["subsonic", "supersonic"],
};

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (mach, t_tstar, p_pstar, rho_rhostar, t0_t0star, p0_p0star, v_vstar)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
    DeviceBindingArgSpec {
        name: "branch",
        description: "Subsonic/supersonic branch for branch-sensitive inverse paths",
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
const FIXED_M_TO_T0: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "t0_t0star")];
const FIXED_M_TO_P0: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "p0_p0star")];
const FIXED_M_TO_V: &[(&str, &str)] = &[("input_kind", "mach"), ("target_kind", "v_vstar")];
const FIXED_T_TO_M: &[(&str, &str)] = &[("input_kind", "t_tstar"), ("target_kind", "mach")];
const FIXED_T0_TO_M: &[(&str, &str)] = &[("input_kind", "t0_t0star"), ("target_kind", "mach")];
const FIXED_P0_TO_M: &[(&str, &str)] = &[("input_kind", "p0_p0star"), ("target_kind", "mach")];
const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc",
        python_name: "rayleigh_calc",
        excel_name: "ENG_RAYLEIGH",
        op: "device.rayleigh_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Rayleigh-flow calculator: input kind -> target kind through Mach pivot",
        rust_example: "eng::devices::rayleigh_calc().solve()?",
        python_example: "engpy.devices.rayleigh_calc(\"mach\", 2.0, \"p_pstar\", 1.4)",
        xloil_example: "=ENG_RAYLEIGH(\"mach\",2.0,\"p_pstar\",1.4,\"\")",
        pyxll_example: "=ENG_RAYLEIGH(\"mach\",2.0,\"p_pstar\",1.4,\"\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.path_text",
        python_name: "rayleigh_path_text",
        excel_name: "ENG_RAYLEIGH_PATH_TEXT",
        op: "device.rayleigh_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Rayleigh-flow calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_path_text(\"t_tstar\", 0.7901234568, \"mach\", 1.4, \"subsonic\")",
        xloil_example: "=ENG_RAYLEIGH_PATH_TEXT(\"t_tstar\",0.7901234568,\"mach\",1.4,\"subsonic\")",
        pyxll_example: "=ENG_RAYLEIGH_PATH_TEXT(\"t_tstar\",0.7901234568,\"mach\",1.4,\"subsonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.pivot",
        python_name: "rayleigh_pivot_mach",
        excel_name: "ENG_RAYLEIGH_PIVOT_MACH",
        op: "device.rayleigh_calc.pivot_mach",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Rayleigh-flow calculator helper: return resolved pivot Mach",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_pivot_mach(\"t0_t0star\", 0.7933884298, \"mach\", 1.4, \"supersonic\")",
        xloil_example: "=ENG_RAYLEIGH_PIVOT_MACH(\"t0_t0star\",0.7933884298,\"mach\",1.4,\"supersonic\")",
        pyxll_example: "=ENG_RAYLEIGH_PIVOT_MACH(\"t0_t0star\",0.7933884298,\"mach\",1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_t_tstar",
        python_name: "rayleigh_from_m_to_t_tstar",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_T_TSTAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_T,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> T/T*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_t_tstar(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_p_pstar",
        python_name: "rayleigh_from_m_to_p_pstar",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_P_PSTAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_P,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> p/p*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_p_pstar(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_rho_rhostar",
        python_name: "rayleigh_from_m_to_rho_rhostar",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_RHO,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> rho/rho*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_rho_rhostar(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_t0_t0star",
        python_name: "rayleigh_from_m_to_t0_t0star",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_T0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> T0/T0*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_t0_t0star(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_p0_p0star",
        python_name: "rayleigh_from_m_to_p0_p0star",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_P0,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> p0/p0*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_p0_p0star(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_m.to_v_vstar",
        python_name: "rayleigh_from_m_to_v_vstar",
        excel_name: "ENG_RAYLEIGH_FROM_M_TO_V_VSTAR",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_M_TO_V,
        args: M_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: Mach -> V/V*",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_m_to_v_vstar(2.0, 1.4)",
        xloil_example: "=ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)",
        pyxll_example: "=ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_t_tstar.to_m",
        python_name: "rayleigh_from_t_tstar_to_m",
        excel_name: "ENG_RAYLEIGH_FROM_T_TSTAR_TO_M",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_T_TO_M,
        args: RATIO_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: T/T* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_t_tstar_to_m(0.7901234568, 1.4, \"subsonic\")",
        xloil_example: "=ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,\"subsonic\")",
        pyxll_example: "=ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,\"subsonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_t0_t0star.to_m",
        python_name: "rayleigh_from_t0_t0star_to_m",
        excel_name: "ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_T0_TO_M,
        args: RATIO_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: T0/T0* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_t0_t0star_to_m(0.7933884298, 1.4, \"supersonic\")",
        xloil_example: "=ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,\"supersonic\")",
        pyxll_example: "=ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,\"supersonic\")",
    },
    DeviceBindingFunctionSpec {
        id: "device.rayleigh_calc.from_p0_p0star.to_m",
        python_name: "rayleigh_from_p0_p0star_to_m",
        excel_name: "ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M",
        op: "device.rayleigh_calc.value",
        fixed_args: FIXED_P0_TO_M,
        args: RATIO_GAMMA_BRANCH_ARGS,
        returns: "f64",
        help: "Convenience Rayleigh path: p0/p0* -> Mach (branch required)",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.rayleigh_from_p0_p0star_to_m(1.1140525032, 1.4, \"subsonic\")",
        xloil_example: "=ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,\"subsonic\")",
        pyxll_example: "=ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,\"subsonic\")",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.rayleigh_calc(\"mach\", 2.0, \"p_pstar\", 1.4)\nengpy.devices.rayleigh_from_m_to_t0_t0star(2.0, 1.4)\nengpy.devices.rayleigh_from_t_tstar_to_m(0.7901234568, 1.4, \"subsonic\")\nengpy.devices.rayleigh_path_text(\"t0_t0star\", 0.7933884298, \"mach\", 1.4, \"supersonic\")\n```\n\n### Excel\n```excel\n=ENG_RAYLEIGH(\"mach\",2.0,\"p_pstar\",1.4,\"\")\n=ENG_RAYLEIGH_FROM_M_TO_T_TSTAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_M_TO_P_PSTAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_M_TO_RHO_RHOSTAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_M_TO_T0_T0STAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_M_TO_P0_P0STAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_M_TO_V_VSTAR(2.0,1.4)\n=ENG_RAYLEIGH_FROM_T_TSTAR_TO_M(0.7901234568,1.4,\"subsonic\")\n=ENG_RAYLEIGH_FROM_T0_T0STAR_TO_M(0.7933884298,1.4,\"supersonic\")\n=ENG_RAYLEIGH_FROM_P0_P0STAR_TO_M(1.1140525032,1.4,\"subsonic\")\n=ENG_RAYLEIGH_PATH_TEXT(\"t_tstar\",0.7901234568,\"mach\",1.4,\"subsonic\")\n```\n\n**Excel arguments**\n- `input_kind`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `t0_t0star`, `p0_p0star`, `v_vstar`\n- `input_value`: input value\n- `target_kind`: `mach`, `t_tstar`, `p_pstar`, `rho_rhostar`, `t0_t0star`, `p0_p0star`, `v_vstar`\n- `gamma`: specific heat ratio\n- `branch`: required for branch-sensitive inverse paths (`subsonic`/`supersonic`)\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style compressible device that resolves a pivot Mach number from one supported Rayleigh input kind and then evaluates the requested target kind.\n\n### Supported input kinds\n- `mach`\n- `t_tstar` (`T/T*`)\n- `p_pstar` (`p/p*`)\n- `rho_rhostar` (`rho/rho*`)\n- `t0_t0star` (`T0/T0*`)\n- `p0_p0star` (`p0/p0*`)\n- `v_vstar` (`V/V*`)\n\n### Supported target kinds\n- `mach`\n- `t_tstar`\n- `p_pstar`\n- `rho_rhostar`\n- `t0_t0star`\n- `p0_p0star`\n- `v_vstar`\n\n### Branch behavior\n- Branch-sensitive inverse paths (`T/T* -> M`, `T0/T0* -> M`, `p0/p0* -> M`) require `subsonic` or `supersonic`.\n\n### Rust\n```rust\nuse eng::devices::{rayleigh_calc, RayleighBranch, RayleighInputKind, RayleighOutputKind};\nlet out = rayleigh_calc()\n    .gamma(1.4)\n    .input(RayleighInputKind::StagnationTemperatureRatio, 0.7933884297520661)\n    .target(RayleighOutputKind::Mach)\n    .branch(RayleighBranch::Supersonic)\n    .solve()?;\nprintln!(\"M={}, value={}\", out.pivot_mach, out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: DEVICE_SPEC.key,
        name: DEVICE_SPEC.name,
        summary: DEVICE_SPEC.summary,
        supported_modes: &[
            "Input kinds: Mach, T/T*, p/p*, rho/rho*, T0/T0*, p0/p0*, V/V*",
            "Branch-aware inversion for selected ratio -> Mach paths",
        ],
        outputs: &["value_si", "pivot_mach", "path diagnostics"],
        route: DEVICE_SPEC.route,
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
            "compressible.rayleigh_temperature_ratio",
            "compressible.rayleigh_pressure_ratio",
            "compressible.rayleigh_density_ratio",
            "compressible.rayleigh_stagnation_temperature_ratio",
            "compressible.rayleigh_stagnation_pressure_ratio",
            "compressible.rayleigh_velocity_ratio",
        ],
        binding_functions: BINDING_FUNCTIONS,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RayleighInputKind {
    Mach,
    TemperatureRatio,
    PressureRatio,
    DensityRatio,
    StagnationTemperatureRatio,
    StagnationPressureRatio,
    VelocityRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RayleighOutputKind {
    Mach,
    TemperatureRatio,
    PressureRatio,
    DensityRatio,
    StagnationTemperatureRatio,
    StagnationPressureRatio,
    VelocityRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RayleighBranch {
    Subsonic,
    Supersonic,
}

impl RayleighBranch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Subsonic => "subsonic",
            Self::Supersonic => "supersonic",
        }
    }
}

pub fn parse_input_kind(raw: &str) -> Option<RayleighInputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(RayleighInputKind::Mach),
        "t_tstar" | "t/t*" | "temperature_ratio" => Some(RayleighInputKind::TemperatureRatio),
        "p_pstar" | "p/p*" | "pressure_ratio" => Some(RayleighInputKind::PressureRatio),
        "rho_rhostar" | "rho/rho*" | "density_ratio" => Some(RayleighInputKind::DensityRatio),
        "t0_t0star" | "t0/t0*" | "stagnation_temperature_ratio" => {
            Some(RayleighInputKind::StagnationTemperatureRatio)
        }
        "p0_p0star" | "p0/p0*" | "stagnation_pressure_ratio" => {
            Some(RayleighInputKind::StagnationPressureRatio)
        }
        "v_vstar" | "v/v*" | "velocity_ratio" => Some(RayleighInputKind::VelocityRatio),
        _ => None,
    }
}

pub fn parse_output_kind(raw: &str) -> Option<RayleighOutputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "mach" | "m" => Some(RayleighOutputKind::Mach),
        "t_tstar" | "t/t*" | "temperature_ratio" => Some(RayleighOutputKind::TemperatureRatio),
        "p_pstar" | "p/p*" | "pressure_ratio" => Some(RayleighOutputKind::PressureRatio),
        "rho_rhostar" | "rho/rho*" | "density_ratio" => Some(RayleighOutputKind::DensityRatio),
        "t0_t0star" | "t0/t0*" | "stagnation_temperature_ratio" => {
            Some(RayleighOutputKind::StagnationTemperatureRatio)
        }
        "p0_p0star" | "p0/p0*" | "stagnation_pressure_ratio" => {
            Some(RayleighOutputKind::StagnationPressureRatio)
        }
        "v_vstar" | "v/v*" | "velocity_ratio" => Some(RayleighOutputKind::VelocityRatio),
        _ => None,
    }
}

pub fn parse_branch(raw: &str) -> Option<RayleighBranch> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "subsonic" => Some(RayleighBranch::Subsonic),
        "supersonic" => Some(RayleighBranch::Supersonic),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct RayleighCalcRequest {
    pub gamma: f64,
    pub input_kind: RayleighInputKind,
    pub input_value: f64,
    pub target_kind: RayleighOutputKind,
    pub branch: Option<RayleighBranch>,
}

#[derive(Debug, Clone)]
pub struct RayleighCalcResponse {
    pub value_si: f64,
    pub pivot_mach: f64,
    pub path: Vec<CalcStep>,
    pub warnings: Vec<String>,
}

impl RayleighCalcResponse {
    pub fn path_text(&self) -> String {
        path_text(&self.path)
    }
}

#[derive(Debug, Error)]
pub enum RayleighCalcError {
    #[error("invalid gamma '{value}' (must be finite and > 1.0)")]
    InvalidGamma { value: f64 },
    #[error("invalid input value for '{kind}': {reason}")]
    InvalidInputDomain { kind: &'static str, reason: String },
    #[error("branch is required for input kind '{kind}' (supported: subsonic, supersonic)")]
    MissingBranch { kind: &'static str },
    #[error(transparent)]
    Equation(#[from] equations::EquationError),
}

pub type Result<T> = std::result::Result<T, RayleighCalcError>;

#[derive(Debug, Clone)]
pub struct RayleighCalculatorDevice {
    req: RayleighCalcRequest,
}

pub fn rayleigh_calc() -> RayleighCalculatorDevice {
    RayleighCalculatorDevice::new()
}

impl Default for RayleighCalculatorDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl RayleighCalculatorDevice {
    pub fn new() -> Self {
        Self {
            req: RayleighCalcRequest {
                gamma: 1.4,
                input_kind: RayleighInputKind::Mach,
                input_value: 1.0,
                target_kind: RayleighOutputKind::PressureRatio,
                branch: None,
            },
        }
    }

    pub fn gamma(mut self, gamma: f64) -> Self {
        self.req.gamma = gamma;
        self
    }

    pub fn input(mut self, kind: RayleighInputKind, value: f64) -> Self {
        self.req.input_kind = kind;
        self.req.input_value = value;
        self
    }

    pub fn target(mut self, kind: RayleighOutputKind) -> Self {
        self.req.target_kind = kind;
        self
    }

    pub fn branch(mut self, branch: RayleighBranch) -> Self {
        self.req.branch = Some(branch);
        self
    }

    pub fn solve(self) -> Result<RayleighCalcResponse> {
        calc(self.req)
    }
}
struct RayleighRuntime;

impl PivotCalcSpec for RayleighRuntime {
    type Request = RayleighCalcRequest;
    type Error = RayleighCalcError;

    fn validate_request(&self, req: &Self::Request) -> Result<()> {
        if !req.gamma.is_finite() || req.gamma <= 1.0 {
            return Err(RayleighCalcError::InvalidGamma { value: req.gamma });
        }
        if !req.input_value.is_finite() {
            return Err(RayleighCalcError::InvalidInputDomain {
                kind: input_kind_label(req.input_kind),
                reason: "must be finite".to_string(),
            });
        }
        match req.input_kind {
            RayleighInputKind::Mach if req.input_value <= 0.0 => {
                return Err(RayleighCalcError::InvalidInputDomain {
                    kind: "Mach",
                    reason: "must be > 0".to_string(),
                });
            }
            _ if req.input_value <= 0.0 => {
                return Err(RayleighCalcError::InvalidInputDomain {
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

        if matches!(req.input_kind, RayleighInputKind::Mach) {
            return Ok(input);
        }

        match req.input_kind {
            RayleighInputKind::TemperatureRatio => {
                let branch = req.branch.ok_or(RayleighCalcError::MissingBranch {
                    kind: input_kind_label(req.input_kind),
                })?;
                let solved = eq
                    .solve(compressible::rayleigh_temperature_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_t_tstar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_temperature_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("t_tstar".to_string(), input), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighInputKind::PressureRatio => {
                let solved = eq
                    .solve(compressible::rayleigh_pressure_ratio::equation())
                    .target_m()
                    .given_p_pstar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_pressure_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("p_pstar".to_string(), input), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighInputKind::DensityRatio => {
                let solved = eq
                    .solve(compressible::rayleigh_density_ratio::equation())
                    .target_m()
                    .given_rho_rhostar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_density_ratio".to_string(),
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
            RayleighInputKind::StagnationTemperatureRatio => {
                let branch = req.branch.ok_or(RayleighCalcError::MissingBranch {
                    kind: input_kind_label(req.input_kind),
                })?;
                let solved = eq
                    .solve(compressible::rayleigh_stagnation_temperature_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_t0_t0star(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_stagnation_temperature_ratio"
                        .to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![
                        ("t0_t0star".to_string(), input),
                        ("gamma".to_string(), gamma),
                    ],
                });
                Ok(solved.value_si)
            }
            RayleighInputKind::StagnationPressureRatio => {
                let branch = req.branch.ok_or(RayleighCalcError::MissingBranch {
                    kind: input_kind_label(req.input_kind),
                })?;
                let solved = eq
                    .solve(compressible::rayleigh_stagnation_pressure_ratio::equation())
                    .target_m()
                    .branch(branch.as_str())
                    .given_p0_p0star(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_stagnation_pressure_ratio".to_string(),
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
            RayleighInputKind::VelocityRatio => {
                let solved = eq
                    .solve(compressible::rayleigh_velocity_ratio::equation())
                    .target_m()
                    .given_v_vstar(input)
                    .given_gamma(gamma)
                    .result()?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_velocity_ratio".to_string(),
                    solved_for: "M".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("v_vstar".to_string(), input), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighInputKind::Mach => unreachable!(),
        }
    }

    fn validate_pivot(&self, pivot_value: f64) -> Result<()> {
        if pivot_value.is_finite() && pivot_value > 0.0 {
            Ok(())
        } else {
            Err(RayleighCalcError::InvalidInputDomain {
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
            RayleighOutputKind::Mach => Ok(mach),
            RayleighOutputKind::TemperatureRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_temperature_ratio::equation(),
                    "t_tstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_temperature_ratio".to_string(),
                    solved_for: "t_tstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighOutputKind::PressureRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_pressure_ratio::equation(),
                    "p_pstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_pressure_ratio".to_string(),
                    solved_for: "p_pstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighOutputKind::DensityRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_density_ratio::equation(),
                    "rho_rhostar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_density_ratio".to_string(),
                    solved_for: "rho_rhostar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighOutputKind::StagnationTemperatureRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_stagnation_temperature_ratio::equation(),
                    "t0_t0star",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_stagnation_temperature_ratio"
                        .to_string(),
                    solved_for: "t0_t0star".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighOutputKind::StagnationPressureRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_stagnation_pressure_ratio::equation(),
                    "p0_p0star",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_stagnation_pressure_ratio".to_string(),
                    solved_for: "p0_p0star".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
            RayleighOutputKind::VelocityRatio => {
                let solved = eq.solve_result(
                    compressible::rayleigh_velocity_ratio::equation(),
                    "v_vstar",
                    [("M", mach), ("gamma", gamma)],
                )?;
                path.push(CalcStep {
                    equation_path_id: "compressible.rayleigh_velocity_ratio".to_string(),
                    solved_for: "v_vstar".to_string(),
                    method: method_label(solved.method),
                    branch: solved.branch,
                    inputs_used: vec![("M".to_string(), mach), ("gamma".to_string(), gamma)],
                });
                Ok(solved.value_si)
            }
        }
    }
}

pub fn calc(req: RayleighCalcRequest) -> Result<RayleighCalcResponse> {
    let out = run_pivot_calculation(&RayleighRuntime, req)?;
    Ok(RayleighCalcResponse {
        value_si: out.value_si,
        pivot_mach: out.pivot_value,
        path: out.path,
        warnings: out.warnings,
    })
}

fn input_kind_label(kind: RayleighInputKind) -> &'static str {
    match kind {
        RayleighInputKind::Mach => "Mach",
        RayleighInputKind::TemperatureRatio => "TemperatureRatio",
        RayleighInputKind::PressureRatio => "PressureRatio",
        RayleighInputKind::DensityRatio => "DensityRatio",
        RayleighInputKind::StagnationTemperatureRatio => "StagnationTemperatureRatio",
        RayleighInputKind::StagnationPressureRatio => "StagnationPressureRatio",
        RayleighInputKind::VelocityRatio => "VelocityRatio",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use equations::{compressible, eq};

    #[test]
    fn m_to_t_tstar_works() {
        let out = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::Mach,
            input_value: 2.0,
            target_kind: RayleighOutputKind::TemperatureRatio,
            branch: None,
        })
        .expect("rayleigh calc");
        let expected = eq
            .solve(compressible::rayleigh_temperature_ratio::equation())
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
        let out = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::Mach,
            input_value: 2.0,
            target_kind: RayleighOutputKind::PressureRatio,
            branch: None,
        })
        .expect("rayleigh calc");
        assert!((out.value_si - 0.36363636363636365).abs() < 1e-10);
    }

    #[test]
    fn p0_p0star_to_m_supports_branches() {
        let input = 1.114052503180089;
        let sub = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::StagnationPressureRatio,
            input_value: input,
            target_kind: RayleighOutputKind::Mach,
            branch: Some(RayleighBranch::Subsonic),
        })
        .expect("sub branch");
        let sup = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::StagnationPressureRatio,
            input_value: input,
            target_kind: RayleighOutputKind::Mach,
            branch: Some(RayleighBranch::Supersonic),
        })
        .expect("sup branch");

        assert!(sub.value_si < 1.0);
        assert!(sup.value_si > 1.0);
    }

    #[test]
    fn missing_branch_errors_on_ambiguous_inverse_paths() {
        let err = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::TemperatureRatio,
            input_value: 0.7901234567901233,
            target_kind: RayleighOutputKind::Mach,
            branch: None,
        })
        .expect_err("missing branch should error");
        assert!(matches!(err, RayleighCalcError::MissingBranch { .. }));
    }

    #[test]
    fn path_contains_registry_equation_ids() {
        let out = calc(RayleighCalcRequest {
            gamma: 1.4,
            input_kind: RayleighInputKind::StagnationTemperatureRatio,
            input_value: 0.7933884297520661,
            target_kind: RayleighOutputKind::VelocityRatio,
            branch: Some(RayleighBranch::Supersonic),
        })
        .expect("rayleigh calc");

        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.rayleigh_stagnation_temperature_ratio")
        );
        assert!(
            out.path
                .iter()
                .any(|s| s.equation_path_id == "compressible.rayleigh_velocity_ratio")
        );
    }
}
