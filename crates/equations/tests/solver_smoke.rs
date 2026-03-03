use equations::{Registry, eq};

#[test]
fn hoop_stress_solves_sigma_h_explicit() {
    let registry = Registry::load_default().expect("load");
    let equation = registry.equation("hoop_stress").expect("equation");
    let sigma_h = equation
        .solve_value("sigma_h", [("P", 2.5e6), ("r", 0.2), ("t", 0.008)])
        .expect("solve");
    assert!((sigma_h - 62.5e6).abs() < 1e-4);
}

#[test]
fn global_eq_facade_solves_value() {
    let sigma_h = eq
        .solve_value(
            "structures.hoop_stress",
            "sigma_h",
            [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
        )
        .expect("solve");
    assert!((sigma_h - 62.5e6).abs() < 1e-4);
}
