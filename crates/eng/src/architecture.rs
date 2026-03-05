use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ArchitectureLayer {
    AtomicEquation,
    EquationFamily,
    ComponentModel,
    SolveWorkflow,
    SolveGraph,
    ExternalBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LayerOwner {
    Equations,
    Eng,
    EngFluids,
    EngMaterials,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OwnershipRule {
    pub layer: ArchitectureLayer,
    pub owner: LayerOwner,
    pub owns: Vec<&'static str>,
    pub does_not_own: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayerBoundaryRule {
    pub layer: ArchitectureLayer,
    pub belongs_here: Vec<&'static str>,
    pub does_not_belong_here: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EquationFamilyPrototype {
    pub key: &'static str,
    pub name: &'static str,
    pub canonical_relation: &'static str,
    pub canonical_equation_path: &'static str,
    pub variants: Vec<FamilyVariantPrototype>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FamilyVariantPrototype {
    pub key: &'static str,
    pub display_name: &'static str,
    pub target: &'static str,
    pub intended_usage: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComponentModelPrototype {
    pub key: &'static str,
    pub name: &'static str,
    pub required_contexts: Vec<&'static str>,
    pub equation_dependencies: Vec<&'static str>,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SolveGraphPrototype {
    pub node_kinds: Vec<&'static str>,
    pub edge_semantics: Vec<&'static str>,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BindingPlan {
    pub targets: Vec<&'static str>,
    pub authoritative_runtime: &'static str,
    pub generated_from: Vec<&'static str>,
    pub naming_rules: Vec<&'static str>,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CatalogEvolutionPlan {
    pub new_sections: Vec<&'static str>,
    pub required_links: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArchitectureSpec {
    pub ownership: Vec<OwnershipRule>,
    pub boundaries: Vec<LayerBoundaryRule>,
    pub family_prototype: EquationFamilyPrototype,
    pub component_prototype: ComponentModelPrototype,
    pub graph_prototype: SolveGraphPrototype,
    pub binding_plan: BindingPlan,
    pub catalog_plan: CatalogEvolutionPlan,
}

pub fn ownership_rules() -> Vec<OwnershipRule> {
    vec![
        OwnershipRule {
            layer: ArchitectureLayer::AtomicEquation,
            owner: LayerOwner::Equations,
            owns: vec![
                "equation registry + normalization + validation",
                "scalar solve behavior + diagnostics",
                "equation-level docs/examples/export fragments",
            ],
            does_not_own: vec![
                "multi-step component orchestration",
                "top-level graph execution engine",
                "excel/python runtime implementations",
            ],
        },
        OwnershipRule {
            layer: ArchitectureLayer::EquationFamily,
            owner: LayerOwner::Equations,
            owns: vec![
                "canonical law identity + variant metadata",
                "variant discovery/docs/search surfaces",
                "family-to-atomic mapping",
            ],
            does_not_own: vec![
                "component iterative strategies",
                "graph scheduling",
                "external binding runtimes",
            ],
        },
        OwnershipRule {
            layer: ArchitectureLayer::ComponentModel,
            owner: LayerOwner::Eng,
            owns: vec![
                "multi-equation orchestration",
                "iteration policy and convergence strategy",
                "context-aware model IO surfaces",
            ],
            does_not_own: vec![
                "single-relation canonical equations",
                "global graph scheduling",
                "backend-native fluid/material implementations",
            ],
        },
        OwnershipRule {
            layer: ArchitectureLayer::SolveWorkflow,
            owner: LayerOwner::Eng,
            owns: vec![
                "shared numeric solve wrappers (root/convergence reporting)",
                "shared ODE integration interfaces for engineering models",
                "station/workflow chaining and provenance tracing",
            ],
            does_not_own: vec![
                "atomic equation definitions",
                "device-specific engineering semantics",
                "binding-generation policy",
            ],
        },
        OwnershipRule {
            layer: ArchitectureLayer::SolveGraph,
            owner: LayerOwner::Eng,
            owns: vec![
                "node/edge orchestration across equations/components",
                "dependency ordering and graph solve execution",
                "cross-domain chained workflows",
            ],
            does_not_own: vec![
                "atomic equation definitions",
                "fluid/material property backends",
                "binding runtime logic",
            ],
        },
        OwnershipRule {
            layer: ArchitectureLayer::ExternalBinding,
            owner: LayerOwner::Eng,
            owns: vec![
                "python/excel surface generation spec",
                "binding package metadata + naming rules",
                "catalog-driven API exposure policy",
            ],
            does_not_own: vec![
                "core numeric implementations",
                "domain registry truth",
                "independent duplicate logic",
            ],
        },
    ]
}

pub fn boundary_rules() -> Vec<LayerBoundaryRule> {
    vec![
        LayerBoundaryRule {
            layer: ArchitectureLayer::AtomicEquation,
            belongs_here: vec![
                "single physical law with scalar variables",
                "explicit and/or numerical solve paths for that one law",
                "baseline full-state test cases",
            ],
            does_not_belong_here: vec![
                "workflow orchestration spanning many laws",
                "component-level iteration/control loops",
                "cross-step graph sequencing",
            ],
        },
        LayerBoundaryRule {
            layer: ArchitectureLayer::EquationFamily,
            belongs_here: vec![
                "multiple common views of one canonical law",
                "variant docs/search aliases",
                "shared assumptions across forms",
            ],
            does_not_belong_here: vec![
                "duplicate full atomic equations for each algebraic view",
                "component orchestration logic",
                "graph node scheduling",
            ],
        },
        LayerBoundaryRule {
            layer: ArchitectureLayer::ComponentModel,
            belongs_here: vec![
                "multi-equation iterative model behavior",
                "high-level engineering input/output interfaces",
                "context resolution across fluid/material states",
            ],
            does_not_belong_here: vec![
                "new atomic law definitions",
                "general graph planner",
                "binding-generation policy",
            ],
        },
        LayerBoundaryRule {
            layer: ArchitectureLayer::SolveWorkflow,
            belongs_here: vec![
                "reusable root/ODE numeric utilities with convergence metadata",
                "ordered step/station workflow execution scaffolding",
                "cross-step provenance, warnings, and structured failure propagation",
            ],
            does_not_belong_here: vec![
                "new equation physics definitions",
                "device-specific binding naming decisions",
                "full arbitrary graph optimization/planning",
            ],
        },
        LayerBoundaryRule {
            layer: ArchitectureLayer::SolveGraph,
            belongs_here: vec![
                "equation/component/property/constant node chaining",
                "edge-based dataflow + dependency execution",
                "multi-node workflow diagnostics",
            ],
            does_not_belong_here: vec![
                "atomic equation authoring",
                "component-specific constitutive details",
                "domain backend implementations",
            ],
        },
        LayerBoundaryRule {
            layer: ArchitectureLayer::ExternalBinding,
            belongs_here: vec![
                "generated thin wrappers for rust-owned behavior",
                "stable naming/signature projection",
                "binding docs/examples from unified catalog",
            ],
            does_not_belong_here: vec![
                "business logic reimplementation",
                "separate unit engine",
                "untracked API behavior divergence",
            ],
        },
    ]
}

pub fn ideal_gas_family_prototype() -> EquationFamilyPrototype {
    EquationFamilyPrototype {
        key: "ideal_gas",
        name: "Ideal Gas Law Family",
        canonical_relation: "P * V = m * R * T",
        canonical_equation_path: "thermo.ideal_gas",
        variants: vec![
            FamilyVariantPrototype {
                key: "pv_equals_mrt",
                display_name: "Pressure-Volume Form",
                target: "P",
                intended_usage: "closed-control-volume mass/temperature/volume calculations",
            },
            FamilyVariantPrototype {
                key: "p_equals_rho_rt",
                display_name: "Density Form",
                target: "P",
                intended_usage: "flow-property form using density instead of total volume/mass",
            },
        ],
    }
}

pub fn two_orifice_component_prototype() -> ComponentModelPrototype {
    ComponentModelPrototype {
        key: "two_orifice",
        name: "Two-Orifice Pressure Splitter (Prototype)",
        required_contexts: vec!["fluid"],
        equation_dependencies: vec![
            "fluids.orifice_mass_flow_incompressible",
            "fluids.continuity_mass_flow",
        ],
        notes: vec![
            "component owns iterative pressure split; equations stay atomic",
            "inputs/outputs are engineering-level (inlet pressure, outlet pressures, total flow)",
            "designed as a future component-layer package target",
        ],
    }
}

pub fn solve_graph_prototype() -> SolveGraphPrototype {
    SolveGraphPrototype {
        node_kinds: vec![
            "atomic_equation",
            "equation_family_variant",
            "component_model",
            "fluid_property_source",
            "material_property_source",
            "constant_source",
        ],
        edge_semantics: vec![
            "edge carries resolved scalar value (SI canonical)",
            "target ports consume upstream node outputs",
            "graph solve performs dependency order + cycle diagnostics",
        ],
        notes: vec![
            "graph layer composes equations/components but does not redefine their internals",
            "cycles require explicit policy (fixed-point/newton) and remain out-of-scope for this pass",
        ],
    }
}

pub fn external_binding_plan() -> BindingPlan {
    BindingPlan {
        targets: vec!["python", "excel"],
        authoritative_runtime: "rust (eng + domain crates)",
        generated_from: vec![
            "unified catalog.json",
            "equation pages/examples metadata",
            "family/component binding manifest entries",
        ],
        naming_rules: vec![
            "python: snake_case modules/functions",
            "excel: SCREAMING_SNAKE_CASE function names + simple scalar signatures",
            "bindings expose plain-SI and unit-string convenience overloads",
        ],
        notes: vec![
            "bindings are thin adapters; no duplicated solver/unit logic",
            "docs/examples in bindings are generated from unified handbook metadata",
        ],
    }
}

pub fn catalog_evolution_plan() -> CatalogEvolutionPlan {
    CatalogEvolutionPlan {
        new_sections: vec!["equation_families", "components", "binding_surfaces"],
        required_links: vec![
            "family_variant_maps_to_equation",
            "component_uses_equation",
            "component_requires_context",
            "binding_exposes_entity",
        ],
    }
}

pub fn architecture_spec() -> ArchitectureSpec {
    ArchitectureSpec {
        ownership: ownership_rules(),
        boundaries: boundary_rules(),
        family_prototype: ideal_gas_family_prototype(),
        component_prototype: two_orifice_component_prototype(),
        graph_prototype: solve_graph_prototype(),
        binding_plan: external_binding_plan(),
        catalog_plan: catalog_evolution_plan(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_spec_has_all_layers() {
        let spec = architecture_spec();
        assert_eq!(spec.ownership.len(), 6);
        assert_eq!(spec.boundaries.len(), 6);
        assert_eq!(spec.family_prototype.key, "ideal_gas");
        assert_eq!(spec.component_prototype.key, "two_orifice");
    }
}
