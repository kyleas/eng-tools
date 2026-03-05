# tf-eng Study Explorer Metadata

`tf-eng` is the Thermoflow-facing bridge over `eng` solve/study execution.

## Responsibilities

- Discover studyable equation/device/workflow targets from `eng` metadata.
- Expose target descriptors (fields, outputs, defaults, sweep candidates).
- Validate form/CLI input before execution.
- Run single solves for equation/device/workflow targets.
- Run studies through `eng` and normalize to plot/table-friendly output.

## Workbench modes

Thermoflow Eng Workbench uses these descriptor surfaces for:

- `Solve`: single-shot evaluation (including equation missing-one-variable target inference).
- `Study`: sweep/study execution.
- `Reference`: metadata inspection (inputs/outputs/branches/displays).

## Live solve and units

- Solve fields accept plain numbers or eng-native quantity strings (`"300 K"`, `"2 in"`, `"10 ft/s"`).
- `tf-eng` validates/normalizes through `eng`/`eng-core` unit parsing semantics.
- Solve readiness and ambiguity diagnostics are emitted before execution:
  - invalid parse
  - missing required fields
  - equation one-unknown inference
  - output selection validity

## Equation display in egui

- Egui rendering uses durable display priority:
  1. unicode equation text
  2. ascii equation text
  3. latex shown in detail disclosure
- This avoids brittle runtime LaTeX dependencies while keeping metadata complete.

## Auto-discovery model

New `eng` equations/devices/workflows appear in Thermoflow without new UI form code as long as they are exposed through the existing `eng` study metadata surfaces.

## Presets

Presets in Thermoflow are data-only pre-fills over the generic descriptor-driven request path.
They do not add custom solver logic.
