use tf_cea::{
    CeaBackend, CeaBackendConfig, CeaProcessAdapter, CombustorModel, EquilibriumProblem,
    NozzleModel, Reactant, RocketProblem, ThermoState,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = CeaProcessAdapter::new(CeaBackendConfig::from_env());

    let equilibrium = EquilibriumProblem {
        reactants: vec![Reactant {
            name: "CH4".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(298.15),
        }],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 2800.0,
        },
        include_condensed_species: false,
    };

    let rocket = RocketProblem {
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
        mixture_ratio_oxidizer_to_fuel: 2.6,
        expansion_ratio: 40.0,
        nozzle_model: NozzleModel::ShiftingEquilibrium,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let eq_result = backend.run_equilibrium(&equilibrium)?;
    println!("equilibrium temperature = {} K", eq_result.temperature_k);

    let rocket_result = backend.run_rocket(&rocket)?;
    println!("vac Isp = {} s", rocket_result.specific_impulse_vac_s);

    Ok(())
}
