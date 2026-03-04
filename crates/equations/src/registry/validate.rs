use std::collections::{BTreeSet, HashSet};

use crate::{
    constants,
    error::{EquationError, Result},
    expr::residual::validate_expression_symbols,
    model::{EquationDef, MethodKind},
    normalize::{
        is_numerically_supported, resolved_case_target_specs, resolved_default_unit,
        resolved_symbol, resolved_target_methods,
    },
    registry::ids::derive_path_id,
};

pub fn validate_registry_definitions(equations: &[EquationDef]) -> Result<()> {
    let mut keys = HashSet::new();
    let mut path_ids = HashSet::new();
    let mut aliases = HashSet::new();

    for eq in equations {
        if eq.key.trim().is_empty() {
            return Err(EquationError::Validation(
                "equation key cannot be empty".to_string(),
            ));
        }
        if !is_valid_short_id(&eq.key) {
            return Err(EquationError::Validation(format!(
                "equation '{}' has invalid key; use a short taxonomy-independent key (letters/digits/_/-, no '.')",
                eq.key
            )));
        }
        if let Some(slug) = &eq.slug
            && (slug.trim().is_empty() || slug.contains('.') || !is_valid_short_id(slug))
        {
            return Err(EquationError::Validation(format!(
                "equation '{}' has invalid slug '{}' (must be short non-dotted identifier)",
                eq.key, slug
            )));
        }
        if !keys.insert(eq.key.clone()) {
            return Err(EquationError::Validation(format!(
                "duplicate equation key '{}'",
                eq.key
            )));
        }
        let path_id = derive_path_id(eq);
        if !path_ids.insert(path_id.clone()) {
            return Err(EquationError::Validation(format!(
                "duplicate path_id '{}'",
                path_id
            )));
        }
        if aliases.contains(&eq.key) || aliases.contains(&path_id) {
            return Err(EquationError::Validation(format!(
                "alias collides with key/path_id for equation '{}'",
                eq.key
            )));
        }

        for alias in &eq.aliases {
            if !is_valid_short_id(alias) || alias.contains('.') {
                return Err(EquationError::Validation(format!(
                    "alias '{}' in equation '{}' is invalid; aliases must be short non-dotted identifiers",
                    alias, eq.key
                )));
            }
            if keys.contains(alias) || path_ids.contains(alias) || !aliases.insert(alias.clone()) {
                return Err(EquationError::Validation(format!(
                    "alias '{}' collides in equation '{}'",
                    alias, eq.key
                )));
            }
        }

        validate_equation(eq)?;
    }

    Ok(())
}

pub fn validate_equation(eq: &EquationDef) -> Result<()> {
    if eq.taxonomy.category.trim().is_empty() {
        return Err(EquationError::Validation(format!(
            "equation '{}' has empty taxonomy.category",
            eq.key
        )));
    }
    if eq.variables.is_empty() {
        return Err(EquationError::Validation(format!(
            "equation '{}' has no variables",
            eq.key
        )));
    }
    if eq.tests.cases.is_empty() {
        return Err(EquationError::Validation(format!(
            "equation '{}' must include at least one test case",
            eq.key
        )));
    }
    let symbol_names: Vec<String> = eq.variables.keys().cloned().collect();
    let constant_ids = constants::expression_identifiers();
    let symbol_set: BTreeSet<String> = symbol_names.iter().cloned().collect();
    let mut expression_symbols = symbol_names.clone();
    expression_symbols.extend(constant_ids.clone());
    expression_symbols.sort();
    expression_symbols.dedup();

    if eq.display.latex.trim().is_empty() {
        return Err(EquationError::Validation(format!(
            "equation '{}' has empty display.latex",
            eq.key
        )));
    }
    if let Some(source) = &eq.source
        && source.source.trim().is_empty()
    {
        return Err(EquationError::Validation(format!(
            "equation '{}' source.source cannot be empty",
            eq.key
        )));
    }

    for (key, var) in &eq.variables {
        let _ = resolved_symbol(key, var.symbol.as_deref());
        if is_greek_ambiguous_key(key) && !has_explicit_symbol(var.symbol.as_deref()) {
            return Err(EquationError::Validation(format!(
                "equation '{}' variable '{}' requires explicit symbol to disambiguate Greek case (for example '\\\\Delta p' vs '\\\\delta p')",
                eq.key, key
            )));
        }
        if resolved_default_unit(&var.dimension, var.default_unit.as_deref()).is_none() {
            return Err(EquationError::Validation(format!(
                "equation '{}' variable '{}' has unknown dimension '{}' with no default_unit override",
                eq.key, key, var.dimension
            )));
        }
        if let Some(resolver) = &var.resolver {
            if resolver.source.trim().is_empty() {
                return Err(EquationError::Validation(format!(
                    "equation '{}' variable '{}' resolver.source cannot be empty",
                    eq.key, key
                )));
            }
            if resolver.property.trim().is_empty() {
                return Err(EquationError::Validation(format!(
                    "equation '{}' variable '{}' resolver.property cannot be empty",
                    eq.key, key
                )));
            }
        }
    }

    validate_expression_symbols_with_context(
        eq,
        "relation.residual",
        &eq.relation.residual,
        &expression_symbols,
    )?;

    if let Some(default_target) = &eq.solve.default_target
        && !symbol_set.contains(default_target)
    {
        let hint = suggestion_suffix(default_target, &symbol_names);
        return Err(EquationError::Validation(format!(
            "equation '{}' solve.default_target '{}' is not a variable{hint}",
            eq.key, default_target
        )));
    }
    if let Some(default_target) = &eq.solve.default_target
        && !eq.solve.explicit_forms.contains_key(default_target)
        && !is_numerically_supported(eq, default_target)
    {
        return Err(EquationError::Validation(format!(
            "equation '{}' solve.default_target '{}' is neither explicitly solvable nor numerically supported",
            eq.key, default_target
        )));
    }
    for (target, expr) in &eq.solve.explicit_forms {
        if !symbol_set.contains(target) {
            return Err(EquationError::Validation(format!(
                "equation '{}' explicit target '{}' is not a variable",
                eq.key, target
            )));
        }
        validate_expression_symbols_with_context(
            eq,
            &format!("solve.explicit_forms.{target}"),
            expr,
            &expression_symbols,
        )?;
    }
    for target in &eq.solve.numerical.unsupported_targets {
        if !symbol_set.contains(target) {
            return Err(EquationError::Validation(format!(
                "equation '{}' numerical unsupported target '{}' is not a variable",
                eq.key, target
            )));
        }
    }
    let mut branch_names = BTreeSet::new();
    for (branch_idx, branch) in eq.branches.iter().enumerate() {
        if !branch_names.insert(branch.name.clone()) {
            return Err(EquationError::Validation(format!(
                "equation '{}' has duplicate branch name '{}'",
                eq.key, branch.name
            )));
        }
        validate_expression_symbols_with_context(
            eq,
            &format!("branches[{branch_idx}].condition"),
            &branch.condition,
            &expression_symbols,
        )?;
    }
    let preferred_count = eq.branches.iter().filter(|b| b.preferred).count();
    if preferred_count > 1 {
        return Err(EquationError::Validation(format!(
            "equation '{}' has multiple preferred branches; at most one branch may set preferred=true",
            eq.key
        )));
    }
    for (case_idx, case) in eq.tests.cases.iter().enumerate() {
        if let Some(branch) = &case.branch
            && !eq.branches.iter().any(|b| &b.name == branch)
        {
            let hint = suggestion_suffix(
                branch,
                &eq.branches
                    .iter()
                    .map(|b| b.name.clone())
                    .collect::<Vec<_>>(),
            );
            return Err(EquationError::Validation(format!(
                "equation '{}' tests.cases[{case_idx}].branch references unknown branch '{}'{hint}",
                eq.key, branch
            )));
        }
        for key in case.full_state.keys() {
            if !symbol_set.contains(key) {
                let hint = suggestion_suffix(key, &symbol_names);
                return Err(EquationError::Validation(format!(
                    "equation '{}' tests.cases[{case_idx}].full_state.{} is unknown{hint}",
                    eq.key, key
                )));
            }
        }
        let missing_vars: Vec<String> = eq
            .variables
            .keys()
            .filter(|v| !case.full_state.contains_key(*v))
            .cloned()
            .collect();
        if !missing_vars.is_empty() {
            return Err(EquationError::Validation(format!(
                "equation '{}' tests.cases[{case_idx}].full_state must include all variables; missing: {}",
                eq.key,
                missing_vars.join(", ")
            )));
        }

        let resolved_targets = resolved_case_target_specs(eq, case);
        for solve_target in &resolved_targets {
            let target = solve_target.target();
            if !symbol_set.contains(target) {
                let hint = suggestion_suffix(target, &symbol_names);
                return Err(EquationError::Validation(format!(
                    "equation '{}' tests.cases[{case_idx}].verify.solve_targets target '{}' not found{hint}",
                    eq.key, target
                )));
            }
            let methods = resolved_target_methods(eq, target, solve_target);
            if methods.is_empty() {
                return Err(EquationError::Validation(format!(
                    "equation '{}' tests.cases[{case_idx}].verify.solve_targets target '{}' has no usable methods",
                    eq.key, target
                )));
            }
            for method in &methods {
                match method {
                    MethodKind::Explicit => {
                        if !eq.solve.explicit_forms.contains_key(target) {
                            return Err(EquationError::Validation(format!(
                                "equation '{}' tests.cases[{case_idx}] requested explicit solve for '{}' but no explicit form exists",
                                eq.key, target
                            )));
                        }
                    }
                    MethodKind::Numerical => {
                        if !is_numerically_supported(eq, target) {
                            return Err(EquationError::Validation(format!(
                                "equation '{}' tests.cases[{case_idx}] requested numerical solve for '{}' but target is numerically unsupported",
                                eq.key, target
                            )));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_expression_symbols_with_context(
    eq: &EquationDef,
    field_path: &str,
    expression: &str,
    valid_symbols: &[String],
) -> Result<()> {
    validate_expression_symbols(expression, valid_symbols).map_err(|err| match err {
        EquationError::UnknownSymbol { symbol, .. } => {
            let hint = suggestion_suffix(&symbol, valid_symbols);
            EquationError::Validation(format!(
                "equation '{}' {} references unknown symbol '{}'{hint}",
                eq.key, field_path, symbol
            ))
        }
        EquationError::ExpressionParse { message, .. } => EquationError::Validation(format!(
            "equation '{}' {} is not a valid expression: {}",
            eq.key, field_path, message
        )),
        other => other,
    })
}

fn suggestion_suffix(input: &str, candidates: &[String]) -> String {
    closest_candidate(input, candidates)
        .map(|s| format!(" (did you mean '{}'?)", s))
        .unwrap_or_default()
}

fn closest_candidate<'a>(input: &str, candidates: &'a [String]) -> Option<&'a str> {
    let mut best: Option<(&str, usize)> = None;
    for c in candidates {
        let d = levenshtein(input, c);
        if d <= 3 {
            match best {
                Some((_, bd)) if d >= bd => {}
                _ => best = Some((c.as_str(), d)),
            }
        }
    }
    best.map(|(s, _)| s)
}

fn levenshtein(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    if a.is_empty() {
        return b.chars().count();
    }
    if b.is_empty() {
        return a.chars().count();
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

fn is_valid_short_id(id: &str) -> bool {
    if id.contains('.') || id.trim().is_empty() {
        return false;
    }
    let mut chars = id.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn has_explicit_symbol(symbol: Option<&str>) -> bool {
    symbol.map(str::trim).is_some_and(|s| !s.is_empty())
}

fn is_greek_ambiguous_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    let greek_tokens = [
        "alpha", "beta", "gamma", "delta", "eps", "epsilon", "zeta", "eta", "theta", "iota",
        "kappa", "lambda", "mu", "nu", "xi", "pi", "rho", "sigma", "tau", "upsilon", "phi", "chi",
        "psi", "omega",
    ];
    greek_tokens
        .iter()
        .any(|t| lower == *t || lower.starts_with(&format!("{t}_")))
}
