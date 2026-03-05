use std::process::Command;

use serde_json::{Value, json};

fn run_invoke(req: Value) -> Value {
    let bin = env!("CARGO_BIN_EXE_eng");
    let output = Command::new(bin)
        .args(["invoke", "--request-json", &req.to_string()])
        .output()
        .expect("run eng invoke");
    assert!(output.status.success(), "eng invoke process failed");
    serde_json::from_slice::<Value>(&output.stdout).expect("parse invoke response json")
}

#[test]
fn invoke_success_response_shape_is_stable() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "req-success-1",
        "op": "constant.get",
        "args": { "key": "g0" }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["protocol_version"], "eng-invoke.v1");
    assert_eq!(resp["op"], "constant.get");
    assert_eq!(resp["request_id"], "req-success-1");
    assert_eq!(resp["ok"], true);
    assert!(resp.get("value").is_some());
    assert!(resp.get("error").is_none() || resp["error"].is_null());
}

#[test]
fn invoke_error_response_shape_is_stable() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "req-error-1",
        "op": "fluid.prop",
        "args": { "fluid": "H2O" }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["protocol_version"], "eng-invoke.v1");
    assert_eq!(resp["op"], "fluid.prop");
    assert_eq!(resp["request_id"], "req-error-1");
    assert_eq!(resp["ok"], false);
    assert_eq!(resp["error"]["code"], "missing_arg");
    assert!(
        resp["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("missing string arg")
    );
}

#[test]
fn invoke_protocol_mismatch_returns_structured_error() {
    let req = json!({
        "protocol_version": "eng-invoke.v9",
        "op": "constant.get",
        "args": { "key": "g0" }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false);
    assert_eq!(resp["error"]["code"], "protocol_version_mismatch");
    assert_eq!(resp["error"]["field"], "protocol_version");
}

#[test]
fn invoke_equation_solve_normalizes_variable_key_case() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "req-case-1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.area_mach",
            "target": "area_ratio",
            "m": 2.0,
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], true, "response: {resp}");
    assert!(resp["value"].as_f64().unwrap_or(0.0) > 0.0);
}

#[test]
fn invoke_equation_solve_area_mach_target_m_without_explicit_branch_succeeds() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "req-area-mach-m-1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.area_mach",
            "target": "M",
            "area_ratio": 2.0,
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], true, "response: {resp}");
    assert!(resp["value"].as_f64().unwrap_or(0.0) > 1.0);
}

#[test]
fn invoke_equation_solve_area_mach_honors_explicit_branch() {
    let sub_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.area_mach",
            "target": "M",
            "area_ratio": 2.0,
            "gamma": 1.4,
            "branch": "subsonic"
        }
    });
    let sub_resp = run_invoke(sub_req);
    assert_eq!(sub_resp["ok"], true, "response: {sub_resp}");
    let m_sub = sub_resp["value"].as_f64().unwrap_or(0.0);
    assert!(
        m_sub > 0.0 && m_sub < 1.0,
        "expected subsonic branch, got {m_sub}"
    );

    let sup_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.area_mach",
            "target": "M",
            "area_ratio": 2.0,
            "gamma": 1.4,
            "branch": "supersonic"
        }
    });
    let sup_resp = run_invoke(sup_req);
    assert_eq!(sup_resp["ok"], true, "response: {sup_resp}");
    let m_sup = sup_resp["value"].as_f64().unwrap_or(0.0);
    assert!(m_sup > 1.0, "expected supersonic branch, got {m_sup}");
}

#[test]
fn invoke_equation_solve_invalid_branch_is_clear_error() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.area_mach",
            "target": "M",
            "area_ratio": 2.0,
            "gamma": 1.4,
            "branch": "not_a_branch"
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "equation_solve_failed");
    let msg = resp["error"]["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("branch") && msg.contains("valid"),
        "expected clear invalid branch message, got: {msg}"
    );
}

#[test]
fn invoke_equation_meta_includes_display_and_units() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "request_id": "req-meta-1",
        "op": "equation.meta",
        "args": {
            "path_id": "fluids.reynolds_number"
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], true, "response: {resp}");
    assert_eq!(resp["value"]["path_id"], "fluids.reynolds_number");
    assert!(
        resp["value"]["display"]["ascii"]
            .as_str()
            .unwrap_or("")
            .contains("Re")
    );
    let vars = resp["value"]["variables"]
        .as_array()
        .expect("variables array");
    assert!(vars.iter().any(|v| v["key"] == "mu"));
    assert!(vars.iter().any(|v| v["default_unit"] == "Pa*s"));
}

#[test]
fn invoke_equation_ascii_and_default_unit_return_scalars() {
    let ascii_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.ascii",
        "args": {
            "path_id": "fluids.reynolds_number"
        }
    });
    let ascii_resp = run_invoke(ascii_req);
    assert_eq!(ascii_resp["ok"], true, "response: {ascii_resp}");
    assert!(ascii_resp["value"].as_str().unwrap_or("").contains("Re"));

    let unit_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.default_unit",
        "args": {
            "path_id": "fluids.reynolds_number",
            "variable": "mu"
        }
    });
    let unit_resp = run_invoke(unit_req);
    assert_eq!(unit_resp["ok"], true, "response: {unit_resp}");
    assert_eq!(unit_resp["value"], "Pa*s");
}

#[test]
fn invoke_isentropic_calc_supports_deg_input_and_output() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.isentropic_calc.value",
        "args": {
            "input_kind": "mach",
            "input_value": 2.0,
            "target_kind": "mach_angle_deg",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], true, "response: {resp}");
    let mu_deg = resp["value"].as_f64().unwrap_or(0.0);
    assert!(
        (mu_deg - 30.0).abs() < 1e-8,
        "expected 30 deg, got {mu_deg}"
    );
}

#[test]
fn invoke_isentropic_calc_requires_branch_for_area_ratio_inversion() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.isentropic_calc.value",
        "args": {
            "input_kind": "area_ratio",
            "input_value": 2.0,
            "target_kind": "mach",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "device_isentropic_calc_failed");
    assert!(
        resp["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("branch is required"),
        "expected missing-branch guidance, got: {}",
        resp["error"]["message"].as_str().unwrap_or("")
    );
}

#[test]
fn invoke_equation_solve_prandtl_meyer_forward_and_inverse_work() {
    let forward_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.prandtl_meyer",
            "target": "nu",
            "M": 2.0,
            "gamma": 1.4
        }
    });
    let forward_resp = run_invoke(forward_req);
    assert_eq!(forward_resp["ok"], true, "response: {forward_resp}");
    let nu = forward_resp["value"].as_f64().unwrap_or(-1.0);
    assert!((nu - 0.460_413_682_082_694_73).abs() < 1e-9, "nu={nu}");

    let inverse_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.solve",
        "args": {
            "path_id": "compressible.prandtl_meyer",
            "target": "M",
            "nu": nu,
            "gamma": 1.4
        }
    });
    let inverse_resp = run_invoke(inverse_req);
    assert_eq!(inverse_resp["ok"], true, "response: {inverse_resp}");
    let m = inverse_resp["value"].as_f64().unwrap_or(-1.0);
    assert!((m - 2.0).abs() < 1e-8, "M={m}");
}

#[test]
fn invoke_isentropic_calc_supports_prandtl_meyer_deg_kinds() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.isentropic_calc.value",
        "args": {
            "input_kind": "prandtl_meyer_angle_deg",
            "input_value": 26.379760813416457,
            "target_kind": "mach",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], true, "response: {resp}");
    let m = resp["value"].as_f64().unwrap_or(0.0);
    assert!((m - 2.0).abs() < 1e-8, "expected M~2, got {m}");
}

#[test]
fn invoke_isentropic_calc_rejects_out_of_domain_prandtl_meyer_angle() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.isentropic_calc.value",
        "args": {
            "input_kind": "prandtl_meyer_angle_deg",
            "input_value": 150.0,
            "target_kind": "mach",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "device_isentropic_calc_failed");
    let msg = resp["error"]["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("PrandtlMeyerAngleRad") && msg.contains("expected 0 <="),
        "expected PM domain guidance, got: {msg}"
    );
}

#[test]
fn invoke_oblique_shock_calc_supports_weak_and_strong_branches() {
    let weak_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.oblique_shock_calc.value",
        "args": {
            "m1": 2.0,
            "input_kind": "theta_deg",
            "input_value": 10.0,
            "target_kind": "beta_deg",
            "gamma": 1.4,
            "branch": "weak"
        }
    });
    let weak_resp = run_invoke(weak_req);
    assert_eq!(weak_resp["ok"], true, "response: {weak_resp}");
    let beta_weak = weak_resp["value"].as_f64().unwrap_or(0.0);

    let strong_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.oblique_shock_calc.value",
        "args": {
            "m1": 2.0,
            "input_kind": "theta_deg",
            "input_value": 10.0,
            "target_kind": "beta_deg",
            "gamma": 1.4,
            "branch": "strong"
        }
    });
    let strong_resp = run_invoke(strong_req);
    assert_eq!(strong_resp["ok"], true, "response: {strong_resp}");
    let beta_strong = strong_resp["value"].as_f64().unwrap_or(0.0);

    assert!(beta_weak > 30.0 && beta_weak < 60.0);
    assert!(beta_strong > 70.0 && beta_strong < 90.0);
}

#[test]
fn invoke_oblique_shock_calc_requires_branch_for_theta_input() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.oblique_shock_calc.value",
        "args": {
            "m1": 2.0,
            "input_kind": "theta_deg",
            "input_value": 10.0,
            "target_kind": "p2_p1",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "device_oblique_shock_calc_failed");
    assert!(
        resp["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("branch is required"),
        "expected branch-required guidance"
    );
}

#[test]
fn invoke_fanno_flow_calc_supports_branch_sensitive_inverse_paths() {
    let sub_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.fanno_flow_calc.value",
        "args": {
            "input_kind": "four_flstar_d",
            "input_value": 0.3049965025814798,
            "target_kind": "mach",
            "gamma": 1.4,
            "branch": "subsonic"
        }
    });
    let sub_resp = run_invoke(sub_req);
    assert_eq!(sub_resp["ok"], true, "response: {sub_resp}");
    let m_sub = sub_resp["value"].as_f64().unwrap_or(0.0);
    assert!(
        m_sub > 0.0 && m_sub < 1.0,
        "expected subsonic M, got {m_sub}"
    );

    let sup_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.fanno_flow_calc.value",
        "args": {
            "input_kind": "four_flstar_d",
            "input_value": 0.3049965025814798,
            "target_kind": "mach",
            "gamma": 1.4,
            "branch": "supersonic"
        }
    });
    let sup_resp = run_invoke(sup_req);
    assert_eq!(sup_resp["ok"], true, "response: {sup_resp}");
    let m_sup = sup_resp["value"].as_f64().unwrap_or(0.0);
    assert!(m_sup > 1.0, "expected supersonic M, got {m_sup}");
}

#[test]
fn invoke_fanno_flow_calc_requires_branch_for_inverse_paths() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.fanno_flow_calc.value",
        "args": {
            "input_kind": "p0_p0star",
            "input_value": 1.33984375,
            "target_kind": "mach",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "device_fanno_flow_calc_failed");
    assert!(
        resp["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("branch is required"),
        "expected branch-required guidance"
    );
}

#[test]
fn invoke_rayleigh_calc_supports_branch_sensitive_inverse_paths() {
    let sub_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.rayleigh_calc.value",
        "args": {
            "input_kind": "t_tstar",
            "input_value": 0.7901234567901233,
            "target_kind": "mach",
            "gamma": 1.4,
            "branch": "subsonic"
        }
    });
    let sub_resp = run_invoke(sub_req);
    assert_eq!(sub_resp["ok"], true, "response: {sub_resp}");
    let m_sub = sub_resp["value"].as_f64().unwrap_or(0.0);
    assert!(
        m_sub > 0.0 && m_sub < 1.0,
        "expected subsonic M, got {m_sub}"
    );

    let sup_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.rayleigh_calc.value",
        "args": {
            "input_kind": "t_tstar",
            "input_value": 0.7901234567901233,
            "target_kind": "mach",
            "gamma": 1.4,
            "branch": "supersonic"
        }
    });
    let sup_resp = run_invoke(sup_req);
    assert_eq!(sup_resp["ok"], true, "response: {sup_resp}");
    let m_sup = sup_resp["value"].as_f64().unwrap_or(0.0);
    assert!(m_sup > 1.0, "expected supersonic M, got {m_sup}");
}

#[test]
fn invoke_rayleigh_calc_requires_branch_for_ambiguous_inverse_paths() {
    let req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "device.rayleigh_calc.value",
        "args": {
            "input_kind": "p0_p0star",
            "input_value": 1.114052503180089,
            "target_kind": "mach",
            "gamma": 1.4
        }
    });
    let resp = run_invoke(req);
    assert_eq!(resp["ok"], false, "response: {resp}");
    assert_eq!(resp["error"]["code"], "device_rayleigh_calc_failed");
    assert!(
        resp["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("branch is required"),
        "expected branch-required guidance"
    );
}

#[test]
fn invoke_format_and_meta_helpers_work() {
    let fmt_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "format.value",
        "args": {
            "value": 2.5e6,
            "in_unit": "Pa",
            "out_unit": "psia"
        }
    });
    let fmt_resp = run_invoke(fmt_req);
    assert_eq!(fmt_resp["ok"], true, "response: {fmt_resp}");
    assert!(fmt_resp["value"].as_f64().unwrap_or(0.0) > 300.0);

    let fmt_psia_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "format.value",
        "args": {
            "value": 101325.0,
            "in_unit": "Pa",
            "out_unit": "psia"
        }
    });
    let fmt_psia_resp = run_invoke(fmt_psia_req);
    assert_eq!(fmt_psia_resp["ok"], true, "response: {fmt_psia_resp}");
    let psia = fmt_psia_resp["value"].as_f64().unwrap_or(0.0);
    assert!((psia - 14.695_948_8).abs() < 1e-4);

    let mismatch_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "format.value",
        "args": {
            "value": 1.0,
            "in_unit": "Pa",
            "out_unit": "m"
        }
    });
    let mismatch_resp = run_invoke(mismatch_req);
    assert_eq!(mismatch_resp["ok"], false, "response: {mismatch_resp}");
    assert_eq!(mismatch_resp["error"]["code"], "format_dimension_mismatch");

    let missing_in_unit_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "format.value",
        "args": {
            "value": 1.0,
            "out_unit": "Pa"
        }
    });
    let missing_in_unit_resp = run_invoke(missing_in_unit_req);
    assert_eq!(
        missing_in_unit_resp["ok"], false,
        "response: {missing_in_unit_resp}"
    );
    assert_eq!(missing_in_unit_resp["error"]["code"], "missing_arg");
    assert_eq!(missing_in_unit_resp["error"]["field"], "in_unit");

    let meta_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "meta.get",
        "args": {
            "entity": "equation",
            "key": "structures.hoop_stress",
            "field": "ascii"
        }
    });
    let meta_resp = run_invoke(meta_req);
    assert_eq!(meta_resp["ok"], true, "response: {meta_resp}");
    assert!(meta_resp["value"].as_str().unwrap_or("").contains("sigma"));

    let targets_text_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.targets.text",
        "args": { "path_id": "structures.hoop_stress" }
    });
    let targets_text_resp = run_invoke(targets_text_req);
    assert_eq!(
        targets_text_resp["ok"], true,
        "response: {targets_text_resp}"
    );
    let targets_text = targets_text_resp["value"].as_str().unwrap_or("");
    assert_eq!(targets_text, "P; r; sigma_h; t");

    let vars_table_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.variables.table",
        "args": { "path_id": "structures.hoop_stress" }
    });
    let vars_table_resp = run_invoke(vars_table_req);
    assert_eq!(vars_table_resp["ok"], true, "response: {vars_table_resp}");
    let rows = vars_table_resp["value"].as_array().expect("table rows");
    assert!(!rows.is_empty());
    assert!(
        rows.iter()
            .all(|r| r.as_array().is_some_and(|a| a.len() == 2))
    );

    let branches_text_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.branches.text",
        "args": { "path_id": "compressible.area_mach" }
    });
    let branches_text_resp = run_invoke(branches_text_req);
    assert_eq!(
        branches_text_resp["ok"], true,
        "response: {branches_text_resp}"
    );
    let branches_text = branches_text_resp["value"].as_str().unwrap_or("");
    assert!(branches_text.contains("subsonic"));
    assert!(branches_text.contains("supersonic"));

    let branches_table_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "equation.branches.table",
        "args": { "path_id": "compressible.area_mach" }
    });
    let branches_table_resp = run_invoke(branches_table_req);
    assert_eq!(
        branches_table_resp["ok"], true,
        "response: {branches_table_resp}"
    );
    let branch_rows = branches_table_resp["value"]
        .as_array()
        .expect("branch rows");
    assert!(!branch_rows.is_empty());
    assert!(
        branch_rows
            .iter()
            .all(|r| r.as_array().is_some_and(|a| a.len() == 2))
    );

    let fluid_count_req = json!({
        "protocol_version": "eng-invoke.v1",
        "op": "fluid.property.count",
        "args": { "key": "H2O" }
    });
    let fluid_count_resp = run_invoke(fluid_count_req);
    assert_eq!(fluid_count_resp["ok"], true, "response: {fluid_count_resp}");
    assert!(fluid_count_resp["value"].as_u64().unwrap_or(0) > 0);
}
