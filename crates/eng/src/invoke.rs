use crate::bindings::{
    INVOKE_PROTOCOL_VERSION, INVOKE_SUPPORTED_OPS, InvokeRequest, InvokeResponse,
};
use eng_core::units::convert_equation_value_from_si;
use equations::normalize::{resolved_default_unit, resolved_display, resolved_symbol};
use serde_json::{Map, Value, json};

pub fn process_invoke_json(req_json: &str) -> InvokeResponse {
    let req: InvokeRequest = match serde_json::from_str(req_json) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                "unknown",
                None,
                "invalid_request_json",
                format!("invalid request json: {e}"),
                None,
                None,
            );
        }
    };
    handle_invoke(req)
}

pub fn process_invoke_json_to_string(req_json: &str) -> Result<String, serde_json::Error> {
    let resp = process_invoke_json(req_json);
    serde_json::to_string(&resp)
}

pub fn handle_invoke(req: InvokeRequest) -> InvokeResponse {
    if req.protocol_version != INVOKE_PROTOCOL_VERSION {
        return InvokeResponse::err(
            req.op,
            req.request_id,
            "protocol_version_mismatch",
            format!(
                "unsupported protocol version '{}'; expected '{}'",
                req.protocol_version, INVOKE_PROTOCOL_VERSION
            ),
            Some("protocol_version"),
            None,
        );
    }
    let op = req.op.clone();
    let request_id = req.request_id.clone();
    let args = if req.args.is_object() {
        req.args
    } else {
        Value::Object(Map::new())
    };
    match op.as_str() {
        "equation.solve" => invoke_equation_solve(&op, request_id, &args),
        "equation.meta" => invoke_equation_meta(&op, request_id, &args),
        "equation.ascii" => invoke_equation_ascii(&op, request_id, &args),
        "equation.default_unit" => invoke_equation_default_unit(&op, request_id, &args),
        "equation.unicode" => invoke_equation_unicode(&op, request_id, &args),
        "equation.latex" => invoke_equation_latex(&op, request_id, &args),
        "equation.targets" => invoke_equation_targets(&op, request_id, &args),
        "equation.variables" => invoke_equation_variables(&op, request_id, &args),
        "equation.name" => invoke_equation_name(&op, request_id, &args),
        "equation.description" => invoke_equation_description(&op, request_id, &args),
        "equation.family" => invoke_equation_family(&op, request_id, &args),
        "format.value" => invoke_format_value(&op, request_id, &args),
        "meta.get" => invoke_meta_get(&op, request_id, &args),
        "device.pipe_loss.solve_delta_p" => invoke_pipe_loss(&op, request_id, &args),
        "fluid.prop" => invoke_fluid_prop(&op, request_id, &args),
        "material.prop" => invoke_material_prop(&op, request_id, &args),
        "constant.get" => invoke_constant_get(&op, request_id, &args),
        _ => InvokeResponse::err(
            op,
            request_id,
            "unknown_operation",
            "unsupported invoke operation",
            Some("op"),
            Some(json!({ "supported": INVOKE_SUPPORTED_OPS })),
        ),
    }
}

fn invoke_equation_solve(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let target = match req_str(args, "target") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("target"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let mut ci_vars: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for var in equation.variables.keys() {
        ci_vars.insert(var.to_ascii_lowercase(), var.clone());
    }

    let mut builder = crate::eq.solve(path_id).for_target(target);
    if let Some(obj) = args.as_object() {
        for (k, v) in obj {
            if k == "path_id" || k == "target" {
                continue;
            }
            let canonical = ci_vars
                .get(&k.to_ascii_lowercase())
                .cloned()
                .unwrap_or_else(|| k.clone());
            if v.is_null() {
                continue;
            }
            if let Some(n) = v.as_f64() {
                builder = builder.given(canonical.clone(), n);
            } else if let Some(s) = v.as_str() {
                builder = builder.given(canonical.clone(), s.to_string());
            } else {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "invalid_arg_type",
                    format!("unsupported input type for '{canonical}'"),
                    Some(&canonical),
                    None,
                );
            }
        }
    }
    match builder.value() {
        Ok(v) => InvokeResponse::ok(op, request_id, json!(v)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "equation_solve_failed",
            e.to_string(),
            None,
            Some(json!({"path_id": path_id, "target": target })),
        ),
    }
}

fn invoke_equation_meta(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let display = resolved_display(equation);
    let variables = equation
        .variables
        .iter()
        .map(|(key, v)| {
            json!({
                "key": key,
                "name": v.name,
                "symbol": resolved_symbol(key, v.symbol.as_deref()),
                "dimension": v.dimension,
                "default_unit": resolved_default_unit(&v.dimension, v.default_unit.as_deref()).unwrap_or_else(|| "?".to_string()),
                "description": v.description.clone().unwrap_or_default(),
                "aliases": v.aliases,
            })
        })
        .collect::<Vec<_>>();
    let solve_targets = equation
        .variables
        .keys()
        .filter_map(|target| {
            let mut methods = Vec::new();
            if equation.solve.explicit_forms.contains_key(target) {
                methods.push("explicit");
            }
            if equations::normalize::is_numerically_supported(equation, target) {
                methods.push("numerical");
            }
            if methods.is_empty() {
                return None;
            }
            Some(json!({
                "target": target,
                "methods": methods,
            }))
        })
        .collect::<Vec<_>>();
    let branches = equation
        .branches
        .iter()
        .map(|b| {
            json!({
                "name": b.name,
                "condition": b.condition,
                "preferred": b.preferred,
            })
        })
        .collect::<Vec<_>>();
    let references = equation
        .references
        .iter()
        .map(|r| json!({ "source": r.source, "note": r.note }))
        .collect::<Vec<_>>();
    let source = equation
        .source
        .as_ref()
        .map(|s| json!({ "source": s.source, "note": s.note }));

    InvokeResponse::ok(
        op,
        request_id,
        json!({
            "path_id": path_id,
            "key": equation.key,
            "name": equation.name,
            "category": equation.taxonomy.category,
            "subcategories": equation.taxonomy.subcategories,
            "default_target": equation.solve.default_target,
            "display": {
                "latex": display.latex,
                "unicode": display.unicode,
                "ascii": display.ascii,
                "description": display.description,
            },
            "variables": variables,
            "solve_targets": solve_targets,
            "branches": branches,
            "assumptions": equation.assumptions,
            "source": source,
            "references": references,
        }),
    )
}

fn invoke_equation_ascii(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let display = resolved_display(equation);
    InvokeResponse::ok(op, request_id, json!(display.ascii))
}

fn invoke_equation_default_unit(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let variable = match req_str(args, "variable") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("variable"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let mut ci_vars: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for var in equation.variables.keys() {
        ci_vars.insert(var.to_ascii_lowercase(), var.clone());
    }
    let canonical = ci_vars
        .get(&variable.to_ascii_lowercase())
        .cloned()
        .unwrap_or_else(|| variable.to_string());
    let Some(var_def) = equation.variables.get(&canonical) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_variable",
            format!("unknown variable '{variable}' for equation '{path_id}'"),
            Some("variable"),
            Some(json!({ "path_id": path_id })),
        );
    };
    let default_unit = resolved_default_unit(&var_def.dimension, var_def.default_unit.as_deref())
        .unwrap_or_else(|| "?".to_string());
    InvokeResponse::ok(op, request_id, json!(default_unit))
}

fn invoke_equation_unicode(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let display = resolved_display(equation);
    InvokeResponse::ok(op, request_id, json!(display.unicode))
}

fn invoke_equation_latex(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let display = resolved_display(equation);
    InvokeResponse::ok(op, request_id, json!(display.latex))
}

fn invoke_equation_targets(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let targets = equation
        .variables
        .keys()
        .filter_map(|target| {
            let explicit = equation.solve.explicit_forms.contains_key(target);
            let numerical = equations::normalize::is_numerically_supported(equation, target);
            if !explicit && !numerical {
                return None;
            }
            Some(json!({
                "target": target,
                "explicit": explicit,
                "numerical": numerical,
            }))
        })
        .collect::<Vec<_>>();
    InvokeResponse::ok(op, request_id, json!(targets))
}

fn invoke_equation_variables(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let vars = equation
        .variables
        .iter()
        .map(|(key, v)| {
            json!({
                "key": key,
                "name": v.name,
                "symbol": resolved_symbol(key, v.symbol.as_deref()),
                "dimension": v.dimension,
                "default_unit": resolved_default_unit(&v.dimension, v.default_unit.as_deref()).unwrap_or_else(|| "?".to_string()),
            })
        })
        .collect::<Vec<_>>();
    InvokeResponse::ok(op, request_id, json!(vars))
}

fn invoke_equation_name(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    match crate::eq.equation(path_id) {
        Ok(eq) => InvokeResponse::ok(op, request_id, json!(eq.name.clone())),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "unknown_equation",
            e.to_string(),
            Some("path_id"),
            None,
        ),
    }
}

fn invoke_equation_description(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let equation = match crate::eq.equation(path_id) {
        Ok(eq) => eq,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_equation",
                e.to_string(),
                Some("path_id"),
                None,
            );
        }
    };
    let display = resolved_display(equation);
    InvokeResponse::ok(op, request_id, json!(display.description))
}

fn invoke_equation_family(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let path_id = match req_str(args, "path_id") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("path_id"), None);
        }
    };
    let registry = match equations::Registry::load_default() {
        Ok(r) => r,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "registry_load_failed",
                e.to_string(),
                None,
                None,
            );
        }
    };
    let families = match equations::equation_families::load_default_validated(registry.equations()) {
        Ok(f) => f,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "family_registry_error",
                e.to_string(),
                None,
                None,
            );
        }
    };
    for family in families {
        for variant in &family.variants {
            if variant.equation_id == path_id {
                return InvokeResponse::ok(
                    op,
                    request_id,
                    json!({
                        "family_key": family.key,
                        "family_name": family.name,
                        "variant_key": variant.key,
                        "variant_name": variant.name,
                    }),
                );
            }
        }
    }
    InvokeResponse::ok(op, request_id, Value::Null)
}

fn invoke_format_value(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let value = match req_f64(args, "value") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("value"), None);
        }
    };
    let quantity = match req_str(args, "quantity") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("quantity"), None);
        }
    };
    let out_unit = match req_str(args, "out_unit") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("out_unit"), None);
        }
    };
    let in_unit = opt_str(args, "in_unit");
    let mode = opt_str(args, "mode").unwrap_or("value");
    let dimension = canonical_dimension(quantity);

    let value_si = if let Some(u) = in_unit {
        match eng_core::units::convert_equation_value_to_si(&dimension, u, value) {
            Ok(v) => v,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "format_conversion_error",
                    e.to_string(),
                    Some("in_unit"),
                    Some(json!({ "quantity": quantity, "dimension": dimension })),
                );
            }
        }
    } else {
        value
    };

    let converted = match convert_equation_value_from_si(&dimension, out_unit, value_si) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "format_conversion_error",
                e.to_string(),
                Some("out_unit"),
                Some(json!({ "quantity": quantity, "dimension": dimension })),
            );
        }
    };

    match mode.to_ascii_lowercase().as_str() {
        "value" => InvokeResponse::ok(op, request_id, json!(converted)),
        "text" => InvokeResponse::ok(op, request_id, json!(format!("{converted} {out_unit}"))),
        other => InvokeResponse::err(
            op,
            request_id,
            "invalid_arg_value",
            format!("unsupported mode '{other}' (use 'value' or 'text')"),
            Some("mode"),
            None,
        ),
    }
}

fn invoke_meta_get(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let entity = match req_str(args, "entity") {
        Ok(v) => v.to_ascii_lowercase(),
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("entity"), None);
        }
    };
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None);
        }
    };
    let field = match req_str(args, "field") {
        Ok(v) => v.to_ascii_lowercase(),
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("field"), None);
        }
    };

    match entity.as_str() {
        "equation" => {
            let eq = match crate::eq.equation(key) {
                Ok(v) => v,
                Err(e) => {
                    return InvokeResponse::err(
                        op,
                        request_id,
                        "unknown_equation",
                        e.to_string(),
                        Some("key"),
                        None,
                    );
                }
            };
            let display = resolved_display(eq);
            let value = match field.as_str() {
                "name" => json!(eq.name),
                "ascii" => json!(display.ascii),
                "unicode" => json!(display.unicode),
                "latex" => json!(display.latex),
                "description" => json!(display.description),
                "variables" => json!(eq.variables.keys().cloned().collect::<Vec<_>>()),
                "targets" => json!(
                    eq.variables
                        .keys()
                        .filter(|target| {
                            eq.solve.explicit_forms.contains_key(*target)
                                || equations::normalize::is_numerically_supported(eq, target)
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                ),
                _ => {
                    return InvokeResponse::err(
                        op,
                        request_id,
                        "unknown_meta_field",
                        format!("unsupported equation field '{field}'"),
                        Some("field"),
                        None,
                    );
                }
            };
            InvokeResponse::ok(op, request_id, value)
        }
        "constant" => match crate::equations::get_constant(key) {
            Some(c) => {
                let value = match field.as_str() {
                    "name" => json!(c.name),
                    "value" => json!(c.value),
                    "unit" => json!(c.unit),
                    "symbol_ascii" => json!(c.symbol_ascii),
                    "symbol_unicode" => json!(c.symbol_unicode),
                    "symbol_latex" => json!(c.symbol_latex),
                    _ => {
                        return InvokeResponse::err(
                            op,
                            request_id,
                            "unknown_meta_field",
                            format!("unsupported constant field '{field}'"),
                            Some("field"),
                            None,
                        );
                    }
                };
                InvokeResponse::ok(op, request_id, value)
            }
            None => InvokeResponse::err(
                op,
                request_id,
                "unknown_constant",
                format!("unknown constant '{key}'"),
                Some("key"),
                None,
            ),
        },
        "fluid" => {
            let entries = eng_fluids::docs_entries();
            if let Some(f) = entries.iter().find(|f| {
                f.key.eq_ignore_ascii_case(key)
                    || f.aliases.iter().any(|a| a.eq_ignore_ascii_case(key))
            }) {
                let value = match field.as_str() {
                    "name" => json!(f.name),
                    "aliases" => json!(f.aliases),
                    "supported_properties" => json!(f.supported_properties),
                    "supported_state_inputs" => json!(f.supported_state_inputs),
                    _ => {
                        return InvokeResponse::err(
                            op,
                            request_id,
                            "unknown_meta_field",
                            format!("unsupported fluid field '{field}'"),
                            Some("field"),
                            None,
                        );
                    }
                };
                InvokeResponse::ok(op, request_id, value)
            } else {
                InvokeResponse::err(
                    op,
                    request_id,
                    "unknown_fluid",
                    format!("unknown fluid '{key}'"),
                    Some("key"),
                    None,
                )
            }
        }
        "material" => {
            let entries = match eng_materials::docs_entries() {
                Ok(v) => v,
                Err(e) => {
                    return InvokeResponse::err(
                        op,
                        request_id,
                        "material_docs_error",
                        e.to_string(),
                        None,
                        None,
                    );
                }
            };
            if let Some(m) = entries.iter().find(|m| {
                m.key.eq_ignore_ascii_case(key)
                    || m.aliases.iter().any(|a| a.eq_ignore_ascii_case(key))
            }) {
                let value = match field.as_str() {
                    "name" => json!(m.name),
                    "aliases" => json!(m.aliases),
                    "properties" => json!(m.properties),
                    "source" => json!(m.source),
                    "description" => json!(m.description),
                    _ => {
                        return InvokeResponse::err(
                            op,
                            request_id,
                            "unknown_meta_field",
                            format!("unsupported material field '{field}'"),
                            Some("field"),
                            None,
                        );
                    }
                };
                InvokeResponse::ok(op, request_id, value)
            } else {
                InvokeResponse::err(
                    op,
                    request_id,
                    "unknown_material",
                    format!("unknown material '{key}'"),
                    Some("key"),
                    None,
                )
            }
        }
        "device" => {
            let entries = crate::devices::docs_entries();
            if let Some(d) = entries.iter().find(|d| d.key.eq_ignore_ascii_case(key)) {
                let value = match field.as_str() {
                    "name" => json!(d.name),
                    "summary" => json!(d.summary),
                    "supported_modes" => json!(d.supported_modes),
                    "outputs" => json!(d.outputs),
                    _ => {
                        return InvokeResponse::err(
                            op,
                            request_id,
                            "unknown_meta_field",
                            format!("unsupported device field '{field}'"),
                            Some("field"),
                            None,
                        );
                    }
                };
                InvokeResponse::ok(op, request_id, value)
            } else {
                InvokeResponse::err(
                    op,
                    request_id,
                    "unknown_device",
                    format!("unknown device '{key}'"),
                    Some("key"),
                    None,
                )
            }
        }
        _ => InvokeResponse::err(
            op,
            request_id,
            "unknown_meta_entity",
            format!("unsupported metadata entity '{entity}'"),
            Some("entity"),
            None,
        ),
    }
}

fn canonical_dimension(input: &str) -> String {
    match input.trim().to_ascii_lowercase().replace(' ', "_").as_str() {
        "pressure" | "stress" => "pressure".to_string(),
        "temp" | "temperature" => "temperature".to_string(),
        "density" | "rho" => "density".to_string(),
        "mu" | "viscosity" | "dynamic_viscosity" => "viscosity".to_string(),
        "k" | "thermal_conductivity" => "thermal_conductivity".to_string(),
        "h" | "heat_transfer_coefficient" => "heat_transfer_coefficient".to_string(),
        "cp" | "specific_heat_capacity" => "specific_heat_capacity".to_string(),
        "cv" | "specific_heat_capacity_cv" => "specific_heat_capacity_cv".to_string(),
        "mass_flow" | "mass_flow_rate" => "mass_flow_rate".to_string(),
        "vol_flow" | "volumetric_flow_rate" => "volumetric_flow_rate".to_string(),
        "length" | "diameter" | "distance" | "roughness" => "length".to_string(),
        "area" => "area".to_string(),
        "volume" => "volume".to_string(),
        "force" => "force".to_string(),
        "moment" => "moment".to_string(),
        "dimensionless" | "ratio" | "friction_factor" => "dimensionless".to_string(),
        other => other.to_string(),
    }
}

fn invoke_pipe_loss(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    use crate::devices::{PipeFrictionModel, pipe_loss};

    let model = match opt_str(args, "friction_model").unwrap_or("Colebrook") {
        "Colebrook" | "colebrook" => PipeFrictionModel::Colebrook,
        "Fixed" | "fixed" => match req_f64(args, "fixed_f") {
            Ok(v) => PipeFrictionModel::Fixed(v),
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "missing_arg",
                    e,
                    Some("fixed_f"),
                    None,
                );
            }
        },
        other => {
            return InvokeResponse::err(
                op,
                request_id,
                "invalid_arg_value",
                format!("invalid friction_model '{other}' (use Colebrook or Fixed)"),
                Some("friction_model"),
                None,
            );
        }
    };
    let mut device = pipe_loss().friction_model(model);
    if let Some(s) = opt_str(args, "rho") {
        device = device.given_rho(s);
    }
    if let Some(s) = opt_str(args, "mu") {
        device = device.given_mu(s);
    }
    if let Some(s) = opt_str(args, "v") {
        device = device.given_v(s);
    }
    if let Some(s) = opt_str(args, "d") {
        device = device.given_d(s);
    }
    if let Some(s) = opt_str(args, "l") {
        device = device.given_l(s);
    }
    if let Some(s) = opt_str(args, "eps") {
        device = device.given_eps(s);
    }

    if let Some(fluid_name) = opt_str(args, "fluid") {
        let in1_key = match req_str(args, "in1_key") {
            Ok(v) => v,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "missing_arg",
                    e,
                    Some("in1_key"),
                    None,
                );
            }
        };
        let in1_value = match req_str(args, "in1_value") {
            Ok(v) => v,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "missing_arg",
                    e,
                    Some("in1_value"),
                    None,
                );
            }
        };
        let in2_key = match req_str(args, "in2_key") {
            Ok(v) => v,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "missing_arg",
                    e,
                    Some("in2_key"),
                    None,
                );
            }
        };
        let in2_value = match req_str(args, "in2_value") {
            Ok(v) => v,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "missing_arg",
                    e,
                    Some("in2_value"),
                    None,
                );
            }
        };
        let Some(fluid_ref) = crate::fluids::find(fluid_name) else {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_fluid",
                format!("unknown fluid '{fluid_name}'"),
                Some("fluid"),
                None,
            );
        };
        let state = match fluid_ref.state(in1_key, in1_value, in2_key, in2_value) {
            Ok(s) => s,
            Err(e) => {
                return InvokeResponse::err(
                    op,
                    request_id,
                    "fluid_state_error",
                    e.to_string(),
                    Some("fluid"),
                    None,
                );
            }
        };
        device = device.fluid(state);
    }

    match device.solve() {
        Ok(r) => InvokeResponse::ok(
            op,
            request_id,
            json!({
                "delta_p": r.delta_p_pa,
                "friction_factor": r.friction_factor,
                "reynolds_number": r.reynolds_number,
            }),
        ),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_solve_failed",
            e.to_string(),
            None,
            Some(json!({ "device": "pipe_loss" })),
        ),
    }
}

fn invoke_fluid_prop(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let fluid_name = match req_str(args, "fluid") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("fluid"), None);
        }
    };
    let in1_key = match req_str(args, "in1_key") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("in1_key"), None);
        }
    };
    let in1_value = match req_str(args, "in1_value") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("in1_value"), None);
        }
    };
    let in2_key = match req_str(args, "in2_key") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("in2_key"), None);
        }
    };
    let in2_value = match req_str(args, "in2_value") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("in2_value"), None);
        }
    };
    let out_prop = match req_str(args, "out_prop") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("out_prop"), None);
        }
    };
    let Some(fluid_ref) = crate::fluids::find(fluid_name) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_fluid",
            format!("unknown fluid '{fluid_name}'"),
            Some("fluid"),
            None,
        );
    };
    let state = match fluid_ref.state(in1_key, in1_value, in2_key, in2_value) {
        Ok(s) => s,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "fluid_state_error",
                e.to_string(),
                None,
                None,
            );
        }
    };
    match state.property_by_name(out_prop) {
        Ok(v) => InvokeResponse::ok(op, request_id, json!(v)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "fluid_property_error",
            e.to_string(),
            Some("out_prop"),
            None,
        ),
    }
}

fn invoke_material_prop(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let material = match req_str(args, "material") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("material"), None);
        }
    };
    let property = match req_str(args, "property") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("property"), None);
        }
    };
    let temperature = match req_str(args, "temperature") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "missing_arg",
                e,
                Some("temperature"),
                None,
            );
        }
    };
    let mat = match crate::materials::get(material) {
        Ok(m) => m,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_material",
                e.to_string(),
                Some("material"),
                None,
            );
        }
    };
    let state = match mat.temperature(temperature) {
        Ok(s) => s,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "material_state_error",
                e.to_string(),
                Some("temperature"),
                None,
            );
        }
    };
    match state.property(property) {
        Ok(v) => InvokeResponse::ok(op, request_id, json!(v)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "material_property_error",
            e.to_string(),
            Some("property"),
            None,
        ),
    }
}

fn invoke_constant_get(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    match crate::equations::get_constant(key) {
        Some(c) => InvokeResponse::ok(op, request_id, json!(c.value)),
        None => InvokeResponse::err(
            op,
            request_id,
            "unknown_constant",
            format!("unknown constant '{key}'"),
            Some("key"),
            None,
        ),
    }
}

fn req_str<'a>(obj: &'a Value, key: &str) -> Result<&'a str, String> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string arg '{key}'"))
}

fn opt_str<'a>(obj: &'a Value, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(Value::as_str)
}

fn req_f64(obj: &Value, key: &str) -> Result<f64, String> {
    obj.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("missing numeric arg '{key}'"))
}
