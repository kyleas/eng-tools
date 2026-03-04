use std::path::PathBuf;

use equations::{Registry, generate_schema_to_path};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn registry_is_valid_and_self_tested() {
    let root = crate_root();
    let registry_dir = root.join("registry");
    let registry = Registry::load_from_dir(&registry_dir).expect("load registry");
    registry
        .validate_with_tests()
        .expect("registry validate with tests");
}

#[test]
fn schema_generation_runs() {
    let root = crate_root();
    let registry = Registry::load_from_dir(root.join("registry")).expect("load");
    registry.validate().expect("validate");
    generate_schema_to_path(root.join("schemas/equation.schema.json")).expect("schema");
}

#[test]
fn validation_rejects_duplicate_branch_names() {
    let temp = tempfile::tempdir().expect("tempdir");
    write_equation(
        temp.path().join("dup_branch.yaml"),
        r#"
key: duplicate_branch
taxonomy: { category: structures }
name: Duplicate Branch
display: { latex: "x = y" }
variables:
  x: { name: X, dimension: dimensionless }
  y: { name: Y, dimension: dimensionless }
residual: "x - y"
solve:
  explicit_forms: { x: y }
branches:
  - name: branch_a
    condition: x - y
    preferred: true
  - name: branch_a
    condition: y - x
    preferred: false
assumptions: []
tests:
  - full_state: { x: "1", y: "1" }
"#,
    );

    let registry = Registry::load_from_dir(temp.path()).expect("load");
    let err = registry.validate().expect_err("expected validation error");
    let msg = err.to_string();
    assert!(msg.contains("duplicate branch name"), "message: {msg}");
}

#[test]
fn validation_rejects_unusable_default_target() {
    let temp = tempfile::tempdir().expect("tempdir");
    write_equation(
        temp.path().join("bad_default_target.yaml"),
        r#"
key: bad_default_target
taxonomy: { category: structures }
name: Bad Default Target
display: { latex: "x = y" }
variables:
  x: { name: X, dimension: dimensionless }
  y: { name: Y, dimension: dimensionless }
residual: "x - y"
solve:
  default_target: x
  explicit_forms: { y: x }
  numerical:
    unsupported_targets: [x]
assumptions: []
tests:
  - full_state: { x: "1", y: "1" }
"#,
    );

    let registry = Registry::load_from_dir(temp.path()).expect("load");
    let err = registry.validate().expect_err("expected validation error");
    let msg = err.to_string();
    assert!(
        msg.contains("default_target 'x' is neither explicitly solvable nor numerically supported"),
        "message: {msg}"
    );
}

fn write_equation(path: PathBuf, yaml: &str) {
    std::fs::write(&path, yaml)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", path.display(), e));
}
