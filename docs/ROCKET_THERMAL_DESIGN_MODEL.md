# Rocket Thermal Design Model

## Scope
This model is a transparent first-pass thermal design workflow for Rocket:
- gas-side convective heating (Bartz-like axial shaping)
- wall conduction (1D through thickness)
- coolant-side convective removal (internal channel correlation)
- film-cooling effectiveness option
- automated channel sizing under pressure-drop constraint

It is not CFD/CHT and not manufacturing-final.

## Cooling Modes
- `AdiabaticWall`: no active cooling credit.
- `Regenerative`: coolant channels + pressure-drop constraint.
- `Film`: film-effectiveness reduction on gas-side driving temperature.
- `RegenerativeFilm`: regen + film combined.

## Core Equations
- `Re = rho * u * Dh / mu`
- `Nu = 0.023 Re^0.8 Pr^0.4` (turbulent), `Nu = 3.66` (laminar fallback)
- `q'' = (Taw_eff - Tcool) / (1/hg + twall/k + 1/hc)`
- `dP = f * (dx/Dh) * (rho * u^2 / 2)`
- Objective: minimize `max(Twall_hot)` with `DeltaP <= DeltaP_max`

## Automated Channel Designer
The optimizer is deterministic and inspectable:
- if `DeltaP` exceeds limit, global channel-area expansion is applied.
- otherwise, local channel expansion is applied near peak wall-temperature station.
- each iteration logs: `peakT`, `DeltaP`, objective value, action, accepted/rejected.

## Provenance and Trust
Each result includes:
- equation traces with source type (`correlation`, `derived`, `constraint`)
- explicit assumptions and impact statements
- station-by-station outputs for heat flux, wall temperatures, coolant state, and geometry

## Deferred
- full CFD/CHT, injector-resolved film physics
- detailed temperature/pressure dependent coolant properties
- structural stress/life coupling
- manufacturing-rule-aware topology optimization
