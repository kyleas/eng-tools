use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VariableDef {
    pub name: String,
    pub symbol: Option<String>,
    pub dimension: String,
    pub default_unit: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub constraints: VariableConstraint,
    pub description: Option<String>,
    pub resolver: Option<VariableResolver>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct VariableConstraint {
    #[serde(default)]
    pub positive: bool,
    #[serde(default)]
    pub nonzero: bool,
    #[serde(default)]
    pub integer: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VariableResolver {
    pub source: String,
    pub kind: ResolverKind,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResolverKind {
    FluidProperty,
    MaterialProperty,
}

impl fmt::Display for ResolverKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverKind::FluidProperty => write!(f, "fluid_property"),
            ResolverKind::MaterialProperty => write!(f, "material_property"),
        }
    }
}
