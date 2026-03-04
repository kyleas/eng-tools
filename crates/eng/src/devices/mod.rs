pub mod framework;
pub mod isentropic;
pub mod normal_shock;
pub mod pipe_loss;

pub use framework::{
    CalcStep as FrameworkCalcStep, CalculatorDeviceSpec, CalculatorKindSpec, PivotCalcResponse,
};
pub use isentropic::{
    CalcStep, IsentropicBranch, IsentropicCalcError, IsentropicCalcRequest, IsentropicCalcResponse,
    IsentropicCalculatorDevice, IsentropicInputKind, IsentropicOutputKind,
    calc as isentropic_calc_from_request, isentropic_calc,
};
pub use normal_shock::{
    NormalShockCalcError, NormalShockCalcRequest, NormalShockCalcResponse, NormalShockCalcStep,
    NormalShockCalculatorDevice, NormalShockInputKind, NormalShockOutputKind,
    calc as normal_shock_calc_from_request, normal_shock_calc,
};
pub use pipe_loss::{PipeFrictionModel, PipeLossDevice, PipeLossError, PipeLossResult, pipe_loss};

#[derive(Debug, Clone)]
pub struct DeviceDocsEntry {
    pub key: String,
    pub name: String,
    pub summary: String,
    pub supported_modes: Vec<String>,
    pub outputs: Vec<String>,
    pub route: String,
}

pub fn docs_entries() -> Vec<DeviceDocsEntry> {
    vec![
        DeviceDocsEntry {
            key: "pipe_loss".to_string(),
            name: "Pipe Pressure Drop".to_string(),
            summary: "Composes Reynolds + friction model + Darcy-Weisbach for pipe pressure loss."
                .to_string(),
            supported_modes: vec!["Fixed friction factor".to_string(), "Colebrook".to_string()],
            outputs: vec![
                "delta_p (Pa)".to_string(),
                "friction_factor".to_string(),
                "reynolds_number".to_string(),
            ],
            route: "devices/pipe_loss.md".to_string(),
        },
        DeviceDocsEntry {
            key: "isentropic_calc".to_string(),
            name: isentropic::DEVICE_SPEC.name.to_string(),
            summary: isentropic::DEVICE_SPEC.summary.to_string(),
            supported_modes: vec![format!(
                "Input kinds: {}",
                isentropic::supported_input_kinds_text()
            )],
            outputs: vec![
                "value_si".to_string(),
                "pivot_mach".to_string(),
                "path diagnostics".to_string(),
            ],
            route: isentropic::DEVICE_SPEC.route.to_string(),
        },
        DeviceDocsEntry {
            key: "normal_shock_calc".to_string(),
            name: "Normal Shock Calculator".to_string(),
            summary:
                "Calculator-style compressible device: solve normal-shock input kinds to target kinds through deterministic M1 pivot orchestration."
                    .to_string(),
            supported_modes: vec![
                "Input kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01".to_string(),
                "Target kinds: M1, M2, p2/p1, rho2/rho1, T2/T1, p02/p01".to_string(),
            ],
            outputs: vec![
                "value_si".to_string(),
                "pivot_m1".to_string(),
                "path diagnostics".to_string(),
            ],
            route: "devices/normal_shock_calc.md".to_string(),
        },
    ]
}
