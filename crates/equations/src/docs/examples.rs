use eng_core::units::{convert_equation_value_to_si, parse_equation_quantity_to_si};

use crate::{
    constants,
    model::{EquationDef, QuantityInput, TestCase},
    normalize::resolved_default_unit,
};

#[derive(Debug, Clone, serde::Serialize)]
pub struct EquationExample {
    pub label: String,
    pub style: String,
    pub code: String,
    pub target: Option<String>,
    pub signature: Option<String>,
    pub argument_order: Vec<ExampleArgument>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExampleArgument {
    pub name: String,
    pub description: String,
}

pub fn build_examples(eq: &EquationDef) -> Vec<EquationExample> {
    let mut out = Vec::new();
    let category_mod = normalize_snake(&eq.taxonomy.category);
    let slug = normalize_snake(eq.effective_slug());
    let target = primary_target(eq);
    let Some(target) = target else {
        return vec![fallback_example(eq)];
    };

    if let Some(primary_case) = primary_case(eq) {
        if let Some(si) =
            render_typed_builder(eq, primary_case, &category_mod, &slug, &target, true)
        {
            out.push(EquationExample {
                label: "typed_builder_si".to_string(),
                style: "typed_builder".to_string(),
                code: si,
                target: None,
                signature: None,
                argument_order: Vec::new(),
            });
        }
        if equation_has_meaningful_units(eq) {
            if let Some(units) =
                render_typed_builder(eq, primary_case, &category_mod, &slug, &target, false)
            {
                out.push(EquationExample {
                    label: "typed_builder_units".to_string(),
                    style: "typed_builder".to_string(),
                    code: units,
                    target: None,
                    signature: None,
                    argument_order: Vec::new(),
                });
            }
        }
        if has_resolvers(eq) {
            if let Some(context_example) =
                render_typed_context_builder(eq, primary_case, &category_mod, &slug, &target)
            {
                out.push(EquationExample {
                    label: "typed_builder_context".to_string(),
                    style: "typed_builder".to_string(),
                    code: context_example,
                    target: None,
                    signature: None,
                    argument_order: Vec::new(),
                });
            }
        }
        for convenience_target in convenience_targets(eq, &target) {
            if let Some((code, signature, argument_order)) =
                render_convenience(eq, primary_case, &category_mod, &slug, &convenience_target)
            {
                out.push(EquationExample {
                    label: format!("convenience_{}", normalize_snake(&convenience_target)),
                    style: "convenience".to_string(),
                    code,
                    target: Some(convenience_target),
                    signature: Some(signature),
                    argument_order,
                });
            }
        }
    }

    if let Some(branch_case) = branch_example_case(eq) {
        if let Some(code) =
            render_typed_builder(eq, branch_case, &category_mod, &slug, &target, false)
        {
            out.push(EquationExample {
                label: "typed_builder_branch".to_string(),
                style: "typed_builder".to_string(),
                code,
                target: None,
                signature: None,
                argument_order: Vec::new(),
            });
        }
    }

    if out.is_empty() {
        out.push(fallback_example(eq));
    }

    out
}

fn fallback_example(eq: &EquationDef) -> EquationExample {
    EquationExample {
        label: "typed_builder".to_string(),
        style: "typed_builder".to_string(),
        code: format!(
            "let value = eq\n    .solve({}::{}::equation())\n    .target_{}()\n    .given_<var>(<value>)\n    .value()?;",
            normalize_snake(&eq.taxonomy.category),
            normalize_snake(eq.effective_slug()),
            normalize_snake(
                &eq.solve
                    .default_target
                    .clone()
                    .unwrap_or_else(|| "target".to_string())
            )
        ),
        target: None,
        signature: None,
        argument_order: Vec::new(),
    }
}

fn primary_target(eq: &EquationDef) -> Option<String> {
    eq.solve
        .default_target
        .clone()
        .filter(|t| eq.variables.contains_key(t))
        .or_else(|| {
            eq.solve
                .explicit_forms
                .keys()
                .find(|t| eq.variables.contains_key(*t))
                .cloned()
        })
        .or_else(|| eq.variables.keys().next().cloned())
}

fn convenience_targets(eq: &EquationDef, primary_target: &str) -> Vec<String> {
    let mut targets: Vec<String> = eq
        .solve
        .explicit_forms
        .keys()
        .filter(|t| eq.variables.contains_key(*t))
        .cloned()
        .collect();
    if let Some(idx) = targets.iter().position(|t| t == primary_target) {
        if idx != 0 {
            targets.swap(0, idx);
        }
    }
    targets
}

fn primary_case(eq: &EquationDef) -> Option<&TestCase> {
    eq.tests
        .cases
        .iter()
        .find(|c| !looks_imperial(c))
        .or_else(|| eq.tests.cases.first())
}

fn looks_imperial(case: &TestCase) -> bool {
    case.full_state.values().any(|v| {
        matches!(
            v,
            QuantityInput::StringValue(s)
                if s.contains("psi")
                    || s.contains("in")
                    || s.contains("ft")
                    || s.contains("lbf")
                    || s.contains("lbm")
        )
    })
}

fn render_typed_builder(
    eq: &EquationDef,
    case: &TestCase,
    category_mod: &str,
    slug: &str,
    target: &str,
    prefer_si_numeric: bool,
) -> Option<String> {
    let givens = build_givens(eq, case, target)?;
    let mut code = String::new();
    code.push_str("let value = eq\n");
    code.push_str(&format!(
        "    .solve({}::{}::equation())\n",
        category_mod, slug
    ));
    code.push_str(&format!("    .target_{}()\n", normalize_snake(target)));
    if let Some(branch) = &case.branch {
        code.push_str(&format!("    .branch_{}()\n", normalize_snake(branch)));
    }
    for g in givens {
        let literal = if prefer_si_numeric {
            g.literal_si.unwrap_or(g.literal_units)
        } else {
            g.literal_units
        };
        code.push_str(&format!(
            "    .given_{}({})\n",
            normalize_snake(&g.var_key),
            literal
        ));
    }
    code.push_str("    .value()?;");
    Some(code)
}

fn render_convenience(
    eq: &EquationDef,
    case: &TestCase,
    category_mod: &str,
    slug: &str,
    target: &str,
) -> Option<(String, String, Vec<ExampleArgument>)> {
    let givens = build_givens(eq, case, target)?;
    let solve_fn = format!(
        "equations::{}::{}::solve_{}",
        category_mod,
        slug,
        normalize_snake(target)
    );
    let mut signature_args = Vec::new();
    let mut arg_docs = Vec::new();
    let mut args = String::new();
    for g in givens {
        signature_args.push(g.var_key.clone());
        arg_docs.push(ExampleArgument {
            name: g.var_key.clone(),
            description: g.var_name.clone(),
        });
        args.push_str(&format!("    {},\n", g.literal_units));
    }
    let signature = format!(
        "{solve_fn}({}) -> Result<f64, _>",
        signature_args.join(", ")
    );
    let call = format!("let value = {solve_fn}(\n{args})?;");
    Some((call, signature, arg_docs))
}

struct GivenValue {
    var_key: String,
    var_name: String,
    literal_units: String,
    literal_si: Option<String>,
}

fn build_givens(eq: &EquationDef, case: &TestCase, target: &str) -> Option<Vec<GivenValue>> {
    let mut out = Vec::new();
    for key in eq.variables.keys() {
        if key == target || is_auto_constant_var(key) {
            continue;
        }
        let var = eq.variables.get(key)?;
        if var.resolver.is_some() {
            continue;
        }
        let input = case.full_state.get(key)?;
        let default_unit = resolved_default_unit(&var.dimension, var.default_unit.as_deref())?;
        let literal_units = format_input_literal(input);
        let literal_si = to_si_number_literal(&var.dimension, &default_unit, input);
        out.push(GivenValue {
            var_key: key.clone(),
            var_name: var.name.clone(),
            literal_units,
            literal_si,
        });
    }
    Some(out)
}

fn has_resolvers(eq: &EquationDef) -> bool {
    eq.variables.values().any(|v| v.resolver.is_some())
}

fn render_typed_context_builder(
    eq: &EquationDef,
    case: &TestCase,
    category_mod: &str,
    slug: &str,
    target: &str,
) -> Option<String> {
    let givens = build_givens(eq, case, target)?;
    let mut code = String::new();
    code.push_str("let value = eq\n");
    code.push_str(&format!(
        "    .solve_with_context({}::{}::equation())\n",
        category_mod, slug
    ));

    let mut sources = std::collections::BTreeSet::new();
    for v in eq.variables.values() {
        if let Some(resolver) = &v.resolver {
            sources.insert(resolver.source.clone());
        }
    }
    for source in sources {
        let lowered = source.to_ascii_lowercase();
        if lowered == "fluid" {
            code.push_str("    .fluid(eng_fluids::water().state_tp(\"300 K\", \"1 bar\")?)\n");
        } else if lowered == "material" {
            code.push_str(
                "    .material(eng_materials::stainless_304().temperature(\"350 K\")?)\n",
            );
        } else if lowered.contains("wall") || lowered.contains("material") {
            code.push_str(&format!(
                "    .context(\"{}\", eng_materials::stainless_304().temperature(\"350 K\")?)\n",
                source
            ));
        } else {
            code.push_str(&format!(
                "    .context(\"{}\", eng_fluids::water().state_tp(\"300 K\", \"1 bar\")?)\n",
                source
            ));
        }
    }
    code.push_str(&format!("    .target_{}()\n", normalize_snake(target)));
    if let Some(branch) = &case.branch {
        code.push_str(&format!("    .branch_{}()\n", normalize_snake(branch)));
    }
    for g in givens {
        code.push_str(&format!(
            "    .given_{}({})\n",
            normalize_snake(&g.var_key),
            g.literal_units
        ));
    }
    code.push_str("    .value()?;");
    Some(code)
}

fn is_auto_constant_var(key: &str) -> bool {
    constants::get_by_identifier(key).is_some()
}

fn format_input_literal(input: &QuantityInput) -> String {
    match input {
        QuantityInput::Scalar(v) => format_float(*v),
        QuantityInput::StringValue(s) => format!("\"{}\"", escape_string(s)),
        QuantityInput::ValueUnit { value, unit } => {
            format!("\"{} {}\"", format_float(*value), escape_string(unit))
        }
    }
}

fn to_si_number_literal(
    dimension: &str,
    default_unit: &str,
    input: &QuantityInput,
) -> Option<String> {
    let value_si = match input {
        QuantityInput::Scalar(v) => {
            convert_equation_value_to_si(dimension, default_unit, *v).ok()?
        }
        QuantityInput::StringValue(s) => parse_equation_quantity_to_si(dimension, s).ok()?,
        QuantityInput::ValueUnit { value, unit } => {
            convert_equation_value_to_si(dimension, unit, *value).ok()?
        }
    };
    Some(format_float(value_si))
}

fn format_float(v: f64) -> String {
    if v == 0.0 {
        return "0.0".to_string();
    }
    if v.abs() >= 1.0e5 || v.abs() < 1.0e-3 {
        format!("{:e}", v)
    } else {
        let mut s = format!("{:.10}", v);
        while s.contains('.') && s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.push('0');
        }
        s
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn normalize_snake(input: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_us = false;
        } else if !prev_us {
            out.push('_');
            prev_us = true;
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() { "x".to_string() } else { out }
}

fn equation_has_meaningful_units(eq: &EquationDef) -> bool {
    eq.variables.iter().any(|(_, v)| {
        resolved_default_unit(&v.dimension, v.default_unit.as_deref()).is_some_and(|u| u != "1")
    })
}

fn branch_example_case(eq: &EquationDef) -> Option<&TestCase> {
    let primary_branch = primary_case(eq).and_then(|c| c.branch.as_deref());
    eq.tests.cases.iter().find(|c| {
        let Some(branch) = c.branch.as_deref() else {
            return false;
        };
        Some(branch) != primary_branch
    })
}
