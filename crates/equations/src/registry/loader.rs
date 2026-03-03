use std::{fs, path::Path};

use walkdir::WalkDir;

use crate::{
    error::{EquationError, Result},
    model::EquationDef,
};

pub fn load_equations_from_dir(dir: &Path) -> Result<Vec<EquationDef>> {
    let mut equations = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.map_err(|e| EquationError::Validation(format!("walkdir error: {e}")))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let is_yaml = path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml"));
        if !is_yaml {
            continue;
        }
        let contents = fs::read_to_string(path).map_err(|source| EquationError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let equation = serde_yaml::from_str::<EquationDef>(&contents).map_err(|source| {
            EquationError::Yaml {
                path: path.to_path_buf(),
                source,
            }
        })?;
        equations.push(equation);
    }
    equations.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(equations)
}
