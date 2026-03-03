use std::collections::HashMap;

use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::model::{
    BranchDef, DiagramMetadata, DisplayMetadata, Reference, RelationDef, SolveConfig, Taxonomy,
    TestCase, TestsConfig, VariableDef,
};

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct EquationDef {
    pub key: String,
    #[serde(default)]
    pub slug: Option<String>,
    pub taxonomy: Taxonomy,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub name: String,
    pub display: DisplayMetadata,
    pub variables: IndexMap<String, VariableDef>,
    #[schemars(description = "Shorthand for relation.residual when form is residual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residual: Option<String>,
    #[serde(default = "default_relation")]
    pub relation: RelationDef,
    pub solve: SolveConfig,
    #[serde(default)]
    pub branches: Vec<BranchDef>,
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub references: Vec<Reference>,
    pub diagram: Option<DiagramMetadata>,
    #[schemars(with = "TestsSchema")]
    pub tests: TestsConfig,
}

impl EquationDef {
    pub fn effective_slug(&self) -> &str {
        self.slug
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(self.key.as_str())
    }

    /// Solve this equation with automatic method selection.
    pub fn solve<I, K>(
        &self,
        target: &str,
        knowns_si: I,
    ) -> crate::error::Result<crate::solve_engine::SolveResponse>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        self.solve_with_method(
            target,
            knowns_si,
            crate::solve_engine::SolveMethod::Auto,
            None,
        )
    }

    /// Solve this equation with explicit method and optional branch.
    pub fn solve_with_method<I, K>(
        &self,
        target: &str,
        knowns_si: I,
        method: crate::solve_engine::SolveMethod,
        branch: Option<&str>,
    ) -> crate::error::Result<crate::solve_engine::SolveResponse>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        let knowns_si: HashMap<String, f64> =
            knowns_si.into_iter().map(|(k, v)| (k.into(), v)).collect();
        crate::solve_engine::solve_equation(
            self,
            crate::solve_engine::SolveRequest {
                target: target.to_string(),
                knowns_si,
                method,
                branch: branch.map(str::to_string),
            },
        )
    }

    /// Solve and return only the SI value.
    pub fn solve_value<I, K>(&self, target: &str, knowns_si: I) -> crate::error::Result<f64>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        Ok(self.solve(target, knowns_si)?.value_si)
    }
}

fn default_relation() -> RelationDef {
    RelationDef {
        form: crate::model::relation::RelationForm::Residual,
        residual: String::new(),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, JsonSchema)]
#[schemars(untagged)]
enum TestsSchema {
    Config(TestsConfig),
    Cases(Vec<TestCase>),
}

#[derive(Debug, Clone, Deserialize)]
struct EquationDefWire {
    key: String,
    #[serde(default)]
    slug: Option<String>,
    taxonomy: Taxonomy,
    #[serde(default)]
    aliases: Vec<String>,
    name: String,
    display: DisplayMetadata,
    variables: IndexMap<String, VariableDef>,
    relation: Option<RelationDef>,
    residual: Option<String>,
    solve: SolveConfig,
    #[serde(default)]
    branches: Vec<BranchDef>,
    assumptions: Vec<String>,
    #[serde(default)]
    references: Vec<Reference>,
    diagram: Option<DiagramMetadata>,
    tests: TestsConfig,
}

impl<'de> Deserialize<'de> for EquationDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EquationDefWire::deserialize(deserializer)?;
        let relation = match (&wire.relation, wire.residual.as_deref()) {
            (Some(relation), None) => relation.clone(),
            (None, Some(residual)) => RelationDef {
                form: crate::model::relation::RelationForm::Residual,
                residual: residual.to_string(),
            },
            (Some(relation), Some(residual)) => {
                if relation.residual.trim() != residual.trim() {
                    return Err(de::Error::custom(
                        "equation defines both 'relation' and 'residual' with different values",
                    ));
                }
                relation.clone()
            }
            (None, None) => {
                return Err(de::Error::custom(
                    "equation must define either 'relation' or 'residual'",
                ));
            }
        };

        Ok(Self {
            key: wire.key,
            slug: wire.slug,
            taxonomy: wire.taxonomy,
            aliases: wire.aliases,
            name: wire.name,
            display: wire.display,
            variables: wire.variables,
            residual: None,
            relation,
            solve: wire.solve,
            branches: wire.branches,
            assumptions: wire.assumptions,
            references: wire.references,
            diagram: wire.diagram,
            tests: wire.tests,
        })
    }
}
