use eng_core::units::qty;
use eng_core::units::typed::{length, pressure};
use equations::{
    Registry, SolveMethod, compressible, eq, families, generate_schema_to_path, run_registry_tests,
    structures,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eq.validate_with_tests()?;

    // 1) Builder SI path.
    let sigma_h = eq
        .solve(structures::hoop_stress::equation())
        .for_target("sigma_h")
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()?;
    println!("[simple] sigma_h = {sigma_h} Pa");

    // 2) Units-aware path with requested output units.
    let sigma_h_mpa = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p("2.5 MPa")
        .given_r("0.2 m")
        .given_t("8 mm")
        .value_in("MPa")?;
    println!("[units] sigma_h = {sigma_h_mpa} MPa");

    // 2b) Typed-unit path (explicit units, no runtime string parsing).
    let sigma_h_typed = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(pressure::mpa(2.5))
        .given_r(length::m(0.2))
        .given_t(length::mm(8.0))
        .value()?;
    println!("[typed] sigma_h = {sigma_h_typed} Pa");

    // 2c) Compile-time quantity literal path.
    let sigma_h_qty = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(qty!("5 MPa + 12 psia"))
        .given_r(qty!("3 ft + 2 in"))
        .given_t(qty!("8 mm"))
        .value()?;
    println!("[qty] sigma_h = {sigma_h_qty} Pa");

    // 3) Same solve, but keep diagnostics.
    let solved = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .givens([("P", 2.5e6), ("r", 0.2), ("t", 0.008)])
        .result()?;
    println!(
        "[result] target={} value={} method={:?} residual_abs={:e} residual_rel={:e}",
        solved.target, solved.value_si, solved.method, solved.residual_abs, solved.residual_rel
    );

    // 4) Equation-centric workflow: fetch once, solve many.
    let hoop = eq.equation("structures.hoop_stress")?;
    let p = hoop.solve_value("P", [("sigma_h", 62.5e6), ("r", 0.2), ("t", 0.008)])?;
    let t = hoop.solve_value("t", [("sigma_h", 62.5e6), ("P", 2.5e6), ("r", 0.2)])?;
    println!("[equation] P={p} Pa, t={t} m");

    // 5) Branch + method path.
    let branch_result = eq
        .solve(compressible::area_mach::equation())
        .target_m()
        .branch_subsonic()
        .method(SolveMethod::Auto)
        .given_gamma(1.4)
        .given_area_ratio(2.0350652623456793)
        .result()?;
    println!(
        "[branch] M={} via {:?}, branch={:?}",
        branch_result.value_si, branch_result.method, branch_result.branch
    );

    // 6) Short helper path (still available).
    let sigma_helper = eq.solve_value(
        "structures.hoop_stress",
        "sigma_h",
        [("P", 2.5e6), ("r", 0.2), ("t", 0.008)],
    )?;
    let sigma_helper_mpa = eq.solve_value_in(
        "structures.hoop_stress",
        "sigma_h",
        [("P", "2.5 MPa"), ("r", "0.2 m"), ("t", "8 mm")],
        "MPa",
    )?;
    println!(
        "[helpers] sigma_h={} Pa / {} MPa",
        sigma_helper, sigma_helper_mpa
    );

    // 7) Generated convenience solve functions for explicit-form targets.
    let sigma_conv = structures::hoop_stress::solve_sigma_h("2.5 MPa", "0.2 m", "8 mm")?;
    let p_conv = structures::hoop_stress::solve_p("62.5 MPa", "0.2 m", "8 mm")?;
    println!("[convenience] sigma_h={} Pa / P={} Pa", sigma_conv, p_conv);

    // 8) Equation family/variant path (ideal gas family).
    let p_ideal_gas = eq
        .solve(families::ideal_gas::density())
        .target_p()
        .given_rho("1.17683 kg/m3")
        .given_r(287.0)
        .given_t("300 K")
        .value()?;
    println!("[family] ideal_gas::density => P={} Pa", p_ideal_gas);

    // 9) Full registry lifecycle operations.
    let registry = Registry::load_default()?;
    registry.validate_with_tests()?;
    let summary = run_registry_tests(&registry)?;
    println!(
        "[tests] registry tests: {} passed / {} failed",
        summary.passed, summary.failed
    );

    // 10) Generation workflows used by docs/tooling.
    generate_schema_to_path("crates/equations/schemas/equation.schema.json")?;
    println!("[artifacts] schema refreshed");
    println!("[artifacts] unified docs export is owned by the eng CLI:");
    println!("            cargo run -p eng --bin eng -- export-docs");

    Ok(())
}
