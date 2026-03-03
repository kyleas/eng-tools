use std::fs;
use std::path::PathBuf;

use tf_cea::{
    BackendExecutable, CeaBackend, CeaBackendConfig, CeaProcessAdapter, CombustorModel,
    EquilibriumProblem, NozzleModel, Reactant, RocketProblem, ThermoState,
};

fn setup_fake_backend() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let script_path = dir.path().join("fake_cea.py");
    let script = r#"#!/usr/bin/env python3
import json
import sys

problem = json.loads(sys.stdin.read())
ptype = problem["problem_type"]
payload = problem["payload"]

if ptype == "equilibrium":
    result = {
      "result_type": "equilibrium",
      "payload": {
        "pressure_pa": 101325.0,
        "temperature_k": 3500.0,
        "mean_molecular_weight_kg_per_kmol": 22.0,
        "gamma": 1.2,
        "species_mole_fractions": [
          {"species": "H2O", "mole_fraction": 0.5},
          {"species": "CO2", "mole_fraction": 0.5}
        ]
      }
    }
elif ptype == "rocket":
    result = {
      "result_type": "rocket",
      "payload": {
        "chamber_temperature_k": 3600.0,
        "chamber_gamma": 1.21,
        "chamber_molecular_weight_kg_per_kmol": 21.5,
        "characteristic_velocity_m_per_s": 1800.0,
        "specific_impulse_vac_s": 320.0,
        "thrust_coefficient_vac": 1.8
      }
    }
else:
    print('unsupported problem type', file=sys.stderr)
    sys.exit(2)

print(json.dumps(result))
"#;
    fs::write(&script_path, script).expect("write fake backend");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&script_path, perms).expect("chmod");
    }

    #[cfg(windows)]
    let executable_path = {
        let wrapper_path = dir.path().join("fake_cea.cmd");
        let wrapper = "@echo off\r\npython \"%~dp0fake_cea.py\"";
        fs::write(&wrapper_path, wrapper).expect("write fake backend wrapper");
        wrapper_path
    };

    #[cfg(not(windows))]
    let executable_path = script_path;

    (dir, executable_path)
}

#[test]
fn equilibrium_vertical_slice_executes_end_to_end() {
    let (_temp, executable_path) = setup_fake_backend();
    let adapter = CeaProcessAdapter::new(CeaBackendConfig {
        executable: Some(BackendExecutable {
            path: executable_path,
        }),
    });

    let problem = EquilibriumProblem {
        reactants: vec![Reactant {
            name: "CH4".to_owned(),
            amount_moles: 1.0,
            temperature_k: Some(298.15),
        }],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 3000.0,
        },
        include_condensed_species: false,
    };

    let result = adapter
        .run_equilibrium(&problem)
        .expect("equilibrium result");
    assert_eq!(result.temperature_k, 3500.0);
    assert_eq!(result.species_mole_fractions.len(), 2);
}

#[test]
fn rocket_vertical_slice_executes_end_to_end() {
    let (_temp, executable_path) = setup_fake_backend();
    let adapter = CeaProcessAdapter::new(CeaBackendConfig {
        executable: Some(BackendExecutable {
            path: executable_path,
        }),
    });

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
        mixture_ratio_oxidizer_to_fuel: 2.6,
        expansion_ratio: 40.0,
        nozzle_model: NozzleModel::ShiftingEquilibrium,
        combustor_model: CombustorModel::InfiniteArea,
    };

    let result = adapter.run_rocket(&problem).expect("rocket result");
    assert_eq!(result.specific_impulse_vac_s, 320.0);
}

#[test]
fn missing_executable_is_reported() {
    let adapter = CeaProcessAdapter::new(CeaBackendConfig { executable: None });
    let problem = EquilibriumProblem {
        reactants: vec![],
        state: ThermoState::PressureTemperature {
            pressure_pa: 101325.0,
            temperature_k: 1000.0,
        },
        include_condensed_species: false,
    };

    let err = adapter
        .run_equilibrium(&problem)
        .expect_err("expected error");
    assert!(err.to_string().contains("not configured"));
}
