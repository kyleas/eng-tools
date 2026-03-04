pub mod pipe_loss;

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
    vec![DeviceDocsEntry {
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
    }]
}
