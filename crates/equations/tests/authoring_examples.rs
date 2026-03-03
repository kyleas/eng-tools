use std::{ffi::OsStr, fs, path::PathBuf};

use equations::Registry;
use walkdir::WalkDir;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn authoring_examples_parse_validate_and_self_test() {
    let examples_dir = crate_root().join("examples/authoring");
    assert!(
        examples_dir.exists(),
        "examples directory missing: {}",
        examples_dir.display()
    );

    let mut seen_any = false;
    for entry in WalkDir::new(&examples_dir) {
        let entry = entry.expect("walkdir entry");
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension() != Some(OsStr::new("yaml")) {
            continue;
        }
        seen_any = true;
        validate_single_example(entry.path().to_path_buf());
    }

    assert!(seen_any, "no YAML authoring examples found");
}

fn validate_single_example(example_file: PathBuf) {
    let temp = tempfile::tempdir().expect("tempdir");
    let temp_file = temp.path().join(
        example_file
            .file_name()
            .expect("example file name should exist"),
    );
    fs::copy(&example_file, &temp_file).unwrap_or_else(|e| {
        panic!(
            "failed to copy example {} to temp: {}",
            example_file.display(),
            e
        )
    });

    let registry = Registry::load_from_dir(temp.path())
        .unwrap_or_else(|e| panic!("failed loading example {}: {}", example_file.display(), e));
    registry.validate_with_tests().unwrap_or_else(|e| {
        panic!(
            "authoring example failed validate_with_tests {}: {}",
            example_file.display(),
            e
        )
    });
}
