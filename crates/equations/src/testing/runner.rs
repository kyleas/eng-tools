use std::collections::HashMap;

use eng_core::units::{convert_equation_value_to_si, parse_equation_quantity_to_si};

use crate::{
    error::{EquationError, Result},
    expr::evaluate_residual,
    model::{EquationDef, MethodKind, QuantityInput},
    normalize::{
        resolved_case_target_specs, resolved_default_unit, resolved_relation_tolerance,
        resolved_target_methods, resolved_target_tolerance,
    },
    registry::Registry,
    solve_engine::validate_constraints,
    solve_engine::{SolveMethod, SolveRequest, solve_equation},
    testing::{
        method_consistency::methods_consistent, residual_check::residual_is_zero,
        solve_check::values_close,
    },
};

#[derive(Debug, Clone, Default)]
pub struct RegistryTestSummary {
    pub passed: usize,
    pub failed: usize,
}

pub fn run_registry_tests(registry: &Registry) -> Result<RegistryTestSummary> {
    let mut summary = RegistryTestSummary::default();
    for eq in registry.equations() {
        for case in &eq.tests.cases {
            match run_case(eq, case.id.as_str()) {
                Ok(()) => summary.passed += 1,
                Err(e) => {
                    return Err(EquationError::TestFailure(format!(
                        "{} / {} failed: {}",
                        eq.key, case.id, e
                    )));
                }
            }
        }
    }
    Ok(summary)
}

fn run_case(eq: &EquationDef, case_id: &str) -> Result<()> {
    let case = eq
        .tests
        .cases
        .iter()
        .find(|c| c.id == case_id)
        .ok_or_else(|| EquationError::TestFailure(format!("missing test case '{case_id}'")))?;

    let mut state_si = HashMap::new();
    for (var, input) in &case.full_state {
        let vdef = eq.variables.get(var).ok_or_else(|| {
            EquationError::TestFailure(format!("unknown variable '{}' in case '{}'", var, case_id))
        })?;
        let v = quantity_to_si(var, &vdef.dimension, &vdef.default_unit, input)?;
        state_si.insert(var.clone(), v);
    }
    validate_constraints(eq, &state_si)?;

    if case.verify.residual_zero {
        let residual = evaluate_residual(&eq.relation.residual, &state_si)?;
        let (mut abs, mut rel) = resolved_relation_tolerance(eq);
        if let Some(case_tol) = &case.tolerances {
            abs = case_tol.abs.unwrap_or(abs);
            rel = case_tol.rel.unwrap_or(rel);
        }
        let ok = residual_is_zero(residual, abs, rel, 1.0);
        if !ok {
            return Err(EquationError::TestFailure(format!(
                "residual check failed for case '{}' residual={residual:e}",
                case_id
            )));
        }
    }

    let targets = resolved_case_target_specs(eq, case);
    for target in &targets {
        let target_name = target.target().to_string();
        let expected = *state_si.get(&target_name).ok_or_else(|| {
            EquationError::TestFailure(format!(
                "target '{}' has no expected value in full_state",
                target_name
            ))
        })?;
        let mut knowns = state_si.clone();
        knowns.remove(&target_name);

        let mut explicit_value = None;
        let mut numerical_value = None;
        let methods = resolved_target_methods(eq, &target_name, target);
        let (abs, rel) =
            resolved_target_tolerance(eq, case.tolerances.as_ref(), target.tolerances(), &methods);
        for method in &methods {
            let method = match method {
                MethodKind::Explicit => SolveMethod::Explicit,
                MethodKind::Numerical => SolveMethod::Numerical,
            };
            let solved = solve_equation(
                eq,
                SolveRequest {
                    target: target_name.clone(),
                    knowns_si: knowns.clone(),
                    method,
                    branch: case.branch.clone(),
                },
            )?;

            if !values_close(expected, solved.value_si, abs, rel) {
                return Err(EquationError::TestFailure(format!(
                    "solve check failed for case '{}' target '{}' via {:?}: expected {}, actual {}",
                    case_id, target_name, method, expected, solved.value_si
                )));
            }
            if !residual_is_zero(solved.residual, abs, rel, expected.abs().max(1.0)) {
                return Err(EquationError::TestFailure(format!(
                    "solved residual check failed for case '{}' target '{}' via {:?}: residual {}",
                    case_id, target_name, method, solved.residual
                )));
            }
            match method {
                SolveMethod::Explicit => explicit_value = Some(solved.value_si),
                SolveMethod::Numerical => numerical_value = Some(solved.value_si),
                SolveMethod::Auto => {}
            }
        }
        if let (Some(explicit), Some(numerical)) = (explicit_value, numerical_value)
            && !methods_consistent(explicit, numerical, abs, rel)
        {
            return Err(EquationError::TestFailure(format!(
                "method consistency failed for case '{}' target '{}': explicit={}, numerical={}",
                case_id, target_name, explicit, numerical
            )));
        }
    }

    Ok(())
}

fn quantity_to_si(
    variable: &str,
    dimension: &str,
    default_unit: &Option<String>,
    input: &QuantityInput,
) -> Result<f64> {
    let default_unit =
        resolved_default_unit(dimension, default_unit.as_deref()).ok_or_else(|| {
            EquationError::Unit {
                variable: variable.to_string(),
                message: format!(
                    "no default unit available for dimension '{}'; provide variable.default_unit",
                    dimension
                ),
            }
        })?;

    match input {
        QuantityInput::Scalar(v) => convert_equation_value_to_si(dimension, &default_unit, *v)
            .map_err(|e| EquationError::Unit {
                variable: variable.to_string(),
                message: e.to_string(),
            }),
        QuantityInput::ValueUnit { value, unit } => {
            convert_equation_value_to_si(dimension, unit, *value).map_err(|e| EquationError::Unit {
                variable: variable.to_string(),
                message: e.to_string(),
            })
        }
        QuantityInput::StringValue(s) => {
            parse_equation_quantity_to_si(dimension, s).map_err(|e| EquationError::Unit {
                variable: variable.to_string(),
                message: e.to_string(),
            })
        }
    }
}
