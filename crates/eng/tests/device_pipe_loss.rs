use eng::{
    devices::{PipeFrictionModel, PipeLossError, pipe_loss},
    fluids,
};

#[test]
fn fixed_f_direct_property_path_solves() {
    let result = pipe_loss()
        .friction_model(PipeFrictionModel::Fixed(0.02))
        .given_rho("1000 kg/m3")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .solve()
        .expect("solve");
    assert!(result.delta_p() > 0.0);
    assert!(result.friction_factor() > 0.0);
    assert!(result.reynolds_number().is_none());
}

#[test]
fn colebrook_direct_property_path_solves() {
    let result = pipe_loss()
        .friction_model(PipeFrictionModel::Colebrook)
        .given_rho("1000 kg/m^3")
        .given_mu("1 cP")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve()
        .expect("solve");
    assert!(result.delta_p() > 0.0);
    assert!(result.friction_factor() > 0.0);
    assert!(result.reynolds_number().unwrap_or_default() > 0.0);
}

#[test]
fn colebrook_fluid_context_path_solves() {
    let result = pipe_loss()
        .friction_model(PipeFrictionModel::Colebrook)
        .fluid(fluids::water().state_tp("300 K", "1 atm").expect("state"))
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .given_eps("0.00015 in")
        .solve()
        .expect("solve");
    assert!(result.delta_p() > 0.0);
    assert!(result.reynolds_number().unwrap_or_default() > 0.0);
}

#[test]
fn colebrook_missing_inputs_report_structured_error() {
    let err = pipe_loss()
        .friction_model(PipeFrictionModel::Colebrook)
        .given_rho("1000 kg/m^3")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .solve()
        .expect_err("expected error");
    match err {
        PipeLossError::MissingInput { input, mode, .. } => {
            assert_eq!(input, "mu");
            assert_eq!(mode, "colebrook");
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn fixed_model_invalid_f_is_rejected() {
    let err = pipe_loss()
        .friction_model(PipeFrictionModel::Fixed(0.0))
        .given_rho("1000 kg/m3")
        .given_v("3 m/s")
        .given_d("0.1 m")
        .given_l("10 m")
        .solve_delta_p()
        .expect_err("expected error");
    match err {
        PipeLossError::InvalidFrictionFactor { value } => assert_eq!(value, 0.0),
        other => panic!("unexpected error: {other}"),
    }
}
