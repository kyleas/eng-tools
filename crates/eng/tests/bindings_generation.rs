use std::{fs, process::Command};

use eng::docs::export_unified_docs_to;
use tempfile::tempdir;

#[test]
fn generated_binding_artifacts_exist_and_are_populated() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let spec = tmp.path().join("bindings").join("binding_spec.json");
    let protocol = tmp.path().join("bindings").join("invoke_protocol.json");
    let py_pkg = tmp.path().join("bindings/python/engpy/__init__.py");
    let py_runtime = tmp.path().join("bindings/python/engpy/_runtime.py");
    let pyproject = tmp.path().join("bindings/python/pyproject.toml");
    let py_eq_init = tmp
        .path()
        .join("bindings/python/engpy/equations/__init__.py");
    let xloil = tmp.path().join("bindings/excel/eng_xloil.py");
    let pyxll = tmp.path().join("bindings/excel/eng_pyxll.py");

    assert!(spec.exists(), "missing {}", spec.display());
    assert!(protocol.exists(), "missing {}", protocol.display());
    assert!(py_pkg.exists(), "missing {}", py_pkg.display());
    assert!(py_runtime.exists(), "missing {}", py_runtime.display());
    assert!(pyproject.exists(), "missing {}", pyproject.display());
    assert!(py_eq_init.exists(), "missing {}", py_eq_init.display());
    assert!(xloil.exists(), "missing {}", xloil.display());
    assert!(pyxll.exists(), "missing {}", pyxll.display());

    let text = fs::read_to_string(spec).expect("read binding spec");
    assert!(text.contains("\"equation.solve\""));
    assert!(text.contains("\"equation.meta\""));
    assert!(text.contains("\"equation.ascii\""));
    assert!(text.contains("\"equation.default_unit\""));
    assert!(text.contains("\"device.pipe_loss.solve_delta_p\""));
    assert!(text.contains("\"fluid.prop\""));
    assert!(text.contains("\"material.prop\""));
    assert!(text.contains("\"constant.get\""));

    let proto_text = fs::read_to_string(protocol).expect("read invoke protocol");
    assert!(proto_text.contains("\"protocol_version\""));
    assert!(proto_text.contains("\"eng-invoke.v1\""));

    let xloil_text = fs::read_to_string(xloil).expect("read xloil module");
    assert!(xloil_text.contains("@xloil.func"));
    assert!(xloil_text.contains("ENG_PIPE_LOSS_DELTA_P"));
    assert!(xloil_text.contains("ENG_EQUATION_META"));
    assert!(xloil_text.contains("ENG_EQUATION_DEFAULT_UNIT"));
    assert!(xloil_text.contains("Arguments:"));
    assert!(xloil_text.contains("density"));
    assert!(xloil_text.contains("roughness"));

    let pyxll_text = fs::read_to_string(pyxll).expect("read pyxll module");
    assert!(pyxll_text.contains("@xl_func"));
    assert!(pyxll_text.contains("ENG_FLUID_PROP"));
    assert!(pyxll_text.contains("ENG_EQUATION_ASCII"));
    assert!(pyxll_text.contains("Arguments:"));
    assert!(pyxll_text.contains("state_prop_1"));

    let runtime_text = fs::read_to_string(py_runtime).expect("read generated runtime");
    assert!(runtime_text.contains("CREATE_NO_WINDOW"));
    assert!(runtime_text.contains("import engpy_native"));
    assert!(runtime_text.contains("def runtime_mode("));
    assert!(runtime_text.contains("def worker_stats("));
    assert!(runtime_text.contains("builtins._ENGPY_CLIENT"));
    assert!(runtime_text.contains("\"encoding\": \"utf-8\""));

    let pyproject_text = fs::read_to_string(pyproject).expect("read generated pyproject");
    assert!(pyproject_text.contains("maturin"));
    assert!(pyproject_text.contains("engpy_native"));
}

#[test]
fn generated_python_package_imports_when_python_available() {
    let python = Command::new("python").arg("--version").output();
    if python.is_err() {
        return;
    }

    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let python_root = tmp.path().join("bindings").join("python");
    let script = format!(
        "import sys; sys.path.insert(0, r'{}'); import engpy; import engpy.equations; import engpy.fluids; import engpy.materials; import engpy.devices; import engpy.constants; import engpy._runtime as rt; \
engpy.constants.get_constant(\"g0\"); s1 = rt.worker_stats(); mode = rt.runtime_mode(); engpy.constants.get_constant(\"pi\"); s2 = rt.worker_stats(); \
assert mode in ('native', 'worker'); assert s2.get('request_count', 0) >= 2; \
assert s2.get('runtime_mode') in ('native', 'worker'); \
assert (mode == 'native') or (s1.get('worker_pid') is not None and s1.get('worker_pid') == s2.get('worker_pid'))",
        python_root.display()
    );
    let status = Command::new("python")
        .arg("-c")
        .arg(script)
        .env("ENG_WORKER_BIN", env!("CARGO_BIN_EXE_eng"))
        .status()
        .expect("run python import smoke");
    assert!(
        status.success(),
        "python import smoke failed for generated package"
    );
}
