use std::collections::HashMap;

use crate::constants;
use crate::error::{EquationError, Result};
use crate::expr::parser::Expr;

pub fn evaluate_expression(
    expr_text: &str,
    expr: &Expr,
    vars: &HashMap<String, f64>,
) -> Result<f64> {
    eval(expr_text, expr, vars)
}

fn eval(expr_text: &str, expr: &Expr, vars: &HashMap<String, f64>) -> Result<f64> {
    match expr {
        Expr::Number(v) => Ok(*v),
        Expr::Var(name) => vars
            .get(name)
            .copied()
            .or_else(|| constants::get_by_identifier(name).map(|c| c.value))
            .ok_or_else(|| EquationError::ExpressionEval {
                expression: expr_text.to_string(),
                message: format!(
                    "missing symbol '{name}' (not provided as variable or known constant)"
                ),
            }),
        Expr::Unary { op, rhs } => {
            let rhs = eval(expr_text, rhs, vars)?;
            match op {
                '-' => Ok(-rhs),
                _ => Err(EquationError::ExpressionEval {
                    expression: expr_text.to_string(),
                    message: format!("unsupported unary operator '{op}'"),
                }),
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let l = eval(expr_text, lhs, vars)?;
            let r = eval(expr_text, rhs, vars)?;
            let v = match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                '^' => l.powf(r),
                _ => {
                    return Err(EquationError::ExpressionEval {
                        expression: expr_text.to_string(),
                        message: format!("unsupported binary operator '{op}'"),
                    });
                }
            };
            Ok(v)
        }
        Expr::Call { name, args } => eval_call(expr_text, name, args, vars),
    }
}

fn eval_call(
    expr_text: &str,
    name: &str,
    args: &[Expr],
    vars: &HashMap<String, f64>,
) -> Result<f64> {
    let vals: Result<Vec<f64>> = args.iter().map(|a| eval(expr_text, a, vars)).collect();
    let vals = vals?;
    let bad_arity = || EquationError::ExpressionEval {
        expression: expr_text.to_string(),
        message: format!("function '{name}' called with wrong number of arguments"),
    };
    match name {
        "sqrt" => vals.first().copied().ok_or_else(bad_arity).map(f64::sqrt),
        "log10" => vals.first().copied().ok_or_else(bad_arity).map(f64::log10),
        "ln" => vals.first().copied().ok_or_else(bad_arity).map(f64::ln),
        "exp" => vals.first().copied().ok_or_else(bad_arity).map(f64::exp),
        "sin" => vals.first().copied().ok_or_else(bad_arity).map(f64::sin),
        "asin" => vals.first().copied().ok_or_else(bad_arity).map(f64::asin),
        "cos" => vals.first().copied().ok_or_else(bad_arity).map(f64::cos),
        "acos" => vals.first().copied().ok_or_else(bad_arity).map(f64::acos),
        "tan" => vals.first().copied().ok_or_else(bad_arity).map(f64::tan),
        "atan" => vals.first().copied().ok_or_else(bad_arity).map(f64::atan),
        "abs" => vals.first().copied().ok_or_else(bad_arity).map(f64::abs),
        "pow" => {
            if vals.len() != 2 {
                return Err(bad_arity());
            }
            Ok(vals[0].powf(vals[1]))
        }
        _ => Err(EquationError::ExpressionEval {
            expression: expr_text.to_string(),
            message: format!("unknown function '{name}'"),
        }),
    }
}
