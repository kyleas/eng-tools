use tf_cea::{
    CeaBackend, CeaError, EquilibriumProblem, EquilibriumResult, Reactant, RocketProblem,
    RocketResult,
};
use tf_rpa::{
    CombustorModel, NozzleChemistryModel, NozzleConstraint, RocketAnalysisProblem,
    RocketAnalysisSolver, StateSource,
};

#[derive(Clone, Default)]
struct FakeBackend;

impl CeaBackend for FakeBackend {
    fn run_equilibrium(
        &self,
        _problem: &EquilibriumProblem,
    ) -> Result<EquilibriumResult, CeaError> {
        Err(CeaError::InvalidResponse(
            "equilibrium not used in these tests".to_owned(),
        ))
    }

    fn run_rocket(&self, _problem: &RocketProblem) -> Result<RocketResult, CeaError> {
        Ok(RocketResult {
            chamber_temperature_k: 3550.0,
            chamber_gamma: 1.22,
            chamber_molecular_weight_kg_per_kmol: 22.1,
            characteristic_velocity_m_per_s: 1790.0,
            specific_impulse_vac_s: 325.0,
            thrust_coefficient_vac: 1.82,
        })
    }
}

fn sample_problem() -> RocketAnalysisProblem {
    RocketAnalysisProblem {
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
    }
}

#[test]
fn solves_representative_case_end_to_end() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let result = solver.solve(&sample_problem()).expect("solve");

    assert_eq!(result.chamber.source, StateSource::CeaBackend);
    assert_eq!(result.chamber.temperature_k, Some(3550.0));
    assert_eq!(result.throat.source, StateSource::NotYetProvidedByBackend);
    assert!(result.specific_impulse_amb_s < result.specific_impulse_vac_s);
    assert_eq!(result.expansion_ratio_used, 40.0);
    assert!(result.effective_exhaust_velocity_vac_m_per_s > 0.0);
    assert!(result.chamber_to_ambient_pressure_ratio > 1.0);
}

#[test]
fn finite_area_combustor_is_plumbed() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let mut problem = sample_problem();
    problem.combustor_model = CombustorModel::FiniteArea {
        contraction_ratio: 3.0,
    };

    let result = solver.solve(&problem).expect("solve");
    assert_eq!(
        result.assumptions.combustor_model,
        CombustorModel::FiniteArea {
            contraction_ratio: 3.0
        }
    );
}

#[test]
fn frozen_at_chamber_is_plumbed() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let mut problem = sample_problem();
    problem.nozzle_chemistry_model = NozzleChemistryModel::FrozenAtChamber;

    let result = solver.solve(&problem).expect("solve");
    assert_eq!(
        result.assumptions.nozzle_chemistry_model,
        NozzleChemistryModel::FrozenAtChamber
    );
}

#[test]
fn frozen_at_throat_is_explicitly_unsupported() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let mut problem = sample_problem();
    problem.nozzle_chemistry_model = NozzleChemistryModel::FrozenAtThroat;

    let err = solver
        .solve(&problem)
        .expect_err("expected unsupported mode");
    assert!(err.to_string().contains("frozen-at-throat"));
}

#[test]
fn exit_pressure_constraint_is_explicitly_unsupported_for_now() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let mut problem = sample_problem();
    problem.nozzle_constraint = NozzleConstraint::ExitPressurePa(50_000.0);

    let err = solver
        .solve(&problem)
        .expect_err("expected unsupported mode");
    assert!(err.to_string().contains("exit-pressure constrained"));
}

#[test]
fn validation_rejects_bad_inputs() {
    let solver = RocketAnalysisSolver::new(FakeBackend);
    let mut problem = sample_problem();
    problem.chamber_pressure_pa = -1.0;

    let err = solver.solve(&problem).expect_err("expected invalid input");
    assert!(err.to_string().contains("chamber_pressure_pa"));
}
