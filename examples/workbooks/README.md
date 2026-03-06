# Engineering Workbook Templates

This directory contains production-style `.engwb` workbook templates for Thermoflow's Engineering Workbook.

## Templates

- `injector_orifice_sizer.engwb`
  - Demonstrates: EasyMark narrative rows, unit-aware constants, equation solves, device study + plot, cross-row refs, intentional invalid row.
  - Key targets: `fluids.orifice_mass_flow_incompressible`, `fluids.circular_pipe_area`, `normal_shock_calc`.

- `pipe_flow_pump_power.engwb`
  - Demonstrates: pipe sizing constants, equation solves, study + plot, invalid row handling, rename-safe refs.
  - Key targets: `fluids.circular_pipe_area`, `fluids.reynolds_number`, `fluids.darcy_weisbach_pressure_drop`, `nozzle_flow_calc`.

- `nozzle_shock_backpressure.engwb`
  - Demonstrates: compressible constants, equation solves, workflow study, device study, plot output, branch usage notes.
  - Key targets: `compressible.area_mach`, `compressible.isentropic_pressure_ratio`, `compressible.normal_shock_pressure_ratio`, `nozzle_normal_shock_chain`, `nozzle_flow_calc`.

- `oblique_vs_cone_shock.engwb`
  - Demonstrates: oblique-shock equation solves, shock/nozzle device studies, comparative plots, invalid row handling.
  - Key targets: `compressible.oblique_shock_theta_beta_m`, `compressible.mach_angle`, `normal_shock_calc`, `nozzle_flow_calc`.

- `engineering_logbook.engwb`
  - Demonstrates: narrative-first logbook style, quick equation solves with one-unknown inference, device study + plot, and intentional parse failure row.
  - Key targets: `structures.hoop_stress`, `compressible.mach_angle`, `isentropic_calc`.

## CLI usage

Validate a workbook:

```powershell
cargo run -p tf-cli -- workbook validate examples/workbooks/<name>.engwb
```

Run a workbook and print JSON results:

```powershell
cargo run -p tf-cli -- workbook run examples/workbooks/<name>.engwb --format json
```

Run a workbook and export CSV tables:

```powershell
cargo run -p tf-cli -- workbook run examples/workbooks/<name>.engwb --format csv --out-dir examples/workbooks/_exports
```

Rename a key and rewrite downstream refs:

```powershell
cargo run -p tf-cli -- workbook rename examples/workbooks/injector_orifice_sizer.engwb dp_orifice dp_injector
```


