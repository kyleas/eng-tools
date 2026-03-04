use equations::{SolveMethod, compressible, eq};
use thiserror::Error;

use super::{DeviceBindingArgSpec, DeviceBindingFunctionSpec, DeviceGenerationSpec};

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

const MAIN_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_kind",
        description: "Input kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)",
    },
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Input value",
    },
    DeviceBindingArgSpec {
        name: "target_kind",
        description: "Target kind (m1, m2, p2_p1, rho2_rho1, t2_t1, p02_p01)",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
];

const M1_GAMMA_ARGS: &[DeviceBindingArgSpec] = &[
    DeviceBindingArgSpec {
        name: "input_value",
        description: "Upstream Mach number M1",
    },
    DeviceBindingArgSpec {
        name: "gamma",
        description: "Specific heat ratio",
    },
];

const FIXED_M1_TO_M2: &[(&str, &str)] = &[("input_kind", "m1"), ("target_kind", "m2")];
const FIXED_M1_TO_P2P1: &[(&str, &str)] = &[("input_kind", "m1"), ("target_kind", "p2_p1")];
const FIXED_M1_TO_RHO2RHO1: &[(&str, &str)] = &[("input_kind", "m1"), ("target_kind", "rho2_rho1")];
const FIXED_M1_TO_T2T1: &[(&str, &str)] = &[("input_kind", "m1"), ("target_kind", "t2_t1")];
const FIXED_M1_TO_P02P01: &[(&str, &str)] = &[("input_kind", "m1"), ("target_kind", "p02_p01")];

const BINDING_FUNCTIONS: &[DeviceBindingFunctionSpec] = &[
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc",
        python_name: "normal_shock_calc",
        excel_name: "ENG_NORMAL_SHOCK",
        op: "device.normal_shock_calc.value",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Normal shock calculator: input kind -> target kind through M1 pivot",
        rust_example: "eng::devices::normal_shock_calc().solve()?",
        python_example: "engpy.devices.normal_shock_calc(\"m1\", 2.0, \"p2_p1\", 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK(\"m1\",2.0,\"p2_p1\",1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK(\"m1\",2.0,\"p2_p1\",1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.pivot",
        python_name: "normal_shock_pivot_m1",
        excel_name: "ENG_NORMAL_SHOCK_PIVOT_M1",
        op: "device.normal_shock_calc.pivot_m1",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "f64",
        help: "Normal shock calculator helper: return resolved pivot M1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_pivot_m1(\"p2_p1\", 4.5, \"m2\", 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_PIVOT_M1(\"p2_p1\",4.5,\"m2\",1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_PIVOT_M1(\"p2_p1\",4.5,\"m2\",1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.path_text",
        python_name: "normal_shock_path_text",
        excel_name: "ENG_NORMAL_SHOCK_PATH_TEXT",
        op: "device.normal_shock_calc.path_text",
        fixed_args: &[],
        args: MAIN_ARGS,
        returns: "str",
        help: "Normal shock calculator helper: compact step trace text",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_path_text(\"p02_p01\", 0.72, \"m2\", 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_PATH_TEXT(\"p02_p01\",0.72,\"m2\",1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_PATH_TEXT(\"p02_p01\",0.72,\"m2\",1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.from_m1.to_m2",
        python_name: "normal_shock_from_m1_to_m2",
        excel_name: "ENG_NORMAL_SHOCK_FROM_M1_TO_M2",
        op: "device.normal_shock_calc.value",
        fixed_args: FIXED_M1_TO_M2,
        args: M1_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience normal-shock path: M1 -> M2",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_from_m1_to_m2(2.0, 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.from_m1.to_p2_p1",
        python_name: "normal_shock_from_m1_to_p2_p1",
        excel_name: "ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1",
        op: "device.normal_shock_calc.value",
        fixed_args: FIXED_M1_TO_P2P1,
        args: M1_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience normal-shock path: M1 -> p2/p1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_from_m1_to_p2_p1(2.0, 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.from_m1.to_rho2_rho1",
        python_name: "normal_shock_from_m1_to_rho2_rho1",
        excel_name: "ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1",
        op: "device.normal_shock_calc.value",
        fixed_args: FIXED_M1_TO_RHO2RHO1,
        args: M1_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience normal-shock path: M1 -> rho2/rho1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_from_m1_to_rho2_rho1(2.0, 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.from_m1.to_t2_t1",
        python_name: "normal_shock_from_m1_to_t2_t1",
        excel_name: "ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1",
        op: "device.normal_shock_calc.value",
        fixed_args: FIXED_M1_TO_T2T1,
        args: M1_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience normal-shock path: M1 -> T2/T1",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_from_m1_to_t2_t1(2.0, 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)",
    },
    DeviceBindingFunctionSpec {
        id: "device.normal_shock_calc.from_m1.to_p02_p01",
        python_name: "normal_shock_from_m1_to_p02_p01",
        excel_name: "ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01",
        op: "device.normal_shock_calc.value",
        fixed_args: FIXED_M1_TO_P02P01,
        args: M1_GAMMA_ARGS,
        returns: "f64",
        help: "Convenience normal-shock path: M1 -> p02/p01",
        rust_example: "eng::invoke::process_invoke_json(\"...\")",
        python_example: "engpy.devices.normal_shock_from_m1_to_p02_p01(2.0, 1.4)",
        xloil_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)",
        pyxll_example: "=ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)",
    },
];

const BINDINGS_MD: &str = "## Bindings\n\n### Python\n```python\nengpy.devices.normal_shock_calc(\"m1\", 2.0, \"p2_p1\", 1.4)\nengpy.devices.normal_shock_from_m1_to_m2(2.0, 1.4)\nengpy.devices.normal_shock_pivot_m1(\"p2_p1\", 4.5, \"m2\", 1.4)\nengpy.devices.normal_shock_path_text(\"p02_p01\", 0.7208738615, \"m2\", 1.4)\n```\n\n### Excel\n```excel\n=ENG_NORMAL_SHOCK(\"m1\",2.0,\"p2_p1\",1.4)\n=ENG_NORMAL_SHOCK_FROM_M1_TO_M2(2.0,1.4)\n=ENG_NORMAL_SHOCK_FROM_M1_TO_P2_P1(2.0,1.4)\n=ENG_NORMAL_SHOCK_FROM_M1_TO_RHO2_RHO1(2.0,1.4)\n=ENG_NORMAL_SHOCK_FROM_M1_TO_T2_T1(2.0,1.4)\n=ENG_NORMAL_SHOCK_FROM_M1_TO_P02_P01(2.0,1.4)\n=ENG_NORMAL_SHOCK_PIVOT_M1(\"p2_p1\",4.5,\"m2\",1.4)\n=ENG_NORMAL_SHOCK_PATH_TEXT(\"p02_p01\",0.7208738615,\"m2\",1.4)\n```\n\n**Excel arguments**\n- `value_kind_in`: `m1`, `m2`, `p2_p1`, `rho2_rho1`, `t2_t1`, `p02_p01`\n- `value_in`: input value\n- `target_kind_out`: same enum family as input kind\n- `gamma`: specific heat ratio\n";

const OVERVIEW_MD: &str = "## Overview\n\nA calculator-style compressible device that resolves upstream Mach (`M1`) from one normal-shock input kind, then computes any supported target kind.\n\n### Supported input kinds\n- `m1`\n- `m2`\n- `p2_p1`\n- `rho2_rho1`\n- `t2_t1`\n- `p02_p01`\n\n### Supported target kinds\n- `m1`\n- `m2`\n- `p2_p1`\n- `rho2_rho1`\n- `t2_t1`\n- `p02_p01`\n\n### Rust\n```rust\nuse eng::devices::{normal_shock_calc, NormalShockInputKind, NormalShockOutputKind};\nlet out = normal_shock_calc()\n    .gamma(1.4)\n    .input(NormalShockInputKind::PressureRatio, 4.5)\n    .target(NormalShockOutputKind::TemperatureRatio)\n    .solve()?;\nprintln!(\"M1={}, T2/T1={}\", out.pivot_m1, out.value_si);\n```\n";

pub fn generation_spec() -> DeviceGenerationSpec {
    DeviceGenerationSpec {
        key: "normal_shock_calc",
        name: "Normal Shock Calculator",
        summary: "Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration.",
        supported_modes: &[
            "Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01",
            "Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01",
        ],
        outputs: &["value_si", "pivot_m1", "path diagnostics"],
        route: "devices/normal_shock_calc.md",
        binding_markdown: BINDINGS_MD,
        overview_markdown: OVERVIEW_MD,
        equation_dependencies: &[
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
