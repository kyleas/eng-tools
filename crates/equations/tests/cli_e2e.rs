use std::{path::PathBuf, process::Command};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn equations_cli_end_to_end_flow() {
    let registry_dir = crate_root().join("registry");
    let temp = tempfile::tempdir().expect("tempdir");
    let schema_out = temp.path().join("equation.schema.json");
    let docs_out = temp.path().join("generated");

    run_cli([
        "generate-schema",
        "--schema-out",
        schema_out.to_str().expect("schema path"),
    ]);
    assert!(schema_out.exists(), "schema file should be generated");

    run_cli([
        "validate",
        "--registry-dir",
        registry_dir.to_str().expect("registry path"),
        "--with-tests",
    ]);

    run_cli([
        "export-docs",
        "--registry-dir",
        registry_dir.to_str().expect("registry path"),
        "--out-dir",
        docs_out.to_str().expect("docs path"),
    ]);

    assert!(docs_out.join("search_index.json").exists());
    assert!(docs_out.join("page_models.json").exists());
    assert!(docs_out.join("navigation.json").exists());
    assert!(docs_out.join("examples_index.json").exists());
    assert!(docs_out.join("constants.json").exists());

    run_cli([
        "export-mdbook",
        "--registry-dir",
        registry_dir.to_str().expect("registry path"),
        "--out-dir",
        docs_out.join("book").to_str().expect("book path"),
    ]);
    assert!(docs_out.join("book").join("book.toml").exists());
    assert!(
        docs_out
            .join("book")
            .join("src")
            .join("SUMMARY.md")
            .exists()
    );
    assert!(
        docs_out
            .join("book")
            .join("src")
            .join("constants")
            .join("index.md")
            .exists()
    );

    run_cli([
        "export-pdf",
        "--registry-dir",
        registry_dir.to_str().expect("registry path"),
        "--out-file",
        docs_out
            .join("equations_handbook.pdf")
            .to_str()
            .expect("pdf path"),
    ]);
    assert!(docs_out.join("equations_handbook.pdf").exists());

    run_cli([
        "lint",
        "--registry-dir",
        registry_dir.to_str().expect("registry path"),
    ]);
}

#[test]
fn equations_cli_scaffold_writes_minimal_template() {
    let temp = tempfile::tempdir().expect("tempdir");
    let out_file = temp
        .path()
        .join("registry")
        .join("structures")
        .join("hoop_stress.yaml");
    run_cli([
        "scaffold",
        "--key",
        "hoop_stress",
        "--category",
        "structures",
        "--name",
        "Thin-Wall Hoop Stress",
        "--out",
        out_file.to_str().expect("out path"),
    ]);
    let content = std::fs::read_to_string(&out_file).expect("read scaffold");
    assert!(content.contains("key: hoop_stress"));
    assert!(content.contains("category: structures"));
    assert!(content.contains("name: Thin-Wall Hoop Stress"));
}

fn run_cli<const N: usize>(args: [&str; N]) {
    let output = Command::new(env!("CARGO_BIN_EXE_equations"))
        .args(args)
        .output()
        .expect("run equations cli");
    if !output.status.success() {
        panic!(
            "cli command failed: {:?}\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
