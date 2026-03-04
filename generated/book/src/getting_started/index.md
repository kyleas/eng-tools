# Getting Started

The top-level crate is `eng`. It re-exports equations, fluids, materials, constants, units, and docs export APIs.

## Dependencies

If `eng` is published, add:

```toml
[dependencies]
eng = "0.1"
```

For local workspace use from the generated handbook root (`generated/book`):

```toml
[dependencies]
eng = { path = "../../crates/eng" }
```

Run locally from this repo:

```bash
cargo run -p eng --example unified_usage
cargo test -p eng core_handbook_workflows_execute
```

## Primary Imports

```rust
use eng::{constants, devices, eq, equations, fluids, materials, qty};
```

## First Successful Solve

```rust
use eng::{eq, equations};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(2.5e6)
    .given_r(0.2)
    .given_t(0.008)
    .value()?;
```

Next: [Input Styles](../input_styles/index.md), [Equations Guide](../equations/guide.md), and [Examples & Workflows](../workflows/index.md).
