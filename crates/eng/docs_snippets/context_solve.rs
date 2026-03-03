use eng::{eq, equations, fluids};

let re = eq
    .solve_with_context(equations::fluids::reynolds_number::equation())
    .fluid(fluids::water().state_tp("300 K", "1 bar")?)
    .for_target("Re")
    .given("V", "3 m/s")
    .given("D", "0.1 m")
    .value()?;
