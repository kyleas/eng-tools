use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const INVOKE_PROTOCOL_VERSION: &str = "eng-invoke.v1";
pub const INVOKE_SUPPORTED_OPS: &[&str] = &[
    "equation.solve",
    "equation.meta",
    "equation.ascii",
    "equation.default_unit",
    "equation.unicode",
    "equation.latex",
    "equation.targets",
    "equation.variables",
    "equation.name",
    "equation.description",
    "equation.family",
    "equation.targets.text",
    "equation.targets.table",
    "equation.target.count",
    "equation.branches.text",
    "equation.branches.table",
    "equation.variables.text",
    "equation.variables.table",
    "equation.variable.count",
    "format.value",
    "meta.get",
    "fluid.properties.text",
    "fluid.properties.table",
    "fluid.property.count",
    "material.properties.text",
    "material.properties.table",
    "material.property.count",
    "device.modes.text",
    "device.mode.count",
    "device.isentropic_calc",
    "device.isentropic_calc.value",
    "device.isentropic_calc.pivot_mach",
    "device.isentropic_calc.path_text",
    "device.normal_shock_calc",
    "device.normal_shock_calc.value",
    "device.normal_shock_calc.pivot_m1",
    "device.normal_shock_calc.path_text",
    "device.oblique_shock_calc",
    "device.oblique_shock_calc.value",
    "device.oblique_shock_calc.path_text",
    "device.fanno_flow_calc",
    "device.fanno_flow_calc.value",
    "device.fanno_flow_calc.pivot_mach",
    "device.fanno_flow_calc.path_text",
    "device.rayleigh_calc",
    "device.rayleigh_calc.value",
    "device.rayleigh_calc.pivot_mach",
    "device.rayleigh_calc.path_text",
    "device.pipe_loss.solve_delta_p",
    "fluid.prop",
    "material.prop",
    "constant.get",
];

fn default_protocol_version() -> String {
    INVOKE_PROTOCOL_VERSION.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeRequest {
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    pub args: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeResponse {
    pub protocol_version: String,
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<InvokeError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
}

impl InvokeResponse {
    pub fn ok(op: impl Into<String>, request_id: Option<String>, value: Value) -> Self {
        Self {
            protocol_version: INVOKE_PROTOCOL_VERSION.to_string(),
            op: op.into(),
            request_id,
            ok: true,
            value: Some(value),
            error: None,
        }
    }

    pub fn err(
        op: impl Into<String>,
        request_id: Option<String>,
        code: impl Into<String>,
        message: impl Into<String>,
        field: Option<&str>,
        detail: Option<Value>,
    ) -> Self {
        Self {
            protocol_version: INVOKE_PROTOCOL_VERSION.to_string(),
            op: op.into(),
            request_id,
            ok: false,
            value: None,
            error: Some(InvokeError {
                code: code.into(),
                message: message.into(),
                field: field.map(ToString::to_string),
                detail,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeProtocolSpec {
    pub protocol_version: String,
    pub request_shape: InvokeShapeDoc,
    pub success_response_shape: InvokeShapeDoc,
    pub error_response_shape: InvokeShapeDoc,
    pub operations: Vec<InvokeOperationDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeShapeDoc {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeOperationDoc {
    pub op: String,
    pub description: String,
}

pub fn invoke_protocol_spec() -> InvokeProtocolSpec {
    InvokeProtocolSpec {
        protocol_version: INVOKE_PROTOCOL_VERSION.to_string(),
        request_shape: InvokeShapeDoc {
            required_fields: vec![
                "protocol_version".to_string(),
                "op".to_string(),
                "args".to_string(),
            ],
            optional_fields: vec!["request_id".to_string()],
        },
        success_response_shape: InvokeShapeDoc {
            required_fields: vec![
                "protocol_version".to_string(),
                "op".to_string(),
                "ok".to_string(),
                "value".to_string(),
            ],
            optional_fields: vec!["request_id".to_string(), "error".to_string()],
        },
        error_response_shape: InvokeShapeDoc {
            required_fields: vec![
                "protocol_version".to_string(),
                "op".to_string(),
                "ok".to_string(),
                "error.code".to_string(),
                "error.message".to_string(),
            ],
            optional_fields: vec![
                "request_id".to_string(),
                "value".to_string(),
                "error.field".to_string(),
                "error.detail".to_string(),
            ],
        },
        operations: INVOKE_SUPPORTED_OPS
            .iter()
            .map(|op| InvokeOperationDoc {
                op: (*op).to_string(),
                description: match *op {
                    "equation.solve" => "Solve an equation target with named inputs".to_string(),
                    "equation.meta" => {
                        "Read structured equation metadata (display, variables, units, targets)"
                            .to_string()
                    }
                    "equation.ascii" => {
                        "Read ASCII equation display form for one equation".to_string()
                    }
                    "equation.default_unit" => {
                        "Read one variable's canonical default unit for an equation".to_string()
                    }
                    "equation.unicode" => {
                        "Read Unicode equation display form for one equation".to_string()
                    }
                    "equation.latex" => {
                        "Read LaTeX equation display form for one equation".to_string()
                    }
                    "equation.targets" => {
                        "Read solve targets supported by one equation".to_string()
                    }
                    "equation.variables" => "Read equation variable keys".to_string(),
                    "equation.name" => "Read equation display name".to_string(),
                    "equation.description" => "Read equation description".to_string(),
                    "equation.family" => {
                        "Read parent equation family metadata when available".to_string()
                    }
                    "equation.targets.text" => {
                        "Read equation targets as one deterministic delimited string".to_string()
                    }
                    "equation.targets.table" => {
                        "Read equation targets as 2-column table rows".to_string()
                    }
                    "equation.target.count" => "Read equation target count".to_string(),
                    "equation.branches.text" => {
                        "Read equation branches as one deterministic delimited string".to_string()
                    }
                    "equation.branches.table" => {
                        "Read equation branches as 2-column table rows [branch, is_preferred]"
                            .to_string()
                    }
                    "equation.variables.text" => {
                        "Read equation variables as one deterministic delimited string".to_string()
                    }
                    "equation.variables.table" => {
                        "Read equation variables as 2-column table rows".to_string()
                    }
                    "equation.variable.count" => "Read equation variable count".to_string(),
                    "format.value" => {
                        "Convert values between explicit input/output units".to_string()
                    }
                    "meta.get" => {
                        "Read scalar/list metadata from equation/device/fluid/material/constant"
                            .to_string()
                    }
                    "fluid.properties.text" => {
                        "Read fluid properties as one deterministic delimited string".to_string()
                    }
                    "fluid.properties.table" => {
                        "Read fluid properties as 2-column table rows".to_string()
                    }
                    "fluid.property.count" => "Read fluid property count".to_string(),
                    "material.properties.text" => {
                        "Read material properties as one deterministic delimited string".to_string()
                    }
                    "material.properties.table" => {
                        "Read material properties as 2-column table rows".to_string()
                    }
                    "material.property.count" => "Read material property count".to_string(),
                    "device.modes.text" => {
                        "Read device supported modes as one deterministic delimited string"
                            .to_string()
                    }
                    "device.mode.count" => "Read device supported mode count".to_string(),
                    "device.isentropic_calc" => {
                        "Run isentropic calculator device and return value+pivot+path diagnostics"
                            .to_string()
                    }
                    "device.isentropic_calc.value" => {
                        "Run isentropic calculator device and return scalar output value"
                            .to_string()
                    }
                    "device.isentropic_calc.pivot_mach" => {
                        "Run isentropic calculator device and return resolved pivot Mach"
                            .to_string()
                    }
                    "device.isentropic_calc.path_text" => {
                        "Run isentropic calculator device and return compact path trace text"
                            .to_string()
                    }
                    "device.normal_shock_calc" => {
                        "Run normal-shock calculator device and return value+pivot+path diagnostics"
                            .to_string()
                    }
                    "device.normal_shock_calc.value" => {
                        "Run normal-shock calculator device and return scalar output value"
                            .to_string()
                    }
                    "device.normal_shock_calc.pivot_m1" => {
                        "Run normal-shock calculator device and return resolved pivot M1"
                            .to_string()
                    }
                    "device.normal_shock_calc.path_text" => {
                        "Run normal-shock calculator device and return compact path trace text"
                            .to_string()
                    }
                    "device.oblique_shock_calc" => {
                        "Run oblique-shock calculator device and return value+resolved geometry+path diagnostics".to_string()
                    }
                    "device.oblique_shock_calc.value" => {
                        "Run oblique-shock calculator device and return scalar output value"
                            .to_string()
                    }
                    "device.oblique_shock_calc.path_text" => {
                        "Run oblique-shock calculator device and return compact path trace text"
                            .to_string()
                    }
                    "device.fanno_flow_calc" => {
                        "Run Fanno-flow calculator device and return value+pivot+path diagnostics"
                            .to_string()
                    }
                    "device.fanno_flow_calc.value" => {
                        "Run Fanno-flow calculator device and return scalar output value"
                            .to_string()
                    }
                    "device.fanno_flow_calc.pivot_mach" => {
                        "Run Fanno-flow calculator device and return resolved pivot Mach"
                            .to_string()
                    }
                    "device.fanno_flow_calc.path_text" => {
                        "Run Fanno-flow calculator device and return compact path trace text"
                            .to_string()
                    }
                    "device.rayleigh_calc" => {
                        "Run Rayleigh-flow calculator device and return value+pivot+path diagnostics"
                            .to_string()
                    }
                    "device.rayleigh_calc.value" => {
                        "Run Rayleigh-flow calculator device and return scalar output value"
                            .to_string()
                    }
                    "device.rayleigh_calc.pivot_mach" => {
                        "Run Rayleigh-flow calculator device and return resolved pivot Mach"
                            .to_string()
                    }
                    "device.rayleigh_calc.path_text" => {
                        "Run Rayleigh-flow calculator device and return compact path trace text"
                            .to_string()
                    }
                    "device.pipe_loss.solve_delta_p" => {
                        "Solve pipe-loss device pressure drop".to_string()
                    }
                    "fluid.prop" => {
                        "Resolve one fluid property from a fluid state pair".to_string()
                    }
                    "material.prop" => {
                        "Resolve one material property at a given temperature".to_string()
                    }
                    "constant.get" => "Read one engineering constant value".to_string(),
                    _ => "Operation".to_string(),
                },
            })
            .collect(),
    }
}
