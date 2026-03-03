use std::collections::HashMap;

use crate::{
    error::{EquationError, Result},
    expr::evaluate_residual,
    model::EquationDef,
    normalize::{
        is_numerically_supported, resolved_numerical_bracket, resolved_numerical_initial_guess,
        resolved_numerical_max_iter,
    },
};

pub fn solve_numerical(
    eq: &EquationDef,
    target: &str,
    knowns: &HashMap<String, f64>,
    bracket_override: Option<(f64, f64)>,
) -> Result<f64> {
    if !is_numerically_supported(eq, target) {
        return Err(EquationError::InvalidSolveTarget {
            equation_key: eq.key.clone(),
            target: target.to_string(),
            valid_targets: eq.variables.keys().cloned().collect::<Vec<_>>().join(", "),
        });
    }

    let target_dimension = eq
        .variables
        .get(target)
        .map(|v| v.dimension.as_str())
        .unwrap_or("dimensionless");

    let tol_abs = eq.solve.numerical.tolerance_abs.unwrap_or(1e-10);
    let tol_rel = eq.solve.numerical.tolerance_rel.unwrap_or(1e-8);
    let max_iter = resolved_numerical_max_iter(eq, target_dimension);
    let (mut lo, mut hi) = find_bracket(eq, target, knowns, target_dimension, bracket_override)?;

    let mut f_lo = residual_at(eq, target, lo, knowns)?;
    let f_hi = residual_at(eq, target, hi, knowns)?;
    if f_lo.abs() <= tol_abs {
        return Ok(lo);
    }
    if f_hi.abs() <= tol_abs {
        return Ok(hi);
    }
    if f_lo.signum() == f_hi.signum() {
        return Err(EquationError::NumericalSolve {
            equation_key: eq.key.clone(),
            target: target.to_string(),
            reason: format!(
                "initial bracket [{lo}, {hi}] does not straddle a root (f(lo)={f_lo:e}, f(hi)={f_hi:e}); check branch selection, domain validity, or provide better bracket/guess"
            ),
        });
    }

    for _ in 0..max_iter {
        let mid = 0.5 * (lo + hi);
        let f_mid = residual_at(eq, target, mid, knowns)?;
        let residual_tol = tol_abs + tol_rel * mid.abs().max(1.0);
        if f_mid.abs() <= residual_tol {
            return Ok(mid);
        }
        if f_lo.signum() == f_mid.signum() {
            lo = mid;
            f_lo = f_mid;
        } else {
            hi = mid;
        }
    }
    Err(EquationError::NumericalSolve {
        equation_key: eq.key.clone(),
        target: target.to_string(),
        reason: format!(
            "no convergence in {max_iter} iterations; likely causes: bracket quality, invalid domain, or branch mismatch"
        ),
    })
}

fn find_bracket(
    eq: &EquationDef,
    target: &str,
    knowns: &HashMap<String, f64>,
    target_dimension: &str,
    bracket_override: Option<(f64, f64)>,
) -> Result<(f64, f64)> {
    if let Some(br) = bracket_override {
        return Ok(br);
    }
    let (a, b) = resolved_numerical_bracket(eq, target_dimension);
    let f_a = residual_at(eq, target, a, knowns)?;
    let f_b = residual_at(eq, target, b, knowns)?;
    if f_a.signum() != f_b.signum() {
        return Ok((a, b));
    }

    let mut center = resolved_numerical_initial_guess(eq, target_dimension);
    if center == 0.0 {
        center = 1.0;
    }
    let mut span = ((b - a).abs() * 0.5).max(center.abs().max(1.0));
    let center_seed = if a <= center && center <= b {
        center
    } else {
        0.5 * (a + b)
    };
    for _ in 0..40 {
        let lo = center_seed - span;
        let hi = center_seed + span;
        let f_lo = residual_at(eq, target, lo, knowns)?;
        let f_hi = residual_at(eq, target, hi, knowns)?;
        if f_lo.signum() != f_hi.signum() {
            return Ok((lo, hi));
        }
        span *= 2.0;
    }
    Err(EquationError::NumericalSolve {
        equation_key: eq.key.clone(),
        target: target.to_string(),
        reason: format!(
            "could not discover a valid bracket from defaults around initial_guess={center_seed}; check branch selection, givens, and target domain"
        ),
    })
}

fn residual_at(
    eq: &EquationDef,
    target: &str,
    value: f64,
    knowns: &HashMap<String, f64>,
) -> Result<f64> {
    let mut vars = knowns.clone();
    vars.insert(target.to_string(), value);
    evaluate_residual(&eq.relation.residual, &vars)
}
