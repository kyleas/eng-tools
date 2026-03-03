# CEA Backend Selection (Phase 5 major backend branch)

## Decision

Thermoflow now introduces `tf-cea` with a **backend-first adapter** that executes an **existing CEA implementation via subprocess**.

Chosen integration path:

- `CeaProcessAdapter` (Rust) -> external CEA bridge executable (`TF_CEA_BACKEND_EXECUTABLE`) -> NASA CEA implementation.

## Option review

1. **Rust-native CEA solver/binding**
   - No verified, mature Rust-native NASA-CEA-equivalent backend was available in this environment.
   - CEA-adjacent tooling may exist, but we did not verify full NASA CEA parity for equilibrium + rocket modes.

2. **Official NASA CEA backend via wrapper** (**selected**)
   - Keeps authoritative physics in existing CEA code.
   - Allows Thermoflow to focus on domain model, normalization, and integration seams.
   - Adapter boundary keeps process/text conventions out of core APIs.

## Explicit non-goal

`tf-cea` does **not** implement Gibbs minimization or new equilibrium/rocket physics from scratch.
