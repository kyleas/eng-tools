use std::{fs, path::PathBuf};

use equations::generate_schema_to_path;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
