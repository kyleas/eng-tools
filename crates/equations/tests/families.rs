use equations::{Registry, eq, equation_families, families};

#[test]
fn ideal_gas_family_loads_and_variants_resolve() {
    let registry = Registry::load_default().expect("load");
    let defs = equation_families::load_default_validated(registry.equations()).expect("families");
    let ideal = defs
        .iter()
        .find(|f| f.key == "ideal_gas")
        .expect("ideal_gas family");
    assert_eq!(ideal.variants.len(), 2);
    assert_eq!(ideal.canonical_equation, "thermo.ideal_gas.mass_volume");
}

#[test]
fn generated_family_api_is_discoverable_and_works() {
    let p = eq
        .solve(families::ideal_gas::density())
        .target_p()
        .given_rho("1.17683 kg/m3")
        .given_r(287.0)
        .given_t("300 K")
        .value()
        .expect("solve");
    assert!((p - 101325.0).abs() < 10.0);
}
