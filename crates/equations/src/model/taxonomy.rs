use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Taxonomy {
    pub category: String,
    #[serde(default)]
    pub subcategories: Vec<String>,
}
