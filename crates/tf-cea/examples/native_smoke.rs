use tf_cea::{
    CeaBackend, CombustorModel, EquilibriumProblem, NativeCeaBackend, NozzleModel, Reactant,
    RocketProblem, ThermoState,
};

/// Example demonstrating the native in-process CEA backend
///
/// This bypasses the factory pattern to directly use the native backend.
/// The factory pattern (`create_backend()`) is preferred for production use.
///
/// Prerequisites:
/// - Prebuilt NASA CEA binaries must be present in third_party/cea/
/// - See docs/BUNDLED_NATIVE_CEA.md for setup details
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Native CEA Backend Example ===\n");

    // Create the native backend directly
    let backend = NativeCeaBackend::new();

    // Example 1: Equilibrium calculation
    println!("1. Equilibrium Calculation (H2 combustion)");
    let equilibrium = EquilibriumProblem {
        reactants: vec![
            Reactant {
                name: "H2".to_owned(),
                amount_moles: 2.0,
                temperature_k: Some(298.15),
            },
            Reactant {
                name: "O2".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(298.15),
            },
        ],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 3000.0,
        },
        include_condensed_species: false,
    };

    let eq_result = backend.run_equilibrium(&equilibrium)?;
    println!("   Temperature:      {:.1} K", eq_result.temperature_k);
    println!("   Pressure:         {:.1} Pa", eq_result.pressure_pa);
    println!(
        "   Molecular Weight: {:.2} kg/kmol",
        eq_result.mean_molecular_weight_kg_per_kmol
    );
    println!("   Gamma (Cp/Cv):    {:.3}", eq_result.gamma);
    println!(
        "   Product Species:  {} species found",
        eq_result.species_mole_fractions.len()
    );
    println!();

    // Example 2: Rocket performance calculation
    println!("2. Rocket Performance (H2/O2)");
    let rocket = RocketProblem {
        oxidizer: Reactant {
            name: "O2".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(90.0),
        },
        fuel: Reactant {
            name: "H2".to_owned(),
            amount_moles: 2.0,
            temperature_k: Some(20.0),
        },
        chamber_pressure_pa: 7_000_000.0,
        mixture_ratio_oxidizer_to_fuel: 6.0,
        expansion_ratio: 40.0,
        nozzle_model: NozzleModel::ShiftingEquilibrium,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let rocket_result = backend.run_rocket(&rocket)?;
    println!(
        "   Chamber Temperature: {:.1} K",
        rocket_result.chamber_temperature_k
    );
    println!("   Chamber Gamma:       {:.3}", rocket_result.chamber_gamma);
    println!(
        "   C* (c-star):         {:.1} m/s",
        rocket_result.characteristic_velocity_m_per_s
    );
    println!(
        "   Isp (vacuum):        {:.1} s",
        rocket_result.specific_impulse_vac_s
    );
    println!(
        "   Cf (vacuum):         {:.3}",
        rocket_result.thrust_coefficient_vac
    );
    println!();

    println!("=== Native CEA Backend: SUCCESS ===");
    println!("\nThis example demonstrates that NASA CEA is running in-process");
    println!("without any external executable or subprocess overhead.");

    Ok(())
}
