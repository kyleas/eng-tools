use eng::{eq, equations};

let sigma_h_pa = eq
    .solve(equations::structures::hoop_stress::equation())
    .target_sigma_h()
    .given_p("5 MPa + 12 psi")
    .given_r("0.2 m")
    .given_t("8 mm")
    .value()?;
