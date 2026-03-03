use std::{collections::BTreeMap, fs, path::Path};

use serde::Serialize;

use crate::{
    constants,
    docs::{page_model::build_page_models, search_index::build_search_index},
    equation_families,
    error::{EquationError, Result},
    model::EquationDef,
};

pub const EXPORT_SCHEMA_VERSION: &str = "1";
pub const EXPORT_MODEL_VERSION: &str = "2026-03-02";

#[derive(Debug, Clone, Serialize)]
struct ArtifactEnvelope<T> {
    schema_version: &'static str,
    model_version: &'static str,
    artifact_type: &'static str,
    items: T,
}

#[derive(Debug, Clone, Serialize)]
struct NavigationNode {
    category: String,
    subcategories: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
struct FluidExport {
    key: String,
    name: String,
    aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MaterialExport {
    key: String,
    name: String,
    aliases: Vec<String>,
    source: String,
    properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct UnifiedCatalog {
    equations: Vec<UnifiedEquationEntry>,
    families: Vec<equation_families::EquationFamilyDef>,
    constants: Vec<crate::constants::EngineeringConstant>,
    fluids: Vec<UnifiedFluidEntry>,
    materials: Vec<UnifiedMaterialEntry>,
    links: Vec<CatalogLink>,
}

#[derive(Debug, Clone, Serialize)]
struct UnifiedEquationEntry {
    key: String,
    path_id: String,
    name: String,
    category: String,
    subcategories: Vec<String>,
    default_target: Option<String>,
    uses_constants: Vec<String>,
    resolver_contexts: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct UnifiedFluidEntry {
    key: String,
    name: String,
    aliases: Vec<String>,
    supported_properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct UnifiedMaterialEntry {
    key: String,
    name: String,
    aliases: Vec<String>,
    properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CatalogLink {
    relation: String,
    from: String,
    to: String,
}

#[derive(Debug, Clone, Serialize)]
struct ArchitectureSpecExport {
    layers: Vec<ArchitectureLayerExport>,
    ownership: Vec<OwnershipExport>,
    prototypes: Vec<PrototypeExport>,
    catalog_plan: CatalogPlanExport,
}

#[derive(Debug, Clone, Serialize)]
struct ArchitectureLayerExport {
    key: &'static str,
    definition: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct OwnershipExport {
    layer: &'static str,
    owner: &'static str,
    owns: Vec<&'static str>,
    does_not_own: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
struct PrototypeExport {
    key: &'static str,
    layer: &'static str,
    description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct CatalogPlanExport {
    new_sections: Vec<&'static str>,
    required_links: Vec<&'static str>,
}

/// Export deterministic docs/search artifacts used by app/docs integrations.
pub fn export_docs_artifacts(equations: &[EquationDef], out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir).map_err(|source| EquationError::Io {
        path: out_dir.to_path_buf(),
        source,
    })?;

    let search = build_search_index(equations);
    let pages = build_page_models(equations);
    let nav = build_navigation(equations);
    let examples = build_examples_index(&pages);
    let families = equation_families::load_default_validated(equations)?;

    write_json(
        out_dir.join("search_index.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "search_index",
            items: search,
        },
    )?;
    write_json(
        out_dir.join("page_models.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "page_models",
            items: pages.clone(),
        },
    )?;
    write_json(
        out_dir.join("navigation.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "navigation",
            items: nav,
        },
    )?;
    write_json(
        out_dir.join("examples_index.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "examples_index",
            items: examples,
        },
    )?;
    write_json(
        out_dir.join("constants.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "constants",
            items: constants::all(),
        },
    )?;
    write_json(
        out_dir.join("families.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "families",
            items: families.clone(),
        },
    )?;
    write_json(
        out_dir.join("fluids.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "fluids",
            items: build_fluids_export(),
        },
    )?;
    write_json(
        out_dir.join("materials.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "materials",
            items: build_materials_export()?,
        },
    )?;
    write_json(
        out_dir.join("catalog.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "catalog",
            items: build_unified_catalog(&pages, &families)?,
        },
    )?;
    write_json(
        out_dir.join("architecture_spec.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "architecture_spec",
            items: build_architecture_spec_export(),
        },
    )?;
    Ok(())
}

fn build_navigation(equations: &[EquationDef]) -> Vec<NavigationNode> {
    let mut by_category: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    for eq in equations {
        let category = eq.taxonomy.category.clone();
        let subcat = eq
            .taxonomy
            .subcategories
            .first()
            .cloned()
            .unwrap_or_else(|| "_root".to_string());
        by_category
            .entry(category)
            .or_default()
            .entry(subcat)
            .or_default()
            .push(eq.effective_slug().to_string());
    }
    by_category
        .into_iter()
        .map(|(category, mut subcategories)| {
            for values in subcategories.values_mut() {
                values.sort();
            }
            NavigationNode {
                category,
                subcategories,
            }
        })
        .collect()
}

fn write_json(path: impl AsRef<Path>, value: &impl Serialize) -> Result<()> {
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json).map_err(|source| EquationError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}

fn build_examples_index(
    pages: &[crate::docs::page_model::EquationPageModel],
) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for p in pages {
        for ex in &p.examples {
            out.push(serde_json::json!({
                "path_id": p.path_id,
                "label": ex.label,
                "style": ex.style,
                "code": ex.code
            }));
        }
    }
    out
}

fn build_fluids_export() -> Vec<FluidExport> {
    let mut out = Vec::new();
    for f in eng_fluids::catalog() {
        out.push(FluidExport {
            key: f.key.to_string(),
            name: f.display_name.to_string(),
            aliases: f.aliases.iter().map(|x| x.to_string()).collect(),
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

fn build_materials_export() -> Result<Vec<MaterialExport>> {
    let mut out = Vec::new();
    let materials = eng_materials::catalog()
        .map_err(|e| EquationError::Validation(format!("materials catalog export failed: {e}")))?;
    for m in materials {
        let def = m.definition().map_err(|e| {
            EquationError::Validation(format!("material '{}' export failed: {e}", m.key()))
        })?;
        out.push(MaterialExport {
            key: def.key,
            name: def.name,
            aliases: def.aliases,
            source: def.source,
            properties: def.properties.keys().cloned().collect(),
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(out)
}

fn build_unified_catalog(
    pages: &[crate::docs::page_model::EquationPageModel],
    families: &[equation_families::EquationFamilyDef],
) -> Result<UnifiedCatalog> {
    let mut equations = Vec::new();
    for p in pages {
        let mut resolver_contexts: Vec<String> = p
            .variables
            .iter()
            .filter_map(|v| v.resolver_source.clone())
            .collect();
        resolver_contexts.sort();
        resolver_contexts.dedup();
        equations.push(UnifiedEquationEntry {
            key: p.key.clone(),
            path_id: p.path_id.clone(),
            name: p.name.clone(),
            category: p.category.clone(),
            subcategories: p.subcategories.clone(),
            default_target: p.default_target.clone(),
            uses_constants: p.uses_constants.iter().map(|c| c.key.clone()).collect(),
            resolver_contexts,
        });
    }
    equations.sort_by(|a, b| a.path_id.cmp(&b.path_id));

    let constants = constants::all().to_vec();

    let mut fluids = Vec::new();
    for f in eng_fluids::catalog() {
        fluids.push(UnifiedFluidEntry {
            key: f.key.to_string(),
            name: f.display_name.to_string(),
            aliases: f.aliases.iter().map(|x| x.to_string()).collect(),
            supported_properties: eng_fluids::SUPPORTED_PROPERTIES
                .iter()
                .map(|x| x.to_string())
                .collect(),
        });
    }
    fluids.sort_by(|a, b| a.key.cmp(&b.key));

    let mut materials = Vec::new();
    for m in eng_materials::catalog()
        .map_err(|e| EquationError::Validation(format!("materials catalog failed: {e}")))?
    {
        let def = m.definition().map_err(|e| {
            EquationError::Validation(format!("material '{}' export failed: {e}", m.key()))
        })?;
        materials.push(UnifiedMaterialEntry {
            key: def.key,
            name: def.name,
            aliases: def.aliases,
            properties: def.properties.keys().cloned().collect(),
        });
    }
    materials.sort_by(|a, b| a.key.cmp(&b.key));

    let mut links = Vec::new();
    for eq in &equations {
        for c in &eq.uses_constants {
            links.push(CatalogLink {
                relation: "equation_uses_constant".to_string(),
                from: eq.path_id.clone(),
                to: c.clone(),
            });
        }
        for ctx in &eq.resolver_contexts {
            links.push(CatalogLink {
                relation: "equation_uses_context".to_string(),
                from: eq.path_id.clone(),
                to: ctx.clone(),
            });
        }
    }
    for family in families {
        links.push(CatalogLink {
            relation: "family_canonical_equation".to_string(),
            from: family.key.clone(),
            to: family.canonical_equation.clone(),
        });
        for variant in &family.variants {
            links.push(CatalogLink {
                relation: "family_variant_maps_to_equation".to_string(),
                from: format!("{}.{}", family.key, variant.key),
                to: variant.equation_id.clone(),
            });
        }
    }
    links.sort_by(|a, b| {
        a.relation
            .cmp(&b.relation)
            .then(a.from.cmp(&b.from))
            .then(a.to.cmp(&b.to))
    });

    Ok(UnifiedCatalog {
        equations,
        families: families.to_vec(),
        constants,
        fluids,
        materials,
        links,
    })
}

fn build_architecture_spec_export() -> ArchitectureSpecExport {
    ArchitectureSpecExport {
        layers: vec![
            ArchitectureLayerExport {
                key: "atomic_equation",
                definition: "one physical relation with scalar-first solve behavior",
            },
            ArchitectureLayerExport {
                key: "equation_family",
                definition: "canonical law with multiple documented/discoverable variants",
            },
            ArchitectureLayerExport {
                key: "component_model",
                definition: "multi-equation iterative engineering model",
            },
            ArchitectureLayerExport {
                key: "solve_graph",
                definition: "node/edge chaining across equations/components/sources",
            },
            ArchitectureLayerExport {
                key: "external_binding",
                definition: "generated Python/Excel surfaces over Rust-owned logic",
            },
        ],
        ownership: vec![
            OwnershipExport {
                layer: "atomic_equation",
                owner: "equations",
                owns: vec![
                    "equation registry + validation",
                    "scalar solve behavior",
                    "equation docs fragments",
                ],
                does_not_own: vec![
                    "component orchestration",
                    "graph scheduling",
                    "binding runtime logic",
                ],
            },
            OwnershipExport {
                layer: "equation_family",
                owner: "equations",
                owns: vec![
                    "family identity + variant metadata",
                    "family discoverability/docs",
                ],
                does_not_own: vec!["component iteration", "graph orchestration"],
            },
            OwnershipExport {
                layer: "component_model",
                owner: "eng",
                owns: vec![
                    "multi-equation orchestration",
                    "iterative convergence policy",
                    "context-aware high-level IO",
                ],
                does_not_own: vec!["atomic law definitions", "graph scheduling"],
            },
            OwnershipExport {
                layer: "solve_graph",
                owner: "eng",
                owns: vec![
                    "dependency ordering",
                    "node/edge solve execution",
                    "chained workflow diagnostics",
                ],
                does_not_own: vec!["atomic law definitions", "component constitutive details"],
            },
            OwnershipExport {
                layer: "external_binding",
                owner: "eng",
                owns: vec![
                    "binding generation spec",
                    "naming/signature adaptation",
                    "catalog-driven surface generation",
                ],
                does_not_own: vec!["solver/unit reimplementation"],
            },
        ],
        prototypes: vec![
            PrototypeExport {
                key: "ideal_gas_family",
                layer: "equation_family",
                description: "P*V=m*R*T canonical identity with pressure-volume and density variants",
            },
            PrototypeExport {
                key: "two_orifice_component",
                layer: "component_model",
                description: "iterative pressure split model using orifice + continuity equations",
            },
        ],
        catalog_plan: CatalogPlanExport {
            new_sections: vec!["equation_families", "components", "binding_surfaces"],
            required_links: vec![
                "family_variant_maps_to_equation",
                "component_uses_equation",
                "component_requires_context",
                "binding_exposes_entity",
            ],
        },
    }
}
