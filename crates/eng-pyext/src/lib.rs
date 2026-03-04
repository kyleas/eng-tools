use eng::invoke::process_invoke_json_to_string;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use serde_json::json;

#[pyfunction]
fn invoke_json(request_json: &str) -> PyResult<String> {
    process_invoke_json_to_string(request_json).map_err(|e| {
        PyRuntimeError::new_err(format!(
            "failed to serialize invoke response for request: {e}"
        ))
    })
}

#[pyfunction]
fn protocol_version() -> &'static str {
    eng::bindings::INVOKE_PROTOCOL_VERSION
}

#[pyfunction]
fn runtime_info() -> String {
    json!({
        "runtime": "engpy_native",
        "protocol_version": eng::bindings::INVOKE_PROTOCOL_VERSION,
        "transport": "in_process",
    })
    .to_string()
}

#[pymodule]
fn engpy_native(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(invoke_json, module)?)?;
    module.add_function(wrap_pyfunction!(protocol_version, module)?)?;
    module.add_function(wrap_pyfunction!(runtime_info, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn invoke_json_returns_protocol_response() {
        let req = r#"{"protocol_version":"eng-invoke.v1","op":"constant.get","request_id":"t1","args":{"key":"g0"}}"#;
        let resp = invoke_json(req).expect("invoke_json");
        let parsed: Value = serde_json::from_str(&resp).expect("parse response");
        assert_eq!(parsed["protocol_version"], "eng-invoke.v1");
        assert_eq!(parsed["ok"], true);
        assert_eq!(parsed["request_id"], "t1");
    }
}
