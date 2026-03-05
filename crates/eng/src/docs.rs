use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::architecture::architecture_spec;
use crate::bindings::{INVOKE_PROTOCOL_VERSION, invoke_protocol_spec};
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
    devices: Vec<DeviceCatalogItem>,
    fluids: Vec<FluidCatalogItem>,
    materials: Vec<MaterialCatalogItem>,
    links: Vec<CatalogLink>,
}

#[derive(Debug, Clone, Serialize)]
struct BindingManifest {
    schema_version: &'static str,
    generated_from: &'static str,
    python_package: String,
    functions: Vec<BindingFunction>,
}

#[derive(Debug, Clone, Serialize)]
struct BindingFunction {
    id: String,
    entity: String,
    source: String,
    python_module: String,
    python_name: String,
    excel_name: String,
    op: String,
    fixed_args: BTreeMap<String, String>,
    args: Vec<BindingArg>,
    returns: String,
    help: String,
    rust_example: String,
    python_example: String,
    xloil_example: String,
    pyxll_example: String,
}

#[derive(Debug, Clone, Serialize)]
struct BindingArg {
    name: String,
    description: String,
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
struct DeviceCatalogItem {
    key: String,
    name: String,
    summary: String,
    supported_modes: Vec<String>,
    outputs: Vec<String>,
    route: String,
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
    devices: Vec<crate::devices::DeviceDocsEntry>,
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
const SNIPPET_RUNTIME_STRING_INPUT: &str = include_str!("../docs_snippets/runtime_string_input.rs");
const SNIPPET_FLUID_PROPERTY_LOOKUP: &str =
    include_str!("../docs_snippets/fluid_property_lookup.rs");
const SNIPPET_FLUID_STATE_CONSTRUCTORS: &str =
    include_str!("../docs_snippets/fluid_state_constructors.rs");
const SNIPPET_FLUID_SATURATION_METADATA: &str =
    include_str!("../docs_snippets/fluid_saturation_metadata.rs");
const SNIPPET_MATERIAL_PROPERTY_LOOKUP: &str =
    include_str!("../docs_snippets/material_property_lookup.rs");
const SNIPPET_CONTEXT_SOLVE: &str = include_str!("../docs_snippets/context_solve.rs");
const SNIPPET_FAMILY_VARIANT_ACCESS: &str =
    include_str!("../docs_snippets/family_variant_access.rs");
const SNIPPET_DEVICE_PIPE_LOSS_FIXED: &str =
    include_str!("../docs_snippets/device_pipe_loss_fixed.rs");
const SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_DIRECT: &str =
    include_str!("../docs_snippets/device_pipe_loss_colebrook_direct.rs");
const SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_FLUID: &str =
    include_str!("../docs_snippets/device_pipe_loss_colebrook_fluid.rs");

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
    let devices = crate::devices::docs_entries();
    let fluids = eng_fluids::docs_entries();
    let materials =
        eng_materials::docs_entries().map_err(|e| EquationError::Validation(e.to_string()))?;
    Ok(UnifiedDocsContribution {
        equations,
        families: equations::equation_families::load_default_validated(registry.equations())?,
        devices,
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
        out_dir.join("devices.json"),
        &ArtifactEnvelope {
            schema_version: EXPORT_SCHEMA_VERSION,
            model_version: EXPORT_MODEL_VERSION,
            artifact_type: "devices",
            items: c
                .devices
                .iter()
                .map(|d| DeviceCatalogItem {
                    key: d.key.clone(),
                    name: d.name.clone(),
                    summary: d.summary.clone(),
                    supported_modes: d.supported_modes.clone(),
                    outputs: d.outputs.clone(),
                    route: d.route.clone(),
                })
                .collect::<Vec<_>>(),
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
    write_generated_bindings(c, out_dir)?;
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
    for d in &c.devices {
        for equation_id in device_equation_dependencies(&d.key) {
            links.push(CatalogLink {
                relation: "device_uses_equation".to_string(),
                from: d.key.clone(),
                to: equation_id.to_string(),
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
        devices: c
            .devices
            .iter()
            .map(|d| DeviceCatalogItem {
                key: d.key.clone(),
                name: d.name.clone(),
                summary: d.summary.clone(),
                supported_modes: d.supported_modes.clone(),
                outputs: d.outputs.clone(),
                route: d.route.clone(),
            })
            .collect(),
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

fn write_generated_bindings(c: &UnifiedDocsContribution, out_dir: &Path) -> Result<()> {
    let bindings_root = out_dir.join("bindings");
    let python_root = bindings_root.join("python");
    let engpy_root = python_root.join("engpy");
    let excel_root = bindings_root.join("excel");
    let xloil_path = excel_root.join("eng_xloil.py");
    let pyxll_path = excel_root.join("eng_pyxll.py");
    if bindings_root.exists() {
        fs::remove_dir_all(&bindings_root).map_err(|source| EquationError::Io {
            path: bindings_root.clone(),
            source,
        })?;
    }

    let manifest = build_binding_manifest(c);
    write_json(bindings_root.join("binding_spec.json"), &manifest)?;
    write_json(
        bindings_root.join("invoke_protocol.json"),
        &invoke_protocol_spec(),
    )?;
    write_text(
        bindings_root.join("README.md"),
        "# Generated Bindings\n\nThis directory is generated by `eng::docs` from the unified catalog.\n\n- `binding_spec.json`: transport-agnostic public binding surface.\n- `invoke_protocol.json`: invoke request/response protocol contract shared by CLI, worker, and native Python runtime.\n",
    )?;
    write_text(
        python_root.join("pyproject.toml"),
        &render_python_binding_pyproject(),
    )?;
    write_text(
        python_root.join("README.md"),
        &render_python_binding_readme(),
    )?;

    write_text(
        engpy_root.join("__init__.py"),
        &render_python_package_init(&manifest),
    )?;
    write_text(engpy_root.join("_runtime.py"), &render_python_runtime())?;
    write_text(
        engpy_root.join("constants.py"),
        &render_python_constants_module(&manifest),
    )?;
    write_text(
        engpy_root.join("devices.py"),
        &render_python_devices_module(&manifest),
    )?;
    write_text(
        engpy_root.join("fluids.py"),
        &render_python_fluids_module(&manifest),
    )?;
    write_text(
        engpy_root.join("materials.py"),
        &render_python_materials_module(&manifest),
    )?;
    write_text(
        engpy_root.join("helpers.py"),
        &render_python_helpers_module(&manifest),
    )?;
    write_text(
        engpy_root.join("study.py"),
        &render_python_study_module(&manifest),
    )?;
    write_text(
        engpy_root.join("equations").join("__init__.py"),
        &render_python_equations_init(&manifest),
    )?;
    write_python_equation_modules(&manifest, &engpy_root.join("equations"))?;
    write_text(xloil_path, &render_xloil_module(&manifest))?;
    write_text(pyxll_path, &render_pyxll_module(&manifest))?;
    Ok(())
}

fn render_python_binding_pyproject() -> String {
    r#"[build-system]
requires = ["maturin>=1.7,<2.0"]
build-backend = "maturin"

[project]
name = "engpy-native"
version = "0.1.0"
description = "Native in-process Python bindings for the eng Rust library"
readme = "README.md"
requires-python = ">=3.9"

[tool.maturin]
manifest-path = "../../../crates/eng-pyext/Cargo.toml"
module-name = "engpy_native"
"#
    .to_string()
}

fn render_python_binding_readme() -> String {
    r#"# Generated Python Bindings

This directory is generated by `eng::docs`.

## Native Runtime (Preferred)

1. Create/activate a Python virtual environment.
2. Install build tooling: `pip install maturin`.
3. Run `maturin develop --manifest-path ../../../crates/eng-pyext/Cargo.toml`.
4. Verify:
   - `import engpy_native`
   - `import engpy._runtime as rt; rt.runtime_mode()` returns `"native"`.

Helper scripts:
- `scripts/setup-native-bindings.ps1`
- `scripts/setup-native-bindings.sh`
- `scripts/verify-native-bindings.ps1`
- `scripts/verify-native-bindings.sh`

## Fallback Runtime

If `engpy_native` is not importable, runtime falls back to `eng worker`.
Use `ENGPY_RUNTIME=native|worker` to force runtime selection.
"#
    .to_string()
}

fn build_binding_manifest(c: &UnifiedDocsContribution) -> BindingManifest {
    let mut functions = Vec::new();
    let page_by_path: BTreeMap<&str, &EquationPageModel> = c
        .equations
        .page_models
        .iter()
        .map(|p| (p.path_id.as_str(), p))
        .collect();

    for page in &c.equations.page_models {
        for target in &page.solve_targets {
            let target_snake = snake_case(&target.target);
            let fn_name = format!("solve_{}", target_snake);
            let mut args = equation_args_for_target(page, &target.target);
            if !page.branches.is_empty() {
                let options = page
                    .branches
                    .iter()
                    .map(|b| b.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                args.push(BindingArg {
                    name: "branch".to_string(),
                    description: format!("Optional branch selection. Supported: {}", options),
                });
            }
            let eq_slug = page
                .path_id
                .rsplit('.')
                .next()
                .map(snake_case)
                .unwrap_or_else(|| snake_case(&page.path_id));
            let module = format!("equations.{}.{}", snake_case(&page.category), eq_slug);
            let excel_name = format!(
                "ENG_{}_{}",
                page.path_id
                    .replace('.', "_")
                    .replace('-', "_")
                    .to_ascii_uppercase(),
                target.target.replace('.', "_").to_ascii_uppercase()
            );
            let mut fixed_args = BTreeMap::new();
            fixed_args.insert("path_id".to_string(), page.path_id.clone());
            fixed_args.insert("target".to_string(), target.target.clone());
            functions.push(BindingFunction {
                id: format!("equation.{}.{}", page.path_id, target.target),
                entity: "equation".to_string(),
                source: page.path_id.clone(),
                python_module: module.clone(),
                python_name: fn_name.clone(),
                excel_name: excel_name.clone(),
                op: "equation.solve".to_string(),
                fixed_args,
                args: args.clone(),
                returns: "f64".to_string(),
                help: format!("Solve {} for {}", page.name, target.target),
                rust_example: format!(
                    "eq.solve(equations::{}::equation()).for_target(\"{}\").value()?",
                    page.path_id.replace('.', "::"),
                    target.target
                ),
                python_example: format!(
                    "engpy.{}.{}({})",
                    module,
                    fn_name,
                    render_python_example_args(&args)
                ),
                xloil_example: format!("={}({})", excel_name, render_excel_example_args(&args)),
                pyxll_example: format!("={}({})", excel_name, render_excel_example_args(&args)),
            });
        }
    }

    for family in &c.families {
        for variant in &family.variants {
            if let Some(page) = page_by_path.get(variant.equation_id.as_str()) {
                for target in &page.solve_targets {
                    let target_snake = snake_case(&target.target);
                    let mut args = equation_args_for_target(page, &target.target);
                    if !page.branches.is_empty() {
                        let options = page
                            .branches
                            .iter()
                            .map(|b| b.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        args.push(BindingArg {
                            name: "branch".to_string(),
                            description: format!(
                                "Optional branch selection. Supported: {}",
                                options
                            ),
                        });
                    }
                    let mut fixed_args = BTreeMap::new();
                    fixed_args.insert("path_id".to_string(), page.path_id.clone());
                    fixed_args.insert("target".to_string(), target.target.clone());
                    let py_mod = format!(
                        "equations.families.{}.{}",
                        snake_case(&family.key),
                        snake_case(&variant.key)
                    );
                    let py_name = format!("solve_{}", target_snake);
                    let excel_name = format!(
                        "ENG_FAMILY_{}_{}_{}",
                        snake_case(&family.key).to_ascii_uppercase(),
                        snake_case(&variant.key).to_ascii_uppercase(),
                        target.target.to_ascii_uppercase()
                    );
                    functions.push(BindingFunction {
                        id: format!("family.{}.{}.{}", family.key, variant.key, target.target),
                        entity: "family_variant".to_string(),
                        source: format!("{}.{}", family.key, variant.key),
                        python_module: py_mod.clone(),
                        python_name: py_name.clone(),
                        excel_name: excel_name.clone(),
                        op: "equation.solve".to_string(),
                        fixed_args,
                        args: args.clone(),
                        returns: "f64".to_string(),
                        help: format!(
                            "Solve {} variant {} for {}",
                            family.name, variant.name, target.target
                        ),
                        rust_example: format!(
                            "eq.solve(equations::{}::equation()).for_target(\"{}\").value()?",
                            page.path_id.replace('.', "::"),
                            target.target
                        ),
                        python_example: format!(
                            "engpy.{}.{}({})",
                            py_mod,
                            py_name,
                            render_python_example_args(&args)
                        ),
                        xloil_example: format!(
                            "={}({})",
                            excel_name,
                            render_excel_example_args(&args)
                        ),
                        pyxll_example: format!(
                            "={}({})",
                            excel_name,
                            render_excel_example_args(&args)
                        ),
                    });
                }
            }
        }
    }

    functions.push(BindingFunction {
        id: "equation.meta".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_meta".to_string(),
        excel_name: "ENG_EQUATION_META".to_string(),
        op: "equation.meta".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id (for example `fluids.reynolds_number`)".to_string(),
        }],
        returns: "dict".to_string(),
        help: "Read equation metadata (display forms, variables, dimensions, units, targets)"
            .to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_meta(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_META(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_META(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.ascii".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_ascii".to_string(),
        excel_name: "ENG_EQUATION_ASCII".to_string(),
        op: "equation.ascii".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read ASCII display form for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_ascii(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_ASCII(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_ASCII(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.default_unit".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_default_unit".to_string(),
        excel_name: "ENG_EQUATION_DEFAULT_UNIT".to_string(),
        op: "equation.default_unit".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "path_id".to_string(),
                description: "Equation path id".to_string(),
            },
            BindingArg {
                name: "variable".to_string(),
                description: "Variable key (case-insensitive)".to_string(),
            },
        ],
        returns: "str".to_string(),
        help: "Read canonical default unit for one equation variable".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example:
            "engpy.equations.meta.equation_default_unit(\"fluids.reynolds_number\", \"mu\")"
                .to_string(),
        xloil_example: "=ENG_EQUATION_DEFAULT_UNIT(\"fluids.reynolds_number\",\"mu\")".to_string(),
        pyxll_example: "=ENG_EQUATION_DEFAULT_UNIT(\"fluids.reynolds_number\",\"mu\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.unicode".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_unicode".to_string(),
        excel_name: "ENG_EQUATION_UNICODE".to_string(),
        op: "equation.unicode".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read Unicode display form for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_unicode(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_UNICODE(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_UNICODE(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.latex".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_latex".to_string(),
        excel_name: "ENG_EQUATION_LATEX".to_string(),
        op: "equation.latex".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read LaTeX display form for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_latex(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_LATEX(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_LATEX(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.targets".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_targets".to_string(),
        excel_name: "ENG_EQUATION_TARGETS".to_string(),
        op: "equation.targets".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "list".to_string(),
        help: "Read solve targets for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_targets(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_TARGETS(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_TARGETS(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.variables".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_variables".to_string(),
        excel_name: "ENG_EQUATION_VARIABLES".to_string(),
        op: "equation.variables".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "list".to_string(),
        help: "Read variable metadata for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_variables(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_VARIABLES(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_VARIABLES(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.name".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_name".to_string(),
        excel_name: "ENG_EQUATION_NAME".to_string(),
        op: "equation.name".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read equation name".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_name(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_NAME(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_NAME(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.description".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_description".to_string(),
        excel_name: "ENG_EQUATION_DESCRIPTION".to_string(),
        op: "equation.description".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read equation description".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_description(\"fluids.reynolds_number\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_DESCRIPTION(\"fluids.reynolds_number\")".to_string(),
        pyxll_example: "=ENG_EQUATION_DESCRIPTION(\"fluids.reynolds_number\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.family".to_string(),
        entity: "equation_meta".to_string(),
        source: "equations".to_string(),
        python_module: "equations.meta".to_string(),
        python_name: "equation_family".to_string(),
        excel_name: "ENG_EQUATION_FAMILY".to_string(),
        op: "equation.family".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "dict|null".to_string(),
        help: "Read parent family/variant metadata for an equation".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.equations.meta.equation_family(\"thermo.ideal_gas.density\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_FAMILY(\"thermo.ideal_gas.density\")".to_string(),
        pyxll_example: "=ENG_EQUATION_FAMILY(\"thermo.ideal_gas.density\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "format.value".to_string(),
        entity: "helper".to_string(),
        source: "helpers".to_string(),
        python_module: "helpers".to_string(),
        python_name: "format_value".to_string(),
        excel_name: "ENG_FORMAT".to_string(),
        op: "format.value".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "value".to_string(),
                description: "Input value in `in_unit`".to_string(),
            },
            BindingArg {
                name: "in_unit".to_string(),
                description: "Input unit expression (for example Pa, m, psia, kg/(m*s))"
                    .to_string(),
            },
            BindingArg {
                name: "out_unit".to_string(),
                description: "Requested output unit expression".to_string(),
            },
        ],
        returns: "f64".to_string(),
        help: "Convert a numeric value from input units to output units (with dimensional checks)"
            .to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.format_value(2500000, \"Pa\", \"psia\")".to_string(),
        xloil_example: "=ENG_FORMAT(2500000,\"Pa\",\"psia\")".to_string(),
        pyxll_example: "=ENG_FORMAT(2500000,\"Pa\",\"psia\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "meta.get".to_string(),
        entity: "helper".to_string(),
        source: "helpers".to_string(),
        python_module: "helpers".to_string(),
        python_name: "meta_get".to_string(),
        excel_name: "ENG_META".to_string(),
        op: "meta.get".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "entity".to_string(),
                description: "equation | device | fluid | material | constant".to_string(),
            },
            BindingArg {
                name: "key".to_string(),
                description: "Entity id/key".to_string(),
            },
            BindingArg {
                name: "field".to_string(),
                description: "Metadata field to read".to_string(),
            },
        ],
        returns: "scalar|list|dict".to_string(),
        help: "General metadata helper for bindings".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example:
            "engpy.helpers.meta_get(\"equation\", \"structures.hoop_stress\", \"ascii\")"
                .to_string(),
        xloil_example: "=ENG_META(\"equation\",\"structures.hoop_stress\",\"ascii\")".to_string(),
        pyxll_example: "=ENG_META(\"equation\",\"structures.hoop_stress\",\"ascii\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "fluid.meta.properties".to_string(),
        entity: "helper".to_string(),
        source: "fluids".to_string(),
        python_module: "helpers".to_string(),
        python_name: "fluid_properties".to_string(),
        excel_name: "ENG_FLUID_PROPERTIES".to_string(),
        op: "meta.get".to_string(),
        fixed_args: BTreeMap::from([
            ("entity".to_string(), "fluid".to_string()),
            ("field".to_string(), "supported_properties".to_string()),
        ]),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Fluid key/alias".to_string(),
        }],
        returns: "list".to_string(),
        help: "Read supported properties for a fluid".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.fluid_properties(\"H2O\")".to_string(),
        xloil_example: "=ENG_FLUID_PROPERTIES(\"H2O\")".to_string(),
        pyxll_example: "=ENG_FLUID_PROPERTIES(\"H2O\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "material.meta.properties".to_string(),
        entity: "helper".to_string(),
        source: "materials".to_string(),
        python_module: "helpers".to_string(),
        python_name: "material_properties".to_string(),
        excel_name: "ENG_MATERIAL_PROPERTIES".to_string(),
        op: "meta.get".to_string(),
        fixed_args: BTreeMap::from([
            ("entity".to_string(), "material".to_string()),
            ("field".to_string(), "properties".to_string()),
        ]),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Material key/alias".to_string(),
        }],
        returns: "list".to_string(),
        help: "Read available properties for a material".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.material_properties(\"stainless_304\")".to_string(),
        xloil_example: "=ENG_MATERIAL_PROPERTIES(\"stainless_304\")".to_string(),
        pyxll_example: "=ENG_MATERIAL_PROPERTIES(\"stainless_304\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "device.meta.modes".to_string(),
        entity: "helper".to_string(),
        source: "devices".to_string(),
        python_module: "helpers".to_string(),
        python_name: "device_modes".to_string(),
        excel_name: "ENG_DEVICE_MODES".to_string(),
        op: "meta.get".to_string(),
        fixed_args: BTreeMap::from([
            ("entity".to_string(), "device".to_string()),
            ("field".to_string(), "supported_modes".to_string()),
        ]),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Device key".to_string(),
        }],
        returns: "list".to_string(),
        help: "Read supported modes for a device".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.device_modes(\"pipe_loss\")".to_string(),
        xloil_example: "=ENG_DEVICE_MODES(\"pipe_loss\")".to_string(),
        pyxll_example: "=ENG_DEVICE_MODES(\"pipe_loss\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.targets.text".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_targets_text".to_string(),
        excel_name: "ENG_EQUATION_TARGETS_TEXT".to_string(),
        op: "equation.targets.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read equation targets as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_targets_text(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_TARGETS_TEXT(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_TARGETS_TEXT(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.variables.text".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_variables_text".to_string(),
        excel_name: "ENG_EQUATION_VARIABLES_TEXT".to_string(),
        op: "equation.variables.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read equation variables as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_variables_text(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_VARIABLES_TEXT(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_VARIABLES_TEXT(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.branches.text".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_branches_text".to_string(),
        excel_name: "ENG_EQUATION_BRANCHES_TEXT".to_string(),
        op: "equation.branches.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read equation branch names as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_branches_text(\"compressible.area_mach\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_BRANCHES_TEXT(\"compressible.area_mach\")".to_string(),
        pyxll_example: "=ENG_EQUATION_BRANCHES_TEXT(\"compressible.area_mach\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.targets.table".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_targets_table".to_string(),
        excel_name: "ENG_EQUATION_TARGETS_TABLE".to_string(),
        op: "equation.targets.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "list[list]".to_string(),
        help: "Read equation targets table rows [target, is_default]".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_targets_table(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_TARGETS_TABLE(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_TARGETS_TABLE(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.variables.table".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_variables_table".to_string(),
        excel_name: "ENG_EQUATION_VARIABLES_TABLE".to_string(),
        op: "equation.variables.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "list[list]".to_string(),
        help: "Read equation variable table rows [variable, default_unit]".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_variables_table(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_VARIABLES_TABLE(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_VARIABLES_TABLE(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.branches.table".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_branches_table".to_string(),
        excel_name: "ENG_EQUATION_BRANCHES_TABLE".to_string(),
        op: "equation.branches.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "list[list]".to_string(),
        help: "Read equation branch table rows [branch, is_preferred]".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_branches_table(\"compressible.area_mach\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_BRANCHES_TABLE(\"compressible.area_mach\")".to_string(),
        pyxll_example: "=ENG_EQUATION_BRANCHES_TABLE(\"compressible.area_mach\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.target.count".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_target_count".to_string(),
        excel_name: "ENG_EQUATION_TARGET_COUNT".to_string(),
        op: "equation.target.count".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "u64".to_string(),
        help: "Read equation target count".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_target_count(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_TARGET_COUNT(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_TARGET_COUNT(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "equation.variable.count".to_string(),
        entity: "helper".to_string(),
        source: "equations".to_string(),
        python_module: "helpers".to_string(),
        python_name: "equation_variable_count".to_string(),
        excel_name: "ENG_EQUATION_VARIABLE_COUNT".to_string(),
        op: "equation.variable.count".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "path_id".to_string(),
            description: "Equation path id".to_string(),
        }],
        returns: "u64".to_string(),
        help: "Read equation variable count".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.equation_variable_count(\"structures.hoop_stress\")"
            .to_string(),
        xloil_example: "=ENG_EQUATION_VARIABLE_COUNT(\"structures.hoop_stress\")".to_string(),
        pyxll_example: "=ENG_EQUATION_VARIABLE_COUNT(\"structures.hoop_stress\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "fluid.properties.text".to_string(),
        entity: "helper".to_string(),
        source: "fluids".to_string(),
        python_module: "helpers".to_string(),
        python_name: "fluid_properties_text".to_string(),
        excel_name: "ENG_FLUID_PROPERTIES_TEXT".to_string(),
        op: "fluid.properties.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Fluid key/alias".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read fluid properties as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.fluid_properties_text(\"H2O\")".to_string(),
        xloil_example: "=ENG_FLUID_PROPERTIES_TEXT(\"H2O\")".to_string(),
        pyxll_example: "=ENG_FLUID_PROPERTIES_TEXT(\"H2O\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "fluid.properties.table".to_string(),
        entity: "helper".to_string(),
        source: "fluids".to_string(),
        python_module: "helpers".to_string(),
        python_name: "fluid_properties_table".to_string(),
        excel_name: "ENG_FLUID_PROPERTIES_TABLE".to_string(),
        op: "fluid.properties.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Fluid key/alias".to_string(),
        }],
        returns: "list[list]".to_string(),
        help: "Read fluid property table rows [property, default_unit]".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.fluid_properties_table(\"H2O\")".to_string(),
        xloil_example: "=ENG_FLUID_PROPERTIES_TABLE(\"H2O\")".to_string(),
        pyxll_example: "=ENG_FLUID_PROPERTIES_TABLE(\"H2O\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "fluid.property.count".to_string(),
        entity: "helper".to_string(),
        source: "fluids".to_string(),
        python_module: "helpers".to_string(),
        python_name: "fluid_property_count".to_string(),
        excel_name: "ENG_FLUID_PROPERTY_COUNT".to_string(),
        op: "fluid.property.count".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Fluid key/alias".to_string(),
        }],
        returns: "u64".to_string(),
        help: "Read fluid property count".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.fluid_property_count(\"H2O\")".to_string(),
        xloil_example: "=ENG_FLUID_PROPERTY_COUNT(\"H2O\")".to_string(),
        pyxll_example: "=ENG_FLUID_PROPERTY_COUNT(\"H2O\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "material.properties.text".to_string(),
        entity: "helper".to_string(),
        source: "materials".to_string(),
        python_module: "helpers".to_string(),
        python_name: "material_properties_text".to_string(),
        excel_name: "ENG_MATERIAL_PROPERTIES_TEXT".to_string(),
        op: "material.properties.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Material key/alias".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read material properties as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.material_properties_text(\"stainless_304\")".to_string(),
        xloil_example: "=ENG_MATERIAL_PROPERTIES_TEXT(\"stainless_304\")".to_string(),
        pyxll_example: "=ENG_MATERIAL_PROPERTIES_TEXT(\"stainless_304\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "material.properties.table".to_string(),
        entity: "helper".to_string(),
        source: "materials".to_string(),
        python_module: "helpers".to_string(),
        python_name: "material_properties_table".to_string(),
        excel_name: "ENG_MATERIAL_PROPERTIES_TABLE".to_string(),
        op: "material.properties.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Material key/alias".to_string(),
        }],
        returns: "list[list]".to_string(),
        help: "Read material property table rows [property, unit]".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.material_properties_table(\"stainless_304\")".to_string(),
        xloil_example: "=ENG_MATERIAL_PROPERTIES_TABLE(\"stainless_304\")".to_string(),
        pyxll_example: "=ENG_MATERIAL_PROPERTIES_TABLE(\"stainless_304\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "material.property.count".to_string(),
        entity: "helper".to_string(),
        source: "materials".to_string(),
        python_module: "helpers".to_string(),
        python_name: "material_property_count".to_string(),
        excel_name: "ENG_MATERIAL_PROPERTY_COUNT".to_string(),
        op: "material.property.count".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Material key/alias".to_string(),
        }],
        returns: "u64".to_string(),
        help: "Read material property count".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.material_property_count(\"stainless_304\")".to_string(),
        xloil_example: "=ENG_MATERIAL_PROPERTY_COUNT(\"stainless_304\")".to_string(),
        pyxll_example: "=ENG_MATERIAL_PROPERTY_COUNT(\"stainless_304\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "device.modes.text".to_string(),
        entity: "helper".to_string(),
        source: "devices".to_string(),
        python_module: "helpers".to_string(),
        python_name: "device_modes_text".to_string(),
        excel_name: "ENG_DEVICE_MODES_TEXT".to_string(),
        op: "device.modes.text".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Device key".to_string(),
        }],
        returns: "str".to_string(),
        help: "Read device modes as delimited text".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.device_modes_text(\"pipe_loss\")".to_string(),
        xloil_example: "=ENG_DEVICE_MODES_TEXT(\"pipe_loss\")".to_string(),
        pyxll_example: "=ENG_DEVICE_MODES_TEXT(\"pipe_loss\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "device.mode.count".to_string(),
        entity: "helper".to_string(),
        source: "devices".to_string(),
        python_module: "helpers".to_string(),
        python_name: "device_mode_count".to_string(),
        excel_name: "ENG_DEVICE_MODE_COUNT".to_string(),
        op: "device.mode.count".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Device key".to_string(),
        }],
        returns: "u64".to_string(),
        help: "Read device mode count".to_string(),
        rust_example: "eng::invoke::process_invoke_json(\"...\")".to_string(),
        python_example: "engpy.helpers.device_mode_count(\"pipe_loss\")".to_string(),
        xloil_example: "=ENG_DEVICE_MODE_COUNT(\"pipe_loss\")".to_string(),
        pyxll_example: "=ENG_DEVICE_MODE_COUNT(\"pipe_loss\")".to_string(),
    });

    for spec in crate::devices::generation_specs() {
        for bf in spec.binding_functions {
            functions.push(BindingFunction {
                id: bf.id.to_string(),
                entity: "device".to_string(),
                source: spec.key.to_string(),
                python_module: "devices".to_string(),
                python_name: bf.python_name.to_string(),
                excel_name: bf.excel_name.to_string(),
                op: bf.op.to_string(),
                fixed_args: bf
                    .fixed_args
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
                args: bf
                    .args
                    .iter()
                    .map(|a| BindingArg {
                        name: a.name.to_string(),
                        description: a.description.to_string(),
                    })
                    .collect(),
                returns: bf.returns.to_string(),
                help: bf.help.to_string(),
                rust_example: bf.rust_example.to_string(),
                python_example: bf.python_example.to_string(),
                xloil_example: bf.xloil_example.to_string(),
                pyxll_example: bf.pyxll_example.to_string(),
            });
        }
    }

    functions.push(BindingFunction {
        id: "study.equation.sweep".to_string(),
        entity: "study".to_string(),
        source: "equation".to_string(),
        python_module: "study".to_string(),
        python_name: "equation_sweep_table".to_string(),
        excel_name: "ENG_STUDY_EQUATION_TABLE".to_string(),
        op: "study.equation.sweep".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "path_id".to_string(),
                description: "Equation path id".to_string(),
            },
            BindingArg {
                name: "target".to_string(),
                description: "Solve target key".to_string(),
            },
            BindingArg {
                name: "sweep_variable".to_string(),
                description: "Variable to sweep".to_string(),
            },
            BindingArg {
                name: "start".to_string(),
                description: "Sweep start".to_string(),
            },
            BindingArg {
                name: "end".to_string(),
                description: "Sweep end".to_string(),
            },
            BindingArg {
                name: "count".to_string(),
                description: "Number of samples".to_string(),
            },
            BindingArg {
                name: "spacing".to_string(),
                description: "Spacing mode (linear or logspace)".to_string(),
            },
            BindingArg {
                name: "branch".to_string(),
                description: "Optional branch".to_string(),
            },
        ],
        returns: "dict(table, spill)".to_string(),
        help: "Generic equation study sweep table (1D axis)".to_string(),
        rust_example: "eng::solve::run_equation_study(...)".to_string(),
        python_example: "engpy.study.equation_sweep_table(path_id=\"compressible.isentropic_pressure_ratio\", target=\"p_p0\", sweep_variable=\"M\", start=0.2, end=3.0, count=20, spacing=\"linear\")".to_string(),
        xloil_example: "=ENG_STUDY_EQUATION_TABLE(\"compressible.isentropic_pressure_ratio\",\"p_p0\",\"M\",0.2,3,20,\"linear\",\"\")".to_string(),
        pyxll_example: "=ENG_STUDY_EQUATION_TABLE(\"compressible.isentropic_pressure_ratio\",\"p_p0\",\"M\",0.2,3,20,\"linear\",\"\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "study.device.isentropic_m_to_p_p0.table".to_string(),
        entity: "study".to_string(),
        source: "device.isentropic_calc".to_string(),
        python_module: "study".to_string(),
        python_name: "isentropic_m_to_p_p0_table".to_string(),
        excel_name: "ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE".to_string(),
        op: "study.device.isentropic_m_to_p_p0.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "gamma".to_string(),
                description: "Specific heat ratio".to_string(),
            },
            BindingArg {
                name: "start".to_string(),
                description: "Mach start".to_string(),
            },
            BindingArg {
                name: "end".to_string(),
                description: "Mach end".to_string(),
            },
            BindingArg {
                name: "count".to_string(),
                description: "Sample count".to_string(),
            },
            BindingArg {
                name: "branch".to_string(),
                description: "Optional branch".to_string(),
            },
        ],
        returns: "dict(table, spill)".to_string(),
        help: "Study table for isentropic device Mach -> p/p0".to_string(),
        rust_example: "eng::solve::study_isentropic_m_to_p_p0(1.4, eng::solve::SweepAxis::linspace(0.2, 3.0, 21), None)".to_string(),
        python_example: "engpy.study.isentropic_m_to_p_p0_table(gamma=1.4, start=0.2, end=3.0, count=21)".to_string(),
        xloil_example: "=ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(1.4,0.2,3,21,\"\")".to_string(),
        pyxll_example: "=ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(1.4,0.2,3,21,\"\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "study.device.nozzle_flow.table".to_string(),
        entity: "study".to_string(),
        source: "device.nozzle_flow_calc".to_string(),
        python_module: "study".to_string(),
        python_name: "nozzle_flow_table".to_string(),
        excel_name: "ENG_STUDY_NOZZLE_FLOW_TABLE".to_string(),
        op: "study.device.nozzle_flow.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "gamma".to_string(),
                description: "Specific heat ratio".to_string(),
            },
            BindingArg {
                name: "start".to_string(),
                description: "Area-ratio start".to_string(),
            },
            BindingArg {
                name: "end".to_string(),
                description: "Area-ratio end".to_string(),
            },
            BindingArg {
                name: "count".to_string(),
                description: "Sample count".to_string(),
            },
            BindingArg {
                name: "branch".to_string(),
                description: "Branch (subsonic/supersonic)".to_string(),
            },
        ],
        returns: "dict(table, spill)".to_string(),
        help: "Study table for nozzle-flow device over area ratio".to_string(),
        rust_example: "eng::solve::study_nozzle_flow_area_ratio(1.4, eng::solve::SweepAxis::linspace(1.2, 3.0, 20), eng::devices::NozzleFlowBranch::Supersonic)".to_string(),
        python_example: "engpy.study.nozzle_flow_table(gamma=1.4, start=1.2, end=3.0, count=20, branch=\"supersonic\")".to_string(),
        xloil_example: "=ENG_STUDY_NOZZLE_FLOW_TABLE(1.4,1.2,3,20,\"supersonic\")".to_string(),
        pyxll_example: "=ENG_STUDY_NOZZLE_FLOW_TABLE(1.4,1.2,3,20,\"supersonic\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "study.device.normal_shock.table".to_string(),
        entity: "study".to_string(),
        source: "device.normal_shock_calc".to_string(),
        python_module: "study".to_string(),
        python_name: "normal_shock_table".to_string(),
        excel_name: "ENG_STUDY_NORMAL_SHOCK_TABLE".to_string(),
        op: "study.device.normal_shock.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "gamma".to_string(),
                description: "Specific heat ratio".to_string(),
            },
            BindingArg {
                name: "start".to_string(),
                description: "M1 start".to_string(),
            },
            BindingArg {
                name: "end".to_string(),
                description: "M1 end".to_string(),
            },
            BindingArg {
                name: "count".to_string(),
                description: "Sample count".to_string(),
            },
        ],
        returns: "dict(table, spill)".to_string(),
        help: "Study table for normal-shock device over M1".to_string(),
        rust_example:
            "eng::solve::study_normal_shock_m1(1.4, eng::solve::SweepAxis::linspace(1.05, 4.0, 20))"
                .to_string(),
        python_example: "engpy.study.normal_shock_table(gamma=1.4, start=1.05, end=4.0, count=20)"
            .to_string(),
        xloil_example: "=ENG_STUDY_NORMAL_SHOCK_TABLE(1.4,1.05,4,20)".to_string(),
        pyxll_example: "=ENG_STUDY_NORMAL_SHOCK_TABLE(1.4,1.05,4,20)".to_string(),
    });
    functions.push(BindingFunction {
        id: "study.workflow.nozzle_normal_shock.table".to_string(),
        entity: "study".to_string(),
        source: "solve.workflow.nozzle_normal_shock".to_string(),
        python_module: "study".to_string(),
        python_name: "nozzle_normal_shock_workflow_table".to_string(),
        excel_name: "ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE".to_string(),
        op: "study.workflow.nozzle_normal_shock.table".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "gamma".to_string(),
                description: "Specific heat ratio".to_string(),
            },
            BindingArg {
                name: "start".to_string(),
                description: "Area-ratio start".to_string(),
            },
            BindingArg {
                name: "end".to_string(),
                description: "Area-ratio end".to_string(),
            },
            BindingArg {
                name: "count".to_string(),
                description: "Sample count".to_string(),
            },
            BindingArg {
                name: "branch".to_string(),
                description: "Nozzle branch (subsonic/supersonic)".to_string(),
            },
        ],
        returns: "dict(table, spill)".to_string(),
        help: "Study table for nozzle + normal-shock chained workflow".to_string(),
        rust_example: "eng::solve::study_nozzle_normal_shock_workflow(1.4, eng::solve::SweepAxis::linspace(1.2, 3.0, 20), eng::devices::NozzleFlowBranch::Supersonic)".to_string(),
        python_example: "engpy.study.nozzle_normal_shock_workflow_table(gamma=1.4, start=1.2, end=3.0, count=20, branch=\"supersonic\")".to_string(),
        xloil_example: "=ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(1.4,1.2,3,20,\"supersonic\")".to_string(),
        pyxll_example: "=ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(1.4,1.2,3,20,\"supersonic\")".to_string(),
    });

    functions.push(BindingFunction {
        id: "fluid.prop".to_string(),
        entity: "fluid".to_string(),
        source: "fluids".to_string(),
        python_module: "fluids".to_string(),
        python_name: "fluid_prop".to_string(),
        excel_name: "ENG_FLUID_PROP".to_string(),
        op: "fluid.prop".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "fluid".to_string(),
                description: "Fluid key/name".to_string(),
            },
            BindingArg {
                name: "in1_key".to_string(),
                description: "State input key 1".to_string(),
            },
            BindingArg {
                name: "in1_value".to_string(),
                description: "State input value 1".to_string(),
            },
            BindingArg {
                name: "in2_key".to_string(),
                description: "State input key 2".to_string(),
            },
            BindingArg {
                name: "in2_value".to_string(),
                description: "State input value 2".to_string(),
            },
            BindingArg {
                name: "out_prop".to_string(),
                description: "Output property key".to_string(),
            },
        ],
        returns: "f64".to_string(),
        help: "Binding-friendly fluid property lookup".to_string(),
        rust_example: "fluids::water().state_tp(\"300 K\", \"1 bar\")?.rho()?".to_string(),
        python_example:
            "engpy.fluids.fluid_prop(\"H2O\", \"T\", \"300 K\", \"P\", \"1 bar\", \"rho\")"
                .to_string(),
        xloil_example: "=ENG_FLUID_PROP(\"H2O\",\"T\",\"300 K\",\"P\",\"1 bar\",\"rho\")"
            .to_string(),
        pyxll_example: "=ENG_FLUID_PROP(\"H2O\",\"T\",\"300 K\",\"P\",\"1 bar\",\"rho\")"
            .to_string(),
    });
    functions.push(BindingFunction {
        id: "material.prop".to_string(),
        entity: "material".to_string(),
        source: "materials".to_string(),
        python_module: "materials".to_string(),
        python_name: "mat_prop".to_string(),
        excel_name: "ENG_MAT_PROP".to_string(),
        op: "material.prop".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![
            BindingArg {
                name: "material".to_string(),
                description: "Material key/name".to_string(),
            },
            BindingArg {
                name: "property".to_string(),
                description: "Property key".to_string(),
            },
            BindingArg {
                name: "temperature".to_string(),
                description: "Temperature input".to_string(),
            },
        ],
        returns: "f64".to_string(),
        help: "Binding-friendly material property lookup".to_string(),
        rust_example:
            "materials::stainless_304().temperature(\"350 K\")?.property(\"elastic_modulus\")?"
                .to_string(),
        python_example:
            "engpy.materials.mat_prop(\"stainless_304\", \"elastic_modulus\", \"350 K\")"
                .to_string(),
        xloil_example: "=ENG_MAT_PROP(\"stainless_304\",\"elastic_modulus\",\"350 K\")".to_string(),
        pyxll_example: "=ENG_MAT_PROP(\"stainless_304\",\"elastic_modulus\",\"350 K\")".to_string(),
    });
    functions.push(BindingFunction {
        id: "constant.get".to_string(),
        entity: "constant".to_string(),
        source: "constants".to_string(),
        python_module: "constants".to_string(),
        python_name: "get_constant".to_string(),
        excel_name: "ENG_CONST".to_string(),
        op: "constant.get".to_string(),
        fixed_args: BTreeMap::new(),
        args: vec![BindingArg {
            name: "key".to_string(),
            description: "Constant key".to_string(),
        }],
        returns: "f64".to_string(),
        help: "Get constant value from registry".to_string(),
        rust_example: "equations::get_constant(\"g0\")?.value".to_string(),
        python_example: "engpy.constants.get_constant(\"g0\")".to_string(),
        xloil_example: "=ENG_CONST(\"g0\")".to_string(),
        pyxll_example: "=ENG_CONST(\"g0\")".to_string(),
    });

    functions.sort_by(|a, b| a.id.cmp(&b.id));
    BindingManifest {
        schema_version: EXPORT_SCHEMA_VERSION,
        generated_from: "generated/catalog.json",
        python_package: "engpy".to_string(),
        functions,
    }
}

fn equation_args_for_target(page: &EquationPageModel, target: &str) -> Vec<BindingArg> {
    page.variables
        .iter()
        .filter(|v| v.key != target)
        .map(|v| BindingArg {
            name: v.key.clone(),
            description: if v.description.trim().is_empty() {
                v.name.clone()
            } else {
                v.description.clone()
            },
        })
        .collect()
}

fn render_python_example_args(args: &[BindingArg]) -> String {
    args.iter()
        .map(|a| format!("{}=\"...\"", snake_case(&a.name)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_excel_example_args(args: &[BindingArg]) -> String {
    args.iter()
        .map(|_| "\"...\"".to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn render_python_runtime() -> String {
    r#"import json
import os
import subprocess
import threading
import uuid
import atexit
import builtins

PROTOCOL_VERSION = "__PROTOCOL_VERSION__"


class EngBindingError(RuntimeError):
    def __init__(self, code, message, op=None, field=None, request_id=None, detail=None):
        super().__init__(f"[{code}] {message}")
        self.code = code
        self.message = message
        self.op = op
        self.field = field
        self.request_id = request_id
        self.detail = detail


def _load_native_runtime():
    try:
        import engpy_native  # type: ignore
        return engpy_native
    except Exception:
        return None


_NATIVE_RUNTIME = _load_native_runtime()


class _NativeClient:
    def __init__(self, native_mod):
        self._native = native_mod
        self._request_count = 0
        self._last_failure = None
        self._last_request_id = None

    def _stop(self):
        # No-op for in-process runtime.
        return None

    def worker_pid(self):
        return None

    def stats(self):
        return {
            "runtime_mode": "native",
            "worker_pid": None,
            "startup_count": 0,
            "restart_count": 0,
            "request_count": self._request_count,
            "last_reused": True,
            "last_request_id": self._last_request_id,
            "last_failure": self._last_failure,
        }

    def invoke(self, op: str, args: dict, request_id=None):
        req_id = request_id or str(uuid.uuid4())
        req = {"protocol_version": PROTOCOL_VERSION, "op": op, "request_id": req_id, "args": args}
        self._request_count += 1
        self._last_request_id = req_id
        try:
            raw = self._native.invoke_json(json.dumps(req))
            data = json.loads(raw)
            self._last_failure = None
            return _validate_response(op, req_id, data)
        except EngBindingError:
            raise
        except Exception as exc:
            self._last_failure = str(exc)
            raise EngBindingError(
                "invoke_native_failed",
                f"native invoke failed: {exc}",
                op=op,
                request_id=req_id,
            )


class _WorkerClient:
    def __init__(self):
        self._lock = threading.RLock()
        self._process = None
        self._startup_count = 0
        self._restart_count = 0
        self._request_count = 0
        self._last_failure = None
        self._last_reused = False
        self._last_request_id = None

    def _resolve_worker_bin(self):
        return os.environ.get("ENG_WORKER_BIN") or os.environ.get("ENG_BIN") or "eng"

    def _popen_kwargs(self):
        kwargs = {
            "stdin": subprocess.PIPE,
            "stdout": subprocess.PIPE,
            # keep stderr drained to avoid blocking on pipe saturation
            "stderr": subprocess.DEVNULL,
            "text": True,
            "bufsize": 1,
            "encoding": "utf-8",
            "errors": "replace",
        }
        if os.name == "nt":
            # Prevent flashing/visible command windows in Excel/xlOil usage.
            creationflags = getattr(subprocess, "CREATE_NO_WINDOW", 0)
            kwargs["creationflags"] = creationflags
            si = subprocess.STARTUPINFO()
            si.dwFlags |= subprocess.STARTF_USESHOWWINDOW
            kwargs["startupinfo"] = si
        return kwargs

    def _spawn(self):
        worker_bin = self._resolve_worker_bin()
        self._process = subprocess.Popen(
            [worker_bin, "worker"],
            **self._popen_kwargs(),
        )
        self._startup_count += 1

    def _ensure_running(self, count_restart=False):
        if self._process is None:
            self._spawn()
            return False
        if self._process.poll() is not None:
            if count_restart:
                self._restart_count += 1
            self._spawn()
            return False
        return True

    def _stop(self):
        with self._lock:
            if self._process is None:
                return
            try:
                if self._process.stdin:
                    self._process.stdin.close()
            except Exception:
                pass
            try:
                self._process.terminate()
            except Exception:
                pass
            self._process = None

    def stats(self):
        with self._lock:
            return {
                "worker_pid": self.worker_pid(),
                "startup_count": self._startup_count,
                "restart_count": self._restart_count,
                "request_count": self._request_count,
                "last_reused": self._last_reused,
                "last_request_id": self._last_request_id,
                "last_failure": self._last_failure,
            }

    def worker_pid(self):
        with self._lock:
            if self._process is None or self._process.poll() is not None:
                return None
            return self._process.pid

    def invoke(self, op: str, args: dict, request_id=None):
        with self._lock:
            req_id = request_id or str(uuid.uuid4())
            req = {"protocol_version": PROTOCOL_VERSION, "op": op, "request_id": req_id, "args": args}
            self._request_count += 1
            self._last_request_id = req_id

            last_exc = None
            for attempt in range(2):
                reused = self._ensure_running(count_restart=(attempt > 0))
                self._last_reused = reused
                try:
                    if self._process.stdin is None or self._process.stdout is None:
                        raise RuntimeError("worker stdio not available")
                    self._process.stdin.write(json.dumps(req) + "\n")
                    self._process.stdin.flush()
                    raw = self._process.stdout.readline()
                    if not raw:
                        raise RuntimeError("worker returned no response")
                    data = json.loads(raw)
                    self._last_failure = None
                    return _validate_response(op, req_id, data)
                except Exception as exc:
                    last_exc = exc
                    self._last_failure = str(exc)
                    self._stop()
            raise EngBindingError(
                "invoke_worker_failed",
                f"worker invocation failed after restart: {last_exc}",
                op=op,
                request_id=req_id,
            )


def _validate_response(op: str, request_id: str, data: dict):
    if data.get("protocol_version") != PROTOCOL_VERSION:
        raise EngBindingError(
            "protocol_version_mismatch",
            f"response protocol version '{data.get('protocol_version')}' does not match '{PROTOCOL_VERSION}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if data.get("op") != op:
        raise EngBindingError(
            "operation_mismatch",
            f"response op '{data.get('op')}' does not match request op '{op}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if data.get("request_id") and data.get("request_id") != request_id:
        raise EngBindingError(
            "request_id_mismatch",
            f"response request_id '{data.get('request_id')}' does not match '{request_id}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if bool(data.get("ok", False)):
        return data.get("value")
    err = data.get("error") or {}
    raise EngBindingError(
        err.get("code", "invoke_error"),
        err.get("message", "unknown invoke error"),
        op=data.get("op", op),
        field=err.get("field"),
        request_id=data.get("request_id"),
        detail=err.get("detail"),
    )


def _select_client():
    runtime_pref = os.environ.get("ENGPY_RUNTIME", "").strip().lower()
    if runtime_pref not in {"", "native", "worker"}:
        raise EngBindingError(
            "invalid_runtime_preference",
            f"ENGPY_RUNTIME must be 'native' or 'worker', got '{runtime_pref}'",
        )
    if runtime_pref != "worker" and _NATIVE_RUNTIME is not None:
        return _NativeClient(_NATIVE_RUNTIME), "native"
    if runtime_pref == "native":
        raise EngBindingError(
            "native_runtime_unavailable",
            "ENGPY_RUNTIME=native requested but engpy_native could not be imported",
        )
    return _WorkerClient(), "worker"


if hasattr(builtins, "_ENGPY_CLIENT"):
    _CLIENT = builtins._ENGPY_CLIENT
    _RUNTIME_MODE = getattr(builtins, "_ENGPY_CLIENT_MODE", "worker")
else:
    _CLIENT, _RUNTIME_MODE = _select_client()
    builtins._ENGPY_CLIENT = _CLIENT
    builtins._ENGPY_CLIENT_MODE = _RUNTIME_MODE
atexit.register(_CLIENT._stop)


def runtime_mode():
    return _RUNTIME_MODE


def worker_pid():
    return _CLIENT.worker_pid()


def worker_stats():
    stats = _CLIENT.stats()
    stats["runtime_mode"] = _RUNTIME_MODE
    return stats


def runtime_info():
    info = worker_stats()
    info["native_fallback_reason"] = getattr(builtins, "_ENGPY_NATIVE_FALLBACK_REASON", None)
    return info


def stop_worker():
    _CLIENT._stop()


def _switch_to_worker_fallback(reason: str):
    global _CLIENT, _RUNTIME_MODE
    worker = _WorkerClient()
    _CLIENT = worker
    _RUNTIME_MODE = "worker"
    builtins._ENGPY_CLIENT = worker
    builtins._ENGPY_CLIENT_MODE = "worker"
    builtins._ENGPY_NATIVE_FALLBACK_REASON = reason
    return worker


def invoke(op: str, args: dict, request_id=None):
    try:
        return _CLIENT.invoke(op, args, request_id=request_id)
    except EngBindingError as exc:
        # If a stale native extension is loaded, allow transparent fallback.
        allow_fallback = os.environ.get("ENGPY_NATIVE_FALLBACK", "1").strip().lower() not in {"0", "false", "no"}
        if (
            allow_fallback
            and _RUNTIME_MODE == "native"
            and exc.code in {"unknown_operation", "protocol_version_mismatch"}
        ):
            reason = f"native incompatibility ({exc.code}) on op '{op}'"
            try:
                return _switch_to_worker_fallback(reason).invoke(op, args, request_id=request_id)
            except Exception as worker_exc:
                raise EngBindingError(
                    "native_incompatible_no_worker",
                    f"{reason}; worker fallback failed: {worker_exc}. Rebuild engpy_native (scripts/setup-native-bindings.*).",
                    op=op,
                    request_id=request_id,
                )
        raise
"#
    .replace("__PROTOCOL_VERSION__", INVOKE_PROTOCOL_VERSION)
}

fn render_python_package_init(manifest: &BindingManifest) -> String {
    format!(
        "from . import constants, devices, fluids, materials, helpers, study\nfrom .equations import *\n\n__all__ = [\"constants\", \"devices\", \"fluids\", \"materials\", \"helpers\", \"study\", \"equations\"]\n# Generated from {}\n",
        manifest.generated_from
    )
}

fn render_python_docstring(f: &BindingFunction) -> String {
    let mut s = String::new();
    s.push_str(&f.help);
    s.push_str("\n\nArgs:\n");
    if f.args.is_empty() {
        s.push_str("  (none)\n");
    } else {
        for a in &f.args {
            s.push_str(&format!("  {}: {}\n", snake_case(&a.name), a.description));
        }
    }
    s.push_str(&format!("Returns:\n  {}\n", f.returns));
    s.replace("\"\"\"", "'''")
}

fn render_excel_help_block(f: &BindingFunction) -> String {
    let mut lines = Vec::new();
    lines.push(f.help.clone());
    if !f.args.is_empty() {
        lines.push("Arguments:".to_string());
        for a in &f.args {
            lines.push(format!(
                "- {}: {}",
                excel_param_name(f, a),
                a.description.trim()
            ));
        }
    }
    lines.push(format!("Returns: {}", f.returns));
    lines.push(format!(
        "Example: {}",
        if f.xloil_example.is_empty() {
            f.pyxll_example.as_str()
        } else {
            f.xloil_example.as_str()
        }
    ));
    lines.join(" | ")
}

fn excel_param_name(f: &BindingFunction, a: &BindingArg) -> String {
    excel_param_name_for_op(f.op.as_str(), &a.name)
}

fn excel_param_name_for_op(op: &str, arg_name: &str) -> String {
    match (op, arg_name) {
        ("device.pipe_loss.solve_delta_p", "rho") => "density".to_string(),
        ("device.pipe_loss.solve_delta_p", "mu") => "viscosity".to_string(),
        ("device.pipe_loss.solve_delta_p", "v") => "velocity".to_string(),
        ("device.pipe_loss.solve_delta_p", "d") => "diameter".to_string(),
        ("device.pipe_loss.solve_delta_p", "l") => "length".to_string(),
        ("device.pipe_loss.solve_delta_p", "eps") => "roughness".to_string(),
        ("device.isentropic_calc", "input_kind")
        | ("device.isentropic_calc.value", "input_kind")
        | ("device.isentropic_calc.pivot_mach", "input_kind")
        | ("device.isentropic_calc.path_text", "input_kind") => "value_kind_in".to_string(),
        ("device.isentropic_calc", "input_value")
        | ("device.isentropic_calc.value", "input_value")
        | ("device.isentropic_calc.pivot_mach", "input_value")
        | ("device.isentropic_calc.path_text", "input_value") => "value_in".to_string(),
        ("device.isentropic_calc", "target_kind")
        | ("device.isentropic_calc.value", "target_kind")
        | ("device.isentropic_calc.pivot_mach", "target_kind")
        | ("device.isentropic_calc.path_text", "target_kind") => "target_kind_out".to_string(),
        ("fluid.prop", "in1_key") => "state_prop_1".to_string(),
        ("fluid.prop", "in1_value") => "state_value_1".to_string(),
        ("fluid.prop", "in2_key") => "state_prop_2".to_string(),
        ("fluid.prop", "in2_value") => "state_value_2".to_string(),
        ("material.prop", "property") => "property_key".to_string(),
        _ => snake_case(arg_name),
    }
}

fn render_python_constants_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "constants")
    {
        out.push_str(&format!(
            "def {}(key):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{\"key\": key}})\n\n",
            f.python_name,
            render_python_docstring(f),
            f.op
        ));
    }
    out
}

fn render_python_devices_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "devices")
    {
        if f.python_name == "pipe_loss_solve_delta_p" {
            out.push_str(
                "def pipe_loss_solve_delta_p(friction_model=\"Colebrook\", fixed_f=None, rho=None, mu=None, v=None, d=None, l=None, eps=None, fluid=None, in1_key=None, in1_value=None, in2_key=None, in2_value=None):\n",
            );
            out.push_str(
                "    \"\"\"Solve pipe pressure drop using composed Reynolds/Colebrook/Darcy behavior.\n\nArgs:\n  friction_model: 'Colebrook' or 'Fixed'\n  fixed_f: fixed Darcy friction factor (required when friction_model='Fixed')\n  rho, mu, v, d, l, eps: direct inputs\n  fluid, in1_key, in1_value, in2_key, in2_value: optional fluid-state context inputs\nReturns:\n  dict with delta_p, friction_factor, reynolds_number\n\"\"\"\n",
            );
            out.push_str("    args = {\n");
            out.push_str("        \"friction_model\": friction_model,\n        \"fixed_f\": fixed_f,\n        \"rho\": rho,\n        \"mu\": mu,\n        \"v\": v,\n        \"d\": d,\n        \"l\": l,\n        \"eps\": eps,\n        \"fluid\": fluid,\n        \"in1_key\": in1_key,\n        \"in1_value\": in1_value,\n        \"in2_key\": in2_key,\n        \"in2_value\": in2_value,\n    }\n");
            out.push_str("    return invoke(\"");
            out.push_str(&f.op);
            out.push_str("\", {k: v for k, v in args.items() if v is not None})\n\n");
            continue;
        }

        let mut params = Vec::new();
        let mut payload_parts = f
            .fixed_args
            .iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<_>>();
        for a in &f.args {
            let p = snake_case(&a.name);
            if a.name == "branch" {
                params.push(format!("{p}=None"));
                payload_parts.push(format!(
                    "**({{\"{}\": {}}} if {} not in (None, \"\") else {{}})",
                    a.name, p, p
                ));
            } else {
                params.push(p.clone());
                payload_parts.push(format!("\"{}\": {}", a.name, p));
            }
        }
        out.push_str(&format!(
            "def {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
            f.python_name,
            params.join(", "),
            render_python_docstring(f),
            f.op,
            payload_parts.join(", ")
        ));
    }
    out
}

fn render_python_fluids_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "fluids")
    {
        out.push_str(&format!(
            "def {}(fluid, in1_key, in1_value, in2_key, in2_value, out_prop):\n",
            f.python_name
        ));
        out.push_str(&format!(
            "    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{\"fluid\": fluid, \"in1_key\": in1_key, \"in1_value\": in1_value, \"in2_key\": in2_key, \"in2_value\": in2_value, \"out_prop\": out_prop}})\n\n",
            render_python_docstring(f), f.op
        ));
    }
    out
}

fn render_python_materials_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "materials")
    {
        out.push_str(&format!(
            "def {}(material, property, temperature):\n",
            f.python_name
        ));
        out.push_str(&format!(
            "    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{\"material\": material, \"property\": property, \"temperature\": temperature}})\n\n",
            render_python_docstring(f), f.op
        ));
    }
    out
}

fn render_python_helpers_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "helpers")
    {
        let mut params = Vec::new();
        let mut payload_parts = f
            .fixed_args
            .iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<_>>();
        for a in &f.args {
            let p = snake_case(&a.name);
            params.push(p.clone());
            payload_parts.push(format!("\"{}\": {}", a.name, p));
        }
        out.push_str(&format!(
            "def {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
            f.python_name,
            params.join(", "),
            render_python_docstring(f),
            f.op,
            payload_parts.join(", ")
        ));
    }
    out
}

fn render_python_study_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str("from ._runtime import invoke\n\n");
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.python_module == "study")
    {
        let mut params = Vec::new();
        let mut payload_parts = f
            .fixed_args
            .iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<_>>();
        for a in &f.args {
            let p = snake_case(&a.name);
            params.push(format!("{p}=None"));
            payload_parts.push(format!(
                "**({{\"{}\": {}}} if {} is not None else {{}})",
                a.name, p, p
            ));
        }
        out.push_str(&format!(
            "def {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
            f.python_name,
            params.join(", "),
            render_python_docstring(f),
            f.op,
            payload_parts.join(", ")
        ));
    }
    out
}

fn render_python_equations_init(manifest: &BindingManifest) -> String {
    let mut top_level = BTreeSet::new();
    for f in &manifest.functions {
        if let Some(rest) = f.python_module.strip_prefix("equations.") {
            if let Some((head, _)) = rest.split_once('.') {
                top_level.insert(snake_case(head));
            } else {
                top_level.insert(snake_case(rest));
            }
        }
    }
    let mut out = String::new();
    out.push_str("from engpy._runtime import invoke\n\n");
    for mod_name in &top_level {
        out.push_str(&format!("from . import {}\n", mod_name));
    }
    out.push_str("\n");
    out.push_str(
        "def solve(path_id, target, **inputs):\n    \"\"\"Generic equation solve fallback.\n\nArgs:\n  path_id: equation id (for example 'fluids.reynolds_number')\n  target: solve target variable key\n  **inputs: named solve inputs\nReturns:\n  f64\n\"\"\"\n    args = {\"path_id\": path_id, \"target\": target}\n    args.update(inputs)\n    return invoke(\"equation.solve\", args)\n\n",
    );
    out.push_str("__all__ = [\n");
    for mod_name in &top_level {
        out.push_str(&format!("    \"{}\",\n", mod_name));
    }
    out.push_str("    \"solve\",\n");
    out.push_str("]\n");
    out
}

fn write_python_equation_modules(manifest: &BindingManifest, equations_root: &Path) -> Result<()> {
    let mut by_module: BTreeMap<String, Vec<&BindingFunction>> = BTreeMap::new();
    for f in &manifest.functions {
        if f.python_module.starts_with("equations.") {
            by_module
                .entry(f.python_module.clone())
                .or_default()
                .push(f);
        }
    }

    let mut package_children_pkgs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut package_children_mods: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut module_functions: BTreeMap<String, Vec<&BindingFunction>> = BTreeMap::new();

    for (module, funcs) in &by_module {
        let rel = module.trim_start_matches("equations.");
        let parts: Vec<String> = rel.split('.').map(snake_case).collect();
        if parts.is_empty() {
            continue;
        }
        let module_name = parts.last().cloned().unwrap_or_default();
        let pkg = if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        };
        package_children_mods
            .entry(pkg.clone())
            .or_default()
            .insert(module_name.clone());
        for i in 1..parts.len() {
            let parent = parts[..i - 1].join(".");
            package_children_pkgs
                .entry(parent)
                .or_default()
                .insert(parts[i - 1].clone());
        }
        module_functions.insert(rel.to_string(), funcs.clone());
    }

    for (module_rel, funcs) in &module_functions {
        let parts: Vec<String> = module_rel.split('.').map(snake_case).collect();
        let mut path = equations_root.to_path_buf();
        for p in &parts[..parts.len() - 1] {
            path = path.join(p);
        }
        let file = path.join(format!("{}.py", parts[parts.len() - 1]));
        let mut out = String::new();
        out.push_str("from engpy._runtime import invoke\n\n");
        for f in funcs {
            let mut params = Vec::new();
            let mut payload_parts = Vec::new();
            for a in &f.args {
                let p = snake_case(&a.name);
                if a.name == "branch" {
                    params.push(format!("{p}=None"));
                    payload_parts.push(format!(
                        "**({{\"{}\": {}}} if {} is not None else {{}})",
                        a.name, p, p
                    ));
                } else {
                    params.push(p.clone());
                    payload_parts.push(format!("\"{}\": {}", a.name, p));
                }
            }
            payload_parts.extend(
                f.fixed_args
                    .iter()
                    .map(|(k, v)| format!("\"{}\": \"{}\"", k, v)),
            );
            out.push_str(&format!(
                "def {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
                f.python_name,
                params.join(", "),
                render_python_docstring(f).replace('\"', "'"),
                f.op,
                payload_parts.join(", ")
            ));
        }
        out.push_str("__all__ = [\n");
        for f in funcs {
            out.push_str(&format!("    \"{}\",\n", f.python_name));
        }
        out.push_str("]\n");
        write_text(file, &out)?;
    }

    let mut packages: BTreeSet<String> = BTreeSet::new();
    for k in package_children_pkgs.keys() {
        packages.insert(k.clone());
    }
    for k in package_children_mods.keys() {
        packages.insert(k.clone());
    }

    for pkg in packages {
        if pkg.is_empty() {
            continue;
        }
        let mut out = String::new();
        let child_pkgs = package_children_pkgs.get(&pkg).cloned().unwrap_or_default();
        let child_mods = package_children_mods.get(&pkg).cloned().unwrap_or_default();
        for child in &child_pkgs {
            out.push_str(&format!("from . import {}\n", child));
        }
        for m in &child_mods {
            out.push_str(&format!("from . import {}\n", m));
        }

        // Migration-safe aliases: only for category packages (depth=1, non-family/meta),
        // and only when alias names are unique.
        let depth = if pkg.is_empty() {
            0
        } else {
            pkg.split('.').count()
        };
        if depth == 1 && pkg != "families" && pkg != "meta" {
            let mut alias_counts: BTreeMap<String, usize> = BTreeMap::new();
            let mut alias_src: BTreeMap<String, (String, String)> = BTreeMap::new();
            for m in &child_mods {
                let module_rel = format!("{}.{}", pkg, m);
                if let Some(funcs) = module_functions.get(&module_rel) {
                    for f in funcs {
                        *alias_counts.entry(f.python_name.clone()).or_insert(0) += 1;
                        alias_src.insert(f.python_name.clone(), (m.clone(), f.python_name.clone()));
                    }
                }
            }
            let mut omitted = Vec::new();
            for (name, count) in alias_counts {
                if count == 1 {
                    if let Some((m, src_name)) = alias_src.get(&name) {
                        out.push_str(&format!("from .{} import {} as {}\n", m, src_name, name));
                    }
                } else {
                    omitted.push(name);
                }
            }
            if !omitted.is_empty() {
                out.push_str("\n# Omitted legacy aliases due to collisions:\n");
                for name in omitted {
                    out.push_str(&format!("# - {}\n", name));
                }
            }
        }

        let mut all_entries = Vec::new();
        for child in child_pkgs {
            all_entries.push(child);
        }
        for m in child_mods {
            all_entries.push(m);
        }
        if !all_entries.is_empty() {
            out.push_str("\n__all__ = [\n");
            for name in all_entries {
                out.push_str(&format!("    \"{}\",\n", name));
            }
            out.push_str("]\n");
        }

        let mut p = equations_root.to_path_buf();
        for seg in pkg.split('.') {
            p = p.join(seg);
        }
        let file = p.join("__init__.py");
        write_text(file, &out)?;
    }

    Ok(())
}

fn render_xloil_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str(
        "try:\n    import xloil\nexcept Exception:\n    class _X:\n        @staticmethod\n        def func(*args, **kwargs):\n            def _d(f):\n                return f\n            return _d\n    xloil = _X()\n\n",
    );
    out.push_str("from engpy._runtime import invoke\n\n");
    for f in &manifest.functions {
        let mut params: Vec<String> = Vec::new();
        let mut kwargs = Vec::new();
        for a in &f.args {
            let p = excel_param_name(f, a);
            if a.name == "branch" {
                params.push(format!("{p}=\"\""));
                kwargs.push(format!(
                    "**({{\"{}\": {}}} if {} not in (None, \"\") else {{}})",
                    a.name, p, p
                ));
            } else {
                params.push(p.clone());
                kwargs.push(format!("\"{}\": {}", a.name, p));
            }
        }
        let mut payload_parts = f
            .fixed_args
            .iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<_>>();
        if f.op.starts_with("study.") {
            payload_parts.push("\"output\": \"spill\"".to_string());
        }
        if !kwargs.is_empty() {
            payload_parts.extend(kwargs);
        }
        out.push_str(&format!(
            "@xloil.func(name=\"{}\", help=\"{}\")\ndef {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
            f.excel_name,
            render_excel_help_block(f).replace('\"', "'"),
            snake_case(&f.excel_name),
            params.join(", "),
            render_excel_help_block(f).replace('\"', "'"),
            f.op,
            payload_parts.join(", ")
        ));
    }
    out
}

fn render_pyxll_module(manifest: &BindingManifest) -> String {
    let mut out = String::new();
    out.push_str(
        "try:\n    from pyxll import xl_func\nexcept Exception:\n    def xl_func(*args, **kwargs):\n        def _d(f):\n            return f\n        return _d\n\n",
    );
    out.push_str("from engpy._runtime import invoke\n\n");
    for f in &manifest.functions {
        let mut params: Vec<String> = Vec::new();
        let mut kwargs = Vec::new();
        for a in &f.args {
            let p = excel_param_name(f, a);
            if a.name == "branch" {
                params.push(format!("{p}=\"\""));
                kwargs.push(format!(
                    "**({{\"{}\": {}}} if {} not in (None, \"\") else {{}})",
                    a.name, p, p
                ));
            } else {
                params.push(p.clone());
                kwargs.push(format!("\"{}\": {}", a.name, p));
            }
        }
        let mut payload_parts = f
            .fixed_args
            .iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<_>>();
        if f.op.starts_with("study.") {
            payload_parts.push("\"output\": \"spill\"".to_string());
        }
        if !kwargs.is_empty() {
            payload_parts.extend(kwargs);
        }
        out.push_str(&format!(
            "@xl_func(name=\"{}\", doc=\"{}\")\ndef {}({}):\n    \"\"\"{}\"\"\"\n    return invoke(\"{}\", {{{}}})\n\n",
            f.excel_name,
            render_excel_help_block(f).replace('\"', "'"),
            snake_case(&f.excel_name),
            params.join(", "),
            render_excel_help_block(f).replace('\"', "'"),
            f.op,
            payload_parts.join(", ")
        ));
    }
    out
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
        // Invariant: MathJax must remain enabled. The handbook equation pages emit LaTeX
        // (`$$...$$` and `\(...\)`), and disabling this breaks core docs rendering.
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
    write_text(src.join("solve/index.md"), &render_solve_layer_page())?;
    write_text(src.join("studies/index.md"), &render_studies_page())?;
    write_text(src.join("bindings/index.md"), &render_bindings_guide())?;
    write_text(
        src.join("yaml_authoring/index.md"),
        &render_yaml_authoring(),
    )?;
    write_text(src.join("devices/guide.md"), &render_devices_guide())?;
    write_text(src.join("devices/index.md"), &render_devices_index(c))?;
    write_text(
        src.join("validation_trust/index.md"),
        &render_validation_trust(),
    )?;
    write_text(src.join("equations/guide.md"), &render_equations_guide())?;
    write_text(
        src.join("equations/families/index.md"),
        &render_families_index(c),
    )?;
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
    for d in &c.devices {
        write_text(
            src.join("devices")
                .join(format!("{}.md", snake_case(&d.key))),
            &render_device_page(d),
        )?;
    }
    for f in &c.fluids {
        write_text(
            src.join("fluids")
                .join(format!("{}.md", snake_case(&f.key))),
            &render_fluid_page(f, c),
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
    md.push_str("- [Engineering Solve Layer](solve/index.md)\n\n");
    md.push_str("- [Studies and Parameter Sweeps](studies/index.md)\n\n");
    md.push_str("- [Bindings (Python/Excel)](bindings/index.md)\n\n");
    md.push_str("## Domain Guides\n\n");
    md.push_str("- [Equations Guide](equations/guide.md)\n");
    md.push_str("- [Devices Guide](devices/guide.md)\n");
    md.push_str("- [Fluids Guide](fluids/guide.md)\n");
    md.push_str("- [Materials Guide](materials/guide.md)\n");
    md.push_str("- [Constants](constants/index.md)\n");
    md.push_str("- [YAML Authoring](yaml_authoring/index.md)\n");
    md.push_str("- [Validation / Trust](validation_trust/index.md)\n");
    md.push_str("- [Architecture Overview](architecture/index.md)\n\n");
    md.push_str("## Catalog\n\n");
    md.push_str("- [Equations](equations/index.md)\n");
    md.push_str("- [Equation Families](equations/families/index.md)\n");
    md.push_str("- [Devices](devices/index.md)\n");
    md.push_str("- [Fluids](fluids/index.md)\n");
    md.push_str("- [Materials](materials/index.md)\n");
    md.push_str(&format!(
        "\n**Library size:** {} equations, {} constants, {} devices, {} fluids, {} materials.\n",
        c.equations.page_models.len(),
        c.equations.constants.len(),
        c.devices.len(),
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
    md.push_str(
        "Atomic equations are scalar-first relations with strong validation and tests.\n\n",
    );
    md.push_str("- [Equation Catalog](./index.md)\n");
    md.push_str("- [Equation Families](./families/index.md)\n\n");
    md.push_str("Family pages and variant pages are equation-scoped and cross-linked from each equation page.\n");
    md
}

fn render_fluids_guide() -> String {
    let mut md = String::new();
    md.push_str("# Fluids Guide\n\n");
    md.push_str("Fluids are first-class engineering objects with typed wrappers, explicit state constructors, a flexible generic state path, direct property accessors, and context integration with equations.\n\n");
    md.push_str("## Recommended Usage Path\n\n");
    md.push_str("- **Preferred fast path**: explicit constructors like `state_tp`, `state_ph`, `state_ps`, `state_rho_h`, `state_pq`, `state_tq`.\n");
    md.push_str("- **Flexible path**: generic `state(\"T\", value, \"P\", value)` where property identity is explicit.\n");
    md.push_str("- Use direct accessor methods (`rho()`, `mu()`, `cp()`, `gamma()`, ...) instead of generic string property reads when writing Rust code.\n\n");
    md.push_str("## Constructor Capability Matrix\n\n");
    md.push_str("| Constructor | Meaning | Typical Use |\n");
    md.push_str("| --- | --- | --- |\n");
    md.push_str("| `state_tp(T, P)` | Temperature + pressure | Most common fast path |\n");
    md.push_str(
        "| `state_ph(P, h)` | Pressure + specific enthalpy | Thermodynamic inversion workflows |\n",
    );
    md.push_str("| `state_ps(P, s)` | Pressure + specific entropy | Isentropic/entropy constrained workflows |\n");
    md.push_str("| `state_rho_h(rho, h)` | Density + specific enthalpy | Density-driven model integration |\n");
    md.push_str("| `state_pq(P, Q)` | Pressure + quality | Two-phase saturation states |\n");
    md.push_str("| `state_tq(T, Q)` | Temperature + quality | Two-phase saturation states |\n");
    md.push_str("| `state(\"T\", v1, \"P\", v2)` | Explicit property-name pair | Flexible bindings/CLI/file paths |\n\n");
    md.push_str("## Generic Property Alias Map\n\n");
    md.push_str("| Canonical | Accepted aliases |\n");
    md.push_str("| --- | --- |\n");
    md.push_str("| Temperature | `T`, `temperature` |\n");
    md.push_str("| Pressure | `P`, `pressure` |\n");
    md.push_str("| Density | `rho`, `density` |\n");
    md.push_str("| Specific enthalpy | `h`, `enthalpy` |\n");
    md.push_str("| Specific entropy | `s`, `entropy` |\n");
    md.push_str("| Quality | `Q`, `quality`, `x` |\n\n");
    md.push_str("Property identity is required in the generic path; there is no unit-only inference. This avoids ambiguity (for example `h` vs `u`).\n\n");
    md.push_str("## Verified State Construction Examples\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_STATE_CONSTRUCTORS.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Direct Property Accessors\n\n");
    md.push_str("| Accessor | Meaning |\n");
    md.push_str("| --- | --- |\n");
    md.push_str("| `pressure()` / `p()` | Pressure (Pa) |\n");
    md.push_str("| `temperature()` / `t()` | Temperature (K) |\n");
    md.push_str("| `density()` / `rho()` | Density (kg/m^3) |\n");
    md.push_str("| `dynamic_viscosity()` / `mu()` | Dynamic viscosity (Pa*s) |\n");
    md.push_str("| `thermal_conductivity()` / `k()` | Thermal conductivity (W/(m*K)) |\n");
    md.push_str("| `specific_heat_capacity()` / `cp()` | Cp (J/(kg*K)) |\n");
    md.push_str("| `specific_heat_capacity_cv()` / `cv()` | Cv (J/(kg*K)) |\n");
    md.push_str("| `gamma()` | Heat capacity ratio |\n");
    md.push_str("| `speed_of_sound()` / `a()` | Speed of sound (m/s) |\n");
    md.push_str("| `specific_enthalpy()` / `h()` | Specific enthalpy (J/kg) |\n");
    md.push_str("| `specific_entropy()` / `s()` | Specific entropy (J/(kg*K)) |\n");
    md.push_str("| `quality()` | Quality in `[0,1]` for Q-based states |\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_FLUID_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Quality, Saturation, and State Metadata\n\n");
    md.push_str("- `saturation_at_pressure(P)` returns `{ liquid, vapor }`\n");
    md.push_str("- `saturation_at_temperature(T)` returns `{ liquid, vapor }`\n");
    md.push_str("- State metadata includes `fluid_key()`, `fluid_name()`, `input_pair()`, `input_pair_label()`, `normalized_inputs()`, `quality()`, and `phase()`\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_FLUID_SATURATION_METADATA.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Equation Context Integration\n\n");
    md.push_str("Use direct property lookup when you need one-off values. Use `solve_with_context(...).fluid(state)` when an equation should auto-resolve fluid-dependent variables.\n\n");
    md.push_str("Use fluid states directly in context solves:\n\n```rust\n");
    md.push_str(SNIPPET_CONTEXT_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Common Errors and Gotchas\n\n");
    md.push_str("- Unknown/invalid generic property keys are rejected with explicit supported-key guidance.\n");
    md.push_str("- Unsupported input pairs (for example `rho,T`) return clear pair diagnostics listing supported pairs.\n");
    md.push_str("- `u`/internal-energy identifiers are intentionally rejected in the generic state-input path to avoid confusion with enthalpy `h`.\n");
    md.push_str(
        "- Parse and backend failures are recoverable and include fluid/pair/property context.\n\n",
    );
    md.push_str("## Catalog\n\n- [Fluids Catalog](./index.md)\n");
    md
}

fn render_materials_guide() -> String {
    let mut md = String::new();
    md.push_str("# Materials Guide\n\n");
    md.push_str(
        "Materials provide temperature-conditioned property lookup from curated datasets.\n\n",
    );
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
    md.push_str("4. **Solve Workflow Layer**: reusable numeric roots/ODE wrappers plus station/workflow chaining and provenance.\n");
    md.push_str("5. **Solve Graph / Chaining**: node/edge orchestration connecting equations, components, constants, and property sources.\n");
    md.push_str("6. **External Bindings**: generated Python/Excel surfaces over Rust-owned implementations.\n\n");

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
    md.push_str("\n```\n\n");

    md.push_str("## 8. Device Workflow: Pipe Pressure Drop (Fixed f)\n\n```rust\n");
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_FIXED.trim());
    md.push_str("\n```\n\n");

    md.push_str("## 9. Device Workflow: Pipe Pressure Drop (Colebrook Direct Inputs)\n\n```rust\n");
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_DIRECT.trim());
    md.push_str("\n```\n\n");

    md.push_str(
        "## 10. Device Workflow: Pipe Pressure Drop (Colebrook + Fluid Context)\n\n```rust\n",
    );
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_FLUID.trim());
    md.push_str("\n```\n\n");
    md.push_str("## 11. Solve Layer Workflow: Nozzle -> Normal Shock Station Chain\n\n```rust\n");
    md.push_str("use eng::devices::NozzleFlowBranch;\n");
    md.push_str(
        "use eng::solve::{NozzleShockWorkflowRequest, run_nozzle_normal_shock_workflow};\n\n",
    );
    md.push_str("let out = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {\n");
    md.push_str("    gamma: 1.4,\n");
    md.push_str("    area_ratio: 2.0,\n");
    md.push_str("    nozzle_branch: NozzleFlowBranch::Supersonic,\n");
    md.push_str("})?;\n");
    md.push_str("println!(\"M_pre={}, M_post={}\", out.pre_shock_mach, out.post_shock_mach);\n");
    md.push_str("println!(\"trace={}\", out.path_text());\n");
    md.push_str("```\n");
    md
}

fn render_solve_layer_page() -> String {
    let mut md = String::new();
    md.push_str("# Engineering Solve Layer\n\n");
    md.push_str("`eng::solve` is the canonical home for reusable multi-step solve/workflow infrastructure.\n\n");
    md.push_str("## Ownership Scope\n\n");
    md.push_str("- Shared numeric root solve wrappers with convergence diagnostics\n");
    md.push_str("- Shared ODE step wrappers for engineering integrations\n");
    md.push_str("- Station/state chaining and step provenance records\n");
    md.push_str("- Workflow-level warnings and structured errors\n\n");
    md.push_str("## Out of Scope\n\n");
    md.push_str("- Atomic equation definitions (stay in YAML/registry)\n");
    md.push_str("- Device-specific binding naming/polish logic\n");
    md.push_str("- Full arbitrary graph optimization engines\n\n");
    md.push_str("## Rust Example\n\n```rust\n");
    md.push_str("use eng::devices::NozzleFlowBranch;\n");
    md.push_str(
        "use eng::solve::{NozzleShockWorkflowRequest, run_nozzle_normal_shock_workflow};\n\n",
    );
    md.push_str("let chain = run_nozzle_normal_shock_workflow(NozzleShockWorkflowRequest {\n");
    md.push_str("    gamma: 1.4,\n");
    md.push_str("    area_ratio: 2.0,\n");
    md.push_str("    nozzle_branch: NozzleFlowBranch::Supersonic,\n");
    md.push_str("})?;\n");
    md.push_str("println!(\"station trace: {}\", chain.path_text());\n");
    md.push_str("```\n\n");
    md.push_str("## Standardized Numeric Homes\n\n");
    md.push_str("- `eng::solve::numeric`: bracketed root solve and scan+bisect helpers.\n");
    md.push_str("- `eng::solve::ode`: reusable RK4 stepping wrapper.\n\n");
    md.push_str("Conical-shock Taylor-Maccoll stepping and branch-sensitive oblique/conical inversions use this layer.\n");
    md
}

fn render_studies_page() -> String {
    let mut md = String::new();
    md.push_str("# Studies and Parameter Sweeps\n\n");
    md.push_str("`eng::solve::study` is the standard subsystem for diagnostics-aware parameter studies across equations, devices, and solve-layer workflows.\n\n");
    md.push_str("## Scope\n\n");
    md.push_str("- 1D sweeps (`values`, `linspace`, `logspace`)\n");
    md.push_str("- per-row status (`ok`/`failed`) without aborting the whole study\n");
    md.push_str("- table-first outputs suitable for Python and Excel spill ranges\n");
    md.push_str("- concise per-row path/provenance summaries\n\n");
    md.push_str("## Rust: Equation Study\n\n");
    md.push_str("```rust\n");
    md.push_str("use std::collections::BTreeMap;\n");
    md.push_str("use eng::solve::{EquationStudySpec, SweepAxis, run_equation_study};\n\n");
    md.push_str("let mut fixed = BTreeMap::new();\n");
    md.push_str("fixed.insert(\"gamma\".to_string(), 1.4);\n");
    md.push_str("let table = run_equation_study(&EquationStudySpec {\n");
    md.push_str("    path_id: \"compressible.isentropic_pressure_ratio\".to_string(),\n");
    md.push_str("    target: \"p_p0\".to_string(),\n");
    md.push_str("    sweep_variable: \"M\".to_string(),\n");
    md.push_str("    fixed_inputs: fixed,\n");
    md.push_str("    branch: None,\n");
    md.push_str("}, SweepAxis::linspace(0.2, 3.0, 21));\n");
    md.push_str("```\n\n");
    md.push_str("## Rust: Device Study\n\n");
    md.push_str("```rust\n");
    md.push_str("use eng::devices::NozzleFlowBranch;\n");
    md.push_str("use eng::solve::{SweepAxis, study_nozzle_flow_area_ratio};\n\n");
    md.push_str("let table = study_nozzle_flow_area_ratio(\n");
    md.push_str("    1.4,\n");
    md.push_str("    SweepAxis::linspace(1.2, 3.0, 20),\n");
    md.push_str("    NozzleFlowBranch::Supersonic,\n");
    md.push_str(");\n");
    md.push_str("```\n\n");
    md.push_str("## Rust: Workflow-Chain Study\n\n");
    md.push_str("```rust\n");
    md.push_str("use eng::devices::NozzleFlowBranch;\n");
    md.push_str("use eng::solve::{SweepAxis, study_nozzle_normal_shock_workflow};\n\n");
    md.push_str("let table = study_nozzle_normal_shock_workflow(\n");
    md.push_str("    1.4,\n");
    md.push_str("    SweepAxis::linspace(1.2, 3.0, 20),\n");
    md.push_str("    NozzleFlowBranch::Supersonic,\n");
    md.push_str(");\n");
    md.push_str("```\n\n");
    md.push_str("## Python / Excel (Targeted v1)\n\n");
    md.push_str("- Python module: `engpy.study`\n");
    md.push_str("- Excel spill-table helpers:\n");
    md.push_str("  - `ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(...)`\n");
    md.push_str("  - `ENG_STUDY_NOZZLE_FLOW_TABLE(...)`\n");
    md.push_str("  - `ENG_STUDY_NORMAL_SHOCK_TABLE(...)`\n");
    md.push_str("  - `ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(...)`\n\n");
    md.push_str("Each helper returns a structured payload with both a rich `table` object and `spill` rows suitable for worksheet charting.\n");
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

fn render_bindings_guide() -> String {
    let mut md = String::new();
    md.push_str("# Bindings (Python and Excel)\n\n");
    md.push_str("Rust remains the authoritative implementation. Generated Python and Excel bindings are thin adapters over the same public binding model.\n\n");
    md.push_str("## Generated Outputs\n\n");
    md.push_str("- `generated/bindings/binding_spec.json`\n");
    md.push_str("- `generated/bindings/invoke_protocol.json`\n");
    md.push_str("- `generated/bindings/python/engpy/...`\n");
    md.push_str("- `generated/bindings/python/engpy/study.py`\n");
    md.push_str(
        "- `generated/bindings/python/pyproject.toml` (maturin build config for `engpy_native`)\n",
    );
    md.push_str("- `generated/bindings/excel/eng_xloil.py`\n");
    md.push_str("- `generated/bindings/excel/eng_pyxll.py`\n\n");
    md.push_str("`binding_spec.json` is transport-agnostic (function names, args, returns, help, examples). `invoke_protocol.json` documents the current runtime transport contract.\n\n");
    md.push_str("## Naming Rules\n\n");
    md.push_str("- Python: namespaced modules (`engpy.equations.<category>.<equation_slug>.*`, `engpy.devices.*`, `engpy.fluids.*`, `engpy.materials.*`, `engpy.constants.*`).\n");
    md.push_str("- Excel: flat worksheet-friendly functions (`ENG_*`).\n");
    md.push_str("- Families are exposed under `engpy.equations.families.<family>.<variant>`.\n\n");
    md.push_str("## Metadata and Diagnostics Functions\n\n");
    md.push_str("- Python:\n");
    md.push_str("  - `engpy.equations.meta.equation_meta(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_ascii(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_unicode(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_latex(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_targets(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_variables(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_name(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_description(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_family(path_id)`\n");
    md.push_str("  - `engpy.equations.meta.equation_default_unit(path_id, variable)`\n");
    md.push_str("  - `engpy.helpers.equation_targets_text(path_id)`\n");
    md.push_str("  - `engpy.helpers.equation_variables_text(path_id)`\n");
    md.push_str("  - `engpy.helpers.equation_branches_text(path_id)`\n");
    md.push_str("  - `engpy.helpers.fluid_properties_text(key)`\n");
    md.push_str("  - `engpy.helpers.material_properties_text(key)`\n");
    md.push_str("  - `engpy.helpers.device_modes_text(key)`\n");
    md.push_str("  - `engpy.helpers.equation_targets_table(path_id)`\n");
    md.push_str("  - `engpy.helpers.equation_variables_table(path_id)`\n");
    md.push_str("  - `engpy.helpers.equation_branches_table(path_id)`\n");
    md.push_str("  - `engpy.helpers.fluid_properties_table(key)`\n");
    md.push_str("  - `engpy.helpers.material_properties_table(key)`\n");
    md.push_str("  - `engpy.helpers.equation_target_count(path_id)`\n");
    md.push_str("  - `engpy.helpers.equation_variable_count(path_id)`\n");
    md.push_str("  - `engpy.helpers.fluid_property_count(key)`\n");
    md.push_str("  - `engpy.helpers.material_property_count(key)`\n");
    md.push_str("  - `engpy.helpers.device_mode_count(key)`\n");
    md.push_str("  - `engpy.helpers.format_value(value, in_unit, out_unit)`\n");
    md.push_str("  - `engpy.helpers.meta_get(entity, key, field)`\n");
    md.push_str("  - `engpy.study.equation_sweep_table(...)`\n");
    md.push_str("  - `engpy.study.isentropic_m_to_p_p0_table(...)`\n");
    md.push_str("  - `engpy.study.nozzle_flow_table(...)`\n");
    md.push_str("  - `engpy.study.normal_shock_table(...)`\n");
    md.push_str("  - `engpy.study.nozzle_normal_shock_workflow_table(...)`\n");
    md.push_str("- Excel:\n");
    md.push_str("  - `ENG_FORMAT(value, in_unit, out_unit)`\n");
    md.push_str("  - `ENG_META(entity, key, field)`\n");
    md.push_str("  - `ENG_EQUATION_META(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_ASCII(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_UNICODE(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_LATEX(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_TARGETS(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_VARIABLES(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_NAME(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_DESCRIPTION(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_FAMILY(path_id)`\n");
    md.push_str("  - `ENG_EQUATION_DEFAULT_UNIT(path_id, variable)`\n\n");
    md.push_str("  - `ENG_STUDY_ISENTROPIC_M_TO_P_P0_TABLE(gamma, start, end, count, [branch])`\n");
    md.push_str("  - `ENG_STUDY_NOZZLE_FLOW_TABLE(gamma, start, end, count, branch)`\n");
    md.push_str("  - `ENG_STUDY_NORMAL_SHOCK_TABLE(gamma, start, end, count)`\n");
    md.push_str(
        "  - `ENG_STUDY_NOZZLE_NORMAL_SHOCK_WORKFLOW_TABLE(gamma, start, end, count, branch)`\n\n",
    );
    md.push_str(
        "  - `ENG_EQUATION_TARGETS_TEXT(path_id)` / `ENG_EQUATION_VARIABLES_TEXT(path_id)`\n",
    );
    md.push_str("  - `ENG_EQUATION_BRANCHES_TEXT(path_id)`\n");
    md.push_str("  - `ENG_FLUID_PROPERTIES_TEXT(fluid_key)` / `ENG_MATERIAL_PROPERTIES_TEXT(material_key)` / `ENG_DEVICE_MODES_TEXT(device_key)`\n");
    md.push_str(
        "  - `ENG_EQUATION_TARGETS_TABLE(path_id)` / `ENG_EQUATION_VARIABLES_TABLE(path_id)`\n",
    );
    md.push_str("  - `ENG_EQUATION_BRANCHES_TABLE(path_id)`\n");
    md.push_str("  - `ENG_FLUID_PROPERTIES_TABLE(fluid_key)` / `ENG_MATERIAL_PROPERTIES_TABLE(material_key)`\n");
    md.push_str(
        "  - `ENG_EQUATION_TARGET_COUNT(path_id)` / `ENG_EQUATION_VARIABLE_COUNT(path_id)`\n",
    );
    md.push_str("  - `ENG_FLUID_PROPERTY_COUNT(fluid_key)` / `ENG_MATERIAL_PROPERTY_COUNT(material_key)` / `ENG_DEVICE_MODE_COUNT(device_key)`\n\n");
    md.push_str("Delimited TEXT helpers use `; ` as a deterministic separator.\n\n");
    md.push_str("Use these helpers for a composable workflow: keep core engineering calls simple, then layer formatting/reference metadata as needed.\n\n");
    md.push_str("### Clean Excel Pattern\n\n");
    md.push_str("```excel\n=ENG_HOOP_STRESS_SIGMA_H(P, r, t)\n=ENG_FORMAT(ENG_HOOP_STRESS_SIGMA_H(P, r, t), \"Pa\", \"psia\")\n=ENG_EQUATION_ASCII(\"structures.hoop_stress\")\n=ENG_META(\"equation\", \"structures.hoop_stress\", \"targets\")\n```\n\n");
    md.push_str("### Excel single-cell text helpers\n\n");
    md.push_str("```excel\n=ENG_EQUATION_TARGETS_TEXT(\"structures.hoop_stress\")\n=ENG_EQUATION_VARIABLES_TEXT(\"structures.hoop_stress\")\n=ENG_EQUATION_BRANCHES_TEXT(\"compressible.area_mach\")\n=ENG_FLUID_PROPERTIES_TEXT(\"H2O\")\n=ENG_MATERIAL_PROPERTIES_TEXT(\"stainless_304\")\n=ENG_DEVICE_MODES_TEXT(\"pipe_loss\")\n```\n\n");
    md.push_str("### Excel spill-range table helpers\n\n");
    md.push_str("```excel\n=ENG_EQUATION_VARIABLES_TABLE(\"structures.hoop_stress\")\n=ENG_EQUATION_TARGETS_TABLE(\"structures.hoop_stress\")\n=ENG_EQUATION_BRANCHES_TABLE(\"compressible.area_mach\")\n=ENG_FLUID_PROPERTIES_TABLE(\"H2O\")\n=ENG_MATERIAL_PROPERTIES_TABLE(\"stainless_304\")\n```\n\n");
    md.push_str("Native in-process runtime supports Python usage on Linux and Windows without requiring a platform-specific executable per call.\n\n");
    md.push_str("## Build / Install (Native Python Runtime)\n\n");
    md.push_str("- From `generated/bindings/python`, build/install the extension with maturin (for example `maturin develop` in an active Python environment).\n");
    md.push_str("- Generated `engpy` wrappers call `engpy_native` in-process by default.\n");
    md.push_str(
        "- If the extension is unavailable, wrappers automatically fall back to worker mode.\n\n",
    );
    md.push_str("### One-command Setup Helpers\n\n");
    md.push_str("- Windows: `scripts/setup-native-bindings.ps1`\n");
    md.push_str("- Linux: `scripts/setup-native-bindings.sh`\n");
    md.push_str("- Runtime verification: `scripts/verify-native-bindings.ps1` or `scripts/verify-native-bindings.sh`\n\n");
    md.push_str("These scripts create/use a virtual environment, install `maturin`, install `engpy_native`, then verify runtime mode.\n\n");
    md.push_str("## Excel Function Help and Intellisense\n\n");
    md.push_str("- Generated xlOil/PyXLL functions now include richer help text: summary, per-argument guidance, return info, and a formula example.\n");
    md.push_str("- Argument names are optimized for Excel readability on binding-friendly functions (for example pipe-loss uses `density`, `viscosity`, `roughness` instead of terse symbols).\n");
    md.push_str(
        "- Function Wizard help is the primary native Excel help surface for custom functions.\n",
    );
    md.push_str("- Excel custom function inline IntelliSense popups can be limited natively; when richer inline tooltip behavior is needed, use an IntelliSense add-in path (for example Excel-DNA IntelliSense with compatible workflows).\n");
    md.push_str("- Shortcut: `Ctrl+Shift+A` inserts function arguments into a worksheet formula to make argument order explicit.\n\n");
    md.push_str("## Runtime Protocol and Transport\n\n");
    md.push_str(
        "- Default runtime is in-process via native Rust/Python extension module (`engpy_native`).\n",
    );
    md.push_str(
        "- Compatibility fallback runtime is persistent `eng worker` over stdio JSON-lines.\n",
    );
    md.push_str("- The per-call envelope is unchanged: `protocol_version`, `op`, optional `request_id`, `args`.\n");
    md.push_str("- Generated Python runtime prefers native in-process mode, and only uses worker when native is unavailable or `ENGPY_RUNTIME=worker` is set.\n");
    md.push_str("- On Windows worker fallback startup is hidden (`CREATE_NO_WINDOW`) to avoid console popup windows during Excel recalculation.\n");
    md.push_str("- Worker fallback executable resolution: `ENG_WORKER_BIN` (fallback: `ENG_BIN`, then `eng`).\n");
    md.push_str("- Runtime preference override: `ENGPY_RUNTIME=native|worker`.\n");
    md.push_str("- Success response: `ok=true`, `value`, plus echoed `protocol_version`/`op`/`request_id`.\n");
    md.push_str("- Error response: `ok=false`, `error.code`, `error.message`, optional `error.field` and `error.detail`.\n");
    md.push_str("- Excel docs show one formula surface because xlOil and PyXLL are generated identically.\n");
    md.push_str("- No engineering physics logic is implemented in generated Python modules.\n");
    md.push_str("- Equation/device/fluid/material/constant behavior remains in Rust.\n\n");
    md.push_str("## Error Model (Bindings)\n\n");
    md.push_str("- Stable error `code` values are intended for wrapper logic and automation.\n");
    md.push_str(
        "- Human-facing `message` remains suitable for worksheet/Python troubleshooting.\n",
    );
    md.push_str("- `field` and `detail` provide argument-level and operation context.\n");
    md.push_str("\n## Runtime Troubleshooting\n\n");
    md.push_str(
        "- Use `engpy._runtime.runtime_mode()` to confirm active runtime (`native` or `worker`).\n",
    );
    md.push_str("- Use `engpy._runtime.worker_stats()` to inspect runtime request counters, last failure, and worker PID when fallback is active.\n");
    md.push_str(
        "- Use `engpy._runtime.worker_pid()` for quick worker PID checks in fallback mode.\n",
    );
    md.push_str("- If needed, call `engpy._runtime.stop_worker()` to force a clean worker restart on next request (no-op in native mode).\n");
    md.push_str("- If mode is unexpectedly `worker`, verify `engpy_native` imports in the same Python environment used by xlOil/PyXLL.\n");
    md.push_str("- Runtime preference override: `ENGPY_RUNTIME=native|worker`.\n");
    md.push_str("- Worker fallback executable overrides: `ENG_WORKER_BIN` and `ENG_BIN`.\n");
    md.push_str("\nVerification snippet:\n\n```python\nimport engpy_native\nimport engpy._runtime as rt\nprint(rt.runtime_mode())\nprint(engpy_native.runtime_info())\n```\n");
    md.push_str("\n## CI vs Local Verification\n\n");
    md.push_str("- CI/repo checks validate generated binding artifacts, protocol/schema, docs generation, and runtime diagnostics surfaces.\n");
    md.push_str("- Native environment activation (`maturin develop` + import in target interpreter) is machine/environment-specific and should be validated with the setup/verify scripts on each dev machine.\n");
    md
}

fn render_binding_examples_for_equation(p: &EquationPageModel) -> String {
    let target = p
        .default_target
        .clone()
        .or_else(|| p.solve_targets.first().map(|t| t.target.clone()))
        .unwrap_or_else(|| "value".to_string());
    let args = equation_args_for_target(p, &target);
    let eq_slug = p
        .path_id
        .rsplit('.')
        .next()
        .map(snake_case)
        .unwrap_or_else(|| snake_case(&p.path_id));
    let python_module = format!(
        "engpy.equations.{}.{}",
        snake_case(&p.category).replace('-', "_"),
        eq_slug
    );
    let python_fn = format!("solve_{}", snake_case(&target));
    let excel_fn = format!(
        "ENG_{}_{}",
        p.path_id.replace('.', "_").to_ascii_uppercase(),
        target.to_ascii_uppercase()
    );
    let mut branch_block = String::new();
    if !p.branches.is_empty() {
        let preferred = p
            .branches
            .iter()
            .find(|b| b.preferred)
            .map(|b| b.name.clone())
            .unwrap_or_else(|| p.branches[0].name.clone());
        let branch_list = p
            .branches
            .iter()
            .map(|b| format!("`{}`", b.name))
            .collect::<Vec<_>>()
            .join(", ");
        branch_block.push_str(&format!(
            "\n**Branch behavior**\n- Default solver behavior uses preferred branch (`{preferred}`) when one is marked.\n- Supported branches: {branch_list}\n\n### Python (explicit branch)\n```python\n{python_module}.{python_fn}({py_args}, branch=\"{preferred}\")\n```\n\n### Excel (explicit branch)\n```excel\n={excel_fn}({xl_args},\"{preferred}\")\n=ENG_EQUATION_BRANCHES_TEXT(\"{path}\")\n=ENG_EQUATION_BRANCHES_TABLE(\"{path}\")\n```\n",
            py_args = render_python_example_args(&args),
            xl_args = render_excel_example_args(&args),
            path = p.path_id
        ));
    }
    format!(
        "## Bindings\n\n### Rust\n```rust\nlet value = eq.solve(equations::{}::equation()).for_target(\"{}\").value()?;\n```\n\n### Python\n```python\n{}.{}({})\n# helper layer\nengpy.helpers.format_value({}.{}({}), \"<in_unit>\", \"<out_unit>\")\nengpy.equations.meta.equation_ascii(\"{}\")\nengpy.helpers.equation_targets_text(\"{}\")\nengpy.helpers.equation_variables_table(\"{}\")\nengpy.helpers.equation_target_count(\"{}\")\n```\n\n### Excel\n```excel\n={}({})\n=ENG_FORMAT({}({}),\"<in_unit>\",\"<out_unit>\")\n=ENG_EQUATION_ASCII(\"{}\")\n=ENG_EQUATION_TARGETS_TEXT(\"{}\")\n=ENG_EQUATION_VARIABLES_TABLE(\"{}\")\n=ENG_EQUATION_TARGET_COUNT(\"{}\")\n```\n\n**Excel arguments**\n{}\n",
        p.path_id.replace('.', "::"),
        target,
        python_module,
        python_fn,
        render_python_example_args(&args),
        python_module,
        python_fn,
        render_python_example_args(&args),
        p.path_id,
        p.path_id,
        p.path_id,
        p.path_id,
        excel_fn,
        render_excel_example_args(&args),
        excel_fn,
        render_excel_example_args(&args),
        p.path_id,
        p.path_id,
        p.path_id,
        p.path_id,
        render_binding_arg_bullets_for_excel_signature(&args, "equation.solve")
    ) + &branch_block
}

fn render_binding_examples_for_family(
    family: &equations::equation_families::EquationFamilyDef,
) -> String {
    let mut md = String::new();
    md.push_str("## Bindings\n\n");
    md.push_str("### Python\n\n");
    md.push_str("```python\n");
    md.push_str(&format!(
        "from engpy.equations.families import {}\n",
        snake_case(&family.key)
    ));
    if let Some(v) = family.variants.first() {
        md.push_str(&format!(
            "{}.{}.solve_<target>(...)\n",
            snake_case(&family.key),
            snake_case(&v.key),
        ));
    }
    md.push_str("```\n\n");
    md.push_str("### Excel\n\n");
    md.push_str("```excel\n=ENG_FAMILY_<FAMILY>_<VARIANT>_<TARGET>(...)\n```\n");
    md
}

fn render_binding_examples_for_device(device_key: &str) -> String {
    crate::devices::generation_specs()
        .into_iter()
        .find(|s| s.key == device_key)
        .map(|s| s.binding_markdown.to_string())
        .unwrap_or_default()
}

fn render_binding_examples_for_fluid() -> String {
    "## Bindings\n\n### Python\n```python\nengpy.fluids.fluid_prop(\"H2O\", \"T\", \"300 K\", \"P\", \"1 bar\", \"rho\")\nengpy.helpers.fluid_properties(\"H2O\")\nengpy.helpers.fluid_properties_text(\"H2O\")\nengpy.helpers.fluid_properties_table(\"H2O\")\nengpy.helpers.fluid_property_count(\"H2O\")\n```\n\n### Excel\n```excel\n=ENG_FLUID_PROP(\"H2O\",\"T\",\"300 K\",\"P\",\"1 bar\",\"rho\")\n=ENG_FLUID_PROPERTIES(\"H2O\")\n=ENG_FLUID_PROPERTIES_TEXT(\"H2O\")\n=ENG_FLUID_PROPERTIES_TABLE(\"H2O\")\n=ENG_FLUID_PROPERTY_COUNT(\"H2O\")\n```\n\n**Excel arguments**\n- `fluid`: fluid key or alias\n- `state_prop_1`, `state_value_1`: first state-defining property and value\n- `state_prop_2`, `state_value_2`: second state-defining property and value\n- `out_prop`: property to return (for example `rho`, `mu`, `cp`)\n".to_string()
}

fn render_binding_examples_for_material() -> String {
    "## Bindings\n\n### Python\n```python\nengpy.materials.mat_prop(\"stainless_304\", \"elastic_modulus\", \"350 K\")\nengpy.helpers.material_properties(\"stainless_304\")\nengpy.helpers.material_properties_text(\"stainless_304\")\nengpy.helpers.material_properties_table(\"stainless_304\")\nengpy.helpers.material_property_count(\"stainless_304\")\n```\n\n### Excel\n```excel\n=ENG_MAT_PROP(\"stainless_304\",\"elastic_modulus\",\"350 K\")\n=ENG_MATERIAL_PROPERTIES(\"stainless_304\")\n=ENG_MATERIAL_PROPERTIES_TEXT(\"stainless_304\")\n=ENG_MATERIAL_PROPERTIES_TABLE(\"stainless_304\")\n=ENG_MATERIAL_PROPERTY_COUNT(\"stainless_304\")\n```\n\n**Excel arguments**\n- `material`: material key or alias\n- `property_key`: material property key (for example `elastic_modulus`)\n- `temperature`: evaluation temperature (recommended with explicit units)\n".to_string()
}

fn render_binding_arg_bullets_for_excel_signature(args: &[BindingArg], op: &str) -> String {
    if args.is_empty() {
        return "- `(no args)`\n".to_string();
    }
    let mut out = String::new();
    for a in args {
        out.push_str(&format!(
            "- `{}`: {}\n",
            excel_param_name_for_op(op, &a.name),
            a.description.trim()
        ));
    }
    out
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
    md.push('\n');
    md.push_str(&render_binding_examples_for_family(f));
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
    md.push_str("Catalog-backed fluid wrappers with typed and generic state APIs.\n\n");
    md.push_str("- [Fluids Guide](./guide.md)\n\n");
    md.push_str("| Fluid | Key | Aliases | Supported State Inputs | Properties |\n");
    md.push_str("| --- | --- | --- | --- | --- |\n");
    for f in &c.fluids {
        md.push_str(&format!(
            "| [{}](./{}.md) | `{}` | {} | `{}` | {} |\n",
            f.name,
            snake_case(&f.key),
            f.key,
            if f.aliases.is_empty() {
                "-".to_string()
            } else {
                f.aliases
                    .iter()
                    .map(|a| format!("`{}`", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
            f.supported_state_inputs.join(", "),
            f.supported_properties.len()
        ));
    }
    md.push_str("\nEach fluid page includes constructor patterns, generic input aliases, direct accessor guidance, saturation helpers, and equation-context usage.\n");
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
    md.push_str("## Equation Table\n\n");
    md.push_str("| Equation | Category | Targets | Constants |\n");
    md.push_str("| --- | --- | --- | --- |\n");
    for page in &c.equations.page_models {
        let target_count = page.solve_targets.len();
        let constants = if page.uses_constants.is_empty() {
            "-".to_string()
        } else {
            page.uses_constants
                .iter()
                .map(|u| format!("`{}`", u.key))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let link = doc_routes::relative_doc_link(
            "equations/index.md",
            &doc_routes::equation_doc_path_from_path_id(&page.path_id),
        );
        md.push_str(&format!(
            "| [{}]({}) | {} | {} | {} |\n",
            page.name,
            link,
            title_case(&page.category),
            target_count,
            constants
        ));
    }
    md.push('\n');
    md.push_str("## Browse By Category\n\n");
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

fn render_fluid_page(f: &eng_fluids::FluidDocsEntry, c: &UnifiedDocsContribution) -> String {
    let aliases = if f.aliases.is_empty() {
        "none".to_string()
    } else {
        f.aliases.join(", ")
    };
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", f.name));
    md.push_str("| Field | Value |\n");
    md.push_str("| --- | --- |\n");
    md.push_str(&format!("| Key | `{}` |\n", f.key));
    md.push_str(&format!("| Aliases | {} |\n", aliases));
    md.push_str(&format!(
        "| Supported state inputs | `{}` |\n",
        f.supported_state_inputs.join(", ")
    ));
    md.push_str(&format!(
        "| Supported properties | {} |\n\n",
        f.supported_properties.len()
    ));

    md.push_str("## Supported State Input Pairs\n\n");
    md.push_str("| Pair | Notes |\n");
    md.push_str("| --- | --- |\n");
    for pair in &f.supported_state_inputs {
        let note = match pair.as_str() {
            "T,P" => "General purpose explicit state constructor (`state_tp`)",
            "P,h" => "Pressure/enthalpy inversion (`state_ph`)",
            "P,s" => "Pressure/entropy inversion (`state_ps`)",
            "rho,h" => "Density/enthalpy construction (`state_rho_h`)",
            "P,Q" => "Two-phase saturation by pressure (`state_pq`)",
            "T,Q" => "Two-phase saturation by temperature (`state_tq`)",
            _ => "Supported by backend model",
        };
        md.push_str(&format!("| `{}` | {} |\n", pair, note));
    }

    md.push_str("\n## Verified Constructor and Generic Examples\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_FLUID_STATE_CONSTRUCTORS.trim());
    md.push_str("```\n\n");

    md.push_str("## Generic Property Aliases\n\n");
    md.push_str("| Canonical | Aliases |\n");
    md.push_str("| --- | --- |\n");
    md.push_str("| Temperature | `T`, `temperature` |\n");
    md.push_str("| Pressure | `P`, `pressure` |\n");
    md.push_str("| Density | `rho`, `density` |\n");
    md.push_str("| Specific enthalpy | `h`, `enthalpy` |\n");
    md.push_str("| Specific entropy | `s`, `entropy` |\n");
    md.push_str("| Quality | `Q`, `quality`, `x` |\n\n");

    md.push_str("## Supported Property Keys\n\n");
    md.push_str("| Property key | Direct accessor |\n| --- | --- |\n");
    for p in &f.supported_properties {
        let accessor = match p.as_str() {
            "pressure" => "`pressure()`, `p()`",
            "temperature" => "`temperature()`, `t()`",
            "density" => "`density()`, `rho()`",
            "dynamic_viscosity" => "`dynamic_viscosity()`, `mu()`",
            "thermal_conductivity" => "`thermal_conductivity()`, `k()`",
            "specific_heat_capacity" => "`specific_heat_capacity()`, `cp()`",
            "specific_heat_capacity_cv" => "`specific_heat_capacity_cv()`, `cv()`",
            "speed_of_sound" => "`speed_of_sound()`, `a()`",
            "specific_enthalpy" => "`specific_enthalpy()`, `h()`",
            "specific_entropy" => "`specific_entropy()`, `s()`",
            "gamma" => "`gamma()`",
            "quality" => "`quality()`",
            _ => "`property_by_name(...)`",
        };
        md.push_str(&format!("| `{}` | {} |\n", p, accessor));
    }

    md.push_str("\n## Direct Property Access Example\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Saturation and Metadata Example\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_SATURATION_METADATA.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Using This Fluid With Equations\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_CONTEXT_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("### Equations currently using `fluid` context\n\n");
    let from = format!("fluids/{}.md", snake_case(&f.key));
    let mut listed = 0usize;
    for page in &c.equations.page_models {
        let uses_fluid = page
            .variables
            .iter()
            .any(|v| v.resolver_source.as_deref() == Some("fluid"));
        if uses_fluid {
            let to = doc_routes::equation_doc_path_from_path_id(&page.path_id);
            let link = doc_routes::relative_doc_link(&from, &to);
            md.push_str(&format!("- [{}]({})\n", page.name, link));
            listed += 1;
            if listed >= 12 {
                break;
            }
        }
    }
    if listed == 0 {
        md.push_str("- No current equation pages declare `fluid` resolver context.\n");
    }
    md.push_str("\n## Error Behavior\n\n");
    md.push_str(
        "- Unsupported input pairs return explicit pair diagnostics with a supported-pairs list.\n",
    );
    md.push_str("- Unknown/invalid generic property keys return actionable key guidance.\n");
    md.push_str(
        "- `u`/internal-energy keys are rejected intentionally to prevent ambiguity with `h`.\n",
    );
    md.push_str("- Backend failures are surfaced as recoverable structured errors with fluid/pair/property context.\n");
    md.push('\n');
    md.push_str(&render_binding_examples_for_fluid());
    md
}

fn render_material_page(m: &eng_materials::MaterialDocsEntry) -> String {
    let aliases = if m.aliases.is_empty() {
        "none".to_string()
    } else {
        m.aliases.join(", ")
    };
    let mut md = format!(
        "# {}\n\n- Key: `{}`\n- Aliases: {}\n- Source: {}\n- Properties: {}\n\n{}\n",
        m.name,
        m.key,
        aliases,
        m.source,
        m.properties.join(", "),
        m.description
    );
    md.push('\n');
    md.push_str(&render_binding_examples_for_material());
    md
}

fn render_category_index(cat: &CategoryPresentation) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", title_case(&cat.name)));
    md.push_str("## Equation Summary\n\n");
    md.push_str(&render_category_equation_summary_cards(cat));
    md.push('\n');
    md.push_str("## Browse\n\n");
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

#[derive(Debug)]
struct EquationSummaryCardRow {
    path_id: String,
    name: String,
    link: String,
    latex: String,
    targets: String,
    default_target: String,
    branches: String,
    subcategory: Option<String>,
}

fn render_category_equation_summary_cards(cat: &CategoryPresentation) -> String {
    let mut rows: Vec<EquationSummaryCardRow> = Vec::new();
    for eq in &cat.root_equations {
        rows.push(EquationSummaryCardRow {
            path_id: eq.path_id.clone(),
            name: eq.page.name.clone(),
            link: format!("./{}.md", eq.slug),
            latex: eq.page.latex.clone(),
            targets: format_compact_targets(&eq.page),
            default_target: eq
                .page
                .default_target
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            branches: format_compact_branches(&eq.page),
            subcategory: None,
        });
    }
    for sub in &cat.subcategories {
        for eq in &sub.equations {
            rows.push(EquationSummaryCardRow {
                path_id: eq.path_id.clone(),
                name: eq.page.name.clone(),
                link: format!("./{}/{}.md", sub.name, eq.slug),
                latex: eq.page.latex.clone(),
                targets: format_compact_targets(&eq.page),
                default_target: eq
                    .page
                    .default_target
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
                branches: format_compact_branches(&eq.page),
                subcategory: Some(title_case(&sub.name)),
            });
        }
    }
    rows.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    render_equation_summary_cards(rows, true)
}

fn render_subcategory_equation_summary_cards(sub: &SubcategoryPresentation) -> String {
    let mut rows: Vec<EquationSummaryCardRow> = sub
        .equations
        .iter()
        .map(|eq| EquationSummaryCardRow {
            path_id: eq.path_id.clone(),
            name: eq.page.name.clone(),
            link: format!("./{}.md", eq.slug),
            latex: eq.page.latex.clone(),
            targets: format_compact_targets(&eq.page),
            default_target: eq
                .page
                .default_target
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            branches: format_compact_branches(&eq.page),
            subcategory: None,
        })
        .collect();
    rows.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    render_equation_summary_cards(rows, false)
}

fn render_equation_summary_cards(
    rows: Vec<EquationSummaryCardRow>,
    include_subcategory: bool,
) -> String {
    let mut md = String::new();
    // Invariant: category-level equation visibility must remain registry-driven.
    // This summary layout is built from category metadata (root + subcategory equations),
    // never from handwritten per-category markdown.
    //
    // Invariant: title and LaTeX are the primary recognition surfaces.
    // Keep metadata secondary and compact, and avoid regressing to dense debug-like cards.
    //
    // Invariant: do not remove the large centered MathJax block. Category/subcategory browse
    // pages are intended for quick equation recognition on normal laptop widths.
    md.push_str(
        "<style>\n\
         .equation-summary-cards { display: grid; grid-template-columns: 1fr; gap: 1.2rem; margin: 0.75rem 0 1.25rem; }\n\
         .equation-summary-card { position: relative; border: 1px solid var(--table-border-color); border-radius: 12px; padding: 1.15rem 1.25rem 1rem; background: rgba(255,255,255,0.03); box-shadow: 0 1px 0 rgba(255,255,255,0.02) inset; }\n\
         .equation-summary-card:hover { border-color: rgba(255,255,255,0.35); background: rgba(255,255,255,0.04); }\n\
         .equation-summary-card-link { position: absolute; inset: 0; z-index: 1; border-radius: 12px; }\n\
         .equation-summary-header { position: relative; z-index: 2; margin-bottom: 0.35rem; }\n\
         .equation-summary-title { font-size: 1.24rem; line-height: 1.3; font-weight: 650; margin: 0 0 0.35rem; }\n\
         .equation-summary-title a { position: relative; z-index: 2; }\n\
         .equation-summary-path { font-family: var(--mono-font); font-size: 0.88rem; opacity: 0.75; margin: 0 0 0.7rem; overflow-wrap: anywhere; }\n\
         .equation-summary-latex { position: relative; z-index: 2; text-align: center; font-size: 1.55rem; line-height: 1.5; margin: 0.5rem 0 0.75rem; padding: 0.35rem 0.5rem; overflow-x: auto; }\n\
         .equation-summary-meta { position: relative; z-index: 2; display: flex; flex-wrap: wrap; gap: 0.35rem 0.45rem; align-items: center; margin-top: 0.35rem; }\n\
         .equation-summary-chip { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.2rem 0.45rem; border: 1px solid var(--table-border-color); border-radius: 999px; font-size: 0.82rem; line-height: 1.2; background: rgba(255,255,255,0.02); }\n\
         .equation-summary-chip-label { opacity: 0.75; font-weight: 500; }\n\
         .equation-summary-chip-value { overflow-wrap: anywhere; }\n\
         .equation-summary-chip-value code { white-space: nowrap; }\n\
         .equation-summary-targets { display: flex; flex-wrap: wrap; gap: 0.25rem; }\n\
         .equation-summary-targets code { display: inline-block; padding: 0.05rem 0.35rem; border-radius: 8px; background: rgba(255,255,255,0.07); }\n\
         @media (min-width: 1200px) { .equation-summary-cards { grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1.35rem; } }\n\
         @media (max-width: 900px) { .equation-summary-card { padding: 1rem; } .equation-summary-title { font-size: 1.12rem; } .equation-summary-latex { font-size: 1.3rem; } }\n\
         </style>\n",
    );
    md.push_str("<div class=\"equation-summary-cards\">\n");
    for row in rows {
        md.push_str("<article class=\"equation-summary-card\">\n");
        md.push_str(&format!(
            "<a class=\"equation-summary-card-link\" href=\"{}\" aria-label=\"Open {}\"></a>\n",
            row.link, row.name
        ));
        md.push_str("<div class=\"equation-summary-header\">\n");
        md.push_str(&format!(
            "<h3 class=\"equation-summary-title\"><a href=\"{}\">{}</a></h3>\n",
            row.link, row.name
        ));
        md.push_str(&format!(
            "<div class=\"equation-summary-path\"><code>{}</code></div>\n",
            row.path_id
        ));
        md.push_str("</div>\n");
        md.push_str(&format!(
            "<div class=\"equation-summary-latex\">\\({}\\)</div>\n",
            row.latex
        ));
        md.push_str("<div class=\"equation-summary-meta\">\n");
        md.push_str(&format!(
            "<div class=\"equation-summary-chip\"><span class=\"equation-summary-chip-label\">Targets</span><span class=\"equation-summary-chip-value equation-summary-targets\">{}</span></div>",
            row.targets
        ));
        md.push_str(&format!(
            "<div class=\"equation-summary-chip\"><span class=\"equation-summary-chip-label\">Default</span><span class=\"equation-summary-chip-value\"><code>{}</code></span></div>",
            row.default_target
        ));
        if row.branches != "-" {
            md.push_str(&format!(
                "<div class=\"equation-summary-chip\"><span class=\"equation-summary-chip-label\">Branches</span><span class=\"equation-summary-chip-value\">{}</span></div>",
                row.branches
            ));
        }
        if include_subcategory {
            if let Some(subcategory) = row.subcategory {
                md.push_str(&format!(
                    "<div class=\"equation-summary-chip\"><span class=\"equation-summary-chip-label\">Subcategory</span><span class=\"equation-summary-chip-value\">{}</span></div>",
                    subcategory
                ));
            }
        }
        md.push_str("\n</div>\n</article>\n");
    }
    md.push_str("</div>\n");
    md
}

fn format_compact_targets(page: &EquationPageModel) -> String {
    if page.solve_targets.is_empty() {
        return "-".to_string();
    }
    page.solve_targets
        .iter()
        .map(|t| format!("<code>{}</code>", t.target))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_compact_branches(page: &EquationPageModel) -> String {
    if page.branches.is_empty() {
        return "-".to_string();
    }
    page.branches
        .iter()
        .map(|b| format!("<code>{}</code>", b.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_subcategory_index(cat: &CategoryPresentation, sub: &SubcategoryPresentation) -> String {
    let mut md = String::new();
    md.push_str(&format!(
        "# {} / {}\n\n",
        title_case(&cat.name),
        title_case(&sub.name)
    ));
    md.push_str("## Equation Summary\n\n");
    md.push_str(&render_subcategory_equation_summary_cards(sub));
    md.push('\n');
    md.push_str("## Browse\n\n");
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
    // Invariant: emit display math with `$$...$$` so mdBook/MathJax consistently renders it.
    // Do not wrap LaTeX in literal brackets like `[ ... ]` and do not escape backslashes here.
    md.push_str(&format!("$$\n{}\n$$\n\n", p.latex));
    md.push_str(&format!("- Unicode: `{}`\n", p.unicode));
    md.push_str(&format!("- ASCII: `{}`\n\n", p.ascii));

    md.push_str("## Variables\n\n");
    md.push_str(
        "<table><thead><tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Unit</th></tr></thead><tbody>\n",
    );
    for v in &p.variables {
        // Invariant: symbols must stay inline-math wrapped in table cells.
        // This preserves LaTeX macros like `\dot{m}` and prevents markdown from treating
        // underscores/braces as plain text.
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
    if let Some((family, variant)) =
        equations::equation_families::family_by_equation_path_id(families, &p.path_id)
    {
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
    md.push('\n');
    md.push_str(&render_binding_examples_for_equation(p));
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
    s.push_str("- [Engineering Solve Layer](solve/index.md)\n");
    s.push_str("- [Studies and Parameter Sweeps](studies/index.md)\n");
    s.push_str("- [Bindings (Python/Excel)](bindings/index.md)\n");
    s.push_str("- [Devices Guide](devices/guide.md)\n");
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
    s.push_str("- [Devices](devices/index.md)\n");
    for d in &c.devices {
        s.push_str(&format!(
            "  - [{}](devices/{}.md)\n",
            d.name,
            snake_case(&d.key)
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

fn render_devices_guide() -> String {
    let mut md = String::new();
    md.push_str("# Devices Guide\n\n");
    md.push_str("Devices/components compose multiple atomic equations into higher-level engineering workflows. Device docs and bindings are generated from typed Rust metadata specs.\n\n");
    for spec in crate::devices::generation_specs() {
        md.push_str(&format!("## {}\n\n", spec.name));
        md.push_str(&format!("- Key: `{}`\n", spec.key));
        md.push_str(&format!("- {}\n", spec.summary));
        md.push_str(&format!("- Route: `{}`\n", spec.route));
        for mode in spec.supported_modes {
            md.push_str(&format!("- {}\n", mode));
        }
        md.push('\n');
    }

    md.push_str("## Fixed-f Mode\n\n```rust\n");
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_FIXED.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Colebrook Mode (direct properties)\n\n```rust\n");
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_DIRECT.trim());
    md.push_str("\n```\n\n");

    md.push_str("## Colebrook Mode (fluid context)\n\n```rust\n");
    md.push_str(SNIPPET_DEVICE_PIPE_LOSS_COLEBROOK_FLUID.trim());
    md.push_str("\n```\n\n");

    md.push_str("See [Devices Index](./index.md) for full generated device pages.\n");
    md
}

fn render_devices_index(c: &UnifiedDocsContribution) -> String {
    let mut md = String::new();
    md.push_str("# Devices\n\n");
    md.push_str("- [Devices Guide](./guide.md)\n\n");
    md.push_str("| Device | Key | Summary | Modes |\n");
    md.push_str("| --- | --- | --- | --- |\n");
    for d in &c.devices {
        md.push_str(&format!(
            "| [{}](./{}.md) | `{}` | {} | {} |\n",
            d.name,
            snake_case(&d.key),
            d.key,
            d.summary,
            d.supported_modes.join(", ")
        ));
    }
    md
}

fn render_device_page(d: &crate::devices::DeviceDocsEntry) -> String {
    let spec = crate::devices::generation_specs()
        .into_iter()
        .find(|s| s.key == d.key)
        .expect("device docs entry must have generation spec");
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", d.name));
    md.push_str(&format!("**Key:** `{}`\n\n", d.key));
    md.push_str(&format!("{}\n\n", d.summary));
    md.push_str(spec.overview_markdown);
    md.push('\n');
    md.push_str("## Modes\n\n");
    for m in &d.supported_modes {
        md.push_str(&format!("- {}\n", m));
    }
    md.push_str("\n## Outputs\n\n");
    for out in &d.outputs {
        md.push_str(&format!("- {}\n", out));
    }
    md.push_str("## Internal Composition\n\n");
    for dep in spec.equation_dependencies {
        md.push_str(&format!(
            "- [{}](../equations/{}.md)\n",
            humanize_path_id(dep),
            dep.replace('.', "/")
        ));
    }
    md.push('\n');
    md.push_str(&render_binding_examples_for_device(&d.key));
    md
}

fn device_equation_dependencies(device_key: &str) -> Vec<&'static str> {
    crate::devices::generation_specs()
        .into_iter()
        .find(|s| s.key == device_key)
        .map(|s| s.equation_dependencies.to_vec())
        .unwrap_or_default()
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

fn humanize_path_id(path_id: &str) -> String {
    path_id
        .split('.')
        .map(title_case)
        .collect::<Vec<_>>()
        .join(" ")
}
