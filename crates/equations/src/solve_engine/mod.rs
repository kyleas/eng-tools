pub mod branch;
pub mod explicit;
pub mod numerical;
pub mod result;

use std::collections::HashMap;

pub use result::{SolveMethod, SolveRequest, SolveResponse};

use crate::{
    error::{EquationError, Result},
    expr::evaluate_residual,
    model::{EquationDef, VariableConstraint},
    normalize::{is_numerically_supported, resolved_constraints},
    solve_engine::{explicit::solve_explicit, numerical::solve_numerical},
};

/// Solve a single equation target using explicit, numerical, or auto mode.
///
/// `SolveMethod::Auto` prefers explicit forms when available, otherwise numerical solve.
pub fn solve_equation(equation: &EquationDef, request: SolveRequest) -> Result<SolveResponse> {
    let valid_targets = equation
        .variables
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    if !equation.variables.contains_key(&request.target) {
        return Err(EquationError::InvalidSolveTarget {
            equation_key: equation.key.clone(),
            target: request.target.clone(),
            valid_targets,
        });
    }

    let selected_branch =
        branch::select_branch(equation, &request.knowns_si, request.branch.as_deref())?;
    let branch_bracket = selected_branch.and_then(|b| match b.name.as_str() {
        "subsonic" => Some((1e-6, 0.999_999)),
        "supersonic" => Some((1.000_001, 50.0)),
        _ => None,
    });

    let method = match request.method {
        SolveMethod::Auto => {
            if equation.solve.explicit_forms.contains_key(&request.target) {
                SolveMethod::Explicit
            } else if is_numerically_supported(equation, &request.target) {
                SolveMethod::Numerical
            } else {
                return Err(EquationError::InvalidSolveTarget {
                    equation_key: equation.key.clone(),
                    target: request.target,
                    valid_targets: equation
                        .variables
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", "),
                });
            }
        }
        m => m,
    };

    let value_si = match method {
        SolveMethod::Explicit => solve_explicit(equation, &request.target, &request.knowns_si)?,
        SolveMethod::Numerical => solve_numerical(
            equation,
            &request.target,
            &request.knowns_si,
            branch_bracket,
        )?,
        SolveMethod::Auto => unreachable!(),
    };

    let mut state = request.knowns_si.clone();
    state.insert(request.target.clone(), value_si);

    validate_constraints(equation, &state)?;
    let residual = evaluate_residual(&equation.relation.residual, &state)?;

    Ok(SolveResponse {
        target: request.target,
        value_si,
        method_used: method,
        residual,
    })
}

pub fn validate_constraints(equation: &EquationDef, vars: &HashMap<String, f64>) -> Result<()> {
    for (name, def) in &equation.variables {
        if let Some(v) = vars.get(name) {
            let effective = resolved_constraints(&def.dimension, &def.constraints);
            validate_single_constraint(name, *v, &effective)?;
        }
    }
    Ok(())
}

fn validate_single_constraint(variable: &str, value: f64, c: &VariableConstraint) -> Result<()> {
    if c.positive && value <= 0.0 {
        return Err(EquationError::Constraint {
            variable: variable.to_string(),
            message: format!("must be positive (got {value})"),
        });
    }
    if c.nonzero && value == 0.0 {
        return Err(EquationError::Constraint {
            variable: variable.to_string(),
            message: format!("must be nonzero (got {value})"),
        });
    }
    if c.integer && value.fract() != 0.0 {
        return Err(EquationError::Constraint {
            variable: variable.to_string(),
            message: format!("must be an integer (got {value})"),
        });
    }
    if let Some(min) = c.min
        && value < min
    {
        return Err(EquationError::Constraint {
            variable: variable.to_string(),
            message: format!("must be >= {min} (got {value})"),
        });
    }
    if let Some(max) = c.max
        && value > max
    {
        return Err(EquationError::Constraint {
            variable: variable.to_string(),
            message: format!("must be <= {max} (got {value})"),
        });
    }
    Ok(())
}
