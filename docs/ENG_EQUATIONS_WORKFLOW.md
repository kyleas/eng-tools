# Engineering Equations Workflow

## Ownership

`eng-core` owns shared engineering infrastructure:
- unit parsing and conversion primitives
- default unit lookup by dimension
- shared numeric/ID/timing foundation

`equations` owns equation behavior:
- YAML registry format, loading, normalization, validation
- expression evaluation and solving
- first-class engineering constants registry + typed API
- registry-defined tests
- schema generation and docs/search/html/pdf exports

`eng` owns the **public unified export orchestration**:
- unified docs/search/catalog artifact exports
- unified mdBook source export
- unified HTML handbook export
- unified PDF handbook export
- canonical top-level output root (`generated/`)

## Authoring Styles

Both styles are supported and normalize to the same runtime model:

1. **Verbose form**
- Explicitly sets symbols, units, tolerances, numerical settings, and per-target methods.
- Use for unusual equations or when strict override control is required.

2. **Minimal form**
- Uses aggressive safe defaults for symbol, units, numerical policy, tolerances, test target expansion, and diagram labels.
- Use for most equations.

Reference examples:
- `crates/equations/examples/authoring/hoop_stress_typical.yaml`
- `crates/equations/examples/authoring/hoop_stress_verbose.yaml`
- `crates/equations/examples/authoring/area_mach_verbose_branches.yaml`
- `crates/equations/examples/authoring/hoop_stress_minimal.yaml`

## Identity Model

Authored:
- `key` (required, short taxonomy-independent id, non-dotted)
- `taxonomy` (required)
- `slug` (optional; defaults to `key`)
- `aliases` (optional; short non-dotted ids only)

Derived:
- `effective_slug = slug || key`
- `path_id = category + subcategories + effective_slug`

## Relation Authoring

Preferred minimal form:
- `residual: "..."`

Verbose form remains supported:
- `relation: { form: residual, residual: "..." }`

## Defaults Philosophy

Defaults are centralized in:
- `crates/equations/src/defaults.rs`
- `crates/equations/src/normalize.rs`

### Variable defaults
- `symbol` optional: defaults to variable key
- `default_unit` optional: inferred from dimension via `eng-core`
- `description` optional
- `constraints` optional: dimension defaults are applied automatically
  - positive by default: `pressure`, `stress`, `length`, `diameter`, `distance`, `roughness`, `friction_factor`, `mach`, `temperature`, `density`, `viscosity`
  - use explicit constraints only for additions/overrides such as `nonzero`, `min`, `max`, `integer`

### Numerical solve defaults
- Policy is exclusion-based: all variables are numerically solvable unless listed in `unsupported_targets`
- Dimension-based defaults for initial guess/bracket/max_iter apply when not overridden

### Test defaults
- single-case shorthand supported:
  - `tests.case.full_state: {...}` (auto id `case_1`)
  - `tests.cases: [{ full_state: {...} }]` (auto ids `case_1`, `case_2`, ...)
  - `tests: [{ full_state: {...} }]` (minimal list shorthand; same auto id behavior)
- `full_state` should include every equation variable; validation enforces complete baseline states.
- If `verify.solve_targets` is omitted, solver checks expand to all supported targets
- Tolerance hierarchy:
1. global defaults
2. equation-level (`tests.relation_tolerance`, `tests.solve_tolerance`)
3. case-level (`case.tolerances`)
4. target-level (`verify.solve_targets[*].tolerances`)

### Diagram defaults
- If `diagram.template` exists and `diagram.labels` is omitted, labels are auto-derived from variable names.

### Display defaults (LaTeX-first)
- `display.latex` is the primary authored representation (required).
- `display.unicode`, `display.ascii`, and `display.description` are optional overrides.

Derivation precedence:
1. Use explicit override if provided.
2. Otherwise derive from canonical expression source:
   - `solve.default_target` explicit form if present
   - otherwise first explicit form (deterministic target ordering)
   - otherwise residual form rendered as `residual = 0`

`ascii` derivation:
- plain deterministic math text formatting

`unicode` derivation:
- starts from same canonical expression
- substitutes variable symbols (if provided) and common LaTeX-like Greek macros
- applies lightweight readability transforms (for example: `sqrt` to root symbol, `*` to middle dot, and simple superscripts)

## Minimal Common Pattern

```yaml
key: hoop_stress
taxonomy:
  category: structures
name: Thin-Wall Hoop Stress
display:
  latex: "\\sigma_h = \\frac{P r}{t}"
variables:
  sigma_h: { name: Hoop stress, dimension: stress }
  P: { name: Internal pressure, dimension: pressure }
  r: { name: Mean radius, dimension: length }
  t: { name: Wall thickness, dimension: length, constraints: { nonzero: true } }
residual: "sigma_h - (P * r / t)"
solve:
  explicit_forms:
    sigma_h: P * r / t
    P: sigma_h * t / r
    r: sigma_h * t / P
    t: P * r / sigma_h
assumptions: [Thin-wall approximation is valid.]
tests:
  - full_state: { P: "2.5 MPa", r: "0.2 m", t: "8 mm", sigma_h: "62.5 MPa" }
```

When omitted:
- `solve.numerical` defaults to numerically supporting all variables
- `case.branch` defaults to no explicit branch selection
- `case.verify` defaults to `residual_zero: true` and solve-all-supported-targets

## Commands

Preferred unified public API (Rust):

```rust
use eng::docs;

let out = docs::export_unified_docs()?;
let book = docs::export_unified_mdbook()?;
let html = docs::export_unified_html()?;
let pdf = docs::export_unified_pdf()?;
let catalog = docs::export_unified_catalog()?;
```

CLI tooling (currently implemented by the `equations` binary):

```bash
cargo run -p equations --bin equations -- generate-schema
cargo run -p equations --bin equations -- validate --with-tests
cargo run -p equations --bin equations -- test-registry
cargo run -p equations --bin equations -- export-docs
cargo run -p equations --bin equations -- export-mdbook
cargo run -p equations --bin equations -- export-html   # alias to export mdBook source/build
cargo run -p equations --bin equations -- export-pdf
cargo run -p equations --bin equations -- lint
cargo run -p equations --bin equations -- scaffold --key hoop_stress --category structures
```

Generated mdBook source tree (default):
- `generated/book/book.toml`
- `generated/book/src/SUMMARY.md`
- `generated/book/src/constants/...`
- `generated/book/src/equations/<category>/...`

## Scale Authoring Checklist

1. Scaffold a new equation file with `equations scaffold`, then fill in variables/residual/explicit forms.
2. Keep `key` short and taxonomy-independent; keep aliases short and non-dotted.
3. Provide at least one complete `full_state` baseline in tests (all variables present).
4. Run `equations validate --with-tests` before committing.
5. Export artifacts after registry updates to keep `generated/` goldens in sync.

## Export Artifact Contract

`search_index.json`, `page_models.json`, and `navigation.json` now use a stable envelope:

- `schema_version`: export schema contract version
- `model_version`: data model version for downstream compatibility checks
- `artifact_type`: one of `search_index`, `page_models`, `navigation`
- `items`: deterministic sorted payload

Additional exported artifacts:
- `examples_index.json` (generated usage snippets from validated test baselines)
- `constants.json` (centralized engineering constants used across equations/docs)
- `architecture_spec.json` (layer boundaries, ownership map, and prototype planning data)

## Constants API

Constants are sourced from `crates/equations/constants/constants.yaml` and generated into typed Rust functions:

```rust
use equations::constants;

let g0 = constants::g0();
let sigma = constants::stefan_boltzmann();
```

Constants referenced by equation expressions are auto-resolved by default at solve time.
Authors and users should not treat these as normal required givens.

Advanced override path:

```rust
use equations::eq;

let isp = eq
    .solve(equations::rockets::specific_impulse_ideal::equation())
    .target_i_sp()
    .given_c_f(1.7684408756881704)
    .given_c_star(1718.7683350153386)
    .override_constant("g0", 9.81)
    .value()?;
```

Recommended constant metadata fields:
- `source`: provenance/reference basis
- `note`: short convention/usage caveat
- `exact`: `true` for defining exact values, `false` for conventional/reference values

## Runtime Usage

Preferred builder API:

```rust
use equations::{eq};

let sigma_h = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

Equation input styles (preferred order):
1. Plain numeric SI (fastest; assumed canonical SI for that variable)
2. Typed constructor (`eng_core::units::typed::*`)
3. Compile-time `qty!("...")` literal
4. Runtime string convenience

```rust
use equations::eq;
use eng_core::units::{qty, typed::{length, pressure}};

let sigma_h = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)                    // SI numeric
    .given_r(length::m(0.2))           // typed constructor
    .given_t(qty!("8 mm"))             // compile-time quantity literal
    .value()?;

let sigma_h_expr = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("5 MPa + 12 psi")         // runtime quantity expression
    .given_r("3 ft + 2 in")
    .given_t(0.008)
    .value()?;
```

Units-aware solve:

```rust
use equations::{eq};

let sigma_h_mpa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("2.5 MPa")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value_in("MPa")?;
```

Branch + diagnostics solve:

```rust
use equations::{eq, SolveMethod};

let result = eq
    .solve(equations::compressible::area_mach())
    .target_m()
    .branch_supersonic()
    .method(SolveMethod::Auto)
    .given_area_ratio(2.0049745454545462)
    .given_gamma(1.4)
    .result()?;
```

Generated convenience functions (explicit-form targets only):

```rust
use equations::{structures};

let sigma_h = structures::solve_hoop_stress_sigma_h("2.5 MPa", "0.2 m", "8 mm")?;
let p = structures::solve_hoop_stress_p("62.5 MPa", "0.2 m", "8 mm")?;
```

Short helpers remain available:

```rust
use equations::{eq};

let sigma_h = eq.solve_value(
    "structures.hoop_stress",
    "sigma_h",
    [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
)?;

let sigma_h_typed = eq.solve_value(
    equations::structures::hoop_stress(),
    "sigma_h",
    [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
)?;

let sigma_h_mpa = eq.solve_value_in(
    "structures.hoop_stress",
    "sigma_h",
    [("P", "2.5 MPa"), ("r", "0.2 m"), ("t", "8 mm")],
    "MPa",
)?;
```

Complete runnable workflow examples:
- `crates/equations/examples/registry_usage.rs` (short path)
- `crates/equations/examples/equation_examples.rs` (all intended workflows)

CI hardening helper (PowerShell):

```powershell
./crates/equations/scripts/check-ci.ps1
```

Unified product verification (code + docs/exports; recommended before merge):

```powershell
./scripts/verify-unified.ps1
```

Docs/export rule: generated artifacts are user-facing product outputs. Missing/stale generated files are treated as failures and must be regenerated in the same pass.
This includes architecture outputs used by the handbook (`generated/book/src/architecture/index.md`) and machine-readable spec (`generated/architecture_spec.json`).

Optional flags:
- `--registry-dir PATH`
- `--schema-out PATH`
- `--out-dir PATH`


