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
    "format.value",
    "meta.get",
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
                    "equation.variables" => {
                        "Read equation variable keys".to_string()
                    }
                    "equation.name" => "Read equation display name".to_string(),
                    "equation.description" => "Read equation description".to_string(),
                    "equation.family" => {
                        "Read parent equation family metadata when available".to_string()
                    }
                    "format.value" => {
                        "Convert/format SI values into requested engineering units".to_string()
                    }
                    "meta.get" => {
                        "Read scalar/list metadata from equation/device/fluid/material/constant"
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
