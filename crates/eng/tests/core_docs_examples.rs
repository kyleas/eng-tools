use eng::{
    eq, equations, fluids, materials, qty,
    units::typed::{length, pressure},
};

#[test]
fn core_handbook_workflows_execute() -> Result<(), Box<dyn std::error::Error>> {
    {
        include!("../docs_snippets/fluid_state_constructors.rs");
    }

    let sigma_h_si = eq
        .solve(equations::structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()?;
    assert!(sigma_h_si > 0.0);

    let sigma_h_typed = eq
        .solve(equations::structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(pressure::mpa(2.5))
        .given_r(length::m(0.2))
        .given_t(length::mm(8.0))
        .value()?;
    assert!(sigma_h_typed > 0.0);

    let sigma_h_qty = eq
        .solve(equations::structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(qty!("5 MPa + 12 psi"))
        .given_r(qty!("0.2 m"))
        .given_t(qty!("8 mm"))
        .value()?;
    assert!(sigma_h_qty > 0.0);

    let sigma_h_runtime = eq
        .solve(equations::structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p("5 MPa + 12 psi")
        .given_r("0.2 m")
        .given_t("8 mm")
        .value()?;
    assert!(sigma_h_runtime > 0.0);

    let water = fluids::water().state_tp("300 K", "1 bar")?;
    assert!(water.rho()? > 0.0);
    assert!(water.mu()? > 0.0);
    let by_name = fluids::air().state("T", "300 K", "P", "1 bar")?;
    assert!(by_name.gamma()? > 1.0);
    let sat = fluids::water().saturation_at_pressure("1 bar")?;
    assert_eq!(sat.liquid.quality(), Some(0.0));
    assert_eq!(sat.vapor.quality(), Some(1.0));

    {
        include!("../docs_snippets/fluid_saturation_metadata.rs");
    }
    {
        include!("../docs_snippets/device_pipe_loss_fixed.rs");
    }
    {
        include!("../docs_snippets/device_pipe_loss_colebrook_direct.rs");
    }
    {
        include!("../docs_snippets/device_pipe_loss_colebrook_fluid.rs");
    }

    let steel = materials::stainless_304().temperature("350 K")?;
    assert!(steel.property("elastic_modulus")? > 0.0);
    assert!(steel.property("yield_strength")? > 0.0);

    let re = eq
        .solve_with_context(equations::fluids::reynolds_number::equation())
        .fluid(fluids::water().state_tp("300 K", "1 bar")?)
        .for_target("Re")
        .given("V", "3 m/s")
        .given("D", "0.1 m")
        .value()?;
    assert!(re > 0.0);

    let p_ideal = eq
        .solve(equations::thermo::ideal_gas::density::equation())
        .target_p()
        .given_rho("1.225 kg/m^3")
        .given_r("287 J/(kg*K)")
        .given_t("288.15 K")
        .value()?;
    assert!(p_ideal > 0.0);

    Ok(())
}
