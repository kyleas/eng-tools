use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DiagramMetadata {
    pub template: String,
    #[serde(default)]
    pub labels: Vec<DiagramLabel>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DiagramLabel {
    pub variable: String,
    pub label: String,
}
