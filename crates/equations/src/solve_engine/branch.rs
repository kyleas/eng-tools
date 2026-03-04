use std::collections::HashMap;

use crate::{
    error::{EquationError, Result},
    expr::{evaluate_expression, parse_expression},
    model::{BranchDef, EquationDef},
    normalize::preferred_branch,
};

pub fn select_branch<'a>(
    equation: &'a EquationDef,
    knowns: &HashMap<String, f64>,
    requested: Option<&str>,
) -> Result<Option<&'a BranchDef>> {
    if equation.branches.is_empty() {
        return Ok(None);
    }
    if let Some(name) = requested {
        if let Some(branch) = equation.branches.iter().find(|b| b.name == name) {
            return Ok(Some(branch));
        }
        let valid = equation
            .branches
            .iter()
            .map(|b| b.name.as_str())
            .collect::<Vec<_>>();
        return Err(EquationError::InvalidBranch {
            equation_key: equation.key.clone(),
            branch: name.to_string(),
            valid_branches: valid.join(", "),
            suggestion: closest_branch_hint(name, &valid),
        });
    }
    for branch in &equation.branches {
        let parsed = parse_expression(&branch.condition)?;
        let cond = match evaluate_expression(&branch.condition, &parsed, knowns) {
            Ok(v) => v,
            // Branch conditions often include the solve target (for example `1 - M`).
            // If target is not known yet, defer to preferred-branch fallback.
            Err(EquationError::UnknownSymbol { .. })
            | Err(EquationError::ExpressionEval { .. }) => {
                continue;
            }
            Err(e) => return Err(e),
        };
        if cond != 0.0 && branch.preferred {
            return Ok(Some(branch));
        }
    }
    Ok(preferred_branch(&equation.branches))
}

fn closest_branch_hint(input: &str, branches: &[&str]) -> String {
    let mut best: Option<(&str, usize)> = None;
    for &b in branches {
        let d = levenshtein(input, b);
        if d <= 3 {
            match best {
                Some((_, bd)) if d >= bd => {}
                _ => best = Some((b, d)),
            }
        }
    }
    best.map(|(b, _)| format!(" (did you mean '{}'?)", b))
        .unwrap_or_default()
}

fn levenshtein(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    let b_len = b.chars().count();
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = usize::from(ca != cb);
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        prev.copy_from_slice(&curr);
    }
    prev[b_len]
}
