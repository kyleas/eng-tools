# tf-eng Study Explorer Metadata

`tf-eng` is the Thermoflow-facing bridge over `eng` study execution.

## Responsibilities

- Discover studyable equation/device/workflow targets from `eng` metadata.
- Expose target descriptors (fields, outputs, defaults, sweep candidates).
- Validate form/CLI input before execution.
- Run studies through `eng` and normalize to plot/table-friendly output.

## Auto-discovery model

New `eng` equations/devices/workflows appear in Thermoflow without new UI form code as long as they are exposed through the existing `eng` study metadata surfaces.

## Presets

Presets in Thermoflow are data-only pre-fills over the generic descriptor-driven request path.
They do not add custom solver logic.
