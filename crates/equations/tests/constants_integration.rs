use equations::{
    Registry, docs::page_model::build_page_models, eq, normalize::resolved_display,
    registry::validate::validate_equation, rockets,
};

#[test]
fn circular_pipe_area_preserves_symbolic_pi_and_evaluates() {
    let registry = Registry::load_default().expect("load default registry");
    let equation = registry
        .equation("fluids.circular_pipe_area")
        .expect("equation lookup");
    let display = resolved_display(equation);
    assert!(display.latex.contains("\\pi"));
    assert!(display.unicode.contains('π'));
    assert!(display.ascii.contains("pi"));

    let area = registry
        .solve_value("fluids.circular_pipe_area", "A", [("D", 0.1)])
        .expect("solve area");
    let expected = std::f64::consts::PI * 0.1_f64.powi(2) / 4.0;
    assert!((area - expected).abs() < 1e-12);
}

#[test]
fn unknown_constant_reference_fails_validation() {
    let registry = Registry::load_default().expect("load default registry");
    let mut equation = registry
        .equation("fluids.circular_pipe_area")
        .expect("equation lookup")
        .clone();
    equation.relation.residual = "A - ((pii / 4) * D^2)".to_string();

    let err = validate_equation(&equation).expect_err("expected unknown-constant validation error");
    let message = err.to_string();
    assert!(message.contains("unknown symbol 'pii'"));
    assert!(message.contains("relation.residual"));
}

#[test]
fn variables_shadow_constants_during_evaluation() {
    let registry = Registry::load_default().expect("load default registry");
    let isp = registry
        .solve_value(
            "rockets.specific_impulse_ideal",
            "I_sp",
            [("C_f", 2.0), ("c_star", 1000.0), ("g0", 10.0)],
        )
        .expect("solve isp");
    assert!((isp - 200.0).abs() < 1e-12);
}

#[test]
fn constants_auto_resolve_without_given_input() {
    let isp = eq
        .solve(rockets::specific_impulse_ideal::equation())
        .target_i_sp()
        .given_c_f(1.7684408756881704)
        .given_c_star(1718.7683350153386)
        .value()
        .expect("solve with auto g0");
    assert!((isp - 309.94684010132147).abs() < 1e-10);
}

#[test]
fn constant_override_changes_solution() {
    let baseline = eq
        .solve(rockets::specific_impulse_ideal::equation())
        .target_i_sp()
        .given_c_f(1.7684408756881704)
        .given_c_star(1718.7683350153386)
        .value()
        .expect("baseline");
    let overridden = eq
        .solve(rockets::specific_impulse_ideal::equation())
        .target_i_sp()
        .given_c_f(1.7684408756881704)
        .given_c_star(1718.7683350153386)
        .override_constant("g0", 9.81)
        .value()
        .expect("overridden");
    assert!(overridden < baseline);
}

#[test]
fn page_models_capture_constant_usage() {
    let registry = Registry::load_default().expect("load default registry");
    let pages = build_page_models(registry.equations());
    let area_page = pages
        .iter()
        .find(|p| p.path_id == "fluids.circular_pipe_area")
        .expect("circular pipe area page");
    assert!(area_page.uses_constants.iter().any(|c| c.key == "pi"));

    let isp_page = pages
        .iter()
        .find(|p| p.path_id == "rockets.specific_impulse_ideal")
        .expect("isp page");
    let convenience = isp_page
        .examples
        .iter()
        .find(|e| e.style == "convenience" && e.target.as_deref() == Some("I_sp"))
        .expect("convenience example");
    assert!(
        !convenience
            .argument_order
            .iter()
            .any(|a| a.name.eq_ignore_ascii_case("g0"))
    );
    assert!(
        !convenience
            .signature
            .as_deref()
            .unwrap_or_default()
            .contains("g0")
    );
}
