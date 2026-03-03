use eng::fluids::{self, FluidProperty};

let state = fluids::water().state_tp("300 K", "1 bar")?;
let rho = state.property(FluidProperty::Density)?;
let mu = state.property(FluidProperty::DynamicViscosity)?;
