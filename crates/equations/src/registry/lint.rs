use std::collections::{BTreeSet, HashMap};

use eng_core::units::convert_equation_value_to_si;

use crate::{
    error::Result,
    expr::{collect_symbols, evaluate_expression, parse_expression},
    model::{EquationDef, QuantityInput},
    normalize::resolved_default_unit,
};

#[derive(Debug, Clone)]
pub struct LintWarning {
    pub equation: String,
    pub code: &'static str,
    pub message: String,
}

pub fn lint_registry(equations: &[EquationDef]) -> Result<Vec<LintWarning>> {
    let mut out = Vec::new();
    for eq in equations {
        lint_equation(eq, &mut out)?;
    }
    Ok(out)
}

fn lint_equation(eq: &EquationDef, out: &mut Vec<LintWarning>) -> Result<()> {
    let residual_expr = parse_expression(&eq.relation.residual)?;
    let used_in_residual: BTreeSet<String> = collect_symbols(&residual_expr).into_iter().collect();

    for var in eq.variables.keys() {
        if !used_in_residual.contains(var) {
            out.push(LintWarning {
                equation: eq.key.clone(),
                code: "unused_variable",
                message: format!(
                    "variable '{}' is declared but not referenced in residual",
                    var
                ),
            });
        }
    }

    for (target, expr) in &eq.solve.explicit_forms {
        let parsed = parse_expression(expr)?;
        let symbols = collect_symbols(&parsed);
        if symbols.iter().any(|s| s == target) {
            out.push(LintWarning {
                equation: eq.key.clone(),
                code: "self_referential_explicit",
                message: format!("explicit form for '{}' appears to reference itself", target),
            });
        }
    }

    if eq.branches.len() > 1 && !eq.branches.iter().any(|b| b.preferred) {
        out.push(LintWarning {
            equation: eq.key.clone(),
            code: "branch_no_preferred",
            message: "equation has multiple branches but no preferred branch".to_string(),
        });
    }

    let mut branch_conditions = BTreeSet::new();
    for b in &eq.branches {
        if !branch_conditions.insert(b.condition.clone()) {
            out.push(LintWarning {
                equation: eq.key.clone(),
                code: "duplicate_branch_condition",
                message: format!(
                    "multiple branches share identical condition '{}'; verify overlap",
                    b.condition
                ),
            });
        }
    }

    if let Some(case) = eq.tests.cases.first() {
        let state = to_si_state(eq, &case.full_state)?;
        for (target, expr) in &eq.solve.explicit_forms {
            if let Some(expected) = state.get(target) {
                let parsed = parse_expression(expr)?;
                if let Ok(v) = evaluate_expression(expr, &parsed, &state) {
                    let abs = (v - expected).abs();
                    let rel = abs / expected.abs().max(1.0);
                    if abs > 1e-6 && rel > 1e-6 {
                        out.push(LintWarning {
                            equation: eq.key.clone(),
                            code: "explicit_residual_mismatch",
                            message: format!(
                                "explicit form for '{}' differs from baseline full_state (abs={}, rel={})",
                                target, abs, rel
                            ),
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

fn to_si_state(
    eq: &EquationDef,
    full_state: &indexmap::IndexMap<String, QuantityInput>,
) -> Result<HashMap<String, f64>> {
    let mut out = HashMap::new();
    for (k, q) in full_state {
        let Some(v) = eq.variables.get(k) else {
            continue;
        };
        let default_unit = resolved_default_unit(&v.dimension, v.default_unit.as_deref())
            .unwrap_or_else(|| "1".to_string());
        let si = match q {
            QuantityInput::Scalar(x) => {
                convert_equation_value_to_si(&v.dimension, &default_unit, *x)
                    .map_err(|e| crate::error::EquationError::Validation(e.to_string()))?
            }
            QuantityInput::StringValue(s) => {
                eng_core::units::parse_equation_quantity_to_si(&v.dimension, s)
                    .map_err(|e| crate::error::EquationError::Validation(e.to_string()))?
            }
            QuantityInput::ValueUnit { value, unit } => {
                convert_equation_value_to_si(&v.dimension, unit, *value)
                    .map_err(|e| crate::error::EquationError::Validation(e.to_string()))?
            }
        };
        out.insert(k.clone(), si);
    }
    Ok(out)
}
