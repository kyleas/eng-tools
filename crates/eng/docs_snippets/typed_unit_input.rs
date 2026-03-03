use eng::{eq, equations};
use eng::core::units::typed::{length, pressure};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(pressure::mpa(2.5))
    .given_r(length::m(0.2))
    .given_t(length::mm(8.0))
    .value()?;
