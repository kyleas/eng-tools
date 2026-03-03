# Engineering Abstraction Layers

## Purpose
Prevent architectural drift as the library grows beyond atomic equations.

This document defines strict boundaries between:
1. atomic equations
2. equation families / variants
3. component models
4. solve graphs / chaining
5. generated external bindings

## Layer Definitions

### 1. Atomic Equation
- One physical relation.
- Scalar-first solve behavior.
- Registry-defined metadata, validation, diagnostics, and tests.
- May include explicit forms and numerical fallback for that same relation.

Not atomic-equation scope:
- multi-step orchestration
- component iteration policy
- graph scheduling

### 2. Equation Family / Variants
- One canonical law identity with multiple common views.
- Family owns shared assumptions and discoverability.
- Variants are documented forms/views that map to canonical equation logic.

Not family scope:
- duplicate independent solver logic for each algebraic form
- component orchestration

### 3. Component Model
- Multi-equation model with engineering-level IO.
- Can orchestrate iterative solves and context resolution.
- Reuses equations/fluids/materials/constants; does not redefine them.

Not component scope:
- defining new atomic laws
- global graph planner

### 4. Solve Graph / Chaining
- Node/edge orchestration across equations, components, constants, and property sources.
- Owns dependency ordering, graph diagnostics, and execution policy.

Not graph scope:
- atomic-law definitions
- component-specific constitutive detail

### 5. Generated External Bindings (Python/Excel)
- Thin generated adapters over Rust-owned behavior.
- Generated from unified catalog + binding metadata.
- No duplicated unit/solver logic.

Not binding scope:
- reimplementation of physics/solver core
- separate source of truth for semantics

## Ownership Map
- `equations`: atomic equations + equation families/variants
- `eng-fluids`: fluid catalog/property backend + fluid docs fragments
- `eng-materials`: material catalog/interpolation + material docs fragments
- `eng`: component layer orchestration, solve-graph layer orchestration, unified docs/catalog assembly, binding spec/generation orchestration
- `eng-core`: shared units/input/quantity semantics consumed by all layers

## Belongs Here / Not Here Rules

### Atomic Equation
Belongs:
- single physical law
- equation-local tests/baselines
- explicit/numerical solve for same law

Does not belong:
- end-to-end workflow orchestration
- multi-node chaining policy

### Equation Family
Belongs:
- canonical law identity
- variant metadata/search/docs views

Does not belong:
- copy-pasted duplicate equations only for display form differences

### Component Model
Belongs:
- iterative engineering behavior over multiple equations
- higher-level IO and convergence rules

Does not belong:
- source-of-truth law definitions

### Solve Graph
Belongs:
- chained execution across nodes
- edge contract and dependency checks

Does not belong:
- constitutive law details

### External Bindings
Belongs:
- generated function/class surfaces
- language-specific naming/signature adaptation

Does not belong:
- independent core behavior

## Prototype Boundaries

### Equation Family Prototype: Ideal Gas
- Canonical law: `P*V = m*R*T`
- Variant views:
  - pressure-volume form
  - density form `P = rho*R*T`
- Variants remain one family, not separate unrelated laws.

### Component Prototype: Two Orifice
- Uses multiple equations (`orifice_mass_flow`, `continuity`) plus iterative split logic.
- Lives in component layer, not atomic equation registry.

## Docs/Catalog Evolution Plan

Unified catalog should evolve to include:
- `equation_families`
- `components`
- `binding_surfaces`

Required links:
- `family_variant_maps_to_equation`
- `component_uses_equation`
- `component_requires_context`
- `binding_exposes_entity`

The generated handbook should include:
- architecture chapter (this boundary model)
- family pages
- component pages
- graph workflow pages
- binding surface pages

