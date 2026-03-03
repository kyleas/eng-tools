use std::collections::BTreeSet;

use equations::{
    GENERATED_EQUATION_IDS, IntoEquationId, Registry, SolveMethod, compressible, eq, fluids,
    registry::ids::derive_path_id, rockets, structures,
};

#[test]
fn generated_constructors_resolve_expected_equations() {
    assert_eq!(
        structures::hoop_stress::equation().equation_id(),
        "structures.hoop_stress"
    );
    assert_eq!(
        fluids::colebrook::equation().equation_id(),
        "fluids.colebrook"
    );
    assert_eq!(
        compressible::area_mach::equation().equation_id(),
        "compressible.area_mach"
    );
}

#[test]
fn typed_target_and_given_methods_work_for_numeric_and_units() {
    let sigma_si = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()
        .expect("typed numeric solve");
    let sigma_mpa = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p("2.5 MPa")
        .given_r("0.2 m")
        .given_t("8 mm")
        .value_in("MPa")
        .expect("typed units solve");
    assert!((sigma_si - 62.5e6).abs() < 1e-4);
    assert!((sigma_mpa - 62.5).abs() < 1e-8);
}

#[test]
fn typed_branch_methods_work_for_branch_equations() {
    let result = eq
        .solve(compressible::area_mach::equation())
        .target_m()
        .branch_supersonic()
        .method(SolveMethod::Auto)
        .given_area_ratio(2.0049745454545462)
        .given_gamma(1.4)
        .result()
        .expect("typed branch solve");
    assert!(result.value_si > 1.0);
    assert_eq!(result.branch.as_deref(), Some("supersonic"));
}

#[test]
fn typed_and_generic_paths_match() {
    let typed = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()
        .expect("typed");
    let generic = eq
        .solve("structures.hoop_stress")
        .for_target("sigma_h")
        .given("P", 2.5e6)
        .given("r", 0.2)
        .given("t", 0.008)
        .value()
        .expect("generic");
    assert!((typed - generic).abs() < 1e-12);
}

#[test]
fn generated_layer_is_in_sync_with_registry() {
    let registry = Registry::load_default().expect("load");
    let generated: BTreeSet<&str> = GENERATED_EQUATION_IDS.iter().copied().collect();
    let actual: BTreeSet<String> = registry.equations().iter().map(derive_path_id).collect();
    assert_eq!(
        generated.len(),
        actual.len(),
        "generated equation count mismatch"
    );
    for path_id in actual {
        assert!(
            generated.contains(path_id.as_str()),
            "generated API missing equation {}",
            path_id
        );
    }
}

#[test]
fn generated_convenience_solve_functions_match_builder() {
    let conv = structures::hoop_stress::solve_sigma_h("2.5 MPa", "0.2 m", "8 mm").expect("conv");
    let builder = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p("2.5 MPa")
        .given_r("0.2 m")
        .given_t("8 mm")
        .value()
        .expect("builder");
    assert!((conv - builder).abs() < 1e-9);
}

#[test]
fn convenience_solver_omits_auto_constant_arguments() {
    let conv = rockets::specific_impulse_ideal::solve_i_sp(1.7684408756881704, 1718.7683350153386)
        .expect("conv");
    let builder = eq
        .solve(rockets::specific_impulse_ideal::equation())
        .target_i_sp()
        .given_c_f(1.7684408756881704)
        .given_c_star(1718.7683350153386)
        .value()
        .expect("builder");
    assert!((conv - builder).abs() < 1e-12);
}
