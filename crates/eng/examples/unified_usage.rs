use eng::{docs, eq, equations, fluids, materials};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = eq
        .solve_with_context(equations::fluids::reynolds_number::equation())
        .fluid(fluids::water().state_tp("300 K", "1 bar")?)
        .for_target("Re")
        .given("V", "3 m/s")
        .given("D", "0.1 m")
        .value()?;

    let p_cr = eq
        .solve_with_context(equations::structures::euler_buckling_load::equation())
        .material(materials::stainless_304().temperature("350 K")?)
        .for_target("P_cr")
        .given("I", "8e-6 m4")
        .given("K", 1.0)
        .given("L", "2 m")
        .value()?;

    let out = docs::export_unified_docs()?;
    println!(
        "Re={re}, P_cr={p_cr}, catalog={}",
        out.join("catalog.json").display()
    );
    Ok(())
}
