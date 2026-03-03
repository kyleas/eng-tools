use std::collections::HashMap;

use crate::error::{EquationError, Result};
use crate::expr::{collect_symbols, evaluate_expression, parse_expression};

pub fn evaluate_residual(residual: &str, vars: &HashMap<String, f64>) -> Result<f64> {
    let expr = parse_expression(residual)?;
    evaluate_expression(residual, &expr, vars)
}

pub fn validate_expression_symbols(expression: &str, valid_symbols: &[String]) -> Result<()> {
    let expr = parse_expression(expression)?;
    let symbols = collect_symbols(&expr);
    for symbol in symbols {
        if !valid_symbols.iter().any(|s| s == &symbol) {
            return Err(EquationError::UnknownSymbol {
                expression: expression.to_string(),
                symbol,
            });
        }
    }
    Ok(())
}
