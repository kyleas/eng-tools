# Bindings (Python and Excel)

Rust remains the authoritative implementation. Generated Python and Excel bindings are thin adapters over the same public binding model.

## Generated Outputs

- `generated/bindings/binding_spec.json`
- `generated/bindings/invoke_protocol.json`
- `generated/bindings/python/engpy/...`
- `generated/bindings/python/engpy/study.py`
- `generated/bindings/python/pyproject.toml` (maturin build config for `engpy_native`)
- `generated/bindings/excel/eng_xloil.py`
- `generated/bindings/excel/eng_pyxll.py`

`binding_spec.json` is transport-agnostic (function names, args, returns, help, examples). `invoke_protocol.json` documents the current runtime transport contract.

## Naming Rules

- Python: namespaced modules (`engpy.equations.<category>.<equation_slug>.*`, `engpy.devices.*`, `engpy.fluids.*`, `engpy.materials.*`, `engpy.constants.*`).
- Excel: flat worksheet-friendly functions (`ENG_*`).
- Families are exposed under `engpy.equations.families.<family>.<variant>`.

## Metadata and Diagnostics Functions

- Python:
  - `engpy.equations.meta.equation_meta(path_id)`
  - `engpy.equations.meta.equation_ascii(path_id)`
  - `engpy.equations.meta.equation_unicode(path_id)`
  - `engpy.equations.meta.equation_latex(path_id)`
  - `engpy.equations.meta.equation_targets(path_id)`
  - `engpy.equations.meta.equation_variables(path_id)`
  - `engpy.equations.meta.equation_name(path_id)`
  - `engpy.equations.meta.equation_description(path_id)`
  - `engpy.equations.meta.equation_family(path_id)`
  - `engpy.equations.meta.equation_default_unit(path_id, variable)`
  - `engpy.helpers.equation_targets_text(path_id)`
  - `engpy.helpers.equation_variables_text(path_id)`
  - `engpy.helpers.equation_branches_text(path_id)`
  - `engpy.helpers.fluid_properties_text(key)`
  - `engpy.helpers.material_properties_text(key)`
  - `engpy.helpers.device_modes_text(key)`
  - `engpy.helpers.equation_targets_table(path_id)`
  - `engpy.helpers.equation_variables_table(path_id)`
  - `engpy.helpers.equation_branches_table(path_id)`
  - `engpy.helpers.fluid_properties_table(key)`
  - `engpy.helpers.material_properties_table(key)`
  - `engpy.helpers.equation_target_count(path_id)`
  - `engpy.helpers.equation_variable_count(path_id)`
  - `engpy.helpers.fluid_property_count(key)`
  - `engpy.helpers.material_property_count(key)`
  - `engpy.helpers.device_mode_count(key)`
  - `engpy.helpers.format_value(value, in_unit, out_unit)`
  - `engpy.helpers.meta_get(entity, key, field)`
  - `engpy.study.equation_sweep_table(...)`
  - `engpy.study.device_table(...)`
  - `engpy.study.workflow_table(...)`
  - `engpy.study.isentropic_m_to_p_p0_table(...)`
  - `engpy.study.nozzle_flow_table(...)`
  - `engpy.study.normal_shock_table(...)`
  - `engpy.study.nozzle_normal_shock_workflow_table(...)`
- Excel:
  - `ENG_FORMAT(value, in_unit, out_unit)`
  - `ENG_META(entity, key, field)`
  - `ENG_EQUATION_META(path_id)`
  - `ENG_EQUATION_ASCII(path_id)`
  - `ENG_EQUATION_UNICODE(path_id)`
  - `ENG_EQUATION_LATEX(path_id)`
  - `ENG_EQUATION_TARGETS(path_id)`
  - `ENG_EQUATION_VARIABLES(path_id)`
  - `ENG_EQUATION_NAME(path_id)`
  - `ENG_EQUATION_DESCRIPTION(path_id)`
  - `ENG_EQUATION_FAMILY(path_id)`
  - `ENG_EQUATION_DEFAULT_UNIT(path_id, variable)`

  - `ENG_STUDY_EQUATION_TABLE(path_id, target, sweep_variable, start, end, count, [spacing], [branch])`
  - `ENG_STUDY_DEVICE_TABLE(device_key, sweep_arg, start, end, count, fixed_args_json, [outputs_csv])`
  - `ENG_STUDY_WORKFLOW_TABLE(workflow_key, sweep_arg, start, end, count, fixed_args_json)`

  - Convenience wrappers:
  - `ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(gamma, start, end, count, [branch])`
  - `ENG_STUDY_NOZZLE_FLOW_TABLE(gamma, start, end, count, branch)`
  - `ENG_STUDY_NORMAL_SHOCK_TABLE(gamma, start, end, count)`
  - `ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(gamma, start, end, count, branch)`

  - `ENG_EQUATION_TARGETS_TEXT(path_id)` / `ENG_EQUATION_VARIABLES_TEXT(path_id)`
  - `ENG_EQUATION_BRANCHES_TEXT(path_id)`
  - `ENG_FLUID_PROPERTIES_TEXT(fluid_key)` / `ENG_MATERIAL_PROPERTIES_TEXT(material_key)` / `ENG_DEVICE_MODES_TEXT(device_key)`
  - `ENG_EQUATION_TARGETS_TABLE(path_id)` / `ENG_EQUATION_VARIABLES_TABLE(path_id)`
  - `ENG_EQUATION_BRANCHES_TABLE(path_id)`
  - `ENG_FLUID_PROPERTIES_TABLE(fluid_key)` / `ENG_MATERIAL_PROPERTIES_TABLE(material_key)`
  - `ENG_EQUATION_TARGET_COUNT(path_id)` / `ENG_EQUATION_VARIABLE_COUNT(path_id)`
  - `ENG_FLUID_PROPERTY_COUNT(fluid_key)` / `ENG_MATERIAL_PROPERTY_COUNT(material_key)` / `ENG_DEVICE_MODE_COUNT(device_key)`

Delimited TEXT helpers use `; ` as a deterministic separator.

Use these helpers for a composable workflow: keep core engineering calls simple, then layer formatting/reference metadata as needed.

### Clean Excel Pattern

```excel
=ENG_HOOP_STRESS_SIGMA_H(P, r, t)
=ENG_FORMAT(ENG_HOOP_STRESS_SIGMA_H(P, r, t), "Pa", "psia")
=ENG_EQUATION_ASCII("structures.hoop_stress")
=ENG_META("equation", "structures.hoop_stress", "targets")
```

### Excel single-cell text helpers

```excel
=ENG_EQUATION_TARGETS_TEXT("structures.hoop_stress")
=ENG_EQUATION_VARIABLES_TEXT("structures.hoop_stress")
=ENG_EQUATION_BRANCHES_TEXT("compressible.area_mach")
=ENG_FLUID_PROPERTIES_TEXT("H2O")
=ENG_MATERIAL_PROPERTIES_TEXT("stainless_304")
=ENG_DEVICE_MODES_TEXT("pipe_loss")
```

### Excel spill-range table helpers

```excel
=ENG_EQUATION_VARIABLES_TABLE("structures.hoop_stress")
=ENG_EQUATION_TARGETS_TABLE("structures.hoop_stress")
=ENG_EQUATION_BRANCHES_TABLE("compressible.area_mach")
=ENG_FLUID_PROPERTIES_TABLE("H2O")
=ENG_MATERIAL_PROPERTIES_TABLE("stainless_304")
```

Native in-process runtime supports Python usage on Linux and Windows without requiring a platform-specific executable per call.

## Build / Install (Native Python Runtime)

- From `generated/bindings/python`, build/install the extension with maturin (for example `maturin develop` in an active Python environment).
- Generated `engpy` wrappers call `engpy_native` in-process by default.
- If the extension is unavailable, wrappers automatically fall back to worker mode.

### One-command Setup Helpers

- Windows: `scripts/setup-native-bindings.ps1`
- Linux: `scripts/setup-native-bindings.sh`
- Runtime verification: `scripts/verify-native-bindings.ps1` or `scripts/verify-native-bindings.sh`

These scripts create/use a virtual environment, install `maturin`, install `engpy_native`, then verify runtime mode.

## Excel Function Help and Intellisense

- Generated xlOil/PyXLL functions now include richer help text: summary, per-argument guidance, return info, and a formula example.
- Argument names are optimized for Excel readability on binding-friendly functions (for example pipe-loss uses `density`, `viscosity`, `roughness` instead of terse symbols).
- Function Wizard help is the primary native Excel help surface for custom functions.
- Excel custom function inline IntelliSense popups can be limited natively; when richer inline tooltip behavior is needed, use an IntelliSense add-in path (for example Excel-DNA IntelliSense with compatible workflows).
- Shortcut: `Ctrl+Shift+A` inserts function arguments into a worksheet formula to make argument order explicit.

## Runtime Protocol and Transport

- Default runtime is in-process via native Rust/Python extension module (`engpy_native`).
- Compatibility fallback runtime is persistent `eng worker` over stdio JSON-lines.
- The per-call envelope is unchanged: `protocol_version`, `op`, optional `request_id`, `args`.
- Generated Python runtime prefers native in-process mode, and only uses worker when native is unavailable or `ENGPY_RUNTIME=worker` is set.
- On Windows worker fallback startup is hidden (`CREATE_NO_WINDOW`) to avoid console popup windows during Excel recalculation.
- Worker fallback executable resolution: `ENG_WORKER_BIN` (fallback: `ENG_BIN`, then `eng`).
- Runtime preference override: `ENGPY_RUNTIME=native|worker`.
- Success response: `ok=true`, `value`, plus echoed `protocol_version`/`op`/`request_id`.
- Error response: `ok=false`, `error.code`, `error.message`, optional `error.field` and `error.detail`.
- Excel docs show one formula surface because xlOil and PyXLL are generated identically.
- No engineering physics logic is implemented in generated Python modules.
- Equation/device/fluid/material/constant behavior remains in Rust.

## Error Model (Bindings)

- Stable error `code` values are intended for wrapper logic and automation.
- Human-facing `message` remains suitable for worksheet/Python troubleshooting.
- `field` and `detail` provide argument-level and operation context.

## Runtime Troubleshooting

- Use `engpy._runtime.runtime_mode()` to confirm active runtime (`native` or `worker`).
- Use `engpy._runtime.worker_stats()` to inspect runtime request counters, last failure, and worker PID when fallback is active.
- Use `engpy._runtime.worker_pid()` for quick worker PID checks in fallback mode.
- If needed, call `engpy._runtime.stop_worker()` to force a clean worker restart on next request (no-op in native mode).
- If mode is unexpectedly `worker`, verify `engpy_native` imports in the same Python environment used by xlOil/PyXLL.
- Runtime preference override: `ENGPY_RUNTIME=native|worker`.
- Worker fallback executable overrides: `ENG_WORKER_BIN` and `ENG_BIN`.

Verification snippet:

```python
import engpy_native
import engpy._runtime as rt
print(rt.runtime_mode())
print(engpy_native.runtime_info())
```

## CI vs Local Verification

- CI/repo checks validate generated binding artifacts, protocol/schema, docs generation, and runtime diagnostics surfaces.
- Native environment activation (`maturin develop` + import in target interpreter) is machine/environment-specific and should be validated with the setup/verify scripts on each dev machine.
