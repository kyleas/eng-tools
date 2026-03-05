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

## Auto-discovery model

New `eng` equations/devices/workflows appear in Thermoflow without new UI form code as long as they are exposed through the existing `eng` study metadata surfaces.

## Presets

Presets in Thermoflow are data-only pre-fills over the generic descriptor-driven request path.
They do not add custom solver logic.
