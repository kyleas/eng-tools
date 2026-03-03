use eng_fluids as fluid_repo;
use eng_materials as material_repo;
use equations::{
    eq, export_docs_artifacts, fluids, generate_schema_to_path, run_registry_tests, structures,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Legacy quick-start example. For the full workflow set, see:
    // crates/equations/examples/equation_examples.rs
    let registry = eq.registry()?;
    registry.validate_with_tests()?;

    let sigma_h_pa = eq
        .solve(structures::hoop_stress::equation())
        .target_sigma_h()
        .given_p(2.5e6)
        .given_r(0.2)
        .given_t(0.008)
        .value()?;
    println!("sigma_h = {} Pa", sigma_h_pa);

    // Context-assisted solve: rho auto-resolved from a named fluid context.
    let re = eq
        .solve_with_context(fluids::reynolds_number::equation())
        .for_target("Re")
        .fluid(fluid_repo::water().state_tp("300 K", "1 bar")?)
        .given("V", "3 m/s")
        .given("D", "0.1 m")
        .value()?;
    println!("Re = {}", re);

    // Context-assisted solve with a material property resolver.
    let p_cr = eq
        .solve_with_context(structures::euler_buckling_load::equation())
        .for_target("P_cr")
        .material(material_repo::stainless_304().temperature("350 K")?)
        .given("I", "8e-6 m4")
        .given("K", 1.0)
        .given("L", "2 m")
        .value()?;
    println!("Euler P_cr = {} N", p_cr);

    let summary = run_registry_tests(&registry)?;
    println!(
        "registry tests: {} passed / {} failed",
        summary.passed, summary.failed
    );

    generate_schema_to_path("crates/equations/schemas/equation.schema.json")?;
    export_docs_artifacts(registry.equations(), "generated")?;
    Ok(())
}
