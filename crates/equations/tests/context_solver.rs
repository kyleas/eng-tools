use eng_fluids::water;
use eng_materials::stainless_304;
use equations::{eq, fluids, structures};

#[test]
fn solve_with_fluid_context_resolves_density() {
    let state = water().state_tp("300 K", "1 bar").expect("state");
    let rho = state.rho().expect("rho");
    let mu = state.mu().expect("mu");

    let with_context = eq
        .solve_with_context(fluids::reynolds_number::equation())
        .for_target("Re")
        .fluid(state.clone())
        .given("V", "2.5 m/s")
        .given("D", "0.1 m")
        .value()
        .expect("solve with context");

    let direct = eq
        .solve(fluids::reynolds_number::equation())
        .for_target("Re")
        .given_rho(rho)
        .given_v("2.5 m/s")
        .given_d("0.1 m")
        .given_mu(mu)
        .value()
        .expect("direct solve");

    assert!((with_context - direct).abs() / direct.abs().max(1.0) < 1e-9);
}

#[test]
fn solve_with_material_context_resolves_elastic_modulus() {
    let wall = stainless_304().temperature("350 K").expect("state");
    let e = wall.property("elastic_modulus").expect("E");

    let with_context = eq
        .solve_with_context(structures::euler_buckling_load::equation())
        .for_target("P_cr")
        .material(wall.clone())
        .given("I", "8e-6 m4")
        .given("K", 1.0)
        .given("L", "2 m")
        .value()
        .expect("solve with material context");

    let direct = eq
        .solve(structures::euler_buckling_load::equation())
        .for_target("P_cr")
        .given_e(e)
        .given_i("8e-6 m4")
        .given_k(1.0)
        .given_l("2 m")
        .value()
        .expect("direct solve");

    assert!((with_context - direct).abs() / direct.abs().max(1.0) < 1e-9);
}

#[test]
fn missing_required_context_fails_clearly() {
    let err = eq
        .solve_with_context(structures::euler_buckling_load::equation())
        .for_target("P_cr")
        .given("I", "8e-6 m4")
        .given("K", 1.0)
        .given("L", "2 m")
        .value()
        .expect_err("should fail");
    let msg = err.to_string();
    assert!(msg.contains("needs context 'material'"));
}
