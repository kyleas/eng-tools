# Architecture Layers

This chapter defines strict ownership boundaries for work after atomic equations.

## Layer Definitions

1. **Atomic Equation**: one physical relation with scalar-first solve behavior and equation-level tests/docs.
2. **Equation Family / Variants**: one canonical law with multiple discoverable forms without duplicating solver logic.
3. **Component Model**: multi-equation iterative engineering model using contexts (fluid/material) as needed.
4. **Solve Graph / Chaining**: node/edge orchestration connecting equations, components, constants, and property sources.
5. **External Bindings**: generated Python/Excel surfaces over Rust-owned implementations.

## Ownership Map

| Layer | Owner | Owns | Does not own |
| --- | --- | --- | --- | --- |
| `AtomicEquation` | `Equations` | equation registry + normalization + validation; scalar solve behavior + diagnostics; equation-level docs/examples/export fragments | multi-step component orchestration; top-level graph execution engine; excel/python runtime implementations |
| `EquationFamily` | `Equations` | canonical law identity + variant metadata; variant discovery/docs/search surfaces; family-to-atomic mapping | component iterative strategies; graph scheduling; external binding runtimes |
| `ComponentModel` | `Eng` | multi-equation orchestration; iteration policy and convergence strategy; context-aware model IO surfaces | single-relation canonical equations; global graph scheduling; backend-native fluid/material implementations |
| `SolveGraph` | `Eng` | node/edge orchestration across equations/components; dependency ordering and graph solve execution; cross-domain chained workflows | atomic equation definitions; fluid/material property backends; binding runtime logic |
| `ExternalBinding` | `Eng` | python/excel surface generation spec; binding package metadata + naming rules; catalog-driven API exposure policy | core numeric implementations; domain registry truth; independent duplicate logic |

## Belongs Here / Not Here Rules

### `AtomicEquation`

Belongs here:
- single physical law with scalar variables
- explicit and/or numerical solve paths for that one law
- baseline full-state test cases

Does not belong here:
- workflow orchestration spanning many laws
- component-level iteration/control loops
- cross-step graph sequencing

### `EquationFamily`

Belongs here:
- multiple common views of one canonical law
- variant docs/search aliases
- shared assumptions across forms

Does not belong here:
- duplicate full atomic equations for each algebraic view
- component orchestration logic
- graph node scheduling

### `ComponentModel`

Belongs here:
- multi-equation iterative model behavior
- high-level engineering input/output interfaces
- context resolution across fluid/material states

Does not belong here:
- new atomic law definitions
- general graph planner
- binding-generation policy

### `SolveGraph`

Belongs here:
- equation/component/property/constant node chaining
- edge-based dataflow + dependency execution
- multi-node workflow diagnostics

Does not belong here:
- atomic equation authoring
- component-specific constitutive details
- domain backend implementations

### `ExternalBinding`

Belongs here:
- generated thin wrappers for rust-owned behavior
- stable naming/signature projection
- binding docs/examples from unified catalog

Does not belong here:
- business logic reimplementation
- separate unit engine
- untracked API behavior divergence

## Prototype: Equation Family (Ideal Gas)

- Key: `ideal_gas`
- Canonical relation: `P * V = m * R * T`
- Canonical equation path: `thermo.ideal_gas`
- Variants:
  - `pv_equals_mrt` (Pressure-Volume Form) target `P`: closed-control-volume mass/temperature/volume calculations
  - `p_equals_rho_rt` (Density Form) target `P`: flow-property form using density instead of total volume/mass

## Prototype: Component Model (Two Orifice)

- Key: `two_orifice`
- Requires contexts: fluid
- Depends on equations: fluids.orifice_mass_flow_incompressible, fluids.continuity_mass_flow
- component owns iterative pressure split; equations stay atomic
- inputs/outputs are engineering-level (inlet pressure, outlet pressures, total flow)
- designed as a future component-layer package target

## Solve Graph Model (Planned)

- Node kinds: atomic_equation, equation_family_variant, component_model, fluid_property_source, material_property_source, constant_source
- Edge semantics: edge carries resolved scalar value (SI canonical); target ports consume upstream node outputs; graph solve performs dependency order + cycle diagnostics
- graph layer composes equations/components but does not redefine their internals
- cycles require explicit policy (fixed-point/newton) and remain out-of-scope for this pass

## External Binding Plan (Python/Excel)

- Targets: python, excel
- Authoritative runtime: rust (eng + domain crates)
- Generated from: unified catalog.json; equation pages/examples metadata; family/component binding manifest entries
- Naming rules:
  - python: snake_case modules/functions
  - excel: SCREAMING_SNAKE_CASE function names + simple scalar signatures
  - bindings expose plain-SI and unit-string convenience overloads
- Notes:
  - bindings are thin adapters; no duplicated solver/unit logic
  - docs/examples in bindings are generated from unified handbook metadata

## Catalog Evolution Plan

- New sections: equation_families, components, binding_surfaces
- Required links: family_variant_maps_to_equation, component_uses_equation, component_requires_context, binding_exposes_entity

The machine-readable form of this chapter is exported as `generated/architecture_spec.json`.
