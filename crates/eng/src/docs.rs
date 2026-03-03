use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::architecture::architecture_spec;
use equations::{
    Registry,
    constants::EngineeringConstant,
    docs::{
        build_equation_docs_contribution,
        contrib::EquationDocsContribution,
        page_model::EquationPageModel,
        presentation::{CategoryPresentation, SubcategoryPresentation},
        routes as doc_routes,
    },
    error::{EquationError, Result},
};
use printpdf::{BuiltinFont, Mm, PdfDocument};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct ArtifactEnvelope<T> {
    schema_version: &'static str,
    model_version: &'static str,
    artifact_type: &'static str,
    items: T,
}

#[derive(Debug, Clone, Serialize)]
struct UnifiedCatalog {
    equations: Vec<UnifiedEquationEntry>,
    families: Vec<FamilyCatalogItem>,
    constants: Vec<EngineeringConstant>,
    fluids: Vec<FluidCatalogItem>,
    materials: Vec<MaterialCatalogItem>,
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
struct CatalogLink {
    relation: String,
    from: String,
    to: String,
}

#[derive(Debug, Clone, Serialize)]
struct FluidCatalogItem {
    key: String,
    name: String,
    aliases: Vec<String>,
    supported_state_inputs: Vec<String>,
    supported_properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct MaterialCatalogItem {
    key: String,
    name: String,
    aliases: Vec<String>,
    description: String,
    source: String,
    properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyCatalogItem {
    key: String,
    name: String,
    description: String,
    canonical_law: String,
    canonical_equation: String,
    route: String,
    variants: Vec<FamilyVariantCatalogItem>,
}

#[derive(Debug, Clone, Serialize)]
struct FamilyVariantCatalogItem {
    key: String,
    name: String,
    equation_id: String,
    display_latex: String,
    when_to_use: String,
    route: String,
    equation_route: String,
}

#[derive(Debug, Clone)]
struct UnifiedDocsContribution {
    equations: EquationDocsContribution,
    families: Vec<equations::equation_families::EquationFamilyDef>,
    fluids: Vec<eng_fluids::FluidDocsEntry>,
    materials: Vec<eng_materials::MaterialDocsEntry>,
}

const EXPORT_SCHEMA_VERSION: &str = "1";
const EXPORT_MODEL_VERSION: &str = "2026-03-03";

const SNIPPET_TOP_LEVEL_IMPORT: &str = include_str!("../docs_snippets/top_level_import.rs");
const SNIPPET_SIMPLE_EQUATION_SOLVE: &str =
    include_str!("../docs_snippets/simple_equation_solve.rs");
const SNIPPET_TYPED_UNIT_INPUT: &str = include_str!("../docs_snippets/typed_unit_input.rs");
const SNIPPET_QTY_MACRO_INPUT: &str = include_str!("../docs_snippets/qty_macro_input.rs");
const SNIPPET_RUNTIME_STRING_INPUT: &str =
    include_str!("../docs_snippets/runtime_string_input.rs");
const SNIPPET_FLUID_PROPERTY_LOOKUP: &str =
    include_str!("../docs_snippets/fluid_property_lookup.rs");
const SNIPPET_MATERIAL_PROPERTY_LOOKUP: &str =
    include_str!("../docs_snippets/material_property_lookup.rs");
const SNIPPET_CONTEXT_SOLVE: &str = include_str!("../docs_snippets/context_solve.rs");
const SNIPPET_FAMILY_VARIANT_ACCESS: &str =
    include_str!("../docs_snippets/family_variant_access.rs");

#[derive(Debug, Clone)]
pub struct MdBookPaths {
    pub source_dir: PathBuf,
    pub html_dir: PathBuf,
    pub html_index: PathBuf,
}

#[derive(Debug, Clone)]
pub enum HtmlBuildStatus {
    Built,
    MdBookNotInstalled { message: String },
}

#[derive(Debug, Clone)]
pub struct HtmlExportReport {
    pub paths: MdBookPaths,
    pub status: HtmlBuildStatus,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn workspace_root() -> PathBuf {
    crate_root()
        .parent()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or_else(crate_root)
}

/// Default unified generated output root (`<workspace>/generated`).
pub fn default_generated_root() -> PathBuf {
    workspace_root().join("generated")
}

/// Default unified book source root (`<workspace>/generated/book`).
pub fn default_book_root() -> PathBuf {
    default_generated_root().join("book")
}

/// Default unified PDF handbook output path (`<workspace>/generated/engineering_handbook.pdf`).
pub fn default_pdf_path() -> PathBuf {
    default_generated_root().join("engineering_handbook.pdf")
}

/// Load and validate the default equations registry used by the unified system.
pub fn load_registry() -> Result<Registry> {
    let registry = Registry::load_default()?;
    registry.validate()?;
    Ok(registry)
}

/// Load and validate a registry from a custom directory.
pub fn load_registry_from_dir(registry_dir: impl AsRef<Path>) -> Result<Registry> {
    let registry = Registry::load_from_dir(registry_dir)?;
    registry.validate()?;
    Ok(registry)
}

/// Export unified docs/search/catalog artifacts to the default output root.
pub fn export_unified_docs() -> Result<PathBuf> {
    let out_dir = default_generated_root();
    export_unified_docs_to(&out_dir)?;
    Ok(out_dir)
}

/// Export unified docs/search/catalog artifacts to a custom output root.
pub fn export_unified_docs_to(out_dir: impl AsRef<Path>) -> Result<()> {
    let contribution = gather_contribution(load_registry()?)?;
    write_unified_json_artifacts(&contribution, out_dir.as_ref())
}

/// Export unified mdBook source to the default book root.
pub fn export_unified_mdbook() -> Result<MdBookPaths> {
    let out_dir = default_book_root();
    export_unified_mdbook_to(&out_dir)
}

/// Export unified mdBook source to a custom book root.
pub fn export_unified_mdbook_to(out_dir: impl AsRef<Path>) -> Result<MdBookPaths> {
    let contribution = gather_contribution(load_registry()?)?;
    generate_mdbook_source(&contribution, out_dir.as_ref())?;
    Ok(MdBookPaths {
        source_dir: out_dir.as_ref().to_path_buf(),
        html_dir: out_dir.as_ref().join("book"),
        html_index: out_dir.as_ref().join("book").join("index.html"),
    })
}

/// Export unified HTML docs (mdBook source + build when available) to the default book root.
pub fn export_unified_html() -> Result<HtmlExportReport> {
    let out_dir = default_book_root();
    export_unified_html_to(&out_dir)
}

/// Export unified HTML docs (mdBook source + build when available) to a custom book root.
pub fn export_unified_html_to(out_dir: impl AsRef<Path>) -> Result<HtmlExportReport> {
    let paths = export_unified_mdbook_to(out_dir)?;
    let status = match run_mdbook_build(&paths.source_dir) {
        Ok(()) => {
            mirror_directory(&paths.html_dir, &default_generated_root().join("html"))?;
            HtmlBuildStatus::Built
        }
        Err(MdBookBuildError::NotInstalled) => HtmlBuildStatus::MdBookNotInstalled {
            message: missing_mdbook_message(),
        },
        Err(MdBookBuildError::BuildFailed(detail)) => {
            return Err(EquationError::Validation(format!(
                "mdbook build failed for {}: {detail}",
                paths.source_dir.display()
            )));
        }
    };
    Ok(HtmlExportReport { paths, status })
}

/// Export unified PDF handbook to the default output file.
pub fn export_unified_pdf() -> Result<PathBuf> {
    let out_file = default_pdf_path();
    export_unified_pdf_to(&out_file)?;
    Ok(out_file)
}

/// Export unified PDF handbook to a custom output file.
pub fn export_unified_pdf_to(out_file: impl AsRef<Path>) -> Result<()> {
    let contribution = gather_contribution(load_registry()?)?;
    render_pdf(&contribution, out_file.as_ref())
}

/// Export unified docs artifacts and return the canonical catalog path.
pub fn export_unified_catalog() -> Result<PathBuf> {
    let out_dir = export_unified_docs()?;
    Ok(out_dir.join("catalog.json"))
}

fn gather_contribution(registry: Registry) -> Result<UnifiedDocsContribution> {
    let equations = build_equation_docs_contribution(registry.equations());
    let fluids = eng_fluids::docs_entries();
    let materials =
        eng_materials::docs_entries().map_err(|e| EquationError::Validation(e.to_string()))?;
    Ok(UnifiedDocsContribution {
        equations,
        families: equations::equation_families::load_default_validated(registry.equations())?,
        fluids,
        materials,
    })
}

fn write_unified_json_artifacts(c: &UnifiedDocsContribution, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir).map_err(|source| EquationError::Io {
        path: out_dir.to_path_buf(),
        source,
    })?;

    write_json(
        out_dir.join("search_index.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "search_index",
            items: c.equations.search_index.clone(),
        },
    )?;
    write_json(
        out_dir.join("page_models.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "page_models",
            items: c.equations.page_models.clone(),
        },
    )?;
    write_json(
        out_dir.join("navigation.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "navigation",
            items: c.equations.navigation.clone(),
        },
    )?;
    write_json(
        out_dir.join("examples_index.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "examples_index",
            items: c.equations.examples_index.clone(),
        },
    )?;
    write_json(
        out_dir.join("constants.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "constants",
            items: c.equations.constants.clone(),
        },
    )?;
    write_json(
        out_dir.join("families.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "families",
            items: c.families.clone(),
        },
    )?;
    write_json(
        out_dir.join("fluids.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "fluids",
            items: c
                .fluids
                .iter()
                .map(|f| FluidCatalogItem {
                    key: f.key.clone(),
                    name: f.name.clone(),
                    aliases: f.aliases.clone(),
                    supported_state_inputs: f.supported_state_inputs.clone(),
                    supported_properties: f.supported_properties.clone(),
                })
                .collect::<Vec<_>>(),
        },
    )?;
    write_json(
        out_dir.join("materials.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "materials",
            items: c
                .materials
                .iter()
                .map(|m| MaterialCatalogItem {
                    key: m.key.clone(),
                    name: m.name.clone(),
                    aliases: m.aliases.clone(),
                    description: m.description.clone(),
                    source: m.source.clone(),
                    properties: m.properties.clone(),
                })
                .collect::<Vec<_>>(),
        },
    )?;
    write_json(
        out_dir.join("catalog.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "catalog",
            items: build_unified_catalog(c),
        },
    )?;
    write_json(
        out_dir.join("architecture_spec.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "architecture_spec",
            items: architecture_spec(),
        },
    )?;
    Ok(())
}

fn build_unified_catalog(c: &UnifiedDocsContribution) -> UnifiedCatalog {
    let equations = c
        .equations
        .page_models
        .iter()
        .map(|p| UnifiedEquationEntry {
            key: p.key.clone(),
            path_id: p.path_id.clone(),
            name: p.name.clone(),
            category: p.category.clone(),
            subcategories: p.subcategories.clone(),
            default_target: p.default_target.clone(),
            uses_constants: p.uses_constants.iter().map(|u| u.key.clone()).collect(),
            resolver_contexts: collect_resolver_contexts(p),
        })
        .collect::<Vec<_>>();

    let mut links = Vec::new();
    for p in &c.equations.page_models {
        for u in &p.uses_constants {
            links.push(CatalogLink {
                relation: "equation_uses_constant".to_string(),
                from: p.path_id.clone(),
                to: u.key.clone(),
            });
        }
        for ctx in collect_resolver_contexts(p) {
            links.push(CatalogLink {
                relation: "equation_uses_context".to_string(),
                from: p.path_id.clone(),
                to: ctx,
            });
        }
    }
    for family in &c.families {
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
    links.sort_by(|a, b| (&a.relation, &a.from, &a.to).cmp(&(&b.relation, &b.from, &b.to)));

    UnifiedCatalog {
        equations,
        families: c
            .families
            .iter()
            .map(|f| FamilyCatalogItem {
                key: f.key.clone(),
                name: f.name.clone(),
                description: f.description.clone(),
                canonical_law: f.canonical_law.clone(),
                canonical_equation: f.canonical_equation.clone(),
                route: doc_routes::family_doc_path(&f.key),
                variants: f
                    .variants
                    .iter()
                    .map(|v| FamilyVariantCatalogItem {
                        key: v.key.clone(),
                        name: v.name.clone(),
                        equation_id: v.equation_id.clone(),
                        display_latex: v.display_latex.clone(),
                        when_to_use: v.when_to_use.clone(),
                        route: format!("{}.{}", f.key, v.key),
                        equation_route: doc_routes::equation_doc_path_from_path_id(&v.equation_id),
                    })
                    .collect(),
            })
            .collect(),
        constants: c.equations.constants.clone(),
        fluids: c
            .fluids
            .iter()
            .map(|f| FluidCatalogItem {
                key: f.key.clone(),
                name: f.name.clone(),
                aliases: f.aliases.clone(),
                supported_state_inputs: f.supported_state_inputs.clone(),
                supported_properties: f.supported_properties.clone(),
            })
            .collect(),
        materials: c
            .materials
            .iter()
            .map(|m| MaterialCatalogItem {
                key: m.key.clone(),
                name: m.name.clone(),
                aliases: m.aliases.clone(),
                description: m.description.clone(),
                source: m.source.clone(),
                properties: m.properties.clone(),
            })
            .collect(),
        links,
    }
}

fn collect_resolver_contexts(page: &EquationPageModel) -> Vec<String> {
    let mut s: BTreeSet<String> = page
        .variables
        .iter()
        .filter_map(|v| v.resolver_source.clone())
        .collect();
    s.retain(|x| !x.trim().is_empty());
    s.into_iter().collect()
}

fn generate_mdbook_source(c: &UnifiedDocsContribution, book_root: &Path) -> Result<()> {
    if book_root.exists() {
        fs::remove_dir_all(book_root).map_err(|source| EquationError::Io {
            path: book_root.to_path_buf(),
            source,
        })?;
    }
    let src = book_root.join("src");
    fs::create_dir_all(&src).map_err(|source| EquationError::Io {
        path: src.clone(),
        source,
    })?;

    write_text(
        book_root.join("book.toml"),
        r#"[book]
title = "Engineering Handbook"
authors = ["eng-tools"]
language = "en"
src = "src"

[output.html]
mathjax-support = true
"#,
    )?;

    write_text(src.join("index.md"), &render_home(c))?;
    write_text(
        src.join("getting_started/index.md"),
        &render_getting_started(),
    )?;
    write_text(src.join("input_styles/index.md"), &render_input_styles())?;
    write_text(
        src.join("units_quantities/index.md"),
        &render_units_quantities(),
    )?;
    write_text(
        src.join("architecture/index.md"),
        &render_architecture_page(),
    )?;
    write_text(src.join("workflows/index.md"), &render_workflows())?;
    write_text(src.join("yaml_authoring/index.md"), &render_yaml_authoring())?;
    write_text(src.join("validation_trust/index.md"), &render_validation_trust())?;
    write_text(src.join("equations/guide.md"), &render_equations_guide())?;
    write_text(src.join("equations/families/index.md"), &render_families_index(c))?;
    write_text(src.join("constants/index.md"), &render_constants_index(c))?;
    write_text(src.join("fluids/guide.md"), &render_fluids_guide())?;
    write_text(src.join("fluids/index.md"), &render_fluids_index(c))?;
    write_text(src.join("materials/guide.md"), &render_materials_guide())?;
    write_text(src.join("materials/index.md"), &render_materials_index(c))?;
    write_text(src.join("equations/index.md"), &render_equations_index(c))?;

    for k in &c.equations.constants {
        write_text(
            src.join("constants").join(format!("{}.md", k.key)),
            &render_constant_page(k),
        )?;
    }
    for fam in &c.families {
        write_text(
            src.join("equations")
                .join("families")
                .join(format!("{}.md", snake_case(&fam.key))),
            &render_family_page(fam),
        )?;
    }
    for f in &c.fluids {
        write_text(
            src.join("fluids")
                .join(format!("{}.md", snake_case(&f.key))),
            &render_fluid_page(f),
        )?;
    }
    for m in &c.materials {
        write_text(
            src.join("materials")
                .join(format!("{}.md", snake_case(&m.key))),
            &render_material_page(m),
        )?;
    }
    write_equation_pages(c, &src)?;
    write_text(src.join("SUMMARY.md"), &render_summary(c))?;
    write_text(src.join("README.md"), &render_mdbook_readme())?;
    Ok(())
}

fn write_equation_pages(c: &UnifiedDocsContribution, src: &Path) -> Result<()> {
    for cat in &c.equations.library.categories {
        write_text(
            src.join("equations").join(&cat.name).join("index.md"),
            &render_category_index(cat),
        )?;
        for eq in &cat.root_equations {
            write_text(
                src.join("equations")
                    .join(&cat.name)
                    .join(format!("{}.md", eq.slug)),
                &render_equation_page(&eq.page, &c.families),
            )?;
        }
        for sub in &cat.subcategories {
            write_text(
                src.join("equations")
                    .join(&cat.name)
                    .join(&sub.name)
                    .join("index.md"),
                &render_subcategory_index(cat, sub),
            )?;
            for eq in &sub.equations {
                write_text(
                    src.join("equations")
                        .join(&cat.name)
                        .join(&sub.name)
                        .join(format!("{}.md", eq.slug)),
                    &render_equation_page(&eq.page, &c.families),
                )?;
            }
        }
    }
    Ok(())
}

fn render_home(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Engineering Handbook\n\n");
    md.push_str("Docs are the primary user interface for this library. Every core workflow example shown here is backed by compile-checked and/or executed test coverage.\n\n");
    md.push_str("## Start Here\n\n");
    md.push_str("- [Getting Started](getting_started/index.md)\n");
    md.push_str("- [Input Styles](input_styles/index.md)\n");
    md.push_str("- [Units & Quantities](units_quantities/index.md)\n");
    md.push_str("- [Examples & Workflows](workflows/index.md)\n\n");
    md.push_str("## Domain Guides\n\n");
    md.push_str("- [Equations Guide](equations/guide.md)\n");
    md.push_str("- [Fluids Guide](fluids/guide.md)\n");
    md.push_str("- [Materials Guide](materials/guide.md)\n");
    md.push_str("- [Constants](constants/index.md)\n");
    md.push_str("- [YAML Authoring](yaml_authoring/index.md)\n");
    md.push_str("- [Validation / Trust](validation_trust/index.md)\n");
    md.push_str("- [Architecture Overview](architecture/index.md)\n\n");
    md.push_str("## Catalog\n\n");
    md.push_str("- [Equations](equations/index.md)\n");
    md.push_str("- [Equation Families](equations/families/index.md)\n");
    md.push_str("- [Fluids](fluids/index.md)\n");
    md.push_str("- [Materials](materials/index.md)\n");
    md.push_str(&format!(
        "\n**Library size:** {} equations, {} constants, {} fluids, {} materials.\n",
        c.equations.page_models.len(),
        c.equations.constants.len(),
        c.fluids.len(),
        c.materials.len()
    ));
    md
}

fn render_getting_started() -> String {
    let mut md = String::new();
    md.push_str("# Getting Started\n\n");
    md.push_str("The top-level crate is `eng`. It re-exports equations, fluids, materials, constants, units, and docs export APIs.\n\n");
    md.push_str("## Dependencies\n\n");
    md.push_str("If `eng` is published, add:\n\n");
    md.push_str("```toml\n");
    md.push_str("[dependencies]\neng = \"0.1\"\n");
    md.push_str("```\n\n");
    md.push_str("For local workspace use from the generated handbook root (`generated/book`):\n\n");
    md.push_str("```toml\n");
    md.push_str("[dependencies]\neng = { path = \"../../crates/eng\" }\n");
    md.push_str("```\n\n");
    md.push_str("Run locally from this repo:\n\n");
    md.push_str("```bash\n");
    md.push_str("cargo run -p eng --example unified_usage\n");
    md.push_str("cargo test -p eng core_handbook_workflows_execute\n");
    md.push_str("```\n\n");
    md.push_str("## Primary Imports\n\n```rust\n");
    md.push_str(SNIPPET_TOP_LEVEL_IMPORT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## First Successful Solve\n\n```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("Next: [Input Styles](../input_styles/index.md), [Equations Guide](../equations/guide.md), and [Examples & Workflows](../workflows/index.md).\n");
    md
}

fn render_input_styles() -> String {
    let mut md = String::new();
    md.push_str("# Input Styles\n\n");
    md.push_str("Use these input styles in order of preference:\n\n");
    md.push_str("1. plain numeric SI (`f64`) for fastest path\n");
    md.push_str("2. typed unit constructors for explicit units in Rust\n");
    md.push_str("3. `qty!(\"...\")` for compile-time parsed literal expressions\n");
    md.push_str("4. runtime strings for boundary input (CLI/UI/import)\n\n");

    md.push_str("## Plain Numeric (Canonical SI)\n\n```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Typed Unit Constructors\n\n```rust\n");
    md.push_str(SNIPPET_TYPED_UNIT_INPUT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## `qty!` Expressions\n\n```rust\n");
    md.push_str(SNIPPET_QTY_MACRO_INPUT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Runtime String Expressions\n\n```rust\n");
    md.push_str(SNIPPET_RUNTIME_STRING_INPUT.trim());
    md.push_str("\n```\n\n");
    md
}

fn render_units_quantities() -> String {
    let mut md = String::new();
    md.push_str("# Units & Quantities\n\n");
    md.push_str("The unit engine is dimension-based. Equivalent expressions reduce to canonical SI using dimensional algebra.\n\n");
    md.push_str("Supported expression operators: `+`, `-`, `*`, `/`, parentheses.\n\n");
    md.push_str("- Addition/subtraction require same dimensions.\n");
    md.push_str("- Multiplication/division combine dimensions algebraically.\n");
    md.push_str("- Bare numbers are dimensionless.\n");
    md.push_str("- Input is validated against each variable's expected dimension.\n\n");
    md.push_str("Temperature note: affine temperature pitfalls are intentionally guarded; use canonical absolute temperature where required.\n");
    md
}

fn render_equations_guide() -> String {
    let mut md = String::new();
    md.push_str("# Equations Guide\n\n");
    md.push_str("Atomic equations are scalar-first relations with strong validation and tests.\n\n");
    md.push_str("- [Equation Catalog](./index.md)\n");
    md.push_str("- [Equation Families](./families/index.md)\n\n");
    md.push_str("Family pages and variant pages are equation-scoped and cross-linked from each equation page.\n");
    md
}

fn render_fluids_guide() -> String {
    let mut md = String::new();
    md.push_str("# Fluids Guide\n\n");
    md.push_str("Fluids are catalog-backed with typed wrappers and state builders.\n\n");
    md.push_str("## Property Lookup\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n\n");
    md.push_str("Use fluid states directly in context solves:\n\n```rust\n");
    md.push_str(SNIPPET_CONTEXT_SOLVE.trim());
    md.push_str("\n```\n");
    md
}

fn render_materials_guide() -> String {
    let mut md = String::new();
    md.push_str("# Materials Guide\n\n");
    md.push_str("Materials provide temperature-conditioned property lookup from curated datasets.\n\n");
    md.push_str("## Property Lookup\n\n```rust\n");
    md.push_str(SNIPPET_MATERIAL_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n");
    md
}

fn render_yaml_authoring() -> String {
    let mut md = String::new();
    md.push_str("# YAML Authoring\n\n");
    md.push_str("Equation YAML files live under `crates/equations/registry/<category>/`. Family YAML files live under `crates/equations/registry/families/`.\n\n");
    md.push_str("## Minimal Equation Shape\n\n");
    md.push_str("```yaml\n");
    md.push_str("key: hoop_stress\n");
    md.push_str("taxonomy:\n");
    md.push_str("  category: structures\n");
    md.push_str("name: Thin-Wall Hoop Stress\n");
    md.push_str("display:\n");
    md.push_str("  latex: \"\\\\sigma_h = \\\\frac{P r}{t}\"\n");
    md.push_str("variables:\n");
    md.push_str("  sigma_h: { dimension: stress, default_unit: Pa }\n");
    md.push_str("  P: { dimension: pressure, default_unit: Pa }\n");
    md.push_str("  r: { dimension: length, default_unit: m }\n");
    md.push_str("  t: { dimension: length, default_unit: m }\n");
    md.push_str("residual:\n");
    md.push_str("  expression: \"sigma_h - P*r/t\"\n");
    md.push_str("tests:\n");
    md.push_str("  baseline:\n");
    md.push_str("    sigma_h: \"62.5 MPa\"\n");
    md.push_str("    P: \"2.5 MPa\"\n");
    md.push_str("    r: \"0.2 m\"\n");
    md.push_str("    t: \"8 mm\"\n");
    md.push_str("```\n\n");
    md.push_str("## Family YAML Shape\n\n");
    md.push_str("```yaml\n");
    md.push_str("key: ideal_gas\n");
    md.push_str("name: Ideal Gas Law\n");
    md.push_str("canonical_equation: thermo.ideal_gas.mass_volume\n");
    md.push_str("variants:\n");
    md.push_str("  - key: mass_volume\n");
    md.push_str("    equation_id: thermo.ideal_gas.mass_volume\n");
    md.push_str("  - key: density\n");
    md.push_str("    equation_id: thermo.ideal_gas.density\n");
    md.push_str("```\n\n");
    md.push_str("Authoring workflow:\n");
    md.push_str("1. add/update equation YAML\n");
    md.push_str("2. add baseline tests in YAML\n");
    md.push_str("3. run validation/tests\n");
    md.push_str("4. regenerate docs/catalog/html exports\n");
    md
}

fn render_validation_trust() -> String {
    let mut md = String::new();
    md.push_str("# Validation / Trust\n\n");
    md.push_str("Trust is enforced by registry validation, solver tests, generated-link checks, and docs export verification.\n\n");
    md.push_str("- Equation and family schema/consistency validation\n");
    md.push_str("- Core solver/path tests (SI, typed, `qty!`, runtime string)\n");
    md.push_str("- mdBook link integrity tests\n");
    md.push_str("- Unified export verification script\n\n");
    md.push_str("Docs examples for core workflows are backed by executable tests.\n");
    md
}

fn render_architecture_page() -> String {
    let spec = architecture_spec();
    let mut md = String::new();
    md.push_str("# Architecture Layers\n\n");
    md.push_str(
        "This chapter defines strict ownership boundaries for work after atomic equations.\n\n",
    );
    md.push_str("## Layer Definitions\n\n");
    md.push_str("1. **Atomic Equation**: one physical relation with scalar-first solve behavior and equation-level tests/docs.\n");
    md.push_str("2. **Equation Family / Variants**: one canonical law with multiple discoverable forms without duplicating solver logic.\n");
    md.push_str("3. **Component Model**: multi-equation iterative engineering model using contexts (fluid/material) as needed.\n");
    md.push_str("4. **Solve Graph / Chaining**: node/edge orchestration connecting equations, components, constants, and property sources.\n");
    md.push_str("5. **External Bindings**: generated Python/Excel surfaces over Rust-owned implementations.\n\n");

    md.push_str("## Ownership Map\n\n");
    md.push_str("| Layer | Owner | Owns | Does not own |\n");
    md.push_str("| --- | --- | --- | --- |\n");
    for r in &spec.ownership {
        md.push_str(&format!(
            "| `{:?}` | `{:?}` | {} | {} |\n",
            r.layer,
            r.owner,
            r.owns.join("; "),
            r.does_not_own.join("; ")
        ));
    }
    md.push('\n');

    md.push_str("## Belongs Here / Not Here Rules\n\n");
    for rule in &spec.boundaries {
        md.push_str(&format!("### `{:?}`\n\n", rule.layer));
        md.push_str("Belongs here:\n");
        for item in &rule.belongs_here {
            md.push_str(&format!("- {}\n", item));
        }
        md.push_str("\nDoes not belong here:\n");
        for item in &rule.does_not_belong_here {
            md.push_str(&format!("- {}\n", item));
        }
        md.push('\n');
    }

    md.push_str("## Prototype: Equation Family (Ideal Gas)\n\n");
    md.push_str(&format!(
        "- Key: `{}`\n- Canonical relation: `{}`\n- Canonical equation path: `{}`\n",
        spec.family_prototype.key,
        spec.family_prototype.canonical_relation,
        spec.family_prototype.canonical_equation_path
    ));
    md.push_str("- Variants:\n");
    for v in &spec.family_prototype.variants {
        md.push_str(&format!(
            "  - `{}` ({}) target `{}`: {}\n",
            v.key, v.display_name, v.target, v.intended_usage
        ));
    }
    md.push('\n');

    md.push_str("## Prototype: Component Model (Two Orifice)\n\n");
    md.push_str(&format!(
        "- Key: `{}`\n- Requires contexts: {}\n- Depends on equations: {}\n",
        spec.component_prototype.key,
        spec.component_prototype.required_contexts.join(", "),
        spec.component_prototype.equation_dependencies.join(", ")
    ));
    for n in &spec.component_prototype.notes {
        md.push_str(&format!("- {}\n", n));
    }
    md.push('\n');

    md.push_str("## Solve Graph Model (Planned)\n\n");
    md.push_str(&format!(
        "- Node kinds: {}\n- Edge semantics: {}\n",
        spec.graph_prototype.node_kinds.join(", "),
        spec.graph_prototype.edge_semantics.join("; ")
    ));
    for n in &spec.graph_prototype.notes {
        md.push_str(&format!("- {}\n", n));
    }
    md.push('\n');

    md.push_str("## External Binding Plan (Python/Excel)\n\n");
    md.push_str(&format!(
        "- Targets: {}\n- Authoritative runtime: {}\n- Generated from: {}\n",
        spec.binding_plan.targets.join(", "),
        spec.binding_plan.authoritative_runtime,
        spec.binding_plan.generated_from.join("; ")
    ));
    md.push_str("- Naming rules:\n");
    for n in &spec.binding_plan.naming_rules {
        md.push_str(&format!("  - {}\n", n));
    }
    md.push_str("- Notes:\n");
    for n in &spec.binding_plan.notes {
        md.push_str(&format!("  - {}\n", n));
    }
    md.push('\n');

    md.push_str("## Catalog Evolution Plan\n\n");
    md.push_str(&format!(
        "- New sections: {}\n- Required links: {}\n",
        spec.catalog_plan.new_sections.join(", "),
        spec.catalog_plan.required_links.join(", ")
    ));
    md.push_str("\nThe machine-readable form of this chapter is exported as `generated/architecture_spec.json`.\n");
    md
}

fn render_workflows() -> String {
    let mut md = String::new();
    md.push_str("# Examples & Workflows\n\n");
    md.push_str("These examples are sourced from verified snippets and corresponding tests.\n\n");

    md.push_str("## 1. Simple Equation Solve\n\n```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 2. Typed Unit Solve\n\n```rust\n");
    md.push_str(SNIPPET_TYPED_UNIT_INPUT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 3. `qty!` Solve\n\n```rust\n");
    md.push_str(SNIPPET_QTY_MACRO_INPUT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 4. Runtime String Solve\n\n```rust\n");
    md.push_str(SNIPPET_RUNTIME_STRING_INPUT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 5. Fluid-Assisted Solve\n\n```rust\n");
    md.push_str(SNIPPET_CONTEXT_SOLVE.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 6. Family Variant Solve\n\n```rust\n");
    md.push_str(SNIPPET_FAMILY_VARIANT_ACCESS.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 7. Direct Fluid/Material Property Lookup\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n\n```rust\n");
    md.push_str(SNIPPET_MATERIAL_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n");
    md
}

fn render_families_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Equation Families\n\n");
    if c.families.is_empty() {
        md.push_str("No equation families are currently defined.\n");
        return md;
    }
    md.push_str("Families group common forms of the same physical law without duplicating conceptual identity.\n\n");
    for f in &c.families {
        md.push_str(&format!(
            "- [{}](./{}.md): canonical `{}` ({} variants)\n",
            f.name,
            snake_case(&f.key),
            f.canonical_equation,
            f.variants.len()
        ));
    }
    md
}

fn render_family_page(f: &equations::equation_families::EquationFamilyDef) -> String {
    let mut md = String::new();
    let family_doc = doc_routes::family_doc_path(&f.key);
    md.push_str(&format!("# {}\n\n", f.name));
    md.push_str(&format!("**Family key:** `{}`\n\n", f.key));
    if !f.description.trim().is_empty() {
        md.push_str(&format!("{}\n\n", f.description.trim()));
    }
    md.push_str(&format!("**Canonical law:** `{}`\n\n", f.canonical_law));
    md.push_str(&format!(
        "**Canonical equation:** [`{}`]({})\n\n",
        f.canonical_equation,
        doc_routes::relative_doc_link(
            &family_doc,
            &doc_routes::equation_doc_path_from_path_id(&f.canonical_equation)
        )
    ));
    if !f.assumptions.is_empty() {
        md.push_str("## Shared Assumptions\n\n");
        for a in &f.assumptions {
            md.push_str(&format!("- {}\n", a));
        }
        md.push('\n');
    }
    if !f.references.is_empty() {
        md.push_str("## Shared References\n\n");
        for r in &f.references {
            md.push_str(&format!("- {}\n", r));
        }
        md.push('\n');
    }
    md.push_str("## Variants\n\n");
    md.push_str("| Variant | Equation | Display | Use when | Notes |\n");
    md.push_str("| --- | --- | --- | --- | --- |\n");
    for v in &f.variants {
        let equation_link = doc_routes::relative_doc_link(
            &family_doc,
            &doc_routes::equation_doc_path_from_path_id(&v.equation_id),
        );
        md.push_str(&format!(
            "| `{}` | [`{}`]({}) | \\({}\\) | {} | {} |\n",
            v.key,
            v.equation_id,
            equation_link,
            v.display_latex,
            if v.when_to_use.trim().is_empty() {
                "-"
            } else {
                v.when_to_use.trim()
            },
            if v.description.trim().is_empty() {
                "-"
            } else {
                v.description.trim()
            }
        ));
    }
    md
}

fn render_constants_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Constants\n\n");
    for k in &c.equations.constants {
        md.push_str(&format!(
            "- [{}](./{}.md): `{}`\n",
            k.name, k.key, k.symbol_unicode
        ));
    }
    md
}

fn render_fluids_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Fluids\n\n");
    for f in &c.fluids {
        md.push_str(&format!("- [{}](./{}.md)\n", f.name, snake_case(&f.key)));
    }
    md
}

fn render_materials_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Materials\n\n");
    for m in &c.materials {
        md.push_str(&format!("- [{}](./{}.md)\n", m.name, snake_case(&m.key)));
    }
    md
}

fn render_equations_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Equation Catalog\n\n");
    md.push_str("- [Guide](./guide.md)\n");
    md.push_str("- [Families](./families/index.md)\n\n");
    for cat in &c.equations.library.categories {
        md.push_str(&format!(
            "- [{}](./{}/index.md)\n",
            title_case(&cat.name),
            cat.name
        ));
    }
    md
}

fn render_constant_page(c: &EngineeringConstant) -> String {
    format!(
        "# {}\n\n- Key: `{}`\n- Symbol: \\({}\\)\n- Value: `{}` `{}`\n- Source: {}\n\n{}\n",
        c.name, c.key, c.symbol_latex, c.value, c.unit, c.source, c.note
    )
}

fn render_fluid_page(f: &eng_fluids::FluidDocsEntry) -> String {
    let aliases = if f.aliases.is_empty() {
        "none".to_string()
    } else {
        f.aliases.join(", ")
    };
    format!(
        "# {}\n\n- Key: `{}`\n- Aliases: {}\n- State inputs: {}\n- Supported properties: {}\n",
        f.name,
        f.key,
        aliases,
        f.supported_state_inputs.join(", "),
        f.supported_properties.join(", ")
    )
}

fn render_material_page(m: &eng_materials::MaterialDocsEntry) -> String {
    let aliases = if m.aliases.is_empty() {
        "none".to_string()
    } else {
        m.aliases.join(", ")
    };
    format!(
        "# {}\n\n- Key: `{}`\n- Aliases: {}\n- Source: {}\n- Properties: {}\n\n{}\n",
        m.name,
        m.key,
        aliases,
        m.source,
        m.properties.join(", "),
        m.description
    )
}

fn render_category_index(cat: &CategoryPresentation) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", title_case(&cat.name)));
    for eq in &cat.root_equations {
        md.push_str(&format!("- [{}](./{}.md)\n", eq.page.name, eq.slug));
    }
    for sub in &cat.subcategories {
        md.push_str(&format!(
            "- [{}](./{}/index.md)\n",
            title_case(&sub.name),
            sub.name
        ));
    }
    md
}

fn render_subcategory_index(cat: &CategoryPresentation, sub: &SubcategoryPresentation) -> String {
    let mut md = String::new();
    md.push_str(&format!(
        "# {} / {}\n\n",
        title_case(&cat.name),
        title_case(&sub.name)
    ));
    for eq in &sub.equations {
        md.push_str(&format!("- [{}](./{}.md)\n", eq.page.name, eq.slug));
    }
    md
}

fn render_equation_page(
    p: &EquationPageModel,
    families: &[equations::equation_families::EquationFamilyDef],
) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", p.name));
    md.push_str(&format!("**Path ID:** `{}`\n\n", p.path_id));
    md.push_str(&format!("\\[\n{}\n\\]\n\n", p.latex));
    md.push_str(&format!("- Unicode: `{}`\n", p.unicode));
    md.push_str(&format!("- ASCII: `{}`\n\n", p.ascii));

    md.push_str("## Variables\n\n");
    md.push_str(
        "<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>\n",
    );
    for v in &p.variables {
        md.push_str(&format!(
            "<tr><td><code>{}</code></td><td>{}</td><td>\\({}\\)</td><td><code>{}</code></td><td><code>{}</code></td></tr>\n",
            v.key, v.name, v.symbol, v.dimension, v.default_unit
        ));
    }
    md.push_str("</tbody></table>\n\n");

    if !p.assumptions.is_empty() {
        md.push_str("## Assumptions\n\n");
        for a in &p.assumptions {
            md.push_str(&format!("- {}\n", a));
        }
        md.push('\n');
    }
    if let Some((family, variant)) = equations::equation_families::family_by_equation_path_id(
        families,
        &p.path_id,
    ) {
        let from = doc_routes::equation_doc_path_from_path_id(&p.path_id);
        let to = doc_routes::family_doc_path(&family.key);
        let family_link = doc_routes::relative_doc_link(&from, &to);
        md.push_str("## Family\n\n");
        md.push_str(&format!(
            "- Family: [`{}`]({})\n- Variant: `{}` (`{}`)\n",
            family.name, family_link, variant.name, variant.key
        ));
        if !variant.when_to_use.trim().is_empty() {
            md.push_str(&format!("- Use when: {}\n", variant.when_to_use.trim()));
        }
        md.push('\n');
    }
    if !p.uses_constants.is_empty() {
        let from = doc_routes::equation_doc_path_from_path_id(&p.path_id);
        md.push_str("## Constants Used\n\n");
        for c in &p.uses_constants {
            let to = doc_routes::constant_doc_path(&c.key);
            let link = doc_routes::relative_doc_link(&from, &to);
            md.push_str(&format!(
                "- [`{}`]({}) ({}) \\({}\\)\n",
                c.key, link, c.name, c.symbol_latex
            ));
        }
        md.push('\n');
    }
    if !p.examples.is_empty() {
        md.push_str("## Examples\n\n");
        for ex in &p.examples {
            md.push_str(&format!(
                "### {}\n\n```rust\n{}\n```\n\n",
                ex.label, ex.code
            ));
        }
    }
    md
}

fn render_summary(c: &UnifiedDocsContribution) -> String {
    let mut s = String::new();
    s.push_str("# Summary\n\n");
    s.push_str("- [Home](index.md)\n");
    s.push_str("- [Getting Started](getting_started/index.md)\n");
    s.push_str("- [Input Styles](input_styles/index.md)\n");
    s.push_str("- [Units & Quantities](units_quantities/index.md)\n");
    s.push_str("- [Examples & Workflows](workflows/index.md)\n");
    s.push_str("- [YAML Authoring](yaml_authoring/index.md)\n");
    s.push_str("- [Validation / Trust](validation_trust/index.md)\n");
    s.push_str("- [Architecture Layers](architecture/index.md)\n");

    s.push_str("- [Equations Guide](equations/guide.md)\n");
    s.push_str("- [Equation Catalog](equations/index.md)\n");
    for cat in &c.equations.library.categories {
        s.push_str(&format!(
            "  - [{}](equations/{}/index.md)\n",
            title_case(&cat.name),
            cat.name
        ));
        for eq in &cat.root_equations {
            s.push_str(&format!(
                "    - [{}](equations/{}/{}.md)\n",
                eq.page.name, cat.name, eq.slug
            ));
        }
        for sub in &cat.subcategories {
            s.push_str(&format!(
                "    - [{}](equations/{}/{}/index.md)\n",
                title_case(&sub.name),
                cat.name,
                sub.name
            ));
            for eq in &sub.equations {
                s.push_str(&format!(
                    "      - [{}](equations/{}/{}/{}.md)\n",
                    eq.page.name, cat.name, sub.name, eq.slug
                ));
            }
        }
    }
    s.push_str("- [Equation Families](equations/families/index.md)\n");
    for family in &c.families {
        s.push_str(&format!(
            "  - [{}](equations/families/{}.md)\n",
            family.name,
            snake_case(&family.key)
        ));
    }
    s.push_str("- [Constants](constants/index.md)\n");
    for cst in &c.equations.constants {
        s.push_str(&format!("  - [{}](constants/{}.md)\n", cst.name, cst.key));
    }
    s.push_str("- [Fluids Guide](fluids/guide.md)\n");
    s.push_str("- [Fluids](fluids/index.md)\n");
    for fluid in &c.fluids {
        s.push_str(&format!(
            "  - [{}](fluids/{}.md)\n",
            fluid.name,
            snake_case(&fluid.key)
        ));
    }
    s.push_str("- [Materials Guide](materials/guide.md)\n");
    s.push_str("- [Materials](materials/index.md)\n");
    for mat in &c.materials {
        s.push_str(&format!(
            "  - [{}](materials/{}.md)\n",
            mat.name,
            snake_case(&mat.key)
        ));
    }
    s
}

fn render_mdbook_readme() -> String {
    "# Generated Unified mdBook\n\nThis directory is generated by `eng::docs`.\n".to_string()
}

fn render_pdf(c: &UnifiedDocsContribution, out_file: &Path) -> Result<()> {
    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent).map_err(|source| EquationError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let (doc, page1, layer1) =
        PdfDocument::new("Engineering Handbook", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| EquationError::Validation(e.to_string()))?;
    let layer = doc.get_page(page1).get_layer(layer1);
    layer.use_text("Engineering Handbook", 20.0, Mm(20.0), Mm(275.0), &font);
    layer.use_text(
        format!(
            "Equations: {}  Constants: {}  Fluids: {}  Materials: {}",
            c.equations.page_models.len(),
            c.equations.constants.len(),
            c.fluids.len(),
            c.materials.len()
        ),
        10.0,
        Mm(20.0),
        Mm(260.0),
        &font,
    );

    let mut y = 245.0;
    for eq in &c.equations.page_models {
        if y < 30.0 {
            break;
        }
        layer.use_text(
            format!("{} ({})", eq.name, eq.path_id),
            10.0,
            Mm(20.0),
            Mm(y),
            &font,
        );
        y -= 10.0;
    }

    let mut writer = std::io::BufWriter::new(fs::File::create(out_file).map_err(|source| {
        EquationError::Io {
            path: out_file.to_path_buf(),
            source,
        }
    })?);
    doc.save(&mut writer)
        .map_err(|e| EquationError::Validation(e.to_string()))?;
    Ok(())
}

fn write_text(path: PathBuf, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| EquationError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(&path, text).map_err(|source| EquationError::Io { path, source })?;
    Ok(())
}

fn write_json(path: PathBuf, value: &impl Serialize) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    write_text(path, &json)
}

fn run_mdbook_build(book_root: &Path) -> std::result::Result<(), MdBookBuildError> {
    let status = Command::new("mdbook")
        .arg("build")
        .arg(book_root)
        .status()
        .map_err(|_| MdBookBuildError::NotInstalled)?;
    if status.success() {
        Ok(())
    } else {
        Err(MdBookBuildError::BuildFailed(format!(
            "mdbook build failed with status {status}"
        )))
    }
}

enum MdBookBuildError {
    NotInstalled,
    BuildFailed(String),
}

fn missing_mdbook_message() -> String {
    "mdBook source generation succeeded, but HTML was not built because `mdbook` was not found in PATH.\n\nInstall mdBook:\n  cargo install mdbook\n"
        .to_string()
}

fn mirror_directory(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        fs::remove_dir_all(target).map_err(|source_err| EquationError::Io {
            path: target.to_path_buf(),
            source: source_err,
        })?;
    }
    copy_dir_recursive(source, target)
}

fn copy_dir_recursive(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target).map_err(|source_err| EquationError::Io {
        path: target.to_path_buf(),
        source: source_err,
    })?;
    for entry in fs::read_dir(source).map_err(|source_err| EquationError::Io {
        path: source.to_path_buf(),
        source: source_err,
    })? {
        let entry = entry.map_err(|e| EquationError::Validation(e.to_string()))?;
        let src = entry.path();
        let dst = target.join(entry.file_name());
        let ft = entry
            .file_type()
            .map_err(|e| EquationError::Validation(e.to_string()))?;
        if ft.is_dir() {
            copy_dir_recursive(&src, &dst)?;
        } else {
            fs::copy(&src, &dst).map_err(|source_err| EquationError::Io {
                path: dst,
                source: source_err,
            })?;
        }
    }
    Ok(())
}

fn snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_sep = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.is_empty() && !prev_is_sep {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_is_sep = false;
        } else {
            if !prev_is_sep && !out.is_empty() {
                out.push('_');
            }
            prev_is_sep = true;
        }
    }
    out.trim_matches('_').to_string()
}

fn title_case(s: &str) -> String {
    s.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), c.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
