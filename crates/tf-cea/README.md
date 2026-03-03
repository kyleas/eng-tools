# tf-cea

`tf-cea` is Thermoflow's backend-first integration crate for CEA-driven thermochemistry and rocket performance.

## Backend policy

- This crate does **not** implement a new equilibrium solver.
- Physics comes from an external CEA backend.
- Current integration path is a subprocess adapter (`CeaProcessAdapter`) using JSON-over-stdio to call a backend bridge executable.

Set `TF_CEA_BACKEND_EXECUTABLE` to a CEA bridge executable path.

## Vertical slices

- Equilibrium analysis: `CeaBackend::run_equilibrium`
- Rocket performance analysis: `CeaBackend::run_rocket`

Both slices use common domain models in `src/model.rs` and can support future nozzle/combustor extensions.
