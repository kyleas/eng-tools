use crate::bindings::{
    INVOKE_PROTOCOL_VERSION, INVOKE_SUPPORTED_OPS, InvokeRequest, InvokeResponse,
};
use equations::model::EquationDef;
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
        "equation.targets.text" => invoke_equation_targets_text(&op, request_id, &args),
        "equation.targets.table" => invoke_equation_targets_table(&op, request_id, &args),
        "equation.target.count" => invoke_equation_target_count(&op, request_id, &args),
        "equation.branches.text" => invoke_equation_branches_text(&op, request_id, &args),
        "equation.branches.table" => invoke_equation_branches_table(&op, request_id, &args),
        "equation.variables.text" => invoke_equation_variables_text(&op, request_id, &args),
        "equation.variables.table" => invoke_equation_variables_table(&op, request_id, &args),
        "equation.variable.count" => invoke_equation_variable_count(&op, request_id, &args),
        "format.value" => invoke_format_value(&op, request_id, &args),
        "meta.get" => invoke_meta_get(&op, request_id, &args),
        "fluid.properties.text" => invoke_fluid_properties_text(&op, request_id, &args),
        "fluid.properties.table" => invoke_fluid_properties_table(&op, request_id, &args),
        "fluid.property.count" => invoke_fluid_property_count(&op, request_id, &args),
        "material.properties.text" => invoke_material_properties_text(&op, request_id, &args),
        "material.properties.table" => invoke_material_properties_table(&op, request_id, &args),
        "material.property.count" => invoke_material_property_count(&op, request_id, &args),
        "device.modes.text" => invoke_device_modes_text(&op, request_id, &args),
        "device.mode.count" => invoke_device_mode_count(&op, request_id, &args),
        "device.isentropic_calc" => invoke_isentropic_calc(&op, request_id, &args),
        "device.isentropic_calc.value" => invoke_isentropic_calc_value(&op, request_id, &args),
        "device.isentropic_calc.pivot_mach" => {
            invoke_isentropic_calc_pivot_mach(&op, request_id, &args)
        }
        "device.isentropic_calc.path_text" => {
            invoke_isentropic_calc_path_text(&op, request_id, &args)
        }
        "device.normal_shock_calc" => invoke_normal_shock_calc(&op, request_id, &args),
        "device.normal_shock_calc.value" => invoke_normal_shock_calc_value(&op, request_id, &args),
        "device.normal_shock_calc.pivot_m1" => {
            invoke_normal_shock_calc_pivot_m1(&op, request_id, &args)
        }
        "device.normal_shock_calc.path_text" => {
            invoke_normal_shock_calc_path_text(&op, request_id, &args)
        }
        "device.oblique_shock_calc" => invoke_oblique_shock_calc(&op, request_id, &args),
        "device.oblique_shock_calc.value" => {
            invoke_oblique_shock_calc_value(&op, request_id, &args)
        }
        "device.oblique_shock_calc.path_text" => {
            invoke_oblique_shock_calc_path_text(&op, request_id, &args)
        }
        "device.fanno_flow_calc" => invoke_fanno_flow_calc(&op, request_id, &args),
        "device.fanno_flow_calc.value" => invoke_fanno_flow_calc_value(&op, request_id, &args),
        "device.fanno_flow_calc.pivot_mach" => {
            invoke_fanno_flow_calc_pivot_mach(&op, request_id, &args)
        }
        "device.fanno_flow_calc.path_text" => {
            invoke_fanno_flow_calc_path_text(&op, request_id, &args)
        }
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
            if k == "branch" {
                if let Some(s) = v.as_str() {
                    if !s.trim().is_empty() {
                        builder = builder.branch(s.trim());
                    }
                } else if !v.is_null() {
                    return InvokeResponse::err(
                        op,
                        request_id,
                        "invalid_arg_type",
                        "branch must be a string",
                        Some("branch"),
                        None,
                    );
                }
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

fn invoke_equation_variables(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
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
    let families = match equations::equation_families::load_default_validated(registry.equations())
    {
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

const LIST_TEXT_DELIMITER: &str = "; ";

fn equation_targets_for(eq: &EquationDef) -> Vec<String> {
    let mut out: Vec<String> = eq
        .variables
        .keys()
        .filter(|target| {
            eq.solve.explicit_forms.contains_key(*target)
                || equations::normalize::is_numerically_supported(eq, target)
        })
        .cloned()
        .collect();
    out.sort();
    out
}

fn equation_variables_for(eq: &EquationDef) -> Vec<String> {
    let mut out: Vec<String> = eq.variables.keys().cloned().collect();
    out.sort();
    out
}

fn equation_branches_for(eq: &EquationDef) -> Vec<(String, bool)> {
    eq.branches
        .iter()
        .map(|b| (b.name.clone(), b.preferred))
        .collect()
}

fn resolve_fluid_entry<'a>(
    key: &str,
    entries: &'a [eng_fluids::FluidDocsEntry],
) -> Option<&'a eng_fluids::FluidDocsEntry> {
    entries.iter().find(|f| {
        f.key.eq_ignore_ascii_case(key) || f.aliases.iter().any(|a| a.eq_ignore_ascii_case(key))
    })
}

fn fluid_property_default_unit(prop: &str) -> &'static str {
    match prop {
        "density" => "kg/m3",
        "specific_heat_capacity" => "J/(kg*K)",
        "specific_heat_capacity_cv" => "J/(kg*K)",
        "gamma" => "1",
        "speed_of_sound" => "m/s",
        "dynamic_viscosity" => "Pa*s",
        "thermal_conductivity" => "W/(m*K)",
        "temperature" => "K",
        "pressure" => "Pa",
        "specific_enthalpy" => "J/kg",
        "specific_entropy" => "J/(kg*K)",
        "quality" => "1",
        _ => "",
    }
}

fn invoke_equation_targets_text(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    let targets = equation_targets_for(eq);
    InvokeResponse::ok(op, request_id, json!(targets.join(LIST_TEXT_DELIMITER)))
}

fn invoke_equation_targets_table(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    let default_target = eq.solve.default_target.clone();
    let rows: Vec<Value> = equation_targets_for(eq)
        .into_iter()
        .map(|t| json!([t, default_target.as_ref().is_some_and(|d| d == &t)]))
        .collect();
    InvokeResponse::ok(op, request_id, json!(rows))
}

fn invoke_equation_target_count(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    InvokeResponse::ok(op, request_id, json!(equation_targets_for(eq).len()))
}

fn invoke_equation_branches_text(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    let branches = equation_branches_for(eq)
        .into_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    InvokeResponse::ok(op, request_id, json!(branches.join(LIST_TEXT_DELIMITER)))
}

fn invoke_equation_branches_table(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    let rows: Vec<Value> = equation_branches_for(eq)
        .into_iter()
        .map(|(name, preferred)| json!([name, preferred]))
        .collect();
    InvokeResponse::ok(op, request_id, json!(rows))
}

fn invoke_equation_variables_text(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    InvokeResponse::ok(
        op,
        request_id,
        json!(equation_variables_for(eq).join(LIST_TEXT_DELIMITER)),
    )
}

fn invoke_equation_variables_table(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    let rows: Vec<Value> = equation_variables_for(eq)
        .into_iter()
        .map(|v| {
            let default_unit = eq
                .variables
                .get(&v)
                .and_then(|def| resolved_default_unit(&def.dimension, def.default_unit.as_deref()))
                .unwrap_or_default();
            json!([v, default_unit])
        })
        .collect();
    InvokeResponse::ok(op, request_id, json!(rows))
}

fn invoke_equation_variable_count(
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
    let eq = match crate::eq.equation(path_id) {
        Ok(v) => v,
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
    InvokeResponse::ok(op, request_id, json!(equation_variables_for(eq).len()))
}

fn invoke_fluid_properties_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let entries = eng_fluids::docs_entries();
    let Some(f) = resolve_fluid_entry(key, &entries) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_fluid",
            format!("unknown fluid '{key}'"),
            Some("key"),
            None,
        );
    };
    InvokeResponse::ok(
        op,
        request_id,
        json!(f.supported_properties.join(LIST_TEXT_DELIMITER)),
    )
}

fn invoke_fluid_properties_table(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let entries = eng_fluids::docs_entries();
    let Some(f) = resolve_fluid_entry(key, &entries) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_fluid",
            format!("unknown fluid '{key}'"),
            Some("key"),
            None,
        );
    };
    let rows: Vec<Value> = f
        .supported_properties
        .iter()
        .map(|p| json!([p, fluid_property_default_unit(p)]))
        .collect();
    InvokeResponse::ok(op, request_id, json!(rows))
}

fn invoke_fluid_property_count(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let entries = eng_fluids::docs_entries();
    let Some(f) = resolve_fluid_entry(key, &entries) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_fluid",
            format!("unknown fluid '{key}'"),
            Some("key"),
            None,
        );
    };
    InvokeResponse::ok(op, request_id, json!(f.supported_properties.len()))
}

fn invoke_material_properties_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let material = match eng_materials::get(key) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_material",
                e.to_string(),
                Some("key"),
                None,
            );
        }
    };
    let def = match material.definition() {
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
    let props: Vec<String> = def.properties.keys().cloned().collect();
    InvokeResponse::ok(op, request_id, json!(props.join(LIST_TEXT_DELIMITER)))
}

fn invoke_material_properties_table(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let material = match eng_materials::get(key) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_material",
                e.to_string(),
                Some("key"),
                None,
            );
        }
    };
    let def = match material.definition() {
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
    let rows: Vec<Value> = def
        .properties
        .iter()
        .map(|(k, v)| json!([k, v.unit]))
        .collect();
    InvokeResponse::ok(op, request_id, json!(rows))
}

fn invoke_material_property_count(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let material = match eng_materials::get(key) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "unknown_material",
                e.to_string(),
                Some("key"),
                None,
            );
        }
    };
    let def = match material.definition() {
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
    InvokeResponse::ok(op, request_id, json!(def.properties.len()))
}

fn invoke_device_modes_text(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let entries = crate::devices::docs_entries();
    let Some(d) = entries.iter().find(|d| d.key.eq_ignore_ascii_case(key)) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_device",
            format!("unknown device '{key}'"),
            Some("key"),
            None,
        );
    };
    InvokeResponse::ok(
        op,
        request_id,
        json!(d.supported_modes.join(LIST_TEXT_DELIMITER)),
    )
}

fn invoke_device_mode_count(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let key = match req_str(args, "key") {
        Ok(v) => v,
        Err(e) => return InvokeResponse::err(op, request_id, "missing_arg", e, Some("key"), None),
    };
    let entries = crate::devices::docs_entries();
    let Some(d) = entries.iter().find(|d| d.key.eq_ignore_ascii_case(key)) else {
        return InvokeResponse::err(
            op,
            request_id,
            "unknown_device",
            format!("unknown device '{key}'"),
            Some("key"),
            None,
        );
    };
    InvokeResponse::ok(op, request_id, json!(d.supported_modes.len()))
}

fn invoke_format_value(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    fn unit_to_si_factor_and_sig(
        unit: &str,
    ) -> Result<(f64, eng_core::units::DimensionSignature), String> {
        let expr = format!("1 {unit}");
        match eng_core::units::parse_quantity_expression(&expr) {
            Ok(v) => Ok((v.value_si, v.signature)),
            Err(e) => Err(e.to_string()),
        }
    }

    let value = match req_f64(args, "value") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("value"), None);
        }
    };
    let in_unit = match req_str(args, "in_unit") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("in_unit"), None);
        }
    };
    let out_unit = match req_str(args, "out_unit") {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(op, request_id, "missing_arg", e, Some("out_unit"), None);
        }
    };
    let mode = opt_str(args, "mode").unwrap_or("value");

    let (in_factor, in_sig) = match unit_to_si_factor_and_sig(in_unit) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "format_conversion_error",
                e,
                Some("in_unit"),
                Some(json!({ "in_unit": in_unit, "out_unit": out_unit })),
            );
        }
    };
    let (out_factor, out_sig) = match unit_to_si_factor_and_sig(out_unit) {
        Ok(v) => v,
        Err(e) => {
            return InvokeResponse::err(
                op,
                request_id,
                "format_conversion_error",
                e,
                Some("out_unit"),
                Some(json!({ "in_unit": in_unit, "out_unit": out_unit })),
            );
        }
    };
    if in_sig != out_sig {
        return InvokeResponse::err(
            op,
            request_id,
            "format_dimension_mismatch",
            format!(
                "input/output unit dimensions do not match: in {:?}, out {:?}",
                in_sig, out_sig
            ),
            Some("out_unit"),
            Some(json!({ "in_unit": in_unit, "out_unit": out_unit })),
        );
    }
    let converted = value * in_factor / out_factor;

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

fn parse_isentropic_input_kind(
    raw: &str,
    value: f64,
) -> Option<(crate::devices::IsentropicInputKind, f64)> {
    crate::devices::isentropic::parse_input_kind(raw, value)
}

fn parse_isentropic_output_kind(raw: &str) -> Option<(crate::devices::IsentropicOutputKind, bool)> {
    crate::devices::isentropic::parse_output_kind(raw)
}

fn parse_isentropic_branch(raw: &str) -> Option<crate::devices::IsentropicBranch> {
    crate::devices::isentropic::parse_branch(raw)
}

struct IsentropicInvokeRequest {
    req: crate::devices::IsentropicCalcRequest,
    output_angle_deg: bool,
}

fn isentropic_request_from_args(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> std::result::Result<IsentropicInvokeRequest, InvokeResponse> {
    let gamma = req_f64(args, "gamma").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("gamma"),
            None,
        )
    })?;
    let input_kind_raw = req_str(args, "input_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_kind"),
            None,
        )
    })?;
    let input_value = req_f64(args, "input_value").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_value"),
            None,
        )
    })?;
    let target_kind_raw = req_str(args, "target_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("target_kind"),
            None,
        )
    })?;
    let (input_kind, input_value) = parse_isentropic_input_kind(input_kind_raw, input_value)
        .ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported input_kind '{input_kind_raw}'"),
                Some("input_kind"),
                None,
            )
        })?;
    let (target_kind, output_angle_deg) = parse_isentropic_output_kind(target_kind_raw)
        .ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported target_kind '{target_kind_raw}'"),
                Some("target_kind"),
                None,
            )
        })?;
    let branch = match args.get("branch").and_then(Value::as_str) {
        Some(s) if !s.trim().is_empty() => Some(parse_isentropic_branch(s).ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported branch '{}'", s),
                Some("branch"),
                None,
            )
        })?),
        _ => None,
    };

    Ok(IsentropicInvokeRequest {
        req: crate::devices::IsentropicCalcRequest {
            gamma,
            input_kind,
            input_value,
            target_kind,
            branch,
        },
        output_angle_deg,
    })
}

fn invoke_isentropic_calc(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let invoke_req = match isentropic_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::isentropic_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(
            op,
            request_id,
            json!({
                "value": if invoke_req.output_angle_deg { r.value_si.to_degrees() } else { r.value_si },
                "value_unit": if invoke_req.output_angle_deg { "deg" } else { "si" },
                "pivot_mach": r.pivot_mach,
                "path": r.path.iter().map(|s| json!({
                    "equation_path_id": s.equation_path_id,
                    "solved_for": s.solved_for,
                    "method": s.method,
                    "branch": s.branch,
                    "inputs_used": s.inputs_used.iter().map(|(k,v)| json!({"key":k, "value": v})).collect::<Vec<_>>()
                })).collect::<Vec<_>>(),
                "path_text": r.path_text(),
                "warnings": r.warnings,
            }),
        ),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_isentropic_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_isentropic_calc_value(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match isentropic_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::isentropic_calc_from_request(invoke_req.req) {
        Ok(r) => {
            let value = if invoke_req.output_angle_deg {
                r.value_si.to_degrees()
            } else {
                r.value_si
            };
            InvokeResponse::ok(op, request_id, json!(value))
        }
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_isentropic_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_isentropic_calc_pivot_mach(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match isentropic_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::isentropic_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.pivot_mach)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_isentropic_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_isentropic_calc_path_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match isentropic_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::isentropic_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.path_text())),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_isentropic_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn parse_normal_shock_input_kind(raw: &str) -> Option<crate::devices::NormalShockInputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "m1" | "mach1" | "upstream_mach" => Some(crate::devices::NormalShockInputKind::M1),
        "m2" | "mach2" | "downstream_mach" => Some(crate::devices::NormalShockInputKind::M2),
        "pressure_ratio" | "p2_p1" | "p2/p1" => {
            Some(crate::devices::NormalShockInputKind::PressureRatio)
        }
        "density_ratio" | "rho2_rho1" | "rho2/rho1" => {
            Some(crate::devices::NormalShockInputKind::DensityRatio)
        }
        "temperature_ratio" | "t2_t1" | "t2/t1" => {
            Some(crate::devices::NormalShockInputKind::TemperatureRatio)
        }
        "stagnation_pressure_ratio" | "p02_p01" | "p02/p01" => {
            Some(crate::devices::NormalShockInputKind::StagnationPressureRatio)
        }
        _ => None,
    }
}

fn parse_normal_shock_output_kind(raw: &str) -> Option<crate::devices::NormalShockOutputKind> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "m1" | "mach1" | "upstream_mach" => Some(crate::devices::NormalShockOutputKind::M1),
        "m2" | "mach2" | "downstream_mach" => Some(crate::devices::NormalShockOutputKind::M2),
        "pressure_ratio" | "p2_p1" | "p2/p1" => {
            Some(crate::devices::NormalShockOutputKind::PressureRatio)
        }
        "density_ratio" | "rho2_rho1" | "rho2/rho1" => {
            Some(crate::devices::NormalShockOutputKind::DensityRatio)
        }
        "temperature_ratio" | "t2_t1" | "t2/t1" => {
            Some(crate::devices::NormalShockOutputKind::TemperatureRatio)
        }
        "stagnation_pressure_ratio" | "p02_p01" | "p02/p01" => {
            Some(crate::devices::NormalShockOutputKind::StagnationPressureRatio)
        }
        _ => None,
    }
}

struct NormalShockInvokeRequest {
    req: crate::devices::NormalShockCalcRequest,
}

fn normal_shock_request_from_args(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> std::result::Result<NormalShockInvokeRequest, InvokeResponse> {
    let gamma = req_f64(args, "gamma").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("gamma"),
            None,
        )
    })?;
    let input_kind_raw = req_str(args, "input_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_kind"),
            None,
        )
    })?;
    let input_value = req_f64(args, "input_value").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_value"),
            None,
        )
    })?;
    let target_kind_raw = req_str(args, "target_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("target_kind"),
            None,
        )
    })?;
    let input_kind = parse_normal_shock_input_kind(input_kind_raw).ok_or_else(|| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "invalid_arg",
            format!("unsupported input_kind '{input_kind_raw}'"),
            Some("input_kind"),
            None,
        )
    })?;
    let target_kind = parse_normal_shock_output_kind(target_kind_raw).ok_or_else(|| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "invalid_arg",
            format!("unsupported target_kind '{target_kind_raw}'"),
            Some("target_kind"),
            None,
        )
    })?;

    Ok(NormalShockInvokeRequest {
        req: crate::devices::NormalShockCalcRequest {
            gamma,
            input_kind,
            input_value,
            target_kind,
        },
    })
}

fn invoke_normal_shock_calc(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let invoke_req = match normal_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::normal_shock_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(
            op,
            request_id,
            json!({
                "value": r.value_si,
                "value_unit": "si",
                "pivot_m1": r.pivot_m1,
                "path": r.path.iter().map(|s| json!({
                    "equation_path_id": s.equation_path_id,
                    "solved_for": s.solved_for,
                    "method": s.method,
                    "inputs_used": s.inputs_used.iter().map(|(k,v)| json!({"key":k, "value": v})).collect::<Vec<_>>()
                })).collect::<Vec<_>>(),
                "path_text": r.path_text(),
                "warnings": r.warnings,
            }),
        ),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_normal_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_normal_shock_calc_value(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match normal_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::normal_shock_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.value_si)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_normal_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_normal_shock_calc_pivot_m1(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match normal_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::normal_shock_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.pivot_m1)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_normal_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_normal_shock_calc_path_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match normal_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::normal_shock_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.path_text())),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_normal_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

struct ObliqueShockInvokeRequest {
    req: crate::devices::ObliqueShockCalcRequest,
    output_angle_deg: bool,
}

fn oblique_shock_request_from_args(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> std::result::Result<ObliqueShockInvokeRequest, InvokeResponse> {
    let gamma = req_f64(args, "gamma").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("gamma"),
            None,
        )
    })?;
    let m1 = req_f64(args, "m1").map_err(|e| {
        InvokeResponse::err(op, request_id.clone(), "missing_arg", e, Some("m1"), None)
    })?;
    let input_kind_raw = req_str(args, "input_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_kind"),
            None,
        )
    })?;
    let input_value_raw = req_f64(args, "input_value").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_value"),
            None,
        )
    })?;
    let target_kind_raw = req_str(args, "target_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("target_kind"),
            None,
        )
    })?;

    let input_kind =
        crate::devices::oblique_shock::parse_input_kind(input_kind_raw, input_value_raw)
            .ok_or_else(|| {
                InvokeResponse::err(
                    op,
                    request_id.clone(),
                    "invalid_arg",
                    format!("unsupported input_kind '{input_kind_raw}'"),
                    Some("input_kind"),
                    None,
                )
            })?;
    let input_value =
        crate::devices::oblique_shock::input_value_to_si(input_kind_raw, input_value_raw);
    let (target_kind, output_angle_deg) =
        crate::devices::oblique_shock::parse_output_kind(target_kind_raw).ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported target_kind '{target_kind_raw}'"),
                Some("target_kind"),
                None,
            )
        })?;
    let branch = opt_str(args, "branch").and_then(crate::devices::oblique_shock::parse_branch);

    Ok(ObliqueShockInvokeRequest {
        req: crate::devices::ObliqueShockCalcRequest {
            gamma,
            m1,
            input_kind,
            input_value,
            target_kind,
            branch,
        },
        output_angle_deg,
    })
}

fn invoke_oblique_shock_calc(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let invoke_req = match oblique_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::oblique_shock_calc_from_request(invoke_req.req) {
        Ok(r) => {
            let value = if invoke_req.output_angle_deg {
                r.value_si.to_degrees()
            } else {
                r.value_si
            };
            InvokeResponse::ok(
                op,
                request_id,
                json!({
                    "value": value,
                    "value_unit": if invoke_req.output_angle_deg { "deg" } else { "si" },
                    "beta_rad": r.beta_rad,
                    "theta_rad": r.theta_rad,
                    "beta_deg": r.beta_rad.to_degrees(),
                    "theta_deg": r.theta_rad.to_degrees(),
                    "mn1": r.mn1,
                    "mn2": r.mn2,
                    "m2": r.m2,
                    "path": r.path.iter().map(|s| json!({
                        "equation_path_id": s.equation_path_id,
                        "solved_for": s.solved_for,
                        "method": s.method,
                        "branch": s.branch,
                        "inputs_used": s.inputs_used.iter().map(|(k,v)| json!({"key":k, "value": v})).collect::<Vec<_>>()
                    })).collect::<Vec<_>>(),
                    "path_text": r.path_text(),
                    "warnings": r.warnings,
                }),
            )
        }
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_oblique_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_oblique_shock_calc_value(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match oblique_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::oblique_shock_calc_from_request(invoke_req.req) {
        Ok(r) => {
            let value = if invoke_req.output_angle_deg {
                r.value_si.to_degrees()
            } else {
                r.value_si
            };
            InvokeResponse::ok(op, request_id, json!(value))
        }
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_oblique_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_oblique_shock_calc_path_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match oblique_shock_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::oblique_shock_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.path_text())),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_oblique_shock_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

struct FannoFlowInvokeRequest {
    req: crate::devices::FannoFlowCalcRequest,
}

fn fanno_flow_request_from_args(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> std::result::Result<FannoFlowInvokeRequest, InvokeResponse> {
    let gamma = req_f64(args, "gamma").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("gamma"),
            None,
        )
    })?;
    let input_kind_raw = req_str(args, "input_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_kind"),
            None,
        )
    })?;
    let input_value = req_f64(args, "input_value").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("input_value"),
            None,
        )
    })?;
    let target_kind_raw = req_str(args, "target_kind").map_err(|e| {
        InvokeResponse::err(
            op,
            request_id.clone(),
            "missing_arg",
            e,
            Some("target_kind"),
            None,
        )
    })?;

    let input_kind =
        crate::devices::fanno_flow::parse_input_kind(input_kind_raw).ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported input_kind '{input_kind_raw}'"),
                Some("input_kind"),
                None,
            )
        })?;
    let target_kind =
        crate::devices::fanno_flow::parse_output_kind(target_kind_raw).ok_or_else(|| {
            InvokeResponse::err(
                op,
                request_id.clone(),
                "invalid_arg",
                format!("unsupported target_kind '{target_kind_raw}'"),
                Some("target_kind"),
                None,
            )
        })?;
    let branch = opt_str(args, "branch").and_then(crate::devices::fanno_flow::parse_branch);

    Ok(FannoFlowInvokeRequest {
        req: crate::devices::FannoFlowCalcRequest {
            gamma,
            input_kind,
            input_value,
            target_kind,
            branch,
        },
    })
}

fn invoke_fanno_flow_calc(op: &str, request_id: Option<String>, args: &Value) -> InvokeResponse {
    let invoke_req = match fanno_flow_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::fanno_flow_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(
            op,
            request_id,
            json!({
                "value": r.value_si,
                "value_unit": "si",
                "pivot_mach": r.pivot_mach,
                "path": r.path.iter().map(|s| json!({
                    "equation_path_id": s.equation_path_id,
                    "solved_for": s.solved_for,
                    "method": s.method,
                    "branch": s.branch,
                    "inputs_used": s.inputs_used.iter().map(|(k,v)| json!({"key":k, "value": v})).collect::<Vec<_>>()
                })).collect::<Vec<_>>(),
                "path_text": r.path_text(),
                "warnings": r.warnings,
            }),
        ),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_fanno_flow_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_fanno_flow_calc_value(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match fanno_flow_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::fanno_flow_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.value_si)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_fanno_flow_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_fanno_flow_calc_pivot_mach(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match fanno_flow_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::fanno_flow_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.pivot_mach)),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_fanno_flow_calc_failed",
            e.to_string(),
            None,
            None,
        ),
    }
}

fn invoke_fanno_flow_calc_path_text(
    op: &str,
    request_id: Option<String>,
    args: &Value,
) -> InvokeResponse {
    let invoke_req = match fanno_flow_request_from_args(op, request_id.clone(), args) {
        Ok(v) => v,
        Err(resp) => return resp,
    };
    match crate::devices::fanno_flow_calc_from_request(invoke_req.req) {
        Ok(r) => InvokeResponse::ok(op, request_id, json!(r.path_text())),
        Err(e) => InvokeResponse::err(
            op,
            request_id,
            "device_fanno_flow_calc_failed",
            e.to_string(),
            None,
            None,
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
