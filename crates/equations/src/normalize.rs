use eng_core::units::default_unit_for_dimension;

use crate::{
    constants,
    defaults::{
        GLOBAL_NUMERICAL_MAX_ITER, GLOBAL_RELATION_TOL_ABS, GLOBAL_RELATION_TOL_REL,
        GLOBAL_SOLVE_TOL_ABS, GLOBAL_SOLVE_TOL_REL, constraint_defaults_for_dimension,
        default_methods_for_target, merge_tolerance, numerical_defaults_for_dimension,
    },
    model::{
        BranchDef, CaseTolerance, EquationDef, MethodKind, TestCase, TestSolveTargetSpec,
        VariableConstraint,
    },
};

#[derive(Debug, Clone)]
pub struct ResolvedDisplay {
    pub latex: String,
    pub unicode: String,
    pub ascii: String,
    pub description: String,
}

pub fn resolved_symbol(var_key: &str, symbol: Option<&str>) -> String {
    symbol
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(var_key)
        .to_string()
}

pub fn resolved_default_unit(dimension: &str, default_unit: Option<&str>) -> Option<String> {
    if let Some(unit) = default_unit.map(str::trim).filter(|u| !u.is_empty()) {
        return Some(unit.to_string());
    }
    default_unit_for_dimension(dimension).map(str::to_string)
}

pub fn resolved_constraints(dimension: &str, authored: &VariableConstraint) -> VariableConstraint {
    let defaults = constraint_defaults_for_dimension(dimension);
    VariableConstraint {
        positive: defaults.positive || authored.positive,
        nonzero: defaults.nonzero || authored.nonzero,
        integer: defaults.integer || authored.integer,
        min: authored.min.or(defaults.min),
        max: authored.max.or(defaults.max),
    }
}

pub fn is_numerically_supported(eq: &EquationDef, target: &str) -> bool {
    !eq.solve
        .numerical
        .unsupported_targets
        .iter()
        .any(|t| t == target)
}

pub fn resolved_target_methods(
    eq: &EquationDef,
    target: &str,
    spec: &TestSolveTargetSpec,
) -> Vec<MethodKind> {
    let defaults = default_methods_for_target(
        eq.solve.explicit_forms.contains_key(target),
        is_numerically_supported(eq, target),
    );
    spec.methods_or(&defaults).to_vec()
}

pub fn default_case_target_specs(eq: &EquationDef) -> Vec<TestSolveTargetSpec> {
    eq.variables
        .keys()
        .filter_map(|target| {
            let methods = default_methods_for_target(
                eq.solve.explicit_forms.contains_key(target),
                is_numerically_supported(eq, target),
            );
            (!methods.is_empty()).then(|| TestSolveTargetSpec::TargetOnly(target.clone()))
        })
        .collect()
}

pub fn resolved_case_target_specs(eq: &EquationDef, case: &TestCase) -> Vec<TestSolveTargetSpec> {
    if case.verify.solve_targets.is_empty() {
        return default_case_target_specs(eq);
    }
    case.verify.solve_targets.clone()
}

pub fn resolved_relation_tolerance(eq: &EquationDef) -> (f64, f64) {
    let abs = eq
        .tests
        .relation_tolerance
        .as_ref()
        .map(|t| t.abs)
        .unwrap_or(GLOBAL_RELATION_TOL_ABS);
    let rel = eq
        .tests
        .relation_tolerance
        .as_ref()
        .map(|t| t.rel)
        .unwrap_or(GLOBAL_RELATION_TOL_REL);
    (abs, rel)
}

pub fn resolved_target_tolerance(
    eq: &EquationDef,
    case_level: Option<&CaseTolerance>,
    target_level: Option<&CaseTolerance>,
    methods: &[MethodKind],
) -> (f64, f64) {
    let (mut abs, mut rel) =
        merge_tolerance(eq.tests.solve_tolerance.as_ref(), case_level, target_level);
    if methods.iter().any(|m| matches!(m, MethodKind::Numerical)) {
        abs = abs.max(
            eq.solve
                .numerical
                .tolerance_abs
                .unwrap_or(GLOBAL_SOLVE_TOL_ABS),
        );
        rel = rel.max(
            eq.solve
                .numerical
                .tolerance_rel
                .unwrap_or(GLOBAL_SOLVE_TOL_REL),
        );
    }
    (abs, rel)
}

pub fn resolved_numerical_bracket(eq: &EquationDef, target_dimension: &str) -> (f64, f64) {
    if let Some([a, b]) = eq.solve.numerical.bracket {
        return (a, b);
    }
    numerical_defaults_for_dimension(target_dimension).bracket
}

pub fn resolved_numerical_initial_guess(eq: &EquationDef, target_dimension: &str) -> f64 {
    eq.solve
        .numerical
        .initial_guess
        .unwrap_or(numerical_defaults_for_dimension(target_dimension).initial_guess)
}

pub fn resolved_numerical_max_iter(eq: &EquationDef, target_dimension: &str) -> u32 {
    eq.solve
        .numerical
        .max_iter
        .unwrap_or(numerical_defaults_for_dimension(target_dimension).max_iter)
        .max(GLOBAL_NUMERICAL_MAX_ITER / 3)
}

pub fn resolved_diagram_labels(eq: &EquationDef) -> Vec<(String, String)> {
    let Some(diagram) = &eq.diagram else {
        return Vec::new();
    };
    if !diagram.labels.is_empty() {
        return diagram
            .labels
            .iter()
            .map(|l| (l.variable.clone(), l.label.clone()))
            .collect();
    }
    eq.variables
        .iter()
        .map(|(key, var)| (key.clone(), var.name.clone()))
        .collect()
}

pub fn preferred_branch(branches: &[BranchDef]) -> Option<&BranchDef> {
    branches
        .iter()
        .find(|b| b.preferred)
        .or_else(|| branches.first())
}

pub fn resolved_display(eq: &EquationDef) -> ResolvedDisplay {
    let latex = eq.display.latex.trim().to_string();
    let source = canonical_display_source(eq);
    let ascii = eq
        .display
        .ascii
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| normalize_ascii_expression(&source));
    let unicode = eq
        .display
        .unicode
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| derive_unicode_expression(eq, &source));
    let description = eq
        .display
        .description
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("")
        .to_string();
    ResolvedDisplay {
        latex,
        unicode,
        ascii,
        description,
    }
}

fn canonical_display_source(eq: &EquationDef) -> String {
    if let Some(default_target) = &eq.solve.default_target
        && let Some(explicit) = eq.solve.explicit_forms.get(default_target)
    {
        return format!("{default_target} = {explicit}");
    }
    if let Some(residual_primary) = first_identifier(&eq.relation.residual)
        && let Some(explicit) = eq.solve.explicit_forms.get(&residual_primary)
    {
        return format!("{residual_primary} = {explicit}");
    }
    if !eq.solve.explicit_forms.is_empty() {
        let mut pairs: Vec<(&String, &String)> = eq.solve.explicit_forms.iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));
        let (target, expr) = pairs[0];
        return format!("{target} = {expr}");
    }
    format!("{} = 0", eq.relation.residual)
}

fn normalize_ascii_expression(expr: &str) -> String {
    let mut out = String::new();
    let mut prev_space = false;
    for c in expr.chars() {
        let spaced = matches!(c, '+' | '-' | '*' | '/' | '=');
        if spaced {
            if !out.ends_with(' ') && !out.is_empty() {
                out.push(' ');
            }
            out.push(c);
            out.push(' ');
            prev_space = true;
        } else if c.is_whitespace() {
            if !prev_space && !out.is_empty() {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(c);
            prev_space = false;
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_identifier(expr: &str) -> Option<String> {
    let mut token = String::new();
    let mut started = false;
    for c in expr.chars() {
        if c.is_ascii_alphabetic() || (!started && c == '_') {
            token.push(c);
            started = true;
            continue;
        }
        if started && (c.is_ascii_alphanumeric() || c == '_') {
            token.push(c);
            continue;
        }
        if started {
            break;
        }
    }
    (!token.is_empty()).then_some(token)
}

fn derive_unicode_expression(eq: &EquationDef, source: &str) -> String {
    let mut text = replace_identifiers_with_symbols(eq, source);
    text = text.replace("sqrt(", "\u{221A}(");
    text = text.replace('*', "\u{00B7}");
    text = normalize_unicode_superscripts(&text);
    normalize_ascii_expression(&text)
}

fn replace_identifiers_with_symbols(eq: &EquationDef, source: &str) -> String {
    let mut out = String::new();
    let mut token = String::new();
    for c in source.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            token.push(c);
            continue;
        }
        if !token.is_empty() {
            out.push_str(&resolved_symbol_unicode(eq, &token));
            token.clear();
        }
        out.push(c);
    }
    if !token.is_empty() {
        out.push_str(&resolved_symbol_unicode(eq, &token));
    }
    out
}

fn resolved_symbol_unicode(eq: &EquationDef, token: &str) -> String {
    let Some(var) = eq.variables.get(token) else {
        if let Some(c) = constants::get_by_identifier(token) {
            return c.symbol_unicode.to_string();
        }
        return token.to_string();
    };
    let symbol = resolved_symbol(token, var.symbol.as_deref());
    latex_like_symbol_to_unicode(&symbol)
}

fn latex_like_symbol_to_unicode(symbol: &str) -> String {
    let mut s = symbol.to_string();
    let table = [
        ("\\sigma", "\u{03C3}"),
        ("\\gamma", "\u{03B3}"),
        ("\\varepsilon", "\u{03B5}"),
        ("\\rho", "\u{03C1}"),
        ("\\mu", "\u{03BC}"),
        ("\\theta", "\u{03B8}"),
        ("\\alpha", "\u{03B1}"),
        ("\\beta", "\u{03B2}"),
        ("\\Delta", "\u{0394}"),
    ];
    for (latex, uni) in table {
        s = s.replace(latex, uni);
    }
    s
}

fn normalize_unicode_superscripts(input: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '^' && i + 1 < chars.len() {
            if let Some(sup) = to_superscript(chars[i + 1]) {
                out.push(sup);
                i += 2;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn to_superscript(c: char) -> Option<char> {
    match c {
        '0' => Some('\u{2070}'),
        '1' => Some('\u{00B9}'),
        '2' => Some('\u{00B2}'),
        '3' => Some('\u{00B3}'),
        '4' => Some('\u{2074}'),
        '5' => Some('\u{2075}'),
        '6' => Some('\u{2076}'),
        '7' => Some('\u{2077}'),
        '8' => Some('\u{2078}'),
        '9' => Some('\u{2079}'),
        _ => None,
    }
}
