# Rocket App First GUI Slice

## Scope delivered

This phase adds the first GUI-facing Rocket app surface in `tf-ui`.

- New top-level **Rocket** tab.
- Visible Rocket subtabs:
  - Performance (functional)
  - Geometry (functional first-pass sizing)
  - Thermal (functional first-pass heat-load estimate)
  - Propellants (placeholder)
  - Studies (placeholder)
  - Data (placeholder)

## Performance subtab capabilities

Performance currently supports backend-driven solve orchestration through `tf-rpa` + `tf-cea`.

Configurable inputs:

- case name
- oxidizer/fuel labels and inlet temperatures
- O/F mixture ratio
- chamber pressure
- ambient pressure
- combustor assumption (`InfiniteArea` / `FiniteArea`)
- nozzle chemistry assumption (`ShiftingEquilibrium`, `FrozenAtChamber`, `FrozenAtThroat`)
- nozzle constraint (`ExpansionRatio` / `ExitPressurePa`)

## Real vs placeholder outputs

Real now:

- chamber summary fields from backend path
- c*
- Cf(vac)
- Cf(amb) (derived)
- Isp(vac)
- Isp(amb) (derived)
- effective exhaust velocity (vac/amb) (derived)
- chamber-to-ambient pressure ratio (derived)

Placeholder seams still visible:

- throat station thermodynamic detail
- exit station thermodynamic detail

## Unsupported assumption handling (explicit)

`tf-rpa` returns clear unsupported errors for:

- frozen-at-throat chemistry mode
- exit-pressure-constrained nozzle solves

The UI surfaces these errors directly and does not fake results.

## Persistence

Rocket workspace persistence is now included in project schema as optional `rocket_workspace`:

- selected Rocket subtab
- performance input case/assumptions

Last computed result is runtime-only in this slice.

## Rocket > Studies (this phase)

Implemented first functional Studies subtab with single-variable sweeps derived from current Performance configuration.

Supported study variables:

- chamber pressure
- mixture ratio
- ambient pressure
- expansion ratio (when base case uses expansion-ratio nozzle constraint)

Supported study outputs:

- chamber temperature

## Thermal Design (current)

Rocket Thermal now supports a transparent first-pass thermal design workflow with:

- regenerative cooling mode
- film cooling mode
- combined regen+film mode
- material/wall properties
- coolant properties and inlet conditions
- station-by-station channel geometry
- pressure-drop-constrained automated channel designer

The thermal UI exposes:

- equations/correlation trace
- assumptions and impact statements
- optimizer iteration trace
- multi-series plots for heat flux, temperatures, pressure, channel geometry, and film effect

See `docs/ROCKET_THERMAL_DESIGN_MODEL.md` for equations, assumptions, and deferred fidelity boundaries.
- chamber gamma
- chamber molecular weight
- c*
- Cf,vac
- Isp,vac
- Isp,amb
- effective exhaust velocity (vac/amb)
- chamber-to-ambient pressure ratio

UI includes:

- study configuration panel
- output metric multi-select
- inline warnings
- results table
- shared multi-series plot of selected outputs vs swept variable (aligned with Fluid sweep plotting style)

Deferred:

- multi-variable sweeps
- optimization workflow
- altitude profile studies
- station-level throat/exit study metrics while backend data remain unavailable

## Rocket > Propellants (this phase)

Implemented first functional Propellants subtab with a searchable preset library and apply-to-Performance workflow.

Current preset library includes:

- LOX/RP-1
- LOX/CH4
- LOX/LH2
- N2O/IPA
- N2O/Ethanol
- MMH/NTO
- UDMH/NTO
- HTP monoprop approximation

Propellants behavior in this slice:

- Preset list is searchable by name/category/oxidizer/fuel/notes.
- Selecting a preset shows category, composition, notes, and recommended O/F.
- "Apply to Performance" updates the current Performance case oxidizer/fuel and recommended values (O/F and temperatures when provided).
- Studies continue to derive from the active Performance case; no separate studies propellant source is introduced.

Deferred:

- custom chemistry editor and full propellant data management UI
- favorites/recent preset management
- multi-case propellant template manager


## Rocket > Geometry (this phase)

Implemented first functional Geometry subtab with Performance-derived sizing workflow.

Supported now:

- Sizing mode: given throat diameter or given throat area
- Derived outputs: throat/exit area+diameter, expansion ratio, chamber pressure reference
- First-pass estimates: chamber area/diameter, chamber length (from L*), nozzle length (style factor + half-angle + truncation)
- Canonical engine model output (axial stations + contour points)
- Inline preview reflecting chamber/converging/throat/diverging/exit proportions
- Contour styles: conical, bell-parabolic, truncated ideal (with truncation ratio)

Deferred:

- contour optimization and CAD/export
- high-fidelity chamber/nozzle design features
- thermal coupling into Geometry


## Rocket > Thermal (this phase)

Implemented first functional Thermal subtab with Performance+Geometry-derived first-pass heat-load estimation.

Supported now:

- Thermal model: Bartz-like convective estimate
- Cooling modes: adiabatic wall, regenerative placeholder, film-cooling placeholder
- Outputs: chamber/throat/exit heat flux, peak heat flux+location, total heat-load estimate
- Visualization: shared multi-series axial plot with selectable heat flux / gas-side HTC / recovery temperature series

Deferred:

- detailed regenerative cooling passage design
- detailed film-cooling modeling
- radiation-coupled and conjugate wall models
- coolant-side hydraulics/pressure-drop coupling


## Rocket > Data (this phase)

Implemented first functional Data subtab for assumptions/provenance/diagnostics.

Supported now:

- Active case summary and backend-chain summary
- Performance/Geometry/Thermal/Propellant assumption summaries
- Provenance labels: Native backend, Derived in Rust, Estimated, Unavailable
- Aggregated warnings for unsupported modes and deferred/estimated model limitations

Deferred:

- full species/thermo database browser
- editable chemistry data workflows
- deep backend-internal introspection tooling
