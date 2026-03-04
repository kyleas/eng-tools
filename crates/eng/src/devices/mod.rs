pub mod pipe_loss;
pub mod isentropic;

pub use pipe_loss::{PipeFrictionModel, PipeLossDevice, PipeLossError, PipeLossResult, pipe_loss};
pub use isentropic::{
    CalcStep, IsentropicBranch, IsentropicCalcError, IsentropicCalcRequest, IsentropicCalcResponse,
    IsentropicCalculatorDevice, IsentropicInputKind, IsentropicOutputKind, calc as isentropic_calc_from_request,
    isentropic_calc,
};

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
            name: "Isentropic Calculator".to_string(),
            summary:
                "Calculator-style compressible device: solve any supported isentropic input to any supported output through Mach pivot orchestration."
                    .to_string(),
            supported_modes: vec![
                "Input kinds: Mach, MachAngle, p/p0, T/T0, rho/rho0, A/A*".to_string(),
                "Branch-aware inversion for A/A*".to_string(),
            ],
            outputs: vec![
                "value_si".to_string(),
                "pivot_mach".to_string(),
                "path diagnostics".to_string(),
            ],
            route: "devices/isentropic_calc.md".to_string(),
        },
    ]
}
