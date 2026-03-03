use tf_cea::{
    CeaBackend, CeaError, EquilibriumProblem, EquilibriumResult, Reactant, RocketProblem,
    RocketResult,
};
use tf_rpa::{
    CombustorModel, NozzleChemistryModel, NozzleConstraint, RocketAnalysisProblem,
    RocketAnalysisSolver,
};

struct DemoBackend;

impl CeaBackend for DemoBackend {
    fn run_equilibrium(
        &self,
        _problem: &EquilibriumProblem,
    ) -> Result<EquilibriumResult, CeaError> {
        Err(CeaError::InvalidResponse(
            "not used in this example".to_owned(),
        ))
    }

    fn run_rocket(&self, _problem: &RocketProblem) -> Result<RocketResult, CeaError> {
        Ok(RocketResult {
            chamber_temperature_k: 3575.0,
            chamber_gamma: 1.21,
            chamber_molecular_weight_kg_per_kmol: 22.4,
            characteristic_velocity_m_per_s: 1810.0,
            specific_impulse_vac_s: 326.0,
            thrust_coefficient_vac: 1.81,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = RocketAnalysisProblem {
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
        chamber_pressure_pa: 7.0e6,
        mixture_ratio_oxidizer_to_fuel: 2.6,
        nozzle_constraint: NozzleConstraint::ExpansionRatio(40.0),
        combustor_model: CombustorModel::InfiniteArea,
        nozzle_chemistry_model: NozzleChemistryModel::ShiftingEquilibrium,
        ambient_pressure_pa: 101_325.0,
    };

    let solver = RocketAnalysisSolver::new(DemoBackend);
    let result = solver.solve(&problem)?;

    println!("=== tf-rpa smoke result ===");
    println!(
        "Tc   : {:.1} K",
        result.chamber.temperature_k.unwrap_or(-1.0)
    );
    println!(
        "MWc  : {:.3} kg/kmol",
        result.chamber.molecular_weight_kg_per_kmol.unwrap_or(-1.0)
    );
    println!("gamma: {:.4}", result.chamber.gamma.unwrap_or(-1.0));
    println!("c*   : {:.2} m/s", result.characteristic_velocity_m_per_s);
    println!("Cf,vac: {:.4}", result.thrust_coefficient_vac);
    println!("Isp,vac: {:.2} s", result.specific_impulse_vac_s);
    println!("Isp,amb: {:.2} s", result.specific_impulse_amb_s);

    Ok(())
}
