{
    let n2_pt = eng::fluids::nitrogen().state_tp(
        eng::units::typed::temperature::k(300.0),
        eng::units::typed::pressure::bar(1.0),
    )?;
    let n2_h = n2_pt.h()?;
    let n2_s = n2_pt.s()?;
    let n2_rho = n2_pt.rho()?;

    let _n2_ph =
        eng::fluids::nitrogen().state_ph(eng::units::typed::pressure::bar(1.0), n2_h)?;
    let _n2_ps =
        eng::fluids::nitrogen().state_ps(eng::units::typed::pressure::bar(1.0), n2_s)?;
    let _n2_rho_h = eng::fluids::nitrogen().state_rho_h(n2_rho, n2_h)?;

    let _air_generic = eng::fluids::air().state("T", "300 K", "P", "1 bar")?;
    let _air_generic_typed = eng::fluids::air().state(
        "P",
        eng::units::typed::pressure::bar(1.0),
        "T",
        eng::units::typed::temperature::k(300.0),
    )?;
}
