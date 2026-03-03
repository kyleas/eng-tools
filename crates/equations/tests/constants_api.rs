use equations::constants;

#[test]
fn generated_constants_are_accessible() {
    let g0 = constants::g0();
    assert_eq!(g0.key, "g0");
    assert!((g0.value - 9.80665).abs() < 1e-12);
    assert!(!g0.exact);
    assert!(g0.source.contains("CODATA"));

    let sb = constants::stefan_boltzmann();
    assert_eq!(sb.dimension, "stefan_boltzmann_constant");
    assert!(!sb.note.is_empty());

    let by_alias = equations::get_constant("standard_gravity").expect("alias lookup");
    assert_eq!(by_alias.key, "g0");
}
