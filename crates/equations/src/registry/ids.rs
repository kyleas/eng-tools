use crate::model::EquationDef;

pub fn derive_path_id(equation: &EquationDef) -> String {
    let mut parts = Vec::with_capacity(2 + equation.taxonomy.subcategories.len());
    parts.push(equation.taxonomy.category.clone());
    parts.extend(equation.taxonomy.subcategories.clone());
    parts.push(equation.effective_slug().to_string());
    parts.join(".")
}
