# tf-rpa

`tf-rpa` is Thermoflow's backend-first rocket performance orchestration crate.

## Ownership

- Owns RPA-style problem/result models for chamber/nozzle analysis workflows.
- Owns assumption handling and validation for combustor/nozzle analysis modes.
- Calls into `tf-cea` for thermochemistry/rocket-performance backend physics.

## What this slice computes now

From CEA backend outputs:

- chamber temperature
- chamber molecular weight
- chamber gamma
- characteristic velocity (c*)
- vacuum thrust coefficient (Cf,vac)
- vacuum specific impulse (Isp,vac)

Derived in `tf-rpa` currently:

- ambient thrust coefficient and ambient Isp using `Cf_amb = Cf_vac - (pa/pc)*eps`

Current placeholders / extension seams:

- throat thermodynamic station summary
- exit thermodynamic station summary
- exit-pressure-constrained nozzle solves
- frozen-at-throat chemistry mode

## Studies (single-variable)

`tf-rpa` now includes first-pass study support:

- `RocketStudyProblem`
- `StudyVariable`
- `StudyRange`
- `StudyOutputMetric`
- `run_single_variable_study(...)`

Current scope is one-variable sweeps with deterministic linear spacing and explicit per-point errors for unsupported/failed solves.


## Geometry (first-pass sizing)

`tf-rpa` now includes first-pass geometry sizing support:

- `RocketGeometryProblem`
- `GeometrySizingMode`
- `RocketGeometryResult`
- `compute_geometry(...)`

Current scope: deterministic throat/exit/chamber sizing transforms and explicit estimated fields (chamber length via L*, nozzle length via conical half-angle).


## Thermal (first-pass heat-load estimate)

`tf-rpa` now includes a first-pass thermal estimator:

- `RocketThermalProblem`
- `ThermalModel`
- `CoolingMode`
- `RocketThermalResult` + axial profile samples
- `compute_thermal(...)`

This slice provides explicit estimated heat-flux/load outputs and notes about missing high-fidelity effects.
