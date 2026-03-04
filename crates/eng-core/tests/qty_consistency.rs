use eng_core::units::{parse_quantity_expression, qty};

#[test]
fn qty_macro_matches_runtime_for_pressure_expression() {
    let runtime = parse_quantity_expression("5 MPa + 12 psia").expect("runtime parse");
    let compiled = qty!("5 MPa + 12 psia");
    assert!((runtime.value_si - compiled.value_si).abs() < 1e-9);
    assert_eq!(runtime.signature, compiled.signature);
}

#[test]
fn qty_macro_matches_runtime_for_viscosity_expression() {
    let runtime = parse_quantity_expression("1 Pa*s + 2 cP").expect("runtime parse");
    let compiled = qty!("1 Pa*s + 2 cP");
    assert!((runtime.value_si - compiled.value_si).abs() < 1e-12);
    assert_eq!(runtime.signature, compiled.signature);
}

#[test]
fn runtime_rejects_invalid_mixed_dimensions() {
    let err = parse_quantity_expression("5 MPa + 3 m").expect_err("expected invalid");
    assert!(err.to_string().contains("differing dimensions"));
}
