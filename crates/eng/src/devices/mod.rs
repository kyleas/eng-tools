pub mod fanno_flow;
pub mod framework;
pub mod isentropic;
pub mod normal_shock;
pub mod oblique_shock;
pub mod pipe_loss;

pub use fanno_flow::{
    FannoFlowBranch, FannoFlowCalcError, FannoFlowCalcRequest, FannoFlowCalcResponse,
    FannoFlowCalculatorDevice, FannoFlowInputKind, FannoFlowOutputKind,
    calc as fanno_flow_calc_from_request, fanno_flow_calc,
};
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
pub use oblique_shock::{
    ObliqueShockBranch, ObliqueShockCalcError, ObliqueShockCalcRequest, ObliqueShockCalcResponse,
    ObliqueShockCalcStep, ObliqueShockCalculatorDevice, ObliqueShockInputKind,
    ObliqueShockOutputKind, calc as oblique_shock_calc_from_request, oblique_shock_calc,
};
pub use pipe_loss::{PipeFrictionModel, PipeLossDevice, PipeLossError, PipeLossResult, pipe_loss};

#[derive(Debug, Clone)]
pub struct DeviceBindingArgSpec {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct DeviceBindingFunctionSpec {
    pub id: &'static str,
    pub python_name: &'static str,
    pub excel_name: &'static str,
    pub op: &'static str,
    pub fixed_args: &'static [(&'static str, &'static str)],
    pub args: &'static [DeviceBindingArgSpec],
    pub returns: &'static str,
    pub help: &'static str,
    pub rust_example: &'static str,
    pub python_example: &'static str,
    pub xloil_example: &'static str,
    pub pyxll_example: &'static str,
}

#[derive(Debug, Clone)]
pub struct DeviceGenerationSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub supported_modes: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub route: &'static str,
    pub binding_markdown: &'static str,
    pub overview_markdown: &'static str,
    pub equation_dependencies: &'static [&'static str],
    pub binding_functions: &'static [DeviceBindingFunctionSpec],
}

pub fn generation_specs() -> Vec<DeviceGenerationSpec> {
    vec![
        pipe_loss::generation_spec(),
        isentropic::generation_spec(),
        normal_shock::generation_spec(),
        oblique_shock::generation_spec(),
        fanno_flow::generation_spec(),
    ]
}

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
    generation_specs()
        .into_iter()
        .map(|s| DeviceDocsEntry {
            key: s.key.to_string(),
            name: s.name.to_string(),
            summary: s.summary.to_string(),
            supported_modes: s.supported_modes.iter().map(|v| v.to_string()).collect(),
            outputs: s.outputs.iter().map(|v| v.to_string()).collect(),
            route: s.route.to_string(),
        })
        .collect()
}
