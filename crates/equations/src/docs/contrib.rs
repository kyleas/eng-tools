use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    constants::{self, EngineeringConstant},
    docs::{
        page_model::{EquationPageModel, build_page_models},
        presentation::{LibraryPresentation, build_library_presentation},
        search_index::{SearchEntry, build_search_index},
    },
    equation_families::{self, EquationFamilyDef},
    model::EquationDef,
};

#[derive(Debug, Clone, Serialize)]
pub struct NavigationNode {
    pub category: String,
    pub subcategories: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct EquationDocsContribution {
    pub search_index: Vec<SearchEntry>,
    pub page_models: Vec<EquationPageModel>,
    pub navigation: Vec<NavigationNode>,
    pub examples_index: Vec<serde_json::Value>,
    pub constants: Vec<EngineeringConstant>,
    pub families: Vec<EquationFamilyDef>,
    pub library: LibraryPresentation,
}

pub fn build_equation_docs_contribution(equations: &[EquationDef]) -> EquationDocsContribution {
    let search_index = build_search_index(equations);
    let page_models = build_page_models(equations);
    let navigation = build_navigation(equations);
    let examples_index = build_examples_index(&page_models);
    let mut constants = constants::all().to_vec();
    constants.sort_by(|a, b| a.key.cmp(b.key));
    let library = build_library_presentation(equations);
    let families = equation_families::load_default_validated(equations).unwrap_or_default();

    EquationDocsContribution {
        search_index,
        page_models,
        navigation,
        examples_index,
        constants,
        families,
        library,
    }
}

fn build_navigation(equations: &[EquationDef]) -> Vec<NavigationNode> {
    let mut by_category: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    for eq in equations {
        let category = eq.taxonomy.category.clone();
        let subcat = eq
            .taxonomy
            .subcategories
            .first()
            .cloned()
            .unwrap_or_else(|| "_root".to_string());
        by_category
            .entry(category)
            .or_default()
            .entry(subcat)
            .or_default()
            .push(eq.effective_slug().to_string());
    }
    by_category
        .into_iter()
        .map(|(category, mut subcategories)| {
            for values in subcategories.values_mut() {
                values.sort();
            }
            NavigationNode {
                category,
                subcategories,
            }
        })
        .collect()
}

fn build_examples_index(pages: &[EquationPageModel]) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for p in pages {
        for ex in &p.examples {
            out.push(serde_json::json!({
                "path_id": p.path_id,
                "label": ex.label,
                "style": ex.style,
                "code": ex.code
            }));
        }
    }
    out
}
