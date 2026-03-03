use tf_cea::{CeaBackend, EquilibriumProblem, NativeCeaBackend, Reactant, ThermoState};

fn main() {
    let backend = NativeCeaBackend::new();

    // Test different species
    let species_to_test = vec![
        "H2", "CH4", "C3H8", "C2H6", "N2H4", "NH3", "O2", "N2O", "N2O4", "H2O2", "F2",
    ];

    for fuel in &species_to_test {
        let problem = EquilibriumProblem {
            reactants: vec![
                Reactant {
                    name: fuel.to_string(),
                    amount_moles: 1.0,
                    temperature_k: Some(298.15),
                },
                Reactant {
                    name: "O2".to_string(),
                    amount_moles: 1.0,
                    temperature_k: Some(298.15),
                },
            ],
            state: ThermoState::PressureTemperature {
                pressure_pa: 101325.0,
                temperature_k: 2500.0,
            },
            include_condensed_species: false,
        };

        match backend.run_equilibrium(&problem) {
            Ok(_) => println!("{} - OK", fuel),
            Err(e) => println!("{} - FAIL: {}", fuel, e),
        }
    }
}
