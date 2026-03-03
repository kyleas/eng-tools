# CEA In-Process Migration Plan (Cross-Platform, No External Executable)

## Goal

Run CEA from inside Thermoflow (UI + CLI) without requiring `TF_CEA_BACKEND_EXECUTABLE` or a separate bridge app.

---

## Current State (why this hurts)

Today `tf-cea` uses a subprocess adapter:

- `CeaProcessAdapter` in `crates/tf-cea/src/adapter.rs`
- executable path from `TF_CEA_BACKEND_EXECUTABLE`

This adds operational friction:

- users must install/manage another binary
- path/env setup differs by OS and shell
- version drift between Thermoflow and bridge binary

---

## Recommended Architecture

Adopt an **in-process backend** with this layering:

1. `tf-cea-native-sys` (new crate)
   - compiles/links NASA CEA Fortran/C sources as a static library
   - exposes minimal FFI symbols

2. `tf-cea-native` (new crate)
   - safe Rust wrapper around FFI
   - converts `tf-cea::model::{BackendProblem, BackendResult}` to/from native calls
   - thread/process isolation strategy for any non-reentrant global state

3. `tf-cea` (existing crate)
   - keeps `CeaBackend` trait as stable seam
   - adds `InProcessCeaBackend` implementation
   - keeps `CeaProcessAdapter` as fallback/dev mode

4. `tf-ui` and `tf-cli`
   - default to in-process backend
   - process backend remains optional via explicit config for debugging/legacy

---

## Why this is the best path

Compared to re-implementing CEA physics in Rust:

- preserves NASA CEA parity
- dramatically lower validation burden
- minimizes scientific-model risk

Compared to process adapter:

- true single-binary UX for users
- fewer setup steps
- easier packaging and reproducibility

---

## Cross-Platform Build Strategy

### Windows

- Build native backend using one toolchain path:
  - preferred: Intel/ifx or gfortran + C static lib output
- Link static artifact into Rust crate (`build.rs` + `cargo:rustc-link-lib=static=...`)

### Linux

- Use gfortran/gcc toolchain in CI and release images
- Build static lib during crate build or prebuild and vendored artifact

### macOS

- Use gfortran (brew) in CI for x86_64 + arm64
- produce universal artifact or dual-target artifacts

### Packaging principle

- distribute one Thermoflow executable bundle that already contains CEA native code
- no runtime bridge executable path required

---

## Runtime Contract Changes

Keep `CeaBackend` trait unchanged:

- `run_equilibrium`
- `run_rocket`

Add backend selection in config (priority order):

1. explicit CLI/UI/backend option
2. env override (`TF_CEA_BACKEND_MODE=inprocess|process`)
3. default = `inprocess`

If in-process is unavailable at build time, fail with clear message and optionally fall back to process mode only when explicitly requested.

---

## Safety / Concurrency

NASA CEA implementations often have global mutable state.

Implement one of:

- global `Mutex` around backend calls (simple, safe)
- single-thread worker model with request queue (better throughput isolation)

Start with `Mutex`, then optimize after correctness validation.

---

## Validation Plan

1. Golden parity suite
   - run representative equilibrium + rocket cases through both backends
   - enforce tolerances for key outputs (`c*`, `Cf`, `Isp`, chamber `T`, `gamma`, `MW`)

2. Determinism checks
   - repeat same case N times, assert stable outputs

3. Cross-platform CI matrix
   - windows-latest, ubuntu-latest, macos-latest
   - `cargo test -p tf-cea`
   - `cargo test -p tf-rpa`
   - smoke `tf-cli rocket solve`

---

## Migration Phases

### Phase 1: Introduce backend mode abstraction

- add backend mode enum/config in `tf-cea`
- central backend factory used by UI + CLI
- keep process backend behavior unchanged

### Phase 2: Add native sys crate

- add `tf-cea-native-sys` with `build.rs`
- compile/link native CEA library on all OS targets

### Phase 3: Safe wrapper + integration

- add `tf-cea-native`
- map problem/result structs
- wire into `tf-cea` as `InProcessCeaBackend`

### Phase 4: Default switch

- UI/CLI default to in-process backend
- process backend becomes opt-in fallback

### Phase 5: Remove executable UX dependency

- remove mandatory executable field in Rocket UI
- keep optional advanced override only if needed

---

## Immediate Repo Tasks

1. Add backend factory in `tf-cea` (mode-based)
2. Switch `tf-ui` Rocket solve path to factory instead of direct `CeaProcessAdapter`
3. Switch `tf-cli rocket solve` path to factory
4. Add golden test vectors in `tf-rpa` tests
5. Add CI jobs for cross-platform native build validation

---

## Decision

To satisfy “cross-platform with no separate app,” Thermoflow should embed CEA as a linked in-process backend and treat subprocess mode as optional legacy/debug path.
