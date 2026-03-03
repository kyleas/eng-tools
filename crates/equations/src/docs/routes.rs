use std::path::{Component, Path, PathBuf};

use crate::docs::presentation::EquationPresentation;

pub fn family_slug(family_key: &str) -> String {
    snake_case(family_key)
}

pub fn family_doc_path(family_key: &str) -> String {
    format!("equations/families/{}.md", family_slug(family_key))
}

pub fn constant_doc_path(key: &str) -> String {
    format!("constants/{key}.md")
}

pub fn fluid_doc_path(key: &str) -> String {
    format!("fluids/{}.md", snake_case(key))
}

pub fn material_doc_path(key: &str) -> String {
    format!("materials/{}.md", snake_case(key))
}

pub fn equation_doc_path_from_path_id(path_id: &str) -> String {
    format!("equations/{}.md", path_id.replace('.', "/"))
}

pub fn equation_doc_path(eq: &EquationPresentation) -> String {
    if let Some(sub) = eq.subcategories.first() {
        format!("equations/{}/{}/{}.md", eq.category, sub, eq.slug)
    } else {
        format!("equations/{}/{}.md", eq.category, eq.slug)
    }
}

pub fn relative_doc_link(from_doc: &str, to_doc: &str) -> String {
    let from_dir = Path::new(from_doc)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let from_parts = clean_parts(from_dir);
    let to_parts = clean_parts(Path::new(to_doc));

    let mut i = 0usize;
    while i < from_parts.len() && i < to_parts.len() && from_parts[i] == to_parts[i] {
        i += 1;
    }

    let mut rel = PathBuf::new();
    for _ in i..from_parts.len() {
        rel.push("..");
    }
    for part in &to_parts[i..] {
        rel.push(part);
    }

    let mut s = rel.to_string_lossy().replace('\\', "/");
    if s.is_empty() {
        s = ".".to_string();
    }
    s
}

fn clean_parts(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|c| match c {
            Component::Normal(p) => Some(p.to_string_lossy().to_string()),
            _ => None,
        })
        .collect()
}

fn snake_case(input: &str) -> String {
    let mut out = String::new();
    let mut prev_is_sep = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.is_empty() && !prev_is_sep {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_is_sep = false;
        } else {
            if !prev_is_sep && !out.is_empty() {
                out.push('_');
            }
            prev_is_sep = true;
        }
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::{family_doc_path, relative_doc_link};

    #[test]
    fn relative_link_handles_nested_equation_pages() {
        let from = "equations/thermo/ideal_gas/density.md";
        let to = family_doc_path("ideal_gas");
        let rel = relative_doc_link(from, &to);
        assert_eq!(rel, "../../families/ideal_gas.md");
    }

    #[test]
    fn relative_link_handles_root_category_pages() {
        let from = "equations/structures/hoop_stress.md";
        let to = family_doc_path("ideal_gas");
        let rel = relative_doc_link(from, &to);
        assert_eq!(rel, "../families/ideal_gas.md");
    }
}
