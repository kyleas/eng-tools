use equations::Registry;

#[test]
fn lint_reports_unused_variable_warning() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        temp.path().join("lint_case.yaml"),
        r#"
key: lint_case
taxonomy: { category: structures }
name: Lint Case
display: { latex: 'x = y' }
variables:
  x: { name: X, dimension: dimensionless }
  y: { name: Y, dimension: dimensionless }
  z: { name: Z, dimension: dimensionless }
residual: "x - y"
solve:
  explicit_forms: { x: y }
assumptions: []
tests:
  - full_state: { x: 1, y: 1, z: 2 }
"#,
    )
    .expect("write");

    let registry = Registry::load_from_dir(temp.path()).expect("load");
    registry.validate().expect("validate");
    let warnings = registry.lint().expect("lint");
    assert!(
        warnings.iter().any(|w| w.code == "unused_variable"),
        "warnings={warnings:?}"
    );
    assert!(
        warnings.iter().any(|w| w.code == "missing_source"),
        "warnings={warnings:?}"
    );
}
