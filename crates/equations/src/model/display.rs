use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DisplayMetadata {
    pub latex: String,
    pub unicode: Option<String>,
    pub ascii: Option<String>,
    pub description: Option<String>,
}
