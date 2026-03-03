use eng::{eq, equations};

let p_pa = eq
    .solve(equations::thermo::ideal_gas::density::equation())
    .target_p()
    .given_rho("1.225 kg/m^3")
    .given_r("287 J/(kg*K)")
    .given_t("288.15 K")
    .value()?;
