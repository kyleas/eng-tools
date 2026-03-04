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
