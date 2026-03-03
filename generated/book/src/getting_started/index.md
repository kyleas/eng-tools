# Getting Started

Use the top-level `eng` facade for unified workflows.

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

You can also run directly from this repo:

```bash
cargo run -p eng --example unified_usage
cargo test -p eng core_handbook_workflows_execute
```

```rust
use eng::{constants, eq, equations, fluids, materials, qty};
```

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
