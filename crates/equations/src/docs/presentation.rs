use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    constants::{self, EngineeringConstant},
    docs::page_model::{EquationPageModel, build_page_models},
    model::EquationDef,
};

#[derive(Debug, Clone, Serialize)]
pub struct LibraryPresentation {
    pub constants: Vec<EngineeringConstant>,
    pub categories: Vec<CategoryPresentation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryPresentation {
    pub name: String,
    pub root_equations: Vec<EquationPresentation>,
    pub subcategories: Vec<SubcategoryPresentation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubcategoryPresentation {
    pub name: String,
    pub equations: Vec<EquationPresentation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EquationPresentation {
    pub path_id: String,
    pub slug: String,
    pub category: String,
    pub subcategories: Vec<String>,
    pub page: EquationPageModel,
}

pub fn build_library_presentation(equations: &[EquationDef]) -> LibraryPresentation {
    let pages = build_page_models(equations);
    let mut categories: BTreeMap<String, CategoryPresentation> = BTreeMap::new();

    for page in pages {
        let category = page.category.clone();
        let subcats = page.subcategories.clone();
        let slug = page.slug.clone();
        let path_id = page.path_id.clone();
        let eq = EquationPresentation {
            path_id,
            slug,
            category: category.clone(),
            subcategories: subcats.clone(),
            page,
        };

        let cat = categories
            .entry(category.clone())
            .or_insert_with(|| CategoryPresentation {
                name: category,
                root_equations: Vec::new(),
                subcategories: Vec::new(),
            });

        if let Some(first) = subcats.first() {
            let mut found = false;
            for sub in &mut cat.subcategories {
                if sub.name == *first {
                    sub.equations.push(eq.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                cat.subcategories.push(SubcategoryPresentation {
                    name: first.clone(),
                    equations: vec![eq],
                });
            }
        } else {
            cat.root_equations.push(eq);
        }
    }

    let mut categories: Vec<CategoryPresentation> = categories.into_values().collect();
    categories.sort_by(|a, b| a.name.cmp(&b.name));
    for cat in &mut categories {
        cat.root_equations.sort_by(|a, b| a.path_id.cmp(&b.path_id));
        cat.subcategories.sort_by(|a, b| a.name.cmp(&b.name));
        for sub in &mut cat.subcategories {
            sub.equations.sort_by(|a, b| a.path_id.cmp(&b.path_id));
        }
    }

    let mut constants = constants::all().to_vec();
    constants.sort_by(|a, b| a.key.cmp(b.key));

    LibraryPresentation {
        constants,
        categories,
    }
}

pub fn flatten_equations(library: &LibraryPresentation) -> Vec<EquationPresentation> {
    let mut out = Vec::new();
    for cat in &library.categories {
        out.extend(cat.root_equations.clone());
        for sub in &cat.subcategories {
            out.extend(sub.equations.clone());
        }
    }
    out.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    out
}
