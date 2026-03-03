# Architecture Layers

Design-first boundary model for capabilities that come after atomic equations.

## Layer Definitions

1. **Atomic Equation**: one physical relation with scalar-first solve behavior.
2. **Equation Family / Variants**: one canonical law with multiple discoverable forms.
3. **Component Model**: iterative orchestration over multiple equations and contexts.
4. **Solve Graph / Chaining**: node/edge execution over equations/components/sources.
5. **External Bindings**: generated Python/Excel adapters over Rust-owned logic.

## Belongs Here / Not Here

### Atomic Equation
- Belongs: one law, scalar inputs/outputs, equation-local tests.
- Not here: multi-step orchestration or workflow graphs.

### Equation Family
- Belongs: canonical law identity + variants (e.g., ideal gas forms).
- Not here: duplicate independent solver logic for each form.

### Component Model
- Belongs: multi-equation iterative behavior (prototype: two_orifice).
- Not here: source-of-truth atomic law definitions.

### Solve Graph
- Belongs: dependency ordering and chained execution.
- Not here: component constitutive details.

### External Bindings
- Belongs: generated signatures/docs from catalog metadata.
- Not here: reimplementation of core solver/unit logic.

## Ownership Map

- `equations`: atomic equations + family metadata.
- `eng-fluids`: fluid catalog/property backends.
- `eng-materials`: material catalogs/interpolation.
- `eng`: unified orchestration for components, graphs, bindings, docs/catalog assembly.
- `eng-core`: shared units/input semantics.

## Prototypes

- **Ideal Gas Family**: canonical `P*V=m*R*T` with pressure-volume and density variants.
- **Two-Orifice Component**: iterative splitter model using `orifice_mass_flow` + `continuity` equations.

Machine-readable architecture metadata is exported as `generated/architecture_spec.json`.
