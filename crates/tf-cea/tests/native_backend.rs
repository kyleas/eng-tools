/// Integration tests for native CEA backend
///
/// These tests execute real equilibrium and rocket calculations through the bundled
/// native NASA CEA library (prebuilt binaries in third_party/cea/).
///
/// To run: `cargo test -p tf-cea --test native_backend`
use tf_cea::{
    CeaBackend, CombustorModel, EquilibriumProblem, NativeCeaBackend, NozzleModel, Reactant,
    RocketProblem, ThermoState,
};

#[test]
#[ignore] // Requires prebuilt NASA CEA binaries; verify third_party/cea/ is populated
fn test_native_equilibrium_h2_o2() {
    let backend = NativeCeaBackend::new();

    let problem = EquilibriumProblem {
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

    let result = backend.run_equilibrium(&problem).unwrap();

    // Verify result is physically reasonable
    assert!(
        result.temperature_k > 1000.0,
        "Temperature should be high for combustion"
    );
    assert!(
        result.temperature_k < 5000.0,
        "Temperature should be below 5000 K"
    );
    assert!(result.gamma > 1.0, "Gamma should be > 1");
    assert!(result.gamma < 2.0, "Gamma should be < 2");
    assert!(
        result.mean_molecular_weight_kg_per_kmol > 0.0,
        "Molecular weight should be positive"
    );
    assert!(
        !result.species_mole_fractions.is_empty(),
        "Should have product species"
    );
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_equilibrium_ch4_combustion() {
    let backend = NativeCeaBackend::new();

    let problem = EquilibriumProblem {
        reactants: vec![
            Reactant {
                name: "CH4".to_owned(),
                amount_moles: 1.0,
                temperature_k: Some(298.15),
            },
            Reactant {
                name: "O2".to_owned(),
                amount_moles: 2.0,
                temperature_k: Some(298.15),
            },
        ],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 2500.0,
        },
        include_condensed_species: false,
    };

    let result = backend.run_equilibrium(&problem).unwrap();

    // Methane combustion should produce high temperature
    assert!(
        result.temperature_k > 2000.0,
        "CH4 combustion temperature should be high"
    );
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_equilibrium_hp_mode() {
    let backend = NativeCeaBackend::new();

    let problem = EquilibriumProblem {
        reactants: vec![Reactant {
            name: "H2".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(298.15),
        }],
        state: ThermoState::PressureEnthalpy {
            pressure_pa: 101325.0,
            enthalpy_j_per_kg: 1_000_000.0,
        },
        include_condensed_species: false,
    };

    let result = backend.run_equilibrium(&problem).unwrap();

    assert!(result.temperature_k > 0.0, "Temperature should be positive");
    assert!(result.pressure_pa > 0.0, "Pressure should be positive");
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_rocket_lox_rp1() {
    let backend = NativeCeaBackend::new();

    let problem = RocketProblem {
        oxidizer: Reactant {
            name: "LOX".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(90.0),
        },
        fuel: Reactant {
            name: "RP-1".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(293.0),
        },
        chamber_pressure_pa: 7_000_000.0,
        mixture_ratio_oxidizer_to_fuel: 2.56,
        expansion_ratio: 40.0,
        nozzle_model: NozzleModel::ShiftingEquilibrium,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let result = backend.run_rocket(&problem).unwrap();

    // Verify rocket performance results are physically reasonable
    assert!(
        result.chamber_temperature_k > 3000.0,
        "LOX/RP-1 chamber temp should be > 3000K"
    );
    assert!(
        result.chamber_temperature_k < 4000.0,
        "LOX/RP-1 chamber temp should be < 4000K"
    );
    assert!(
        result.specific_impulse_vac_s > 300.0,
        "LOX/RP-1 Isp should be > 300s"
    );
    assert!(
        result.specific_impulse_vac_s < 400.0,
        "LOX/RP-1 Isp should be < 400s"
    );
    assert!(
        result.characteristic_velocity_m_per_s > 1500.0,
        "C* should be > 1500 m/s"
    );
    assert!(result.thrust_coefficient_vac > 1.0, "Cf should be > 1.0");
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_rocket_h2_o2() {
    let backend = NativeCeaBackend::new();

    let problem = RocketProblem {
        oxidizer: Reactant {
            name: "O2(L)".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(90.0),
        },
        fuel: Reactant {
            name: "H2(L)".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(20.0),
        },
        chamber_pressure_pa: 6_000_000.0,
        mixture_ratio_oxidizer_to_fuel: 6.0,
        expansion_ratio: 50.0,
        nozzle_model: NozzleModel::ShiftingEquilibrium,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let result = backend.run_rocket(&problem).unwrap();

    // H2/O2 should have higher Isp than hydrocarbon fuels
    assert!(
        result.specific_impulse_vac_s > 400.0,
        "H2/O2 Isp should be > 400s"
    );
    assert!(
        result.chamber_temperature_k > 3000.0,
        "H2/O2 chamber temp should be high"
    );
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_rocket_frozen_flow() {
    let backend = NativeCeaBackend::new();

    let problem = RocketProblem {
        oxidizer: Reactant {
            name: "LOX".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(90.0),
        },
        fuel: Reactant {
            name: "RP-1".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(293.0),
        },
        chamber_pressure_pa: 7_000_000.0,
        mixture_ratio_oxidizer_to_fuel: 2.56,
        expansion_ratio: 40.0,
        nozzle_model: NozzleModel::Frozen,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let result = backend.run_rocket(&problem).unwrap();

    // Frozen flow typically has lower Isp than shifting equilibrium
    assert!(
        result.specific_impulse_vac_s > 250.0,
        "Frozen Isp should still be reasonable"
    );
    assert!(
        result.specific_impulse_vac_s < 380.0,
        "Frozen Isp should be lower than equilibrium"
    );
}

#[test]
#[ignore] // Requires NASA CEA source
fn test_native_backend_reuse() {
    // Verify that backend can be reused for multiple calculations
    let backend = NativeCeaBackend::new();

    let eq_problem = EquilibriumProblem {
        reactants: vec![Reactant {
            name: "H2".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(298.15),
        }],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 1000.0,
        },
        include_condensed_species: false,
    };

    // Run multiple calculations with the same backend
    for _ in 0..3 {
        let result = backend.run_equilibrium(&eq_problem).unwrap();
        assert!(
            result.temperature_k > 0.0,
            "Result should be valid on reuse"
        );
    }
}

#[test]
fn test_native_backend_instantiation() {
    // This test doesn't require CEA source, just verifies construction works
    let _backend = NativeCeaBackend::new();
    // If we get here without panic, construction succeeded
}
