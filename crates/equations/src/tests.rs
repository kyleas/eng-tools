use std::collections::HashMap;

use crate::expr::{evaluate_expression, parse_expression};

#[test]
fn expression_engine_smoke() {
    let expr = parse_expression("sqrt(a^2 + b^2)").expect("parse");
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), 3.0);
    vars.insert("b".to_string(), 4.0);
    let v = evaluate_expression("sqrt(a^2 + b^2)", &expr, &vars).expect("eval");
    assert!((v - 5.0).abs() < 1e-12);
}
