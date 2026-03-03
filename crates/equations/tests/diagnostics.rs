use equations::{EquationError, SolveMethod, eq, structures};

#[test]
fn unknown_equation_shows_suggestion() {
    let err = eq
        .solve_value(
            "structures.hoop_strss",
            "sigma_h",
            [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
        )
        .expect_err("should fail");
    match err {
        EquationError::UnknownEquationId { id, suggestion } => {
            assert_eq!(id, "structures.hoop_strss");
            assert!(suggestion.contains("hoop_stress"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn invalid_target_lists_valid_targets() {
    let err = eq
        .solve(structures::hoop_stress::equation())
        .for_target("sigmahh")
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()
        .expect_err("should fail");
    match err {
        EquationError::InvalidSolveTarget { valid_targets, .. } => {
            assert!(valid_targets.contains("sigma_h"));
            assert!(valid_targets.contains("P"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn unknown_given_variable_lists_valid_variables() {
    let err = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", 2.5e6)
        .given("pp", 2.5e6)
        .given("r", 0.2)
        .given("t", 0.008)
        .value()
        .expect_err("should fail");
    match err {
        EquationError::Validation(msg) => {
            assert!(msg.contains("unknown given variable"));
            assert!(msg.contains("valid variables"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn missing_givens_error_is_actionable() {
    let err = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", 2.5e6)
        .given("r", 0.2)
        .value()
        .expect_err("should fail");
    match err {
        EquationError::Validation(msg) => {
            assert!(msg.contains("missing givens"));
            assert!(msg.contains("t"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn invalid_branch_lists_valid_branches() {
    let err = eq
        .solve("compressible.area_mach")
        .for_target("M")
        .branch("super")
        .given("gamma", 1.4)
        .given("area_ratio", 2.0049745454545462)
        .value()
        .expect_err("should fail");
    match err {
        EquationError::InvalidBranch { valid_branches, .. } => {
            assert!(valid_branches.contains("supersonic"));
            assert!(valid_branches.contains("subsonic"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn numerical_failure_message_includes_hints() {
    let err = eq
        .solve("compressible.area_mach")
        .for_target("M")
        .method(SolveMethod::Numerical)
        .branch("subsonic")
        .given("gamma", 1.4)
        .given("area_ratio", 0.2)
        .value()
        .expect_err("should fail");
    match err {
        EquationError::NumericalSolve { reason, .. } => {
            assert!(
                reason.contains("branch")
                    || reason.contains("domain")
                    || reason.contains("bracket")
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}
