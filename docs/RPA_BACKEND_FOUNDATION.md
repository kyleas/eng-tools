# RPA Backend Foundation (Phase 5/6 backend slice)

## Phase 0: tf-cea gap snapshot

`tf-cea` currently provides:

- CEA backend interface (`CeaBackend`)
- process adapter (`CeaProcessAdapter`)
- rocket call with chamber-level and integrated performance outputs (`RocketResult`)

Gaps relative to RPA-style chamber/nozzle workflows:

- no explicit nozzle constraint mode for exit-pressure solves
- no station-level throat/exit state summaries in backend output
- no explicit frozen-at-throat mode

## New ownership boundary

`tf-rpa` is introduced to own:

- rocket analysis problem definitions
- assumption semantics and validation
- solve orchestration through `tf-cea`
- normalized RPA-style result objects

`tf-rpa` does **not** implement CEA physics.

## First vertical slice

Implemented end-to-end path:

- validate a `RocketAnalysisProblem`
- map assumptions into `tf-cea::RocketProblem`
- execute `CeaBackend::run_rocket`
- return `RocketAnalysisResult` including:
  - chamber summary
  - placeholder throat/exit summaries
  - c*, Cf(vac), Isp(vac)
  - derived Cf(amb), Isp(amb)
  - assumption metadata and computation notes

## Supported assumptions now

- combustor: `InfiniteArea`, `FiniteArea { contraction_ratio }`
- nozzle chemistry: `ShiftingEquilibrium`, `FrozenAtChamber`

## Explicitly unsupported (clear errors)

- `NozzleChemistryModel::FrozenAtThroat`
- `NozzleConstraint::ExitPressurePa(..)`

These are intentionally preserved as API seams for later backend expansion.
