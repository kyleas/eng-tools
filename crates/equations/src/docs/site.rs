use std::{
    fs,
    io::BufWriter,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

use eng_fluids::FluidRef;
use eng_materials::{MaterialDef, MaterialRef};
use printpdf::{BuiltinFont, Mm, PdfDocument};

use crate::{
    docs::{
        presentation::{LibraryPresentation, build_library_presentation, flatten_equations},
        routes,
    },
    equation_families::{self, EquationFamilyDef},
    error::{EquationError, Result},
    model::EquationDef,
};

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

const SNIPPET_TOP_LEVEL_IMPORT: &str = include_str!("../../../eng/docs_snippets/top_level_import.rs");
const SNIPPET_SIMPLE_EQUATION_SOLVE: &str =
    include_str!("../../../eng/docs_snippets/simple_equation_solve.rs");
const SNIPPET_TYPED_UNIT_INPUT: &str =
    include_str!("../../../eng/docs_snippets/typed_unit_input.rs");
const SNIPPET_QTY_MACRO_INPUT: &str =
    include_str!("../../../eng/docs_snippets/qty_macro_input.rs");
const SNIPPET_RUNTIME_STRING_INPUT: &str =
    include_str!("../../../eng/docs_snippets/runtime_string_input.rs");
const SNIPPET_FLUID_PROPERTY_LOOKUP: &str =
    include_str!("../../../eng/docs_snippets/fluid_property_lookup.rs");
const SNIPPET_MATERIAL_PROPERTY_LOOKUP: &str =
    include_str!("../../../eng/docs_snippets/material_property_lookup.rs");
const SNIPPET_CONTEXT_SOLVE: &str = include_str!("../../../eng/docs_snippets/context_solve.rs");
const SNIPPET_FAMILY_VARIANT_ACCESS: &str =
    include_str!("../../../eng/docs_snippets/family_variant_access.rs");

pub fn export_mdbook_source(
    equations: &[EquationDef],
    out_dir: impl AsRef<Path>,
) -> Result<MdBookPaths> {
    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir).map_err(|source| EquationError::Io {
        path: out_dir.to_path_buf(),
        source,
    })?;
    reset_mdbook_output_dir(out_dir)?;

    let library = build_library_presentation(equations);
    generate_mdbook_source(&library, out_dir)?;
    write_text(out_dir.join("README.md"), &render_mdbook_readme())?;

    Ok(MdBookPaths {
        source_dir: out_dir.to_path_buf(),
        html_dir: out_dir.join("book"),
        html_index: out_dir.join("book").join("index.html"),
    })
}

pub fn export_html_docs(
    equations: &[EquationDef],
    out_dir: impl AsRef<Path>,
) -> Result<HtmlExportReport> {
    let paths = export_mdbook_source(equations, out_dir)?;
    let status = match run_mdbook_build(&paths.source_dir) {
        Ok(()) => HtmlBuildStatus::Built,
        Err(MdBookBuildError::NotInstalled) => {
            let message = missing_mdbook_message();
            fs::write(paths.source_dir.join("MDBOOK_BUILD_REQUIRED.txt"), &message).map_err(
                |source| EquationError::Io {
                    path: paths.source_dir.join("MDBOOK_BUILD_REQUIRED.txt"),
                    source,
                },
            )?;
            HtmlBuildStatus::MdBookNotInstalled { message }
        }
        Err(MdBookBuildError::BuildFailed(detail)) => {
            return Err(EquationError::Validation(format!(
                "mdbook build failed for {}: {detail}",
                paths.source_dir.display()
            )));
        }
    };

    Ok(HtmlExportReport { paths, status })
}

pub fn export_pdf_handbook(equations: &[EquationDef], out_file: impl AsRef<Path>) -> Result<()> {
    let out_file = out_file.as_ref();
    let base_dir = out_file
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let book_root = base_dir.join("book");

    let library = build_library_presentation(equations);
    if !book_root.exists() {
        generate_mdbook_source(&library, &book_root)?;
        write_text(book_root.join("README.md"), &render_mdbook_readme())?;
    }
    let _ = run_mdbook_build(&book_root);

    render_pdf_from_presentation(&library, out_file).map_err(|e| {
        EquationError::Validation(format!(
            "mdBook-based PDF path failed in fallback renderer: {e}"
        ))
    })
}

fn generate_mdbook_source(lib: &LibraryPresentation, book_root: &Path) -> Result<()> {
    let families = equation_families::load_from_dir(equation_families::default_families_dir())?;
    let src_root = book_root.join("src");
    fs::create_dir_all(&src_root).map_err(|source| EquationError::Io {
        path: src_root.clone(),
        source,
    })?;

    let book_toml = r#"[book]
title = "Engineering Handbook"
authors = ["eng-tools"]
language = "en"
src = "src"

[output.html]
mathjax-support = true
"#;
    write_text(book_root.join("book.toml"), book_toml)?;

    write_text(src_root.join("index.md"), &render_book_home(lib))?;
    write_text(
        src_root.join("getting_started/index.md"),
        &render_getting_started_page(),
    )?;
    write_text(
        src_root.join("input_styles/index.md"),
        &render_input_styles_page(),
    )?;
    write_text(
        src_root.join("units_quantities/index.md"),
        &render_units_quantities_page(),
    )?;
    write_text(
        src_root.join("architecture/index.md"),
        &render_architecture_page(),
    )?;
    write_text(
        src_root.join("workflows/index.md"),
        &render_workflows_page(),
    )?;
    write_text(
        src_root.join("yaml_authoring/index.md"),
        &render_yaml_authoring_page(),
    )?;
    write_text(
        src_root.join("validation_trust/index.md"),
        &render_validation_trust_page(),
    )?;
    write_text(
        src_root.join("equations/guide.md"),
        &render_equations_guide_page(),
    )?;
    write_text(
        src_root.join("equations/families/index.md"),
        &render_families_index(&families),
    )?;
    write_text(
        src_root.join("equations/index.md"),
        &render_equations_catalog_page(lib),
    )?;
    write_text(
        src_root.join("constants/index.md"),
        &render_constants_index(lib),
    )?;
    write_text(src_root.join("fluids/index.md"), &render_fluids_index()?)?;
    write_text(
        src_root.join("materials/index.md"),
        &render_materials_index()?,
    )?;

    for c in &lib.constants {
        write_text(
            src_root.join("constants").join(format!("{}.md", c.key)),
            &render_constant_page(c, lib),
        )?;
    }
    for family in &families {
        write_text(
            src_root
                .join("equations")
                .join("families")
                .join(format!("{}.md", snake_case(&family.key))),
            &render_family_page(family),
        )?;
    }
    for fluid in eng_fluids::catalog() {
        write_text(
            src_root
                .join("fluids")
                .join(format!("{}.md", snake_case(fluid.key))),
            &render_fluid_page(*fluid, lib),
        )?;
    }
    for mat in eng_materials::catalog().map_err(|e| EquationError::Validation(e.to_string()))? {
        write_text(
            src_root
                .join("materials")
                .join(format!("{}.md", snake_case(mat.key()))),
            &render_material_page(&mat, lib)?,
        )?;
    }

    for cat in &lib.categories {
        write_text(
            src_root.join("equations").join(&cat.name).join("index.md"),
            &render_category_index(cat),
        )?;
        for eq in &cat.root_equations {
            write_text(
                src_root
                    .join("equations")
                    .join(&cat.name)
                    .join(format!("{}.md", eq.slug)),
                &render_equation_page(eq, &families),
            )?;
        }
        for sub in &cat.subcategories {
            write_text(
                src_root
                    .join("equations")
                    .join(&cat.name)
                    .join(&sub.name)
                    .join("index.md"),
                &render_subcategory_index(cat, sub),
            )?;
            for eq in &sub.equations {
                write_text(
                    src_root
                        .join("equations")
                        .join(&cat.name)
                        .join(&sub.name)
                        .join(format!("{}.md", eq.slug)),
                    &render_equation_page(eq, &families),
                )?;
            }
        }
    }

    write_text(src_root.join("SUMMARY.md"), &render_summary(lib, &families))?;
    Ok(())
}

pub fn serve_mdbook(book_root: impl AsRef<Path>, open_browser: bool) -> Result<()> {
    let mut cmd = Command::new("mdbook");
    cmd.arg("serve").arg(book_root.as_ref());
    if open_browser {
        cmd.arg("--open");
    }
    let status = cmd.status().map_err(|_| {
        EquationError::Validation(
            "mdbook executable not found in PATH. Install with: cargo install mdbook".to_string(),
        )
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(EquationError::Validation(format!(
            "mdbook serve failed with status {status}"
        )))
    }
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
    "mdBook source generation succeeded, but HTML was not built because `mdbook` was not found in PATH.

Install mdBook:
  cargo install mdbook

Then run:
  mdbook build
  mdbook serve --open
"
    .to_string()
}

fn render_mdbook_readme() -> String {
    "# Generated mdBook

This directory is generated by the unified engineering exporter.

## Preview locally

```bash
mdbook serve --open
```

## Build static HTML

```bash
mdbook build
```

HTML output is written to `book/index.html`.
"
    .to_string()
}

fn render_book_home(lib: &LibraryPresentation) -> String {
    let mut md = String::new();
    md.push_str("# Engineering Handbook\n\n");
    md.push_str("This handbook is generated from the unified engineering catalog (equations, constants, fluids, materials).\n\n");
    let equation_count: usize = lib
        .categories
        .iter()
        .map(|c| {
            c.root_equations.len()
                + c.subcategories
                    .iter()
                    .map(|s| s.equations.len())
                    .sum::<usize>()
        })
        .sum();
    md.push_str("## Start Here\n\n");
    md.push_str("- [Getting Started](getting_started/index.md)\n");
    md.push_str("- [Input Styles](input_styles/index.md)\n");
    md.push_str("- [Units & Quantities](units_quantities/index.md)\n");
    md.push_str("- [Examples & Workflows](workflows/index.md)\n\n");
    md.push_str("## Domain Guides\n\n");
    md.push_str("- [Equations Guide](equations/guide.md)\n");
    md.push_str("- [Equation Families](equations/families/index.md)\n");
    md.push_str("- [Fluids Catalog](fluids/index.md)\n");
    md.push_str("- [Materials Catalog](materials/index.md)\n");
    md.push_str("- [Constants Reference](constants/index.md)\n");
    md.push_str("- [YAML Authoring](yaml_authoring/index.md)\n");
    md.push_str("- [Validation / Trust](validation_trust/index.md)\n");
    md.push_str("- [Architecture Layers](architecture/index.md)\n");
    md.push_str("- [Equation Catalog](equations/index.md)\n\n");
    md.push_str(&format!(
        "**Library size:** {} equations, {} constants.\n\n",
        equation_count,
        lib.constants.len()
    ));

    md.push_str("## Browse by Category\n\n");
    for cat in &lib.categories {
        md.push_str(&format!(
            "- [{}](equations/{}/index.md)\n",
            title_case(&cat.name),
            cat.name
        ));
        for eq in &cat.root_equations {
            md.push_str(&format!(
                "  - [{}](equations/{}/{}.md)\n",
                eq.page.name, cat.name, eq.slug
            ));
        }
        for sub in &cat.subcategories {
            md.push_str(&format!(
                "  - [{}](equations/{}/{}/index.md)\n",
                title_case(&sub.name),
                cat.name,
                sub.name
            ));
            for eq in &sub.equations {
                md.push_str(&format!(
                    "    - [{}](equations/{}/{}/{}.md)\n",
                    eq.page.name, cat.name, sub.name, eq.slug
                ));
            }
        }
    }
    md
}

fn render_getting_started_page() -> String {
    let mut md = String::new();
    md.push_str("# Getting Started\n\n");
    md.push_str("Use the top-level `eng` facade for unified workflows.\n\n");
    md.push_str("## Dependencies\n\n");
    md.push_str("If `eng` is published, add:\n\n");
    md.push_str("```toml\n");
    md.push_str("[dependencies]\neng = \"0.1\"\n");
    md.push_str("```\n\n");
    md.push_str("For local workspace use from the generated handbook root (`generated/book`):\n\n");
    md.push_str("```toml\n");
    md.push_str("[dependencies]\neng = { path = \"../../crates/eng\" }\n");
    md.push_str("```\n\n");
    md.push_str("You can also run directly from this repo:\n\n");
    md.push_str("```bash\n");
    md.push_str("cargo run -p eng --example unified_usage\n");
    md.push_str("cargo test -p eng core_handbook_workflows_execute\n");
    md.push_str("```\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_TOP_LEVEL_IMPORT.trim());
    md.push_str("\n```\n\n");
    md.push_str("```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n");
    md
}

fn render_input_styles_page() -> String {
    let mut md = String::new();
    md.push_str("# Input Styles\n\n");
    md.push_str(
        "All equation input methods support four styles:\n\n1. plain numeric SI\n2. typed unit constructors\n3. `qty!(\"...\")`\n4. runtime strings\n\n",
    );
    md.push_str("## Which Style Should I Use?\n\n");
    md.push_str("- **Fastest path**: plain SI numeric (`f64`) when values are already canonical.\n");
    md.push_str("- **Most explicit Rust path**: typed unit constructors (`pressure::mpa(2.5)`).\n");
    md.push_str("- **Preferred expression path in Rust runtime code**: `qty!(\"...\")`.\n");
    md.push_str("- **Boundary convenience path**: runtime strings from CLI/UI/files.\n\n");
    md.push_str("## Why `qty!` Is Preferred in Rust Code\n\n");
    md.push_str("- Fixed literal expressions are validated from source literals, which catches malformed expressions earlier.\n");
    md.push_str("- The resulting quantity is already canonicalized and dimension-tagged, so you avoid repeatedly parsing freeform runtime strings.\n");
    md.push_str("- `qty!` uses the same dimensional rules as runtime strings, so behavior stays consistent.\n");
    md.push_str("- Runtime strings remain the right choice for user-entered or file/CLI-provided values.\n\n");
    md.push_str("## Performance Notes\n\n");
    md.push_str("- `f64` and typed constructors are lowest-overhead internal paths.\n");
    md.push_str("- `qty!` is preferred for static expressions in Rust code.\n");
    md.push_str("- Runtime strings are boundary convenience and include parse/validation cost.\n\n");
    md.push_str("## Plain SI\n\n```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Typed Units\n\n```rust\n");
    md.push_str(SNIPPET_TYPED_UNIT_INPUT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## `qty!`\n\n```rust\n");
    md.push_str(SNIPPET_QTY_MACRO_INPUT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Runtime Strings\n\n```rust\n");
    md.push_str(SNIPPET_RUNTIME_STRING_INPUT.trim());
    md.push_str("\n```\n");
    md
}

fn render_units_quantities_page() -> String {
    let mut md = String::new();
    md.push_str("# Units & Quantities\n\n");
    md.push_str("The engineering unit system is **dimension-based**, not hardcoded permutation-based. Unit expressions are parsed, reduced with dimensional algebra, and validated against expected variable dimensions.\n\n");
    md.push_str("## System Model\n\n");
    md.push_str("- Atomic unit registry defines aliases, canonical symbols, scale factors, and dimension signatures.\n");
    md.push_str("- Parser handles unit expressions and quantity arithmetic.\n");
    md.push_str("- Reducer computes canonical SI value + final dimension signature.\n");
    md.push_str("- Validator checks the final dimension against the variable's expected dimension.\n\n");
    md.push_str("## What Is Supported\n\n");
    md.push_str("- Atomic units + strict aliases (`m`, `ft`, `Pa`, `psi`, `L`, `gal`, `s`, `min`, ...).\n");
    md.push_str("- Compound units (`kg/(m*s)`, `Pa*s`, `N*s/m^2`, `W/(m*K)`, `J/(kg*K)`).\n");
    md.push_str("- Quantity expressions with `+`, `-`, `*`, `/`, and parentheses.\n");
    md.push_str("- Equivalent expressions normalized to the same canonical dimensions.\n\n");
    md.push_str("## What Gets Checked\n\n");
    md.push_str("- Parse syntax validity.\n");
    md.push_str("- Unit token/alias validity.\n");
    md.push_str("- Dimensional operator rules:\n");
    md.push_str("  - `+` / `-` require same dimensions.\n");
    md.push_str("  - `*` / `/` combine exponents algebraically.\n");
    md.push_str("- Final expression dimension matches expected variable dimension.\n\n");
    md.push_str("## Advanced Conversion Behavior\n\n");
    md.push_str("- Dynamic viscosity equivalence: `Pa*s` == `kg/(m*s)` == `N*s/m^2`.\n");
    md.push_str("- Volumetric flow conversions across time/volume units (`gal/min`, `L/s`, `m^3/hr`).\n");
    md.push_str("- Thermal/transport forms normalize by dimension even when syntactically different.\n\n");
    md.push_str("## What Is Intentionally Restricted\n\n");
    md.push_str("- Mixed-dimension addition/subtraction is rejected.\n");
    md.push_str("- Unknown or ambiguous unit tokens are rejected, not guessed.\n");
    md.push_str("- Affine temperature pitfalls are guarded; prefer canonical absolute temperature (`K`) unless a dedicated path says otherwise.\n\n");
    md.push_str("## Input Paths and Tradeoffs\n\n");
    md.push_str("- `f64`: fastest if already SI.\n");
    md.push_str("- Typed constructors: explicit units with low overhead.\n");
    md.push_str("- `qty!(...)`: preferred fixed-expression Rust path.\n");
    md.push_str("- Runtime strings: boundary-input convenience path.\n\n");
    md.push_str("## Example Validation Outcomes\n\n");
    md.push_str("- Valid: `5 MPa + 12 psi` (same pressure dimension).\n");
    md.push_str("- Valid: `3 ft + 2 in` (same length dimension).\n");
    md.push_str("- Invalid: `5 MPa + 3 m` (pressure + length).\n");
    md.push_str("- Invalid: unknown unit token (`blarg`) or malformed expression.\n");
    md
}

fn render_yaml_authoring_page() -> String {
    let mut md = String::new();
    md.push_str("# YAML Authoring\n\n");
    md.push_str("Equation files: `crates/equations/registry/<category>/...yaml`\n");
    md.push_str("Family files: `crates/equations/registry/families/*.yaml`\n\n");
    md.push_str("Default workflow: use **minimal/typical** form. Use verbose overrides only when they add real value.\n\n");
    md.push_str("## File Shape Guidance\n\n");
    md.push_str("1. **Minimal**: smallest valid equation with baseline test.\n");
    md.push_str("2. **Typical (recommended)**: minimal + assumptions/references + explicit forms where useful.\n");
    md.push_str("3. **Verbose/override**: only when default derivation/rendering behavior needs explicit overrides.\n\n");
    md.push_str("## Required vs Optional (Equation)\n\n");
    md.push_str("| Field | Required | Notes |\n");
    md.push_str("| --- | --- | --- |\n");
    md.push_str("| `key` | yes | Stable equation key. |\n");
    md.push_str("| `taxonomy.category` | yes | Top-level grouping. |\n");
    md.push_str("| `name` | yes | Human-readable name. |\n");
    md.push_str("| `variables` | yes | Variable metadata. |\n");
    md.push_str("| `residual.expression` | yes | Source-of-truth relation. |\n");
    md.push_str("| `tests.baseline` | yes | Trust/regression baseline. |\n");
    md.push_str("| `display` | optional (recommended) | Latex/unicode/ascii authoring control. |\n");
    md.push_str("| `solve.explicit_forms` | optional | Add explicit target forms when available. |\n");
    md.push_str("| `assumptions`/`references` | optional (recommended) | Concise trust metadata. |\n");
    md.push_str("| `solve.unsupported_targets` | optional (rare) | Only for truly unsafe/unsupported numerical paths. |\n\n");

    md.push_str("## Minimal Equation Example\n\n```yaml\n");
    md.push_str("key: hoop_stress\n");
    md.push_str("taxonomy: { category: structures }\n");
    md.push_str("name: Thin-Wall Hoop Stress\n");
    md.push_str("display: { latex: \"\\\\sigma_h = \\\\frac{P r}{t}\" }\n");
    md.push_str("variables:\n  sigma_h: { dimension: stress, default_unit: Pa }\n  P: { dimension: pressure, default_unit: Pa }\n  r: { dimension: length, default_unit: m }\n  t: { dimension: length, default_unit: m }\n");
    md.push_str("residual: { expression: \"sigma_h - P*r/t\" }\n");
    md.push_str("tests:\n  baseline: { sigma_h: \"62.5 MPa\", P: \"2.5 MPa\", r: \"0.2 m\", t: \"8 mm\" }\n");
    md.push_str("```\n");

    md.push_str("\n## Typical (Recommended) Equation Example\n\n```yaml\n");
    md.push_str("key: reynolds_number\n");
    md.push_str("taxonomy:\n  category: fluids\n");
    md.push_str("name: Reynolds Number\n");
    md.push_str("display:\n  latex: \"Re = \\\\frac{\\\\rho V D}{\\\\mu}\"\n");
    md.push_str("variables:\n");
    md.push_str("  Re: { name: Reynolds number, dimension: dimensionless, default_unit: \"1\" }\n");
    md.push_str("  rho: { name: Fluid density, dimension: density, default_unit: kg/m3, resolver: { source: fluid, kind: fluid_property, property: density } }\n");
    md.push_str("  V: { name: Mean velocity, dimension: velocity, default_unit: m/s }\n");
    md.push_str("  D: { name: Pipe diameter, dimension: length, default_unit: m }\n");
    md.push_str("  mu: { name: Dynamic viscosity, dimension: dynamic_viscosity, default_unit: Pa*s, resolver: { source: fluid, kind: fluid_property, property: dynamic_viscosity } }\n");
    md.push_str("residual:\n  expression: \"Re - rho*V*D/mu\"\n");
    md.push_str("assumptions:\n  - Single-phase continuum flow.\n");
    md.push_str("tests:\n  baseline:\n    Re: 300000\n    rho: \"998 kg/m3\"\n    V: \"3 m/s\"\n    D: \"0.1 m\"\n    mu: \"1e-3 Pa*s\"\n");
    md.push_str("```\n");

    md.push_str("\n## Verbose / Override Example\n\n```yaml\n");
    md.push_str("key: darcy_weisbach_pressure_drop\n");
    md.push_str("taxonomy:\n  category: fluids\n  subcategory: internal_flow\n");
    md.push_str("name: Darcy-Weisbach Pressure Drop\n");
    md.push_str("display:\n  latex: \"\\\\Delta p = f \\\\frac{L}{D} \\\\frac{\\\\rho V^2}{2}\"\n  unicode: \"delta_p = f (L/D) (rho V^2 / 2)\"\n  ascii: \"delta_p = f*(L/D)*(rho*V^2/2)\"\n");
    md.push_str("variables:\n");
    md.push_str("  delta_p: { name: Pressure drop, symbol: \"\\\\Delta p\", dimension: pressure, default_unit: Pa }\n");
    md.push_str("  f: { name: Darcy friction factor, symbol: f, dimension: friction_factor, default_unit: \"1\" }\n");
    md.push_str("  L: { name: Pipe length, symbol: L, dimension: length, default_unit: m }\n");
    md.push_str("  D: { name: Pipe diameter, symbol: D, dimension: length, default_unit: m }\n");
    md.push_str("  rho: { name: Fluid density, symbol: \"\\\\rho\", dimension: density, default_unit: kg/m^3 }\n");
    md.push_str("  V: { name: Mean velocity, symbol: V, dimension: velocity, default_unit: m/s }\n");
    md.push_str("residual:\n  expression: \"delta_p - f*(L/D)*(rho*V^2/2)\"\n");
    md.push_str("solve:\n  explicit_forms:\n    delta_p: \"f*(L/D)*(rho*V^2/2)\"\n    f: \"delta_p/((L/D)*(rho*V^2/2))\"\n");
    md.push_str("assumptions:\n  - Steady, incompressible internal flow form.\n");
    md.push_str("references:\n  - source: Standard fluid mechanics texts\n");
    md.push_str("tests:\n  baseline:\n    delta_p: \"9000 Pa\"\n    f: 0.02\n    L: \"10 m\"\n    D: \"0.1 m\"\n    rho: \"1000 kg/m^3\"\n    V: \"3 m/s\"\n");
    md.push_str("```\n");

    md.push_str("\n## Family YAML Example\n\n```yaml\n");
    md.push_str("key: ideal_gas\n");
    md.push_str("name: Ideal Gas Law\n");
    md.push_str("description: Common forms of the ideal-gas law under one canonical family.\n");
    md.push_str("canonical_equation: thermo.ideal_gas.mass_volume\n");
    md.push_str("canonical_law: \"P * V = m * R * T\"\n");
    md.push_str("variants:\n");
    md.push_str("  - key: mass_volume\n");
    md.push_str("    equation_id: thermo.ideal_gas.mass_volume\n");
    md.push_str("    display_latex: \"P V = m R T\"\n");
    md.push_str("  - key: density\n");
    md.push_str("    equation_id: thermo.ideal_gas.density\n");
    md.push_str("    display_latex: \"P = \\\\rho R T\"\n");
    md.push_str("```\n\n");
    md.push_str("## Optional vs Required Summary\n\n");
    md.push_str("- Required: `key`, `taxonomy.category`, `name`, `variables`, `residual.expression`, `tests.baseline`.\n");
    md.push_str("- Optional but recommended: `display`, `assumptions`, `references`, explicit solve forms.\n");
    md.push_str("- Rare optional: `unsupported_targets` (only when numerical solving is genuinely unsafe/misleading).\n");
    md.push_str("- Family files require `variants`; shared assumptions/references are optional but recommended.\n");
    md
}
fn render_validation_trust_page() -> String {
    let mut md = String::new();
    md.push_str("# Validation / Trust\n\n");
    md.push_str("- Registry validation for equations/families\n");
    md.push_str("- Solver-path tests (SI, typed units, `qty!`, runtime strings)\n");
    md.push_str("- mdBook link integrity tests\n");
    md.push_str("- Unified verification script with docs regeneration\n");
    md
}

fn render_equations_guide_page() -> String {
    let mut md = String::new();
    md.push_str("# Equations Guide\n\n");
    md.push_str("Atomic equations and equation families are presented as one coherent system.\n\n");
    md.push_str("- [Equation Catalog](./index.md)\n");
    md.push_str("- [Equation Families](./families/index.md)\n\n");
    md.push_str("## Family Variant Access Example\n\n```rust\n");
    md.push_str(SNIPPET_FAMILY_VARIANT_ACCESS.trim());
    md.push_str("\n```\n");
    md
}

fn render_architecture_page() -> String {
    let mut md = String::new();
    md.push_str("# Architecture Layers\n\n");
    md.push_str(
        "Design-first boundary model for capabilities that come after atomic equations.\n\n",
    );
    md.push_str("## Layer Definitions\n\n");
    md.push_str(
        "1. **Atomic Equation**: one physical relation with scalar-first solve behavior.\n",
    );
    md.push_str(
        "2. **Equation Family / Variants**: one canonical law with multiple discoverable forms.\n",
    );
    md.push_str(
        "3. **Component Model**: iterative orchestration over multiple equations and contexts.\n",
    );
    md.push_str(
        "4. **Solve Graph / Chaining**: node/edge execution over equations/components/sources.\n",
    );
    md.push_str(
        "5. **External Bindings**: generated Python/Excel adapters over Rust-owned logic.\n\n",
    );

    md.push_str("## Belongs Here / Not Here\n\n");
    md.push_str("### Atomic Equation\n");
    md.push_str("- Belongs: one law, scalar inputs/outputs, equation-local tests.\n");
    md.push_str("- Not here: multi-step orchestration or workflow graphs.\n\n");
    md.push_str("### Equation Family\n");
    md.push_str("- Belongs: canonical law identity + variants (e.g., ideal gas forms).\n");
    md.push_str("- Not here: duplicate independent solver logic for each form.\n\n");
    md.push_str("### Component Model\n");
    md.push_str("- Belongs: multi-equation iterative behavior (prototype: two_orifice).\n");
    md.push_str("- Not here: source-of-truth atomic law definitions.\n\n");
    md.push_str("### Solve Graph\n");
    md.push_str("- Belongs: dependency ordering and chained execution.\n");
    md.push_str("- Not here: component constitutive details.\n\n");
    md.push_str("### External Bindings\n");
    md.push_str("- Belongs: generated signatures/docs from catalog metadata.\n");
    md.push_str("- Not here: reimplementation of core solver/unit logic.\n\n");

    md.push_str("## Ownership Map\n\n");
    md.push_str("- `equations`: atomic equations + family metadata.\n");
    md.push_str("- `eng-fluids`: fluid catalog/property backends.\n");
    md.push_str("- `eng-materials`: material catalogs/interpolation.\n");
    md.push_str(
        "- `eng`: unified orchestration for components, graphs, bindings, docs/catalog assembly.\n",
    );
    md.push_str("- `eng-core`: shared units/input semantics.\n\n");

    md.push_str("## Prototypes\n\n");
    md.push_str("- **Ideal Gas Family**: canonical `P*V=m*R*T` with pressure-volume and density variants.\n");
    md.push_str("- **Two-Orifice Component**: iterative splitter model using `orifice_mass_flow` + `continuity` equations.\n\n");

    md.push_str("Machine-readable architecture metadata is exported as `generated/architecture_spec.json`.\n");
    md
}

fn render_summary(lib: &LibraryPresentation, families: &[EquationFamilyDef]) -> String {
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
    for cat in &lib.categories {
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
    for f in families {
        s.push_str(&format!(
            "  - [{}](equations/families/{}.md)\n",
            f.name,
            snake_case(&f.key)
        ));
    }
    s.push_str("- [Constants](constants/index.md)\n");
    for c in &lib.constants {
        s.push_str(&format!("  - [{}](constants/{}.md)\n", c.name, c.key));
    }
    s.push_str("- [Fluids](fluids/index.md)\n");
    for fluid in eng_fluids::catalog() {
        s.push_str(&format!(
            "  - [{}](fluids/{}.md)\n",
            fluid.display_name,
            snake_case(fluid.key)
        ));
    }
    s.push_str("- [Materials](materials/index.md)\n");
    if let Ok(materials) = eng_materials::catalog() {
        for mat in materials {
            s.push_str(&format!(
                "  - [{}](materials/{}.md)\n",
                title_case(mat.key()),
                snake_case(mat.key())
            ));
        }
    }
    s
}

fn render_fluids_index() -> Result<String> {
    let mut md = String::new();
    md.push_str("# Fluids Catalog\n\n");
    md.push_str("Catalog-backed fluid wrappers generated from the supported backend set.\n\n");
    md.push_str("<table>\n");
    md.push_str("  <thead><tr><th>Key</th><th>Name</th><th>Aliases</th><th>Rust Wrapper</th></tr></thead>\n");
    md.push_str("  <tbody>\n");
    for fluid in eng_fluids::catalog() {
        let alias = if fluid.aliases.is_empty() {
            "-".to_string()
        } else {
            fluid.aliases.join(", ")
        };
        md.push_str(&format!(
            "    <tr><td><a href=\"{}.md\"><code>{}</code></a></td><td>{}</td><td>{}</td><td><code>eng_fluids::{}</code></td></tr>\n",
            snake_case(fluid.key),
            escape_html(fluid.key),
            escape_html(fluid.display_name),
            escape_html(&alias),
            snake_case(fluid.display_name)
        ));
    }
    md.push_str("  </tbody>\n</table>\n");
    Ok(md)
}

fn render_fluid_page(fluid: FluidRef, lib: &LibraryPresentation) -> String {
    let fn_name = snake_case(fluid.display_name);
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", fluid.display_name));
    md.push_str("<table>\n");
    md.push_str("  <thead><tr><th>Field</th><th>Value</th></tr></thead>\n");
    md.push_str("  <tbody>\n");
    md.push_str(&format!(
        "    <tr><td>Key</td><td><code>{}</code></td></tr>\n",
        escape_html(fluid.key)
    ));
    md.push_str(&format!(
        "    <tr><td>Aliases</td><td>{}</td></tr>\n",
        if fluid.aliases.is_empty() {
            "-".to_string()
        } else {
            fluid
                .aliases
                .iter()
                .map(|a| format!("<code>{}</code>", escape_html(a)))
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    md.push_str(
        "    <tr><td>Supported state inputs</td><td><code>state_tp(T, P)</code></td></tr>\n",
    );
    md.push_str(&format!(
        "    <tr><td>Supported properties</td><td>{}</td></tr>\n",
        eng_fluids::SUPPORTED_PROPERTIES
            .iter()
            .map(|p| format!("<code>{}</code>", p))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    md.push_str("  </tbody>\n</table>\n\n");
    md.push_str("## Example\n\n");
    md.push_str("```rust\n");
    md.push_str("use eng_fluids as fluids;\n\n");
    md.push_str(&format!(
        "let state = fluids::{}().state_tp(\"300 K\", \"1 bar\")?;\n",
        fn_name
    ));
    md.push_str("let rho = state.property(fluids::FluidProperty::Density)?;\n");
    md.push_str("println!(\"rho = {rho} kg/m^3\");\n");
    md.push_str("```\n");
    let linked = equations_using_context("fluid", lib);
    if !linked.is_empty() {
        md.push_str("\n## Example Equations Using Fluid Context\n\n");
        for (name, rel) in linked {
            md.push_str(&format!("- [{}]({})\n", name, rel));
        }
    }
    md
}

fn render_materials_index() -> Result<String> {
    let materials =
        eng_materials::catalog().map_err(|e| EquationError::Validation(e.to_string()))?;
    let mut md = String::new();
    md.push_str("# Materials Catalog\n\n");
    md.push_str("Metadata + dense property series with interpolation.\n\n");
    md.push_str("<table>\n");
    md.push_str("  <thead><tr><th>Key</th><th>Rust Wrapper</th><th>Properties</th></tr></thead>\n");
    md.push_str("  <tbody>\n");
    for mat in materials {
        let def = mat
            .definition()
            .map_err(|e| EquationError::Validation(e.to_string()))?;
        let props = def
            .properties
            .keys()
            .map(|p| format!("<code>{}</code>", escape_html(p)))
            .collect::<Vec<_>>()
            .join(", ");
        md.push_str(&format!(
            "    <tr><td><a href=\"{}.md\"><code>{}</code></a></td><td><code>eng_materials::{}</code></td><td>{}</td></tr>\n",
            snake_case(mat.key()),
            escape_html(mat.key()),
            snake_case(mat.key()),
            props
        ));
    }
    md.push_str("  </tbody>\n</table>\n");
    Ok(md)
}

fn render_material_page(material: &MaterialRef, lib: &LibraryPresentation) -> Result<String> {
    let def: MaterialDef = material
        .definition()
        .map_err(|e| EquationError::Validation(e.to_string()))?;
    let wrapper = snake_case(material.key());
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", def.name));
    md.push_str("<table>\n");
    md.push_str("  <thead><tr><th>Field</th><th>Value</th></tr></thead>\n");
    md.push_str("  <tbody>\n");
    md.push_str(&format!(
        "    <tr><td>Key</td><td><code>{}</code></td></tr>\n",
        escape_html(&def.key)
    ));
    md.push_str(&format!(
        "    <tr><td>Aliases</td><td>{}</td></tr>\n",
        if def.aliases.is_empty() {
            "-".to_string()
        } else {
            def.aliases
                .iter()
                .map(|a| format!("<code>{}</code>", escape_html(a)))
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    md.push_str(&format!(
        "    <tr><td>Source</td><td>{}</td></tr>\n",
        escape_html(&def.source)
    ));
    md.push_str("  </tbody>\n</table>\n\n");
    if !def.description.trim().is_empty() {
        md.push_str(&format!("{}\n\n", escape_html(&def.description)));
    }
    md.push_str("## Properties\n\n");
    md.push_str("<table>\n");
    md.push_str("  <thead><tr><th>Property</th><th>Dimension</th><th>Unit</th><th>Points</th><th>Interpolation</th></tr></thead>\n");
    md.push_str("  <tbody>\n");
    for (prop, series) in &def.properties {
        md.push_str(&format!(
            "    <tr><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td>{}</td><td><code>{}</code></td></tr>\n",
            escape_html(prop),
            escape_html(&series.dimension),
            escape_html(&series.unit),
            series.points.len(),
            escape_html(&series.interpolation)
        ));
    }
    md.push_str("  </tbody>\n</table>\n\n");
    md.push_str("## Example\n\n```rust\n");
    md.push_str("use eng_materials as materials;\n\n");
    md.push_str(&format!(
        "let wall = materials::{}().temperature(\"350 K\")?;\n",
        wrapper
    ));
    if let Some(prop) = def.properties.keys().next() {
        md.push_str(&format!(
            "let value = wall.property(\"{}\")?;\n",
            escape_string(prop)
        ));
        md.push_str("println!(\"property = {value}\");\n");
    }
    md.push_str("```\n");
    let linked = equations_using_context("material", lib);
    if !linked.is_empty() {
        md.push_str("\n## Example Equations Using Material Context\n\n");
        for (name, rel) in linked {
            md.push_str(&format!("- [{}]({})\n", name, rel));
        }
    }
    Ok(md)
}

fn render_constants_index(lib: &LibraryPresentation) -> String {
    let mut md = String::new();
    md.push_str("# Constants Reference\n\n");
    md.push_str("<table>\n");
    md.push_str("  <thead>\n");
    md.push_str(
        "    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Value</th><th>Unit</th><th>Trust</th></tr>\n",
    );
    md.push_str("  </thead>\n");
    md.push_str("  <tbody>\n");
    for c in &lib.constants {
        md.push_str(&format!(
            "    <tr><td><a href=\"{}.md\"><code>{}</code></a></td><td>{}</td><td>{}</td><td>{:.12e}</td><td><code>{}</code></td><td>{}</td></tr>\n",
            escape_html(&c.key),
            escape_html(&c.key),
            escape_html(c.name),
            render_symbol_cell(c.symbol_latex, true),
            c.value,
            escape_html(c.unit),
            if c.exact { "exact" } else { "reference" },
        ));
    }
    md.push_str("  </tbody>\n");
    md.push_str("</table>\n");
    md
}

fn render_constant_page(
    c: &crate::constants::EngineeringConstant,
    lib: &LibraryPresentation,
) -> String {
    let mut md = String::new();
    let fn_name = snake_case(c.key);
    md.push_str(&format!("# {}\n\n", c.name));
    md.push_str("<table>\n");
    md.push_str("  <thead>\n");
    md.push_str("    <tr><th>Field</th><th>Value</th></tr>\n");
    md.push_str("  </thead>\n");
    md.push_str("  <tbody>\n");
    md.push_str(&format!(
        "    <tr><td>Key</td><td><code>{}</code></td></tr>\n",
        escape_html(c.key)
    ));
    md.push_str(&format!(
        "    <tr><td>Symbol</td><td>{}</td></tr>\n",
        render_symbol_cell(c.symbol_latex, true)
    ));
    md.push_str(&format!(
        "    <tr><td>Dimension</td><td><code>{}</code></td></tr>\n",
        escape_html(c.dimension)
    ));
    md.push_str(&format!(
        "    <tr><td>Value</td><td>{:.12e} <code>{}</code></td></tr>\n",
        c.value,
        escape_html(c.unit)
    ));
    md.push_str(&format!(
        "    <tr><td>Trust</td><td>{}</td></tr>\n",
        if c.exact {
            "Exact / defined"
        } else {
            "Conventional / reference"
        }
    ));
    md.push_str(&format!(
        "    <tr><td>Source</td><td>{}</td></tr>\n",
        escape_html(c.source)
    ));
    md.push_str(&format!(
        "    <tr><td>Note</td><td>{}</td></tr>\n",
        escape_html(c.note)
    ));
    if !c.aliases.is_empty() {
        md.push_str(&format!(
            "    <tr><td>Aliases</td><td><code>{}</code></td></tr>\n",
            escape_html(&c.aliases.join(", "))
        ));
    }
    md.push_str("  </tbody>\n");
    md.push_str("</table>\n");
    if !c.description.trim().is_empty() {
        md.push_str("\n");
        md.push_str(c.description);
        md.push_str("\n");
    }
    md.push_str("\n## Rust Usage\n\n");
    md.push_str("```rust\n");
    md.push_str("use eng::{constants};\n");
    md.push_str("use equations::get_constant;\n\n");
    md.push_str(&format!(
        "let c = constants::{}();\n\
         assert_eq!(c.key, \"{}\");\n\
         println!(\"{} = {} {{}}\", c.value, c.unit);\n\n",
        fn_name,
        escape_string(c.key),
        escape_string(c.key),
        render_symbol_label(c)
    ));
    md.push_str(&format!(
        "let by_id = get_constant(\"{}\").expect(\"constant lookup\");\n\
         assert_eq!(by_id.key, \"{}\");\n",
        escape_string(c.key),
        escape_string(c.key)
    ));
    if let Some(alias) = c.aliases.first() {
        md.push_str(&format!(
            "let by_alias = get_constant(\"{}\").expect(\"alias lookup\");\n\
             assert_eq!(by_alias.key, \"{}\");\n",
            escape_string(alias),
            escape_string(c.key)
        ));
    }
    md.push_str("```\n");
    let linked = equations_using_constant(c.key, lib);
    if !linked.is_empty() {
        md.push_str("\n## Equations Using This Constant\n\n");
        for (name, rel) in linked {
            md.push_str(&format!("- [{}]({})\n", name, rel));
        }
    }
    md
}

fn render_category_index(cat: &crate::docs::presentation::CategoryPresentation) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", title_case(&cat.name)));
    if !cat.root_equations.is_empty() {
        md.push_str("## Equations\n\n");
        for eq in &cat.root_equations {
            md.push_str(&format!("- [{}]({}.md)\n", eq.page.name, eq.slug));
        }
        md.push_str("\n");
    }
    if !cat.subcategories.is_empty() {
        md.push_str("## Subcategories\n\n");
        for sub in &cat.subcategories {
            md.push_str(&format!(
                "- [{}]({}/index.md)\n",
                title_case(&sub.name),
                sub.name
            ));
        }
        md.push_str("\n");
    }
    md
}

fn render_subcategory_index(
    cat: &crate::docs::presentation::CategoryPresentation,
    sub: &crate::docs::presentation::SubcategoryPresentation,
) -> String {
    let mut md = String::new();
    md.push_str(&format!(
        "# {} / {}\n\n",
        title_case(&cat.name),
        title_case(&sub.name)
    ));
    for eq in &sub.equations {
        md.push_str(&format!("- [{}]({}.md)\n", eq.page.name, eq.slug));
    }
    md
}

fn render_equation_page(
    eq: &crate::docs::presentation::EquationPresentation,
    families: &[EquationFamilyDef],
) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", eq.page.name));
    md.push_str(&format!(
        "**Path:** `{}`  \n**Category:** `{}`\n\n",
        eq.path_id, eq.category
    ));

    md.push_str("## Equation\n\n");
    md.push_str("$$\n");
    md.push_str(&eq.page.latex);
    md.push_str("\n$$\n\n");
    md.push_str(&format!("- Unicode: `{}`\n", eq.page.unicode));
    md.push_str(&format!("- ASCII: `{}`\n\n", eq.page.ascii));

    if !eq.page.assumptions.is_empty() {
        md.push_str("## Assumptions\n\n");
        for a in &eq.page.assumptions {
            md.push_str(&format!("- {}\n", a));
        }
        md.push_str("\n");
    }
    if let Some((family, variant)) =
        crate::equation_families::family_by_equation_path_id(families, &eq.path_id)
    {
        let from = routes::equation_doc_path(eq);
        let to = routes::family_doc_path(&family.key);
        let family_link = routes::relative_doc_link(&from, &to);
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

    md.push_str("## Variables\n\n");
    md.push_str("<table>\n");
    md.push_str("  <thead>\n");
    md.push_str("    <tr><th>Key</th><th>Name</th><th>Symbol</th><th>Dimension</th><th>Default Unit</th><th>Resolver</th></tr>\n");
    md.push_str("  </thead>\n");
    md.push_str("  <tbody>\n");
    for v in &eq.page.variables {
        md.push_str(&format!(
            "    <tr><td><code>{}</code></td><td>{}</td><td>{}</td><td><code>{}</code></td><td><code>{}</code></td><td>{}</td></tr>\n",
            escape_html(&v.key),
            escape_html(&v.name),
            render_symbol_cell(&v.symbol, v.symbol_authored),
            escape_html(&v.dimension),
            escape_html(&v.default_unit),
            match (&v.resolver_source, &v.resolver_kind, &v.resolver_property) {
                (Some(src), Some(kind), Some(prop)) => format!(
                    "<code>{}:{}</code> from <code>{}</code>",
                    escape_html(kind),
                    escape_html(prop),
                    escape_html(src)
                ),
                _ => "<code>-</code>".to_string(),
            }
        ));
    }
    md.push_str("  </tbody>\n");
    md.push_str("</table>\n\n");

    let resolvers: Vec<String> = eq
        .page
        .variables
        .iter()
        .filter_map(|v| {
            Some(format!(
                "`{}` from context `{}` via `{}`:`{}`",
                v.key,
                v.resolver_source.as_ref()?,
                v.resolver_kind.as_ref()?,
                v.resolver_property.as_ref()?
            ))
        })
        .collect();
    if !resolvers.is_empty() {
        md.push_str("## Resolvable from Contexts\n\n");
        for r in resolvers {
            md.push_str(&format!("- {}\n", r));
        }
        md.push_str("\n");
    }

    md.push_str("## Solve Targets\n\n");
    for s in &eq.page.solve_targets {
        md.push_str(&format!("- `{}`: {}\n", s.target, s.methods.join(", ")));
    }
    md.push_str("\n");

    if !eq.page.branches.is_empty() {
        md.push_str("## Branches\n\n");
        for b in &eq.page.branches {
            md.push_str(&format!(
                "- `{}` (`{}`){}\n",
                b.name,
                b.condition,
                if b.preferred { " preferred" } else { "" }
            ));
        }
        md.push_str("\n");
    }

    if !eq.page.uses_constants.is_empty() {
        let from = routes::equation_doc_path(eq);
        md.push_str("## Constants Used\n\n");
        md.push_str("<ul>\n");
        for c in &eq.page.uses_constants {
            let to = routes::constant_doc_path(&c.key);
            let link = routes::relative_doc_link(&from, &to);
            md.push_str(&format!(
                "  <li><a href=\"{}\"><code>{}</code></a>: {} - {}</li>\n",
                escape_html(&link),
                escape_html(&c.key),
                escape_html(&c.name),
                render_symbol_cell(&c.symbol_latex, true)
            ));
        }
        md.push_str("</ul>\n\n");
    }

    if !eq.page.examples.is_empty() {
        md.push_str("## Examples\n\n");
        let mut convenience_examples = Vec::new();
        let mut other_examples = Vec::new();
        for ex in &eq.page.examples {
            if ex.style == "convenience" {
                convenience_examples.push(ex);
            } else {
                other_examples.push(ex);
            }
        }

        for ex in other_examples {
            md.push_str(&format!(
                "### {}\n\n",
                pretty_example_heading(&ex.label, &ex.style)
            ));
            md.push_str("```rust\n");
            md.push_str(&ex.code);
            md.push_str("\n```\n\n");
        }

        if !convenience_examples.is_empty() {
            md.push_str("### Available Convenience Functions\n\n");
            md.push_str("Direct solve helpers are available for these targets.\n\n");
            md.push_str("<table>\n");
            md.push_str("  <thead>\n");
            md.push_str(
                "    <tr><th>Solves for</th><th>Function</th><th>Required inputs</th></tr>\n",
            );
            md.push_str("  </thead>\n");
            md.push_str("  <tbody>\n");
            for ex in &convenience_examples {
                let target = ex.target.as_deref().unwrap_or("target");
                let inputs = if ex.argument_order.is_empty() {
                    "<code>-</code>".to_string()
                } else {
                    ex.argument_order
                        .iter()
                        .map(|a| format!("<code>{}</code>", escape_html(&a.name)))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let function_sig = convenience_function_display(ex);
                md.push_str(&format!(
                    "    <tr><td><code>{}</code></td><td><code>{}</code></td><td>{}</td></tr>\n",
                    escape_html(target),
                    escape_html(&function_sig),
                    inputs
                ));
            }
            md.push_str("  </tbody>\n");
            md.push_str("</table>\n\n");

            let focused = select_focused_convenience_example(eq, &convenience_examples);
            if let Some(target) = &focused.target {
                md.push_str(&format!("### Solve `{}`\n\n", target));
            } else {
                md.push_str("### Convenience Example\n\n");
            }
            if let Some(signature) = &focused.signature {
                md.push_str("**Function signature**\n\n");
                md.push_str("```rust\n");
                md.push_str(signature);
                md.push_str("\n```\n\n");
            }
            md.push_str("**Example**\n\n");
            md.push_str("```rust\n");
            md.push_str(&focused.code);
            md.push_str("\n```\n\n");
        }

        let notes = example_notes(eq);
        if !notes.is_empty() {
            md.push_str("### Notes\n\n");
            for n in notes {
                md.push_str(&format!("- {}\n", n));
            }
            md.push_str("\n");
        }
    }

    if !eq.page.references.is_empty() {
        md.push_str("## References\n\n");
        for r in &eq.page.references {
            let note = r.note.clone().unwrap_or_default();
            md.push_str(&format!("- {} {}\n", r.source, note));
        }
        md.push_str("\n");
    }

    if !eq.page.aliases.is_empty() {
        md.push_str("## Aliases\n\n");
        md.push_str(&format!("`{}`\n", eq.page.aliases.join("`, `")));
    }

    md
}

fn write_text(path: impl AsRef<Path>, text: &str) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| EquationError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, text).map_err(|source| EquationError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn reset_mdbook_output_dir(out_dir: &Path) -> Result<()> {
    for rel in [
        Path::new("src"),
        Path::new("book"),
        Path::new("book.toml"),
        Path::new("README.md"),
        Path::new("MDBOOK_BUILD_REQUIRED.txt"),
    ] {
        let target = out_dir.join(rel);
        if !target.exists() {
            continue;
        }
        if target.is_dir() {
            remove_dir_all_with_retry(&target)?;
        } else {
            match fs::remove_file(&target) {
                Ok(()) => {}
                Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
                Err(source) => {
                    return Err(EquationError::Io {
                        path: target.clone(),
                        source,
                    });
                }
            }
        }
    }
    Ok(())
}

fn remove_dir_all_with_retry(target: &Path) -> Result<()> {
    let mut last_error = None;
    for _ in 0..5 {
        match fs::remove_dir_all(target) {
            Ok(()) => return Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(err) => {
                last_error = Some(err);
                thread::sleep(Duration::from_millis(80));
            }
        }
    }
    Err(EquationError::Io {
        path: target.to_path_buf(),
        source: last_error.expect("remove_dir_all_with_retry should capture an error"),
    })
}

fn render_pdf_from_presentation(
    lib: &LibraryPresentation,
    out_file: &Path,
) -> std::result::Result<(), String> {
    let (doc, page1, layer1) = PdfDocument::new(
        "Engineering Equations Handbook",
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("pdf font error: {e}"))?;

    {
        let layer = doc.get_page(page1).get_layer(layer1);
        layer.use_text(
            "Engineering Equations Handbook",
            16.0,
            Mm(10.0),
            Mm(285.0),
            &font,
        );
        layer.use_text("Table of Contents", 12.0, Mm(10.0), Mm(275.0), &font);
        layer.use_text("1. Constants Reference", 10.0, Mm(12.0), Mm(268.0), &font);
        layer.use_text(
            "2. Equation Catalog by Category",
            10.0,
            Mm(12.0),
            Mm(262.0),
            &font,
        );
        layer.use_text(
            "LaTeX form is shown as the primary equation line on each page.",
            9.0,
            Mm(10.0),
            Mm(252.0),
            &font,
        );
    }

    let (mut current_page, mut current_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Constants");
    {
        let mut y = 285.0;
        let layer = doc.get_page(current_page).get_layer(current_layer);
        layer.use_text("Constants Reference", 14.0, Mm(10.0), Mm(y), &font);
        y -= 8.0;
        for c in &lib.constants {
            if y < 35.0 {
                let (p, l) = doc.add_page(Mm(210.0), Mm(297.0), "Constants");
                current_page = p;
                current_layer = l;
                y = 285.0;
            }
            let layer = doc.get_page(current_page).get_layer(current_layer);
            layer.use_text(
                format!("{} ({}) = {:.8e} {}", c.name, c.key, c.value, c.unit),
                10.0,
                Mm(10.0),
                Mm(y),
                &font,
            );
            y -= 4.5;
            layer.use_text(
                format!(
                    "symbol={}, trust={}",
                    c.symbol_ascii,
                    if c.exact { "exact" } else { "reference" }
                ),
                8.0,
                Mm(14.0),
                Mm(y),
                &font,
            );
            y -= 4.0;
            if !c.source.trim().is_empty() {
                layer.use_text(format!("source: {}", c.source), 8.0, Mm(14.0), Mm(y), &font);
                y -= 4.0;
            }
            if !c.note.trim().is_empty() {
                layer.use_text(format!("note: {}", c.note), 8.0, Mm(14.0), Mm(y), &font);
                y -= 4.0;
            }
            y -= 2.0;
        }
    }

    for eq in flatten_equations(lib) {
        let (p, l) = doc.add_page(Mm(210.0), Mm(297.0), "Equation");
        current_page = p;
        current_layer = l;
        let mut y = 285.0;
        let layer = doc.get_page(current_page).get_layer(current_layer);
        layer.use_text(eq.page.name.clone(), 13.0, Mm(10.0), Mm(y), &font);
        y -= 7.0;
        layer.use_text(
            format!("Path: {} | Category: {}", eq.path_id, eq.category),
            8.5,
            Mm(10.0),
            Mm(y),
            &font,
        );
        y -= 8.0;
        layer.use_text("Equation (LaTeX primary):", 10.0, Mm(10.0), Mm(y), &font);
        y -= 5.0;
        layer.use_text(eq.page.latex.clone(), 9.0, Mm(12.0), Mm(y), &font);
        y -= 6.0;
        layer.use_text(
            format!("Unicode: {}", eq.page.unicode),
            8.0,
            Mm(12.0),
            Mm(y),
            &font,
        );
        y -= 4.5;
        layer.use_text(
            format!("ASCII: {}", eq.page.ascii),
            8.0,
            Mm(12.0),
            Mm(y),
            &font,
        );
        y -= 6.0;
        layer.use_text("Variables:", 9.5, Mm(10.0), Mm(y), &font);
        y -= 4.5;
        for v in eq.page.variables.iter().take(8) {
            layer.use_text(
                format!("{}: {} [{} {}]", v.key, v.name, v.dimension, v.default_unit),
                8.0,
                Mm(12.0),
                Mm(y),
                &font,
            );
            y -= 4.0;
            if y < 25.0 {
                break;
            }
        }
        if y > 30.0 {
            layer.use_text("Solve Targets:", 9.5, Mm(10.0), Mm(y), &font);
            y -= 4.5;
            for s in &eq.page.solve_targets {
                layer.use_text(
                    format!("{}: {}", s.target, s.methods.join(", ")),
                    8.0,
                    Mm(12.0),
                    Mm(y),
                    &font,
                );
                y -= 4.0;
                if y < 25.0 {
                    break;
                }
            }
        }
    }

    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut writer = BufWriter::new(fs::File::create(out_file).map_err(|e| e.to_string())?);
    doc.save(&mut writer)
        .map_err(|e| format!("pdf save error: {e}"))?;
    Ok(())
}

fn title_case(s: &str) -> String {
    s.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn pretty_example_heading(label: &str, style: &str) -> String {
    let base = match style {
        "typed_builder" => "Typed Builder",
        "convenience" => "Convenience Solve",
        _ => "Example",
    };
    match label {
        "typed_builder_si" => "Typed Builder (SI Numeric)".to_string(),
        "typed_builder_units" => "Typed Builder (Units-Aware)".to_string(),
        "typed_builder_branch" => "Typed Builder (Branch Example)".to_string(),
        "typed_builder_context" => "Typed Builder (Context-Assisted)".to_string(),
        "convenience" => "Convenience Solve".to_string(),
        _ => format!("{} - {}", base, title_case(label)),
    }
}

fn render_symbol_cell(symbol: &str, symbol_authored: bool) -> String {
    let trimmed = symbol.trim();
    if trimmed.is_empty() {
        return "<code>-</code>".to_string();
    }
    let latex = normalize_symbol_to_latex(trimmed);
    if symbol_authored || is_math_like_symbol(trimmed) {
        return format!("<span class=\"math inline\">\\({}\\)</span>", latex);
    }
    format!("<code>{}</code>", escape_html(trimmed))
}

fn is_math_like_symbol(symbol: &str) -> bool {
    symbol
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '\\' | '{' | '}' | '^'))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn normalize_symbol_to_latex(symbol: &str) -> String {
    if symbol.contains('\\') || symbol.contains('{') || symbol.contains('}') || symbol.contains('^')
    {
        return symbol.to_string();
    }
    if let Some(frac) = normalize_ratio_symbol(symbol) {
        return frac;
    }
    let mut parts = symbol.split('_');
    let head_raw = parts.next().unwrap_or(symbol);
    let mut tail = parts.collect::<Vec<_>>();
    let head = normalize_atomic_symbol(head_raw);
    if tail.len() == 1 && tail[0].eq_ignore_ascii_case("star") {
        format!("{}^*", head)
    } else if tail.is_empty() {
        head
    } else if tail.len() == 1 {
        let tail_part = normalize_tail_part(tail.remove(0));
        format!("{}_{{{}}}", head, tail_part)
    } else {
        let joined = tail
            .into_iter()
            .map(normalize_tail_part)
            .collect::<Vec<_>>()
            .join("_");
        format!("{}_{{{}}}", head, joined)
    }
}

fn maybe_greek_to_latex(token: &str) -> String {
    match token {
        "eps" => "\\epsilon".to_string(),
        "alpha" | "beta" | "gamma" | "delta" | "epsilon" | "zeta" | "eta" | "theta" | "iota"
        | "kappa" | "lambda" | "mu" | "nu" | "xi" | "pi" | "rho" | "sigma" | "tau" | "upsilon"
        | "phi" | "chi" | "psi" | "omega" => {
            format!("\\{}", token)
        }
        _ => token.to_string(),
    }
}

fn normalize_tail_part(part: &str) -> String {
    if part.eq_ignore_ascii_case("star") {
        "*".to_string()
    } else {
        part.to_string()
    }
}

fn split_trailing_digits(token: &str) -> (&str, Option<&str>) {
    let chars: Vec<(usize, char)> = token.char_indices().collect();
    let mut split = token.len();
    for (idx, ch) in chars.iter().rev() {
        if ch.is_ascii_digit() {
            split = *idx;
        } else {
            break;
        }
    }
    if split == token.len() {
        (token, None)
    } else {
        (&token[..split], Some(&token[split..]))
    }
}

fn normalize_atomic_symbol(token: &str) -> String {
    let (base, sub_digits) = split_trailing_digits(token);
    let mut out = maybe_greek_to_latex(base);
    if let Some(digits) = sub_digits {
        out = format!("{}_{{{}}}", out, digits);
    }
    out
}

fn normalize_ratio_symbol(symbol: &str) -> Option<String> {
    let mut parts = symbol.split('_');
    let lhs = parts.next()?;
    let rhs = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    // Ratio-like heuristic: rhs repeats lhs stem (p_p0, T_T0, rho_rho0).
    if rhs.starts_with(lhs) {
        return Some(format!(
            "\\frac{{{}}}{{{}}}",
            normalize_atomic_symbol(lhs),
            normalize_atomic_symbol(rhs)
        ));
    }
    None
}

fn example_notes(eq: &crate::docs::presentation::EquationPresentation) -> Vec<String> {
    let mut notes = Vec::new();
    if eq.page.variables.iter().any(|v| v.default_unit != "1") {
        notes.push(
            "Returns SI by default; use `.value_in(\"<unit>\")` for display units.".to_string(),
        );
    }
    if !eq.page.branches.is_empty() {
        notes.push(
            "Branch selection may be required for inverse solves when multiple roots are possible."
                .to_string(),
        );
    }
    notes
}

fn select_focused_convenience_example<'a>(
    eq: &crate::docs::presentation::EquationPresentation,
    convenience_examples: &'a [&crate::docs::page_model::ExampleSummary],
) -> &'a crate::docs::page_model::ExampleSummary {
    if let Some(default_target) = &eq.page.default_target
        && let Some(hit) = convenience_examples
            .iter()
            .find(|e| e.target.as_deref() == Some(default_target.as_str()))
    {
        return hit;
    }
    convenience_examples[0]
}

fn convenience_function_display(ex: &crate::docs::page_model::ExampleSummary) -> String {
    if let Some(signature) = &ex.signature {
        let no_ret = signature.split(" -> ").next().unwrap_or(signature);
        if let Some(tail) = no_ret.rsplit("::").next() {
            return tail.to_string();
        }
        return no_ret.to_string();
    }
    match &ex.target {
        Some(target) => format!("solve_{}(...)", snake_case(target)),
        None => "solve_<target>(...)".to_string(),
    }
}

fn snake_case(input: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_us = false;
        } else if !prev_us {
            out.push('_');
            prev_us = true;
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() { "x".to_string() } else { out }
}

fn render_symbol_label(c: &crate::constants::EngineeringConstant) -> String {
    if !c.symbol_unicode.trim().is_empty() {
        c.symbol_unicode.to_string()
    } else if !c.symbol_ascii.trim().is_empty() {
        c.symbol_ascii.to_string()
    } else {
        c.key.to_string()
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_workflows_page() -> String {
    let mut md = String::new();
    md.push_str("# Examples & Workflows\n\n");
    md.push_str("## Unified Top-Level API\n\n```rust\n");
    md.push_str(SNIPPET_TOP_LEVEL_IMPORT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Simple Equation Solve\n\n```rust\n");
    md.push_str(SNIPPET_SIMPLE_EQUATION_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Typed Unit Solve\n\n```rust\n");
    md.push_str(SNIPPET_TYPED_UNIT_INPUT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## `qty!` Solve\n\n```rust\n");
    md.push_str(SNIPPET_QTY_MACRO_INPUT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Runtime String Solve\n\n```rust\n");
    md.push_str(SNIPPET_RUNTIME_STRING_INPUT.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Fluid-Assisted Solve\n\n```rust\n");
    md.push_str(SNIPPET_CONTEXT_SOLVE.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Family Variant Solve\n\n```rust\n");
    md.push_str(SNIPPET_FAMILY_VARIANT_ACCESS.trim());
    md.push_str("\n```\n\n");
    md.push_str("## Property Lookups\n\n```rust\n");
    md.push_str(SNIPPET_FLUID_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n\n```rust\n");
    md.push_str(SNIPPET_MATERIAL_PROPERTY_LOOKUP.trim());
    md.push_str("\n```\n");
    md
}

fn render_families_index(families: &[EquationFamilyDef]) -> String {
    let mut md = String::new();
    md.push_str("# Equation Families\n\n");
    if families.is_empty() {
        md.push_str("No equation families are currently defined.\n");
        return md;
    }
    md.push_str("Families capture one physical law with multiple common user-facing forms.\n\n");
    for f in families {
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

fn render_family_page(family: &EquationFamilyDef) -> String {
    let mut md = String::new();
    let family_doc = routes::family_doc_path(&family.key);
    md.push_str(&format!("# {}\n\n", family.name));
    md.push_str(&format!("**Family key:** `{}`\n\n", family.key));
    if !family.description.trim().is_empty() {
        md.push_str(&format!("{}\n\n", family.description.trim()));
    }
    md.push_str(&format!(
        "**Canonical law:** `{}`\n\n",
        family.canonical_law
    ));
    md.push_str(&format!(
        "**Canonical equation:** [`{}`]({})\n\n",
        family.canonical_equation,
        routes::relative_doc_link(
            &family_doc,
            &routes::equation_doc_path_from_path_id(&family.canonical_equation)
        )
    ));
    if !family.assumptions.is_empty() {
        md.push_str("## Shared Assumptions\n\n");
        for a in &family.assumptions {
            md.push_str(&format!("- {}\n", a));
        }
        md.push('\n');
    }
    if !family.references.is_empty() {
        md.push_str("## Shared References\n\n");
        for r in &family.references {
            md.push_str(&format!("- {}\n", r));
        }
        md.push('\n');
    }
    md.push_str("## Variants\n\n");
    md.push_str("| Variant | Equation | Display | Use when | Notes |\n");
    md.push_str("| --- | --- | --- | --- | --- |\n");
    for v in &family.variants {
        let equation_link = routes::relative_doc_link(
            &family_doc,
            &routes::equation_doc_path_from_path_id(&v.equation_id),
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

fn equations_using_constant(
    constant_key: &str,
    lib: &LibraryPresentation,
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for eq in flatten_equations(lib) {
        if eq
            .page
            .uses_constants
            .iter()
            .any(|c| c.key.eq_ignore_ascii_case(constant_key))
        {
            out.push((eq.page.name.clone(), equation_relative_path(&eq)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn equations_using_context(context: &str, lib: &LibraryPresentation) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for eq in flatten_equations(lib) {
        if eq.page.variables.iter().any(|v| {
            v.resolver_source
                .as_deref()
                .is_some_and(|s| s.eq_ignore_ascii_case(context))
        }) {
            out.push((eq.page.name.clone(), equation_relative_path(&eq)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn equation_relative_path(eq: &crate::docs::presentation::EquationPresentation) -> String {
    format!("../{}", routes::equation_doc_path(eq))
}

fn render_equations_catalog_page(lib: &LibraryPresentation) -> String {
    let mut md = String::new();
    md.push_str("# Equation Catalog\n\n");
    md.push_str(
        "Browse all generated equations in one table. Use links to open full equation reference pages.\n\n",
    );
    md.push_str("| Category | Equation | Path ID | Default Target | Variables | Constants Used |\n");
    md.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for eq in flatten_equations(lib) {
        let equation_link =
            routes::relative_doc_link("equations/index.md", &routes::equation_doc_path(&eq));
        let constants_used = if eq.page.uses_constants.is_empty() {
            "-".to_string()
        } else {
            eq.page
                .uses_constants
                .iter()
                .map(|c| format!("`{}`", c.key))
                .collect::<Vec<_>>()
                .join(", ")
        };
        md.push_str(&format!(
            "| {} | [{}]({}) | `{}` | `{}` | {} | {} |\n",
            title_case(&eq.category),
            eq.page.name,
            equation_link,
            eq.path_id,
            eq.page.default_target.as_deref().unwrap_or("-"),
            eq.page.variables.len(),
            constants_used
        ));
    }
    md.push_str("\nUse the chapter tree for category/subcategory browsing.\n");
    md
}


