use std::{fs, path::Path};

use schemars::schema_for;

use crate::{
    error::{EquationError, Result},
    model::EquationDef,
};

/// Generate JSON Schema for equation YAML authoring.
pub fn generate_schema_to_path(path: impl AsRef<Path>) -> Result<()> {
    let schema = schema_for!(EquationDef);
    let json = serde_json::to_string_pretty(&schema)?;
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| EquationError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, json).map_err(|source| EquationError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}
