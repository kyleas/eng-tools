use serde::Serialize;

use crate::{
    model::EquationDef,
    normalize::{is_numerically_supported, resolved_display},
    registry::ids::derive_path_id,
};

#[derive(Debug, Clone, Serialize)]
pub struct SearchEntry {
    pub key: String,
    pub path_id: String,
    pub name: String,
    pub category: String,
    pub subcategories: Vec<String>,
    pub description: String,
    pub aliases: Vec<String>,
    pub solve_targets: Vec<String>,
}

pub fn build_search_index(equations: &[EquationDef]) -> Vec<SearchEntry> {
    let mut entries: Vec<SearchEntry> = equations
        .iter()
        .map(|eq| {
            let disp = resolved_display(eq);
            SearchEntry {
                key: eq.key.clone(),
                path_id: derive_path_id(eq),
                name: eq.name.clone(),
                category: eq.taxonomy.category.clone(),
                subcategories: eq.taxonomy.subcategories.clone(),
                description: if disp.description.is_empty() {
                    eq.name.clone()
                } else {
                    disp.description
                },
                aliases: eq.aliases.clone(),
                solve_targets: eq
                    .variables
                    .keys()
                    .filter(|target| {
                        eq.solve.explicit_forms.contains_key(*target)
                            || is_numerically_supported(eq, target)
                    })
                    .cloned()
                    .collect(),
            }
        })
        .collect();
    entries.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    entries
}
