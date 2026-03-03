use eng::materials;

let state = materials::stainless_304().temperature("350 K")?;
let e = state.property("elastic_modulus")?;
let sy = state.property("yield_strength")?;
