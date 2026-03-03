use std::collections::HashMap;

use crate::{
    error::{EquationError, Result},
    expr::{evaluate_expression, parse_expression},
    model::EquationDef,
};

pub fn solve_explicit(eq: &EquationDef, target: &str, vars: &HashMap<String, f64>) -> Result<f64> {
    let expr =
        eq.solve
            .explicit_forms
            .get(target)
            .ok_or_else(|| EquationError::InvalidSolveTarget {
                equation_key: eq.key.clone(),
                target: target.to_string(),
                valid_targets: eq.variables.keys().cloned().collect::<Vec<_>>().join(", "),
            })?;
    let parsed = parse_expression(expr)?;
    evaluate_expression(expr, &parsed, vars)
}
