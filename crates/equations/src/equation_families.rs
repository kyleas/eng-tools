use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{EquationError, Result},
    model::EquationDef,
    registry::ids::derive_path_id,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquationFamilyDef {
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub canonical_law: String,
    pub canonical_equation: String,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub references: Vec<String>,
    pub variants: Vec<EquationFamilyVariant>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EquationFamilyVariant {
    pub key: String,
    pub name: String,
    pub equation_id: String,
    pub display_latex: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub when_to_use: String,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub references: Vec<String>,
}

pub fn default_families_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("families")
}

pub fn load_from_dir(dir: impl AsRef<Path>) -> Result<Vec<EquationFamilyDef>> {
    let dir = dir.as_ref();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|source| EquationError::Io {
        path: dir.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|e| EquationError::Validation(e.to_string()))?;
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|source| EquationError::Io {
            path: path.clone(),
            source,
        })?;
        let mut family: EquationFamilyDef = serde_yaml::from_str(&text).map_err(|e| {
            EquationError::Validation(format!(
                "{}: failed to parse family yaml: {e}",
                path.display()
            ))
        })?;
        family.key = family.key.trim().to_string();
        family.name = family.name.trim().to_string();
        out.push(family);
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(out)
}

pub fn load_default_validated(equations: &[EquationDef]) -> Result<Vec<EquationFamilyDef>> {
    let families = load_from_dir(default_families_dir())?;
    validate_families(&families, equations)?;
    Ok(families)
}

pub fn validate_families(families: &[EquationFamilyDef], equations: &[EquationDef]) -> Result<()> {
    let mut family_keys = BTreeSet::new();
    let mut equation_to_family: BTreeMap<String, String> = BTreeMap::new();
    let mut by_any_id = BTreeSet::new();
    for eq in equations {
        by_any_id.insert(eq.key.clone());
        by_any_id.insert(derive_path_id(eq));
        for a in &eq.aliases {
            by_any_id.insert(a.clone());
        }
    }

    for fam in families {
        if fam.key.trim().is_empty() {
            return Err(EquationError::Validation(
                "equation family key cannot be empty".to_string(),
            ));
        }
        if !family_keys.insert(fam.key.clone()) {
            return Err(EquationError::Validation(format!(
                "duplicate equation family key '{}'",
                fam.key
            )));
        }
        if fam.variants.is_empty() {
            return Err(EquationError::Validation(format!(
                "equation family '{}' must have at least one variant",
                fam.key
            )));
        }
        if fam.canonical_law.trim().is_empty() {
            return Err(EquationError::Validation(format!(
                "equation family '{}' canonical_law cannot be empty",
                fam.key
            )));
        }
        if !by_any_id.contains(&fam.canonical_equation) {
            return Err(EquationError::Validation(format!(
                "equation family '{}' canonical_equation '{}' does not resolve to a known equation",
                fam.key, fam.canonical_equation
            )));
        }

        let mut variant_keys = BTreeSet::new();
        let mut canonical_in_variants = false;
        for v in &fam.variants {
            if v.key.trim().is_empty() {
                return Err(EquationError::Validation(format!(
                    "equation family '{}' has variant with empty key",
                    fam.key
                )));
            }
            if !variant_keys.insert(v.key.clone()) {
                return Err(EquationError::Validation(format!(
                    "equation family '{}' has duplicate variant key '{}'",
                    fam.key, v.key
                )));
            }
            if !by_any_id.contains(&v.equation_id) {
                return Err(EquationError::Validation(format!(
                    "equation family '{}' variant '{}' references unknown equation '{}'",
                    fam.key, v.key, v.equation_id
                )));
            }
            if v.equation_id == fam.canonical_equation {
                canonical_in_variants = true;
            }
            if let Some(prev_family) =
                equation_to_family.insert(v.equation_id.clone(), fam.key.clone())
                && prev_family != fam.key
            {
                return Err(EquationError::Validation(format!(
                    "equation '{}' is assigned to multiple families ('{}', '{}')",
                    v.equation_id, prev_family, fam.key
                )));
            }
            if v.display_latex.trim().is_empty() {
                return Err(EquationError::Validation(format!(
                    "equation family '{}' variant '{}' display_latex cannot be empty",
                    fam.key, v.key
                )));
            }
        }
        if !canonical_in_variants {
            return Err(EquationError::Validation(format!(
                "equation family '{}' canonical_equation '{}' must also appear in variants",
                fam.key, fam.canonical_equation
            )));
        }
    }

    Ok(())
}

pub fn family_by_equation_path_id<'a>(
    families: &'a [EquationFamilyDef],
    path_id: &str,
) -> Option<(&'a EquationFamilyDef, &'a EquationFamilyVariant)> {
    for fam in families {
        for var in &fam.variants {
            if var.equation_id == path_id {
                return Some((fam, var));
            }
        }
    }
    None
}

pub fn family_index_by_equation_id(
    families: &[EquationFamilyDef],
) -> BTreeMap<String, (String, String)> {
    let mut out = BTreeMap::new();
    for fam in families {
        for v in &fam.variants {
            out.insert(v.equation_id.clone(), (fam.key.clone(), v.key.clone()));
        }
    }
    out
}

pub fn effective_variant_assumptions(
    family: &EquationFamilyDef,
    variant: &EquationFamilyVariant,
) -> Vec<String> {
    let mut out = family.assumptions.clone();
    out.extend(variant.assumptions.iter().cloned());
    out.retain(|s| !s.trim().is_empty());
    out
}

pub fn effective_variant_references(
    family: &EquationFamilyDef,
    variant: &EquationFamilyVariant,
) -> Vec<String> {
    let mut out = family.references.clone();
    out.extend(variant.references.iter().cloned());
    out.retain(|s| !s.trim().is_empty());
    out
}
