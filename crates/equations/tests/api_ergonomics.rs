use eng_core::units::{dimensionless, length, pressure, qty};
use equations::{SolveMethod, eq};

#[test]
fn builder_si_numeric_path_returns_value() {
    let sigma_h = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", 2.5e6)
        .given("r", 0.2)
        .given("t", 0.008)
        .value()
        .expect("solve");
    assert!((sigma_h - 62.5e6).abs() < 1e-4);
}

#[test]
fn builder_units_aware_path_returns_requested_units() {
    let sigma_h_mpa = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", "2.5 MPa")
        .given("r", "0.2 m")
        .given("t", "8 mm")
        .value_in("MPa")
        .expect("solve");
    assert!((sigma_h_mpa - 62.5).abs() < 1e-8);
}

#[test]
fn builder_branch_result_path_returns_diagnostics() {
    let result = eq
        .solve("compressible.area_mach")
        .for_target("M")
        .branch("supersonic")
        .method(SolveMethod::Auto)
        .given("area_ratio", 2.0049745454545462)
        .given("gamma", 1.4)
        .result()
        .expect("solve");
    assert!(result.value_si > 1.0);
    assert_eq!(result.method, SolveMethod::Numerical);
    assert_eq!(result.branch.as_deref(), Some("supersonic"));
    assert!(result.residual_abs < 1e-6);
}

#[test]
fn helper_solve_value_in_is_available() {
    let sigma_h_mpa = eq
        .solve_value_in(
            "structures.hoop_stress",
            "sigma_h",
            [("P", "2.5 MPa"), ("r", "0.2 m"), ("t", "8 mm")],
            "MPa",
        )
        .expect("solve");
    assert!((sigma_h_mpa - 62.5).abs() < 1e-8);
}

#[test]
fn builder_typed_unit_path_returns_value() {
    let sigma_h = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", pressure::mpa(2.5))
        .given("r", length::m(0.2))
        .given("t", length::mm(8.0))
        .value()
        .expect("solve");
    assert!((sigma_h - 62.5e6).abs() < 1e-4);
}

#[test]
fn typed_dimension_mismatch_is_rejected() {
    let err = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", length::mm(8.0))
        .given("r", length::m(0.2))
        .given("t", length::mm(8.0))
        .value()
        .expect_err("expected mismatch");
    assert!(
        err.to_string().contains("typed quantity kind"),
        "unexpected error: {err}"
    );

    let _ = dimensionless::ratio(0.5);
}

#[test]
fn builder_runtime_quantity_expression_path_returns_value() {
    let sigma_h = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", "5 MPa + 12 psia")
        .given("r", "3 ft + 2 in")
        .given("t", "8 mm")
        .value()
        .expect("solve");
    assert!(sigma_h > 0.0);
}

#[test]
fn builder_qty_macro_expression_path_returns_value() {
    let sigma_h = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", qty!("5 MPa + 12 psia"))
        .given("r", qty!("3 ft + 2 in"))
        .given("t", qty!("8 mm"))
        .value()
        .expect("solve");
    assert!(sigma_h > 0.0);
}

#[test]
fn runtime_expression_mixed_dimension_is_rejected() {
    let err = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", "5 MPa + 3 m")
        .given("r", 0.2)
        .given("t", 0.008)
        .value()
        .expect_err("expected mixed-dimension failure");
    assert!(err.to_string().contains("differing dimensions"));
}

#[test]
fn qty_macro_dimension_mismatch_is_rejected_by_given_dimension() {
    let err = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", qty!("3 ft + 2 in"))
        .given("r", 0.2)
        .given("t", 0.008)
        .value()
        .expect_err("expected dimension mismatch");
    assert!(err.to_string().contains("dimension mismatch"));
}

#[test]
fn prandtl_meyer_forward_inverse_and_units_work() {
    let gamma = 1.4;
    let nu = eq
        .solve("compressible.prandtl_meyer")
        .for_target("nu")
        .given("M", 2.0)
        .given("gamma", gamma)
        .value()
        .expect("nu solve");
    assert!((nu - 0.460_413_682_082_694_73).abs() < 1e-10);

    let m = eq
        .solve("compressible.prandtl_meyer")
        .for_target("M")
        .given("nu", nu)
        .given("gamma", gamma)
        .value()
        .expect("M solve");
    assert!((m - 2.0).abs() < 1e-8);

    let m_from_deg = eq
        .solve("compressible.prandtl_meyer")
        .for_target("M")
        .given("nu", "26.379760813416457 deg")
        .given("gamma", gamma)
        .value()
        .expect("M solve from deg");
    assert!((m_from_deg - 2.0).abs() < 1e-8);
}

#[test]
fn prandtl_meyer_out_of_domain_nu_errors() {
    let err = eq
        .solve("compressible.prandtl_meyer")
        .for_target("M")
        .given("nu", 3.0)
        .given("gamma", 1.4)
        .value()
        .expect_err("expected out-of-domain PM angle");
    let msg = err.to_string();
    assert!(
        msg.contains("could not discover a valid bracket")
            || msg.contains("does not straddle a root")
            || msg.contains("invalid domain"),
        "unexpected error: {msg}"
    );
}
