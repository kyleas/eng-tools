use std::{fs, path::PathBuf};

use equations::{Registry, docs::export_docs_artifacts, generate_schema_to_path};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn workspace_root() -> PathBuf {
    crate_root()
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or_else(crate_root)
}

fn normalize_newlines(s: String) -> String {
    s.replace("\r\n", "\n")
}

fn assert_file_matches(expected: PathBuf, actual: PathBuf) {
    let expected_text =
        normalize_newlines(fs::read_to_string(&expected).unwrap_or_else(|e| {
            panic!("failed to read expected file {}: {}", expected.display(), e)
        }));
    let actual_text = normalize_newlines(
        fs::read_to_string(&actual)
            .unwrap_or_else(|e| panic!("failed to read actual file {}: {}", actual.display(), e)),
    );
    assert_eq!(
        expected_text,
        actual_text,
        "golden artifact mismatch for {}\nregenerate artifacts and commit updated files",
        expected.display()
    );
}

#[test]
fn generated_schema_matches_committed_golden() {
    let root = crate_root();
    let temp = tempfile::tempdir().expect("tempdir");
    let generated = temp.path().join("equation.schema.json");
    generate_schema_to_path(&generated).expect("generate schema");

    let committed = root.join("schemas/equation.schema.json");
    assert_file_matches(committed, generated);
}

#[test]
fn exported_docs_match_committed_golden() {
    let root = crate_root();
    let registry = Registry::load_from_dir(root.join("registry")).expect("load registry");
    registry.validate().expect("validate registry");

    let temp = tempfile::tempdir().expect("tempdir");
    export_docs_artifacts(registry.equations(), temp.path()).expect("export docs");

    for file in [
        "search_index.json",
        "page_models.json",
        "navigation.json",
        "examples_index.json",
        "constants.json",
    ] {
        assert_file_matches(
            workspace_root().join("generated").join(file),
            temp.path().join(file),
        );
    }
}
