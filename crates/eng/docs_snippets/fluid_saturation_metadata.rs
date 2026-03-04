{
    let sat = eng::fluids::water().saturation_at_pressure("1 bar")?;
    let q_liq = sat.liquid.quality();
    let q_vap = sat.vapor.quality();
    let pair = sat.liquid.input_pair_label();
    let fluid_key = sat.liquid.fluid_key();
    let _inputs = sat.liquid.normalized_inputs();

    assert_eq!(q_liq, Some(0.0));
    assert_eq!(q_vap, Some(1.0));
    assert_eq!(fluid_key, "H2O");
    assert_eq!(pair, "P,Q");
}
