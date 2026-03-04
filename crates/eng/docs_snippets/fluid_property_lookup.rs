use eng::fluids;
use eng::units::typed::{pressure, temperature};

let state = fluids::water().state_tp(temperature::k(300.0), pressure::bar(1.0))?;
let rho = state.rho()?;
let mu = state.mu()?;
let cp = state.cp()?;

let state_generic = fluids::air().state("T", "300 K", "P", "1 bar")?;
let gamma = state_generic.gamma()?;
