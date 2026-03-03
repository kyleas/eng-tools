use eng::{eq, equations, qty};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p(qty!("5 MPa + 12 psi"))
    .given_r(qty!("0.2 m"))
    .given_t(qty!("8 mm"))
    .value()?;
