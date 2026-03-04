use std::{fs, process::Command};

use eng::docs::export_unified_docs_to;
use regex::Regex;
use tempfile::tempdir;
use walkdir::WalkDir;

#[test]
fn generated_binding_artifacts_exist_and_are_populated() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let spec = tmp.path().join("bindings").join("binding_spec.json");
    let protocol = tmp.path().join("bindings").join("invoke_protocol.json");
    let py_pkg = tmp.path().join("bindings/python/engpy/__init__.py");
    let py_runtime = tmp.path().join("bindings/python/engpy/_runtime.py");
    let py_helpers = tmp.path().join("bindings/python/engpy/helpers.py");
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
    assert!(py_helpers.exists(), "missing {}", py_helpers.display());
    assert!(pyproject.exists(), "missing {}", pyproject.display());
    assert!(py_eq_init.exists(), "missing {}", py_eq_init.display());
    assert!(xloil.exists(), "missing {}", xloil.display());
    assert!(pyxll.exists(), "missing {}", pyxll.display());

    let text = fs::read_to_string(spec).expect("read binding spec");
    assert!(text.contains("\"equation.solve\""));
    assert!(text.contains("\"equation.meta\""));
    assert!(text.contains("\"equation.ascii\""));
    assert!(text.contains("\"equation.default_unit\""));
    assert!(text.contains("\"equation.unicode\""));
    assert!(text.contains("\"equation.latex\""));
    assert!(text.contains("\"equation.targets\""));
    assert!(text.contains("\"equation.variables\""));
    assert!(text.contains("\"equation.name\""));
    assert!(text.contains("\"equation.description\""));
    assert!(text.contains("\"equation.family\""));
    assert!(text.contains("\"format.value\""));
    assert!(text.contains("\"meta.get\""));
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
    assert!(xloil_text.contains("ENG_FORMAT"));
    assert!(xloil_text.contains("ENG_META"));
    assert!(xloil_text.contains("ENG_EQUATION_TARGETS_TEXT"));
    assert!(xloil_text.contains("ENG_EQUATION_VARIABLES_TABLE"));
    assert!(xloil_text.contains("ENG_EQUATION_BRANCHES_TEXT"));
    assert!(xloil_text.contains("ENG_EQUATION_BRANCHES_TABLE"));
    assert!(xloil_text.contains("ENG_EQUATION_TARGET_COUNT"));
    assert!(xloil_text.contains("ENG_FLUID_PROPERTIES_TEXT"));
    assert!(xloil_text.contains("ENG_FLUID_PROPERTIES_TABLE"));
    assert!(xloil_text.contains("ENG_FLUID_PROPERTY_COUNT"));
    assert!(xloil_text.contains("ENG_MATERIAL_PROPERTIES_TEXT"));
    assert!(xloil_text.contains("ENG_MATERIAL_PROPERTIES_TABLE"));
    assert!(xloil_text.contains("ENG_MATERIAL_PROPERTY_COUNT"));
    assert!(xloil_text.contains("ENG_DEVICE_MODES_TEXT"));
    assert!(xloil_text.contains("ENG_DEVICE_MODE_COUNT"));
    assert!(xloil_text.contains("ENG_ISENTROPIC("));
    assert!(xloil_text.contains("ENG_ISENTROPIC_PIVOT_MACH"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_PATH_TEXT"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_FROM_M_TO_P_P0"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_FROM_NU_DEG_TO_M"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_FROM_M_TO_NU_DEG"));
    assert!(xloil_text.contains("ENG_ISENTROPIC_FROM_A_ASTAR_TO_M"));
    assert!(xloil_text.contains("ENG_EQUATION_TARGETS"));
    assert!(xloil_text.contains("ENG_COMPRESSIBLE_PRANDTL_MEYER_NU"));
    assert!(xloil_text.contains("ENG_COMPRESSIBLE_PRANDTL_MEYER_M"));
    assert!(xloil_text.contains("Arguments:"));
    assert!(xloil_text.contains("density"));
    assert!(xloil_text.contains("roughness"));

    let pyxll_text = fs::read_to_string(pyxll).expect("read pyxll module");
    assert!(pyxll_text.contains("@xl_func"));
    assert!(pyxll_text.contains("ENG_FLUID_PROP"));
    assert!(pyxll_text.contains("ENG_EQUATION_ASCII"));
    assert!(pyxll_text.contains("ENG_EQUATION_UNICODE"));
    assert!(pyxll_text.contains("ENG_EQUATION_LATEX"));
    assert!(pyxll_text.contains("ENG_FORMAT"));
    assert!(pyxll_text.contains("ENG_META"));
    assert!(pyxll_text.contains("ENG_EQUATION_TARGETS_TEXT"));
    assert!(pyxll_text.contains("ENG_EQUATION_VARIABLES_TABLE"));
    assert!(pyxll_text.contains("ENG_EQUATION_BRANCHES_TEXT"));
    assert!(pyxll_text.contains("ENG_EQUATION_BRANCHES_TABLE"));
    assert!(pyxll_text.contains("ENG_EQUATION_TARGET_COUNT"));
    assert!(pyxll_text.contains("ENG_FLUID_PROPERTIES_TEXT"));
    assert!(pyxll_text.contains("ENG_FLUID_PROPERTIES_TABLE"));
    assert!(pyxll_text.contains("ENG_FLUID_PROPERTY_COUNT"));
    assert!(pyxll_text.contains("ENG_MATERIAL_PROPERTIES_TEXT"));
    assert!(pyxll_text.contains("ENG_MATERIAL_PROPERTIES_TABLE"));
    assert!(pyxll_text.contains("ENG_MATERIAL_PROPERTY_COUNT"));
    assert!(pyxll_text.contains("ENG_DEVICE_MODES_TEXT"));
    assert!(pyxll_text.contains("ENG_DEVICE_MODE_COUNT"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC("));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_PIVOT_MACH"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_PATH_TEXT"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_FROM_M_TO_P_P0"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_FROM_MU_DEG_TO_P_P0"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_FROM_NU_DEG_TO_M"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_FROM_M_TO_NU_DEG"));
    assert!(pyxll_text.contains("ENG_ISENTROPIC_FROM_A_ASTAR_TO_M"));
    assert!(pyxll_text.contains("ENG_COMPRESSIBLE_PRANDTL_MEYER_NU"));
    assert!(pyxll_text.contains("ENG_COMPRESSIBLE_PRANDTL_MEYER_M"));
    assert!(pyxll_text.contains("Arguments:"));
    assert!(pyxll_text.contains("state_prop_1"));

    let runtime_text = fs::read_to_string(py_runtime).expect("read generated runtime");
    assert!(runtime_text.contains("CREATE_NO_WINDOW"));
    assert!(runtime_text.contains("import engpy_native"));
    assert!(runtime_text.contains("def runtime_mode("));
    assert!(runtime_text.contains("def worker_stats("));
    assert!(runtime_text.contains("def runtime_info("));
    assert!(runtime_text.contains("builtins._ENGPY_CLIENT"));
    assert!(runtime_text.contains("\"encoding\": \"utf-8\""));
    assert!(runtime_text.contains("native_incompatible_no_worker"));
    assert!(runtime_text.contains("_switch_to_worker_fallback"));

    assert!(xloil_text.contains("(area_ratio, gamma, branch=\"\")"));
    assert!(pyxll_text.contains("(area_ratio, gamma, branch=\"\")"));
    assert!(xloil_text.contains("(rho, v, d, mu):"));

    let py_devices = fs::read_to_string(tmp.path().join("bindings/python/engpy/devices.py"))
        .expect("read generated python devices module");
    assert!(py_devices.contains("def isentropic_calc("));
    assert!(py_devices.contains("def isentropic_pivot_mach("));
    assert!(py_devices.contains("def isentropic_path_text("));
    assert!(py_devices.contains("def isentropic_from_nu_deg_to_m("));
    assert!(py_devices.contains("def isentropic_from_m_to_nu_deg("));

    let py_pm = fs::read_to_string(
        tmp.path()
            .join("bindings/python/engpy/equations/compressible/prandtl_meyer.py"),
    )
    .expect("read generated python PM module");
    assert!(py_pm.contains("def solve_nu("));
    assert!(py_pm.contains("def solve_m("));

    let pyproject_text = fs::read_to_string(pyproject).expect("read generated pyproject");
    assert!(pyproject_text.contains("maturin"));
    assert!(pyproject_text.contains("engpy_native"));
}

#[test]
fn generated_excel_bindings_only_use_supported_invoke_ops() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let protocol = tmp.path().join("bindings").join("invoke_protocol.json");
    let xloil = tmp.path().join("bindings/excel/eng_xloil.py");
    let pyxll = tmp.path().join("bindings/excel/eng_pyxll.py");

    let proto_text = fs::read_to_string(protocol).expect("read invoke protocol");
    let supported: serde_json::Value =
        serde_json::from_str(&proto_text).expect("parse invoke protocol json");
    let supported_ops: std::collections::HashSet<String> = supported["operations"]
        .as_array()
        .expect("protocol operations array")
        .iter()
        .filter_map(|v| v["op"].as_str())
        .map(|s| s.to_string())
        .collect();

    let invoke_re = Regex::new(r#"invoke\("([^"]+)""#).expect("regex");
    for path in [xloil, pyxll] {
        let text = fs::read_to_string(&path).expect("read generated excel binding");
        for cap in invoke_re.captures_iter(&text) {
            let op = cap.get(1).expect("capture op").as_str();
            assert!(
                supported_ops.contains(op),
                "generated op '{}' in {} is not in invoke protocol supported ops",
                op,
                path.display()
            );
        }
    }
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
        "import sys; sys.path.insert(0, r'{}'); import engpy; import engpy.equations; import engpy.fluids; import engpy.materials; import engpy.devices; import engpy.constants; import engpy.helpers; import engpy._runtime as rt; \
engpy.constants.get_constant(\"g0\"); s1 = rt.worker_stats(); mode = rt.runtime_mode(); engpy.constants.get_constant(\"pi\"); s2 = rt.worker_stats(); \
engpy.helpers.meta_get(\"equation\", \"structures.hoop_stress\", \"ascii\"); \
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

#[test]
fn generated_python_equation_modules_are_collision_free() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let eq_root = tmp.path().join("bindings/python/engpy/equations");
    let def_re = Regex::new(r"(?m)^def\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(").expect("regex");
    for entry in WalkDir::new(&eq_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n == "__init__.py")
        {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("py") {
            continue;
        }
        let text = fs::read_to_string(path).expect("read generated python equation module");
        let mut counts = std::collections::BTreeMap::<String, usize>::new();
        for cap in def_re.captures_iter(&text) {
            let name = cap.get(1).expect("def capture").as_str().to_string();
            *counts.entry(name).or_insert(0) += 1;
        }
        let dups: Vec<String> = counts
            .into_iter()
            .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
            .collect();
        assert!(
            dups.is_empty(),
            "duplicate function definitions in {}: {:?}",
            path.display(),
            dups
        );
    }
}

#[test]
fn binding_spec_python_names_are_unique_per_module_for_equations() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");
    let spec_path = tmp.path().join("bindings").join("binding_spec.json");
    let spec_text = fs::read_to_string(spec_path).expect("read binding spec");
    let spec: serde_json::Value = serde_json::from_str(&spec_text).expect("parse binding spec");
    let mut seen = std::collections::BTreeMap::<(String, String), String>::new();
    for f in spec["functions"].as_array().expect("functions array") {
        let module = f["python_module"].as_str().unwrap_or("").to_string();
        let name = f["python_name"].as_str().unwrap_or("").to_string();
        let id = f["id"].as_str().unwrap_or("").to_string();
        if !module.starts_with("equations.") {
            continue;
        }
        if let Some(prev) = seen.insert((module.clone(), name.clone()), id.clone()) {
            panic!(
                "duplicate python symbol {}.{} mapped to ids '{}' and '{}'",
                module, name, prev, id
            );
        }
    }
}

#[test]
fn generated_python_nested_equation_module_spotcheck_executes() {
    let python = Command::new("python").arg("--version").output();
    if python.is_err() {
        return;
    }
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");
    let python_root = tmp.path().join("bindings").join("python");
    let script = format!(
        "import sys; sys.path.insert(0, r'{}'); import engpy.equations.compressible.area_mach as area_mach; v = area_mach.solve_m(2.0, 1.4); assert v > 1.0",
        python_root.display()
    );
    let status = Command::new("python")
        .arg("-c")
        .arg(script)
        .env("ENG_WORKER_BIN", env!("CARGO_BIN_EXE_eng"))
        .status()
        .expect("run python nested module spotcheck");
    assert!(status.success(), "python nested module spotcheck failed");
}

#[test]
fn all_registered_devices_emit_binding_functions() {
    let tmp = tempdir().expect("tempdir");
    export_unified_docs_to(tmp.path()).expect("export unified docs");

    let spec_text = fs::read_to_string(tmp.path().join("bindings").join("binding_spec.json"))
        .expect("read binding spec");
    let spec: serde_json::Value = serde_json::from_str(&spec_text).expect("parse binding spec");
    let funcs = spec["functions"].as_array().expect("functions array");

    let xloil_text = fs::read_to_string(tmp.path().join("bindings/excel/eng_xloil.py"))
        .expect("read xloil module");
    let pyxll_text = fs::read_to_string(tmp.path().join("bindings/excel/eng_pyxll.py"))
        .expect("read pyxll module");
    let py_devices = fs::read_to_string(tmp.path().join("bindings/python/engpy/devices.py"))
        .expect("read generated python devices module");

    for device in eng::devices::generation_specs() {
        for binding_fn in device.binding_functions {
            let in_manifest = funcs
                .iter()
                .any(|f| f["id"] == binding_fn.id && f["op"] == binding_fn.op);
            assert!(
                in_manifest,
                "binding spec missing function id={} op={}",
                binding_fn.id, binding_fn.op
            );

            assert!(
                xloil_text.contains(binding_fn.excel_name),
                "xloil binding missing {}",
                binding_fn.excel_name
            );
            assert!(
                pyxll_text.contains(binding_fn.excel_name),
                "pyxll binding missing {}",
                binding_fn.excel_name
            );
            assert!(
                py_devices.contains(&format!("def {}(", binding_fn.python_name)),
                "python devices module missing function {}",
                binding_fn.python_name
            );
        }
    }
}
