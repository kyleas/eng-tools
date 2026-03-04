# Materials Guide

Materials provide temperature-conditioned property lookup from curated datasets.

## Property Lookup

```rust
use eng::materials;

let state = materials::stainless_304().temperature("350 K")?;
let e = state.property("elastic_modulus")?;
let sy = state.property("yield_strength")?;
```
