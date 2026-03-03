use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde_yaml::Value;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct EquationMeta {
    path_id: String,
    category: String,
    subcategories: Vec<String>,
    fn_name: String,
    type_name: String,
    builder_name: String,
    name: String,
    display_line: String,
    variables: Vec<VariableMeta>,
    assumptions: Vec<String>,
    default_target: Option<String>,
    branches: Vec<String>,
    explicit_targets: BTreeSet<String>,
    numerical_targets: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct VariableMeta {
    key: String,
    name: String,
    dimension: String,
    default_unit: String,
}

#[derive(Debug, Clone)]
struct ConstantMeta {
    key: String,
    name: String,
    symbol_latex: String,
    symbol_unicode: String,
    symbol_ascii: String,
    dimension: String,
    unit: String,
    value: f64,
    exact: bool,
    source: String,
    note: String,
    description: String,
    aliases: Vec<String>,
}

#[derive(Debug, Clone)]
struct FamilyMeta {
    key: String,
    name: String,
    description: String,
    canonical_equation: String,
    variants: Vec<FamilyVariantMeta>,
}

#[derive(Debug, Clone)]
struct FamilyVariantMeta {
    key: String,
    name: String,
    equation_id: String,
}

#[derive(Default)]
struct ModuleNode {
    equations: Vec<EquationMeta>,
    children: BTreeMap<String, ModuleNode>,
}

fn main() {
    if let Err(err) = run() {
        panic!("failed to generate typed equations API: {err}");
    }
}

fn run() -> Result<(), String> {
    let crate_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?);
    let registry_dir = crate_root.join("registry");
    let families_dir = crate_root.join("families");
    let constants_file = crate_root.join("constants").join("constants.yaml");

    println!("cargo:rerun-if-changed={}", registry_dir.display());
    println!("cargo:rerun-if-changed={}", families_dir.display());
    println!("cargo:rerun-if-changed={}", constants_file.display());
    for entry in WalkDir::new(&registry_dir) {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    let equations = load_equation_meta(&registry_dir)?;
    let constants = load_constant_meta(&constants_file)?;
    let families = load_family_meta(&families_dir)?;
    let constant_ids = constant_identifiers(&constants);
    let generated = render_typed_api(&equations, &constant_ids);
    let generated_constants = render_typed_constants(&constants);
    let generated_families = render_typed_families(&families, &equations);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").map_err(|e| e.to_string())?);
    let out_file = out_dir.join("typed_equations.rs");
    let out_constants_file = out_dir.join("typed_constants.rs");
    let out_families_file = out_dir.join("typed_families.rs");
    fs::write(&out_file, generated).map_err(|e| e.to_string())?;
    fs::write(&out_constants_file, generated_constants).map_err(|e| e.to_string())?;
    fs::write(&out_families_file, generated_families).map_err(|e| e.to_string())?;
    Ok(())
}

fn load_constant_meta(path: &Path) -> Result<Vec<ConstantMeta>, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let yaml: Value =
        serde_yaml::from_str(&content).map_err(|e| format!("{}: {}", path.display(), e))?;
    let seq = yaml
        .as_sequence()
        .ok_or_else(|| format!("{} must be a YAML list", path.display()))?;
    let mut out = Vec::new();
    for item in seq {
        let key = item
            .get("key")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{} constant missing key", path.display()))?
            .to_string();
        let name = item
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(&key)
            .to_string();
        let symbol_ascii = item
            .get("symbol_ascii")
            .and_then(Value::as_str)
            .or_else(|| item.get("symbol").and_then(Value::as_str))
            .unwrap_or(&key)
            .to_string();
        let symbol_latex = item
            .get("symbol_latex")
            .and_then(Value::as_str)
            .or_else(|| item.get("symbol").and_then(Value::as_str))
            .unwrap_or(&symbol_ascii)
            .to_string();
        let symbol_unicode = item
            .get("symbol_unicode")
            .and_then(Value::as_str)
            .unwrap_or(&symbol_ascii)
            .to_string();
        let dimension = item
            .get("dimension")
            .and_then(Value::as_str)
            .unwrap_or("dimensionless")
            .to_string();
        let unit = item
            .get("unit")
            .and_then(Value::as_str)
            .unwrap_or("1")
            .to_string();
        let value = item.get("value").and_then(Value::as_f64).ok_or_else(|| {
            format!(
                "{} constant '{}' missing numeric value",
                path.display(),
                key
            )
        })?;
        let exact = item.get("exact").and_then(Value::as_bool).unwrap_or(false);
        let source = item
            .get("source")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let note = item
            .get("note")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let description = item
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let aliases = item
            .get("aliases")
            .and_then(Value::as_sequence)
            .map(|s| {
                s.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        out.push(ConstantMeta {
            key,
            name,
            symbol_latex,
            symbol_unicode,
            symbol_ascii,
            dimension,
            unit,
            value,
            exact,
            source,
            note,
            description,
            aliases,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(out)
}

fn load_equation_meta(registry_dir: &Path) -> Result<Vec<EquationMeta>, String> {
    let mut out = Vec::new();
    for entry in WalkDir::new(registry_dir) {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
        let yaml: Value =
            serde_yaml::from_str(&content).map_err(|e| format!("{}: {}", path.display(), e))?;
        let key = get_str(&yaml, "key").ok_or_else(|| format!("{} missing key", path.display()))?;
        let category = get_nested_str(&yaml, &["taxonomy", "category"])
            .ok_or_else(|| format!("{} missing taxonomy.category", path.display()))?;
        let subcategories =
            get_nested_seq_str(&yaml, &["taxonomy", "subcategories"]).unwrap_or_default();
        let slug = get_str(&yaml, "slug").unwrap_or_else(|| key.clone());
        let path_id = {
            let mut s = category.clone();
            for c in &subcategories {
                s.push('.');
                s.push_str(c);
            }
            s.push('.');
            s.push_str(&slug);
            s
        };
        let variables = get_variables(&yaml);
        let branches = get_branch_names(&yaml);
        let assumptions = get_assumptions(&yaml);
        let display_line = get_display_line(&yaml).unwrap_or_else(|| "residual form".to_string());
        let name = get_str(&yaml, "name").unwrap_or_else(|| key.clone());
        let default_target = get_nested_str(&yaml, &["solve", "default_target"]);
        let explicit_targets = get_explicit_targets(&yaml);
        let numerical_targets = get_numerical_targets(&yaml, &variables);

        let safe_path = normalize_snake(&path_id.replace('.', "_"));
        let type_name = format!("{}Equation", to_pascal(&safe_path));
        let builder_name = format!("{}SolveBuilder", to_pascal(&safe_path));
        let fn_name = normalize_snake(&slug);

        out.push(EquationMeta {
            path_id,
            category: normalize_snake(&category),
            subcategories: subcategories
                .into_iter()
                .map(|s| normalize_snake(&s))
                .collect(),
            fn_name,
            type_name,
            builder_name,
            name,
            display_line,
            variables,
            assumptions,
            default_target,
            branches,
            explicit_targets,
            numerical_targets,
        });
    }

    out.sort_by(|a, b| a.path_id.cmp(&b.path_id));
    Ok(out)
}

fn load_family_meta(families_dir: &Path) -> Result<Vec<FamilyMeta>, String> {
    if !families_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in WalkDir::new(families_dir) {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
        let yaml: Value =
            serde_yaml::from_str(&content).map_err(|e| format!("{}: {}", path.display(), e))?;
        let key = get_str(&yaml, "key").ok_or_else(|| format!("{} missing key", path.display()))?;
        let name = get_str(&yaml, "name").unwrap_or_else(|| key.clone());
        let description = get_str(&yaml, "description").unwrap_or_default();
        let canonical_equation = get_str(&yaml, "canonical_equation")
            .ok_or_else(|| format!("{} missing canonical_equation", path.display()))?;
        let variants = yaml
            .get("variants")
            .and_then(Value::as_sequence)
            .ok_or_else(|| format!("{} missing variants list", path.display()))?
            .iter()
            .map(|v| {
                let key = v
                    .get("key")
                    .and_then(Value::as_str)
                    .ok_or_else(|| format!("{} variant missing key", path.display()))?
                    .to_string();
                let name = v
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or(&key)
                    .to_string();
                let equation_id = v
                    .get("equation_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        format!("{} variant '{}' missing equation_id", path.display(), key)
                    })?
                    .to_string();
                Ok(FamilyVariantMeta {
                    key,
                    name,
                    equation_id,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        out.push(FamilyMeta {
            key: normalize_snake(&key),
            name,
            description,
            canonical_equation,
            variants,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    Ok(out)
}

fn render_typed_families(families: &[FamilyMeta], equations: &[EquationMeta]) -> String {
    let mut out = String::new();
    out.push_str("// @generated by build.rs; do not edit manually.\n");
    out.push_str("pub const GENERATED_FAMILY_IDS: &[&str] = &[\n");
    for f in families {
        out.push_str(&format!("    \"{}\",\n", f.key));
    }
    out.push_str("];\n\n");

    let by_id: BTreeMap<String, &EquationMeta> =
        equations.iter().map(|e| (e.path_id.clone(), e)).collect();

    for family in families {
        let mod_name = safe_ident(&normalize_snake(&family.key));
        push_indent(&mut out, 0);
        out.push_str(&format!("pub mod {} {{\n", mod_name));
        let mut doc = String::new();
        doc.push_str(&family.name);
        if !family.description.trim().is_empty() {
            doc.push_str("\n\n");
            doc.push_str(&family.description);
        }
        push_doc_block(&mut out, 1, &doc);
        push_indent(&mut out, 1);
        out.push_str(&format!(
            "pub const CANONICAL_EQUATION: &str = \"{}\";\n",
            family.canonical_equation
        ));
        push_indent(&mut out, 1);
        out.push_str("pub const VARIANTS: &[&str] = &[\n");
        for v in &family.variants {
            push_indent(&mut out, 2);
            out.push_str(&format!("\"{}\",\n", v.key));
        }
        push_indent(&mut out, 1);
        out.push_str("];\n\n");

        for variant in &family.variants {
            if let Some(eq) = by_id.get(&variant.equation_id) {
                let fn_name = safe_ident(&normalize_snake(&variant.key));
                let call_path = equation_call_path(eq);
                let mut vdoc = String::new();
                vdoc.push_str(&variant.name);
                vdoc.push_str("\n\n");
                vdoc.push_str(&format!("**Variant key:** `{}`\n", variant.key));
                vdoc.push_str(&format!("**Equation:** `{}`\n", variant.equation_id));
                push_doc_block(&mut out, 1, &vdoc);
                push_indent(&mut out, 1);
                let type_path = equation_type_path(eq);
                out.push_str(&format!(
                    "pub fn {}() -> {} {{ {} }}\n\n",
                    fn_name, type_path, call_path
                ));
            }
        }
        push_indent(&mut out, 1);
        out.push_str("pub fn help() -> &'static str {\n");
        push_indent(&mut out, 2);
        out.push_str(&format!(
            "\"{} family with {} variants\"\n",
            escape_doc_line(&family.name),
            family.variants.len()
        ));
        push_indent(&mut out, 1);
        out.push_str("}\n");
        push_indent(&mut out, 0);
        out.push_str("}\n\n");
    }

    out
}

fn equation_call_path(eq: &EquationMeta) -> String {
    let mut parts: Vec<String> = eq.path_id.split('.').map(normalize_snake).collect();
    if parts.is_empty() {
        return "crate::eq.solve(\"unknown\")".to_string();
    }
    let mut path = String::from("crate::");
    for p in parts.drain(..) {
        path.push_str(&safe_ident(&p));
        path.push_str("::");
    }
    path.push_str("equation()");
    path
}

fn equation_type_path(eq: &EquationMeta) -> String {
    let mut parts: Vec<String> = eq.path_id.split('.').map(normalize_snake).collect();
    let mut path = String::from("crate::");
    for p in parts.drain(..) {
        path.push_str(&safe_ident(&p));
        path.push_str("::");
    }
    path.push_str(&eq.type_name);
    path
}

fn render_typed_api(equations: &[EquationMeta], constant_identifiers: &BTreeSet<String>) -> String {
    let mut root = ModuleNode::default();
    for eq in equations {
        let mut path = vec![eq.category.clone()];
        path.extend(eq.subcategories.clone());
        insert_equation(&mut root, &path, eq.clone());
    }

    let mut out = String::new();
    out.push_str("// @generated by build.rs; do not edit manually.\n");
    out.push_str("pub const GENERATED_EQUATION_IDS: &[&str] = &[\n");
    for eq in equations {
        out.push_str(&format!("    \"{}\",\n", eq.path_id));
    }
    out.push_str("];\n\n");
    render_module_children(&mut out, &root, 0, constant_identifiers);
    out
}

fn render_typed_constants(constants: &[ConstantMeta]) -> String {
    let mut out = String::new();
    out.push_str("// @generated by build.rs; do not edit manually.\n");
    out.push_str("pub const ALL_CONSTANTS: &[crate::constants::EngineeringConstant] = &[\n");
    for c in constants {
        out.push_str("    crate::constants::EngineeringConstant {\n");
        out.push_str(&format!("        key: \"{}\",\n", c.key));
        out.push_str(&format!(
            "        name: \"{}\",\n",
            escape_doc_line(&c.name)
        ));
        out.push_str(&format!(
            "        symbol_latex: \"{}\",\n",
            escape_doc_line(&c.symbol_latex)
        ));
        out.push_str(&format!(
            "        symbol_unicode: \"{}\",\n",
            escape_doc_line(&c.symbol_unicode)
        ));
        out.push_str(&format!(
            "        symbol_ascii: \"{}\",\n",
            escape_doc_line(&c.symbol_ascii)
        ));
        out.push_str(&format!(
            "        dimension: \"{}\",\n",
            escape_doc_line(&c.dimension)
        ));
        out.push_str(&format!(
            "        unit: \"{}\",\n",
            escape_doc_line(&c.unit)
        ));
        out.push_str(&format!("        value: {:.16e},\n", c.value));
        out.push_str(&format!("        exact: {},\n", c.exact));
        out.push_str(&format!(
            "        source: \"{}\",\n",
            escape_doc_line(&c.source)
        ));
        out.push_str(&format!(
            "        note: \"{}\",\n",
            escape_doc_line(&c.note)
        ));
        out.push_str(&format!(
            "        description: \"{}\",\n",
            escape_doc_line(&c.description)
        ));
        out.push_str("        aliases: &[");
        for (idx, alias) in c.aliases.iter().enumerate() {
            if idx > 0 {
                out.push_str(", ");
            }
            out.push('"');
            out.push_str(&escape_doc_line(alias));
            out.push('"');
        }
        out.push_str("],\n");
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");
    for c in constants {
        let fn_name = safe_ident(&normalize_snake(&c.key));
        let mut doc = String::new();
        doc.push_str(&c.name);
        doc.push_str("\n\n");
        doc.push_str(&format!("**Key:** `{}`\n", c.key));
        doc.push_str(&format!("**Symbol (LaTeX):** `{}`\n", c.symbol_latex));
        doc.push_str(&format!("**Symbol (Unicode):** `{}`\n", c.symbol_unicode));
        doc.push_str(&format!("**Symbol (ASCII):** `{}`\n", c.symbol_ascii));
        doc.push_str(&format!("**Dimension:** `{}`\n", c.dimension));
        doc.push_str(&format!("**Unit:** `{}`\n", c.unit));
        doc.push_str(&format!("**Value:** `{:.12e}`\n", c.value));
        doc.push_str(&format!(
            "**Trust:** `{}`\n",
            if c.exact {
                "exact/defined"
            } else {
                "conventional/reference"
            }
        ));
        if !c.source.trim().is_empty() {
            doc.push_str(&format!("**Source:** {}\n", c.source));
        }
        if !c.note.trim().is_empty() {
            doc.push_str(&format!("**Note:** {}\n", c.note));
        }
        if !c.aliases.is_empty() {
            doc.push_str(&format!("**Aliases:** `{}`\n", c.aliases.join("`, `")));
        }
        if !c.description.trim().is_empty() {
            doc.push_str("\n");
            doc.push_str(&c.description);
            doc.push('\n');
        }
        push_doc_block(&mut out, 0, &doc);
        push_indent(&mut out, 0);
        out.push_str(&format!(
            "pub fn {}() -> crate::constants::EngineeringConstant {{\n",
            fn_name
        ));
        push_indent(&mut out, 1);
        out.push_str(&format!(
            "crate::constants::get(\"{}\").expect(\"generated constant must exist\")\n",
            c.key
        ));
        push_indent(&mut out, 0);
        out.push_str("}\n");
    }
    out
}

fn render_module_children(
    out: &mut String,
    node: &ModuleNode,
    indent: usize,
    constant_identifiers: &BTreeSet<String>,
) {
    for (name, child) in &node.children {
        push_indent(out, indent);
        out.push_str(&format!("pub mod {} {{\n", safe_ident(name)));
        render_module_content(out, child, indent + 1, constant_identifiers);
        push_indent(out, indent);
        out.push_str("}\n\n");
    }
}

fn render_module_content(
    out: &mut String,
    node: &ModuleNode,
    indent: usize,
    constant_identifiers: &BTreeSet<String>,
) {
    for eq in &node.equations {
        render_equation_api(out, eq, indent, constant_identifiers);
    }
    render_module_children(out, node, indent, constant_identifiers);
}

fn render_equation_api(
    out: &mut String,
    eq: &EquationMeta,
    indent: usize,
    constant_identifiers: &BTreeSet<String>,
) {
    let eq_mod = safe_ident(&eq.fn_name);
    push_indent(out, indent);
    out.push_str(&format!("pub mod {} {{\n", eq_mod));

    push_indent(out, indent + 1);
    out.push_str("#[derive(Debug, Clone, Copy)]\n");
    push_indent(out, indent + 1);
    out.push_str(&format!("pub struct {};\n", eq.type_name));
    push_doc_block(out, indent + 1, &constructor_doc(eq, constant_identifiers));
    push_indent(out, indent + 1);
    out.push_str(&format!(
        "pub fn equation() -> {} {{ {} }}\n",
        eq.type_name, eq.type_name
    ));

    push_indent(out, indent + 1);
    out.push_str(&format!(
        "impl crate::api::IntoEquationId for {} {{\n",
        eq.type_name
    ));
    push_indent(out, indent + 2);
    out.push_str(&format!(
        "fn equation_id(&self) -> &str {{ \"{}\" }}\n",
        eq.path_id
    ));
    push_indent(out, indent + 1);
    out.push_str("}\n");

    push_indent(out, indent + 1);
    out.push_str(&format!(
        "impl crate::api::SolveStart for {} {{\n",
        eq.type_name
    ));
    push_indent(out, indent + 2);
    out.push_str(&format!("type Builder = {};\n", eq.builder_name));
    push_indent(out, indent + 2);
    out.push_str("fn into_builder(self, facade: crate::EqFacade) -> Self::Builder {\n");
    push_indent(out, indent + 3);
    out.push_str(&format!(
        "{} {{ inner: crate::api::SolveBuilder::new(facade, \"{}\") }}\n",
        eq.builder_name, eq.path_id
    ));
    push_indent(out, indent + 2);
    out.push_str("}\n");
    push_indent(out, indent + 1);
    out.push_str("}\n");

    push_indent(out, indent + 1);
    out.push_str("#[derive(Debug, Clone)]\n");
    push_indent(out, indent + 1);
    out.push_str(&format!(
        "pub struct {} {{ inner: crate::api::SolveBuilder }}\n",
        eq.builder_name
    ));

    push_indent(out, indent + 1);
    out.push_str(&format!("impl {} {{\n", eq.builder_name));
    push_doc_line(out, indent + 2, "Select solve target by raw variable key.");
    push_indent(out, indent + 2);
    out.push_str("pub fn for_target(self, target: &str) -> Self { Self { inner: self.inner.for_target(target) } }\n");
    push_doc_line(out, indent + 2, "Provide multiple known variable values.");
    push_indent(out, indent + 2);
    out.push_str("pub fn givens<I, K, V>(self, values: I) -> Self where I: IntoIterator<Item=(K,V)>, K: Into<String>, V: crate::IntoSolveInput { Self { inner: self.inner.givens(values) } }\n");
    push_doc_line(
        out,
        indent + 2,
        "Override a registry constant by key/symbol/alias for this solve call.",
    );
    push_indent(out, indent + 2);
    out.push_str("pub fn override_constant<V: crate::IntoSolveInput>(self, name: &str, value: V) -> Self { Self { inner: self.inner.override_constant(name, value) } }\n");
    push_doc_line(
        out,
        indent + 2,
        "Override multiple registry constants for this solve call.",
    );
    push_indent(out, indent + 2);
    out.push_str("pub fn override_constants<I, K, V>(self, values: I) -> Self where I: IntoIterator<Item=(K,V)>, K: Into<String>, V: crate::IntoSolveInput { Self { inner: self.inner.override_constants(values) } }\n");
    push_doc_line(out, indent + 2, "Select branch by raw branch name.");
    push_indent(out, indent + 2);
    out.push_str(
        "pub fn branch(self, branch: &str) -> Self { Self { inner: self.inner.branch(branch) } }\n",
    );
    push_doc_line(out, indent + 2, "Choose solve method.");
    push_indent(out, indent + 2);
    out.push_str("pub fn method(self, method: crate::SolveMethod) -> Self { Self { inner: self.inner.method(method) } }\n");
    push_doc_line(out, indent + 2, "Solve and return SI scalar value.");
    push_indent(out, indent + 2);
    out.push_str("pub fn value(self) -> crate::Result<f64> { self.inner.value() }\n");
    push_doc_line(
        out,
        indent + 2,
        "Solve and return scalar value converted to requested unit.",
    );
    push_indent(out, indent + 2);
    out.push_str(
        "pub fn value_in(self, unit: &str) -> crate::Result<f64> { self.inner.value_in(unit) }\n",
    );
    push_doc_line(out, indent + 2, "Solve and return full diagnostics result.");
    push_indent(out, indent + 2);
    out.push_str(
        "pub fn result(self) -> crate::Result<crate::SolveResult> { self.inner.result() }\n",
    );

    let target_names = unique_method_names(
        eq.variables
            .iter()
            .map(|v| format!("target_{}", normalize_snake(&v.key)))
            .collect(),
    );
    let target_map: BTreeMap<String, String> = eq
        .variables
        .iter()
        .zip(target_names.iter())
        .map(|(v, m)| (v.key.clone(), m.clone()))
        .collect();
    for (var, m) in eq.variables.iter().zip(target_names.iter()) {
        push_doc_block(out, indent + 2, &target_method_doc(eq, var));
        push_indent(out, indent + 2);
        out.push_str(&format!(
            "pub fn {}(self) -> Self {{ Self {{ inner: self.inner.for_target(\"{}\") }} }}\n",
            m, var.key
        ));
    }
    let given_names = unique_method_names(
        eq.variables
            .iter()
            .map(|v| format!("given_{}", normalize_snake(&v.key)))
            .collect(),
    );
    let given_map: BTreeMap<String, String> = eq
        .variables
        .iter()
        .zip(given_names.iter())
        .map(|(v, m)| (v.key.clone(), m.clone()))
        .collect();
    for (var, m) in eq.variables.iter().zip(given_names.iter()) {
        push_doc_block(out, indent + 2, &given_method_doc(eq, var));
        push_indent(out, indent + 2);
        out.push_str(&format!(
            "pub fn {}<V: crate::IntoSolveInput>(self, value: V) -> Self {{ Self {{ inner: self.inner.given(\"{}\", value) }} }}\n",
            m, var.key
        ));
    }
    let branch_names = unique_method_names(
        eq.branches
            .iter()
            .map(|b| format!("branch_{}", normalize_snake(b)))
            .collect(),
    );
    for (branch, m) in eq.branches.iter().zip(branch_names.iter()) {
        push_doc_block(out, indent + 2, &branch_method_doc(eq, branch));
        push_indent(out, indent + 2);
        out.push_str(&format!(
            "pub fn {}(self) -> Self {{ Self {{ inner: self.inner.branch(\"{}\") }} }}\n",
            m, branch
        ));
    }
    push_indent(out, indent + 1);
    out.push_str("}\n\n");

    for target in &eq.explicit_targets {
        let Some(target_method) = target_map.get(target) else {
            continue;
        };
        let inputs: Vec<&VariableMeta> = eq
            .variables
            .iter()
            .filter(|v| &v.key != target && !is_auto_constant_var(&v.key, constant_identifiers))
            .collect();
        let fn_name = safe_ident(&format!("solve_{}", normalize_snake(target)));
        push_doc_block(out, indent + 1, &convenience_solve_doc(eq, target, &inputs));
        push_indent(out, indent + 1);
        out.push_str(&format!("pub fn {}(", fn_name));
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!(
                "{}: impl crate::IntoSolveInput",
                safe_ident(&normalize_snake(&input.key))
            ));
        }
        out.push_str(") -> crate::Result<f64> {\n");
        push_indent(out, indent + 2);
        out.push_str("crate::eq.solve(equation())\n");
        push_indent(out, indent + 3);
        out.push_str(&format!(".{}()\n", target_method));
        for input in &inputs {
            let gm = given_map.get(&input.key).expect("given method map");
            push_indent(out, indent + 3);
            out.push_str(&format!(
                ".{}({})\n",
                gm,
                safe_ident(&normalize_snake(&input.key))
            ));
        }
        push_indent(out, indent + 3);
        out.push_str(".value()\n");
        push_indent(out, indent + 1);
        out.push_str("}\n\n");
    }

    // Optional self-describing helpers for discoverability.
    push_indent(out, indent + 1);
    out.push_str("pub fn help() -> &'static str {\n");
    push_indent(out, indent + 2);
    out.push_str(&format!(
        "\"{} ({})\"\n",
        escape_doc_line(&eq.name),
        eq.path_id
    ));
    push_indent(out, indent + 1);
    out.push_str("}\n");
    push_indent(out, indent + 1);
    out.push_str("pub fn examples() -> &'static str {\n");
    push_indent(out, indent + 2);
    out.push_str("\"Use eq.solve(equation()) for builder flow; use solve_<target>(...) for direct explicit solves.\"\n");
    push_indent(out, indent + 1);
    out.push_str("}\n");

    push_indent(out, indent);
    out.push_str("}\n\n");

    // Backward-compatibility shims (flat style convenience functions).
    for target in &eq.explicit_targets {
        let inputs: Vec<&VariableMeta> = eq
            .variables
            .iter()
            .filter(|v| &v.key != target && !is_auto_constant_var(&v.key, constant_identifiers))
            .collect();
        let old_flat = safe_ident(&format!(
            "solve_{}_{}",
            normalize_snake(&eq.fn_name),
            normalize_snake(target)
        ));
        let new_fn = safe_ident(&format!("solve_{}", normalize_snake(target)));
        push_indent(out, indent);
        out.push_str(&format!("pub fn {}(", old_flat));
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!(
                "{}: impl crate::IntoSolveInput",
                safe_ident(&normalize_snake(&input.key))
            ));
        }
        out.push_str(") -> crate::Result<f64> {\n");
        push_indent(out, indent + 1);
        out.push_str(&format!("{}::{}(", eq_mod, new_fn));
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&safe_ident(&normalize_snake(&input.key)));
        }
        out.push_str(")\n");
        push_indent(out, indent);
        out.push_str("}\n\n");
    }
}

fn insert_equation(root: &mut ModuleNode, module_path: &[String], equation: EquationMeta) {
    let mut node = root;
    for p in module_path {
        node = node.children.entry(p.clone()).or_default();
    }
    node.equations.push(equation);
}

fn get_str(yaml: &Value, key: &str) -> Option<String> {
    yaml.get(key).and_then(Value::as_str).map(str::to_string)
}

fn get_nested_str(yaml: &Value, path: &[&str]) -> Option<String> {
    let mut cur = yaml;
    for p in path {
        cur = cur.get(*p)?;
    }
    cur.as_str().map(str::to_string)
}

fn get_nested_seq_str(yaml: &Value, path: &[&str]) -> Option<Vec<String>> {
    let mut cur = yaml;
    for p in path {
        cur = cur.get(*p)?;
    }
    let seq = cur.as_sequence()?;
    Some(
        seq.iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
    )
}

fn get_variables(yaml: &Value) -> Vec<VariableMeta> {
    let mut out = Vec::new();
    let Some(vars) = yaml.get("variables").and_then(Value::as_mapping) else {
        return out;
    };
    for (k, v) in vars {
        let Some(key) = k.as_str() else { continue };
        let name = v
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(key)
            .to_string();
        let dimension = v
            .get("dimension")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let default_unit = v
            .get("default_unit")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| default_unit_for_dimension(&dimension).to_string());
        out.push(VariableMeta {
            key: key.to_string(),
            name,
            dimension,
            default_unit,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

fn get_assumptions(yaml: &Value) -> Vec<String> {
    yaml.get("assumptions")
        .and_then(Value::as_sequence)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn get_display_line(yaml: &Value) -> Option<String> {
    let display = yaml.get("display")?;
    let direct = display
        .get("unicode")
        .and_then(Value::as_str)
        .or_else(|| display.get("ascii").and_then(Value::as_str))
        .map(str::to_string);
    if direct.is_some() {
        return direct;
    }

    derive_plain_equation_line(yaml)
}

fn derive_plain_equation_line(yaml: &Value) -> Option<String> {
    let solve = yaml.get("solve")?;
    let explicit = solve.get("explicit_forms")?.as_mapping()?;
    if explicit.is_empty() {
        return yaml
            .get("residual")
            .and_then(Value::as_str)
            .map(|r| format!("{r} = 0"));
    }

    if let Some(default_target) = solve.get("default_target").and_then(Value::as_str)
        && let Some(expr) = explicit.get(&Value::String(default_target.to_string()))
        && let Some(expr) = expr.as_str()
    {
        return Some(format!("{default_target} = {expr}"));
    }

    let mut pairs: Vec<(String, String)> = explicit
        .iter()
        .filter_map(|(k, v)| Some((k.as_str()?.to_string(), v.as_str()?.to_string())))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs.first().map(|(k, v)| format!("{k} = {v}"))
}

fn get_branch_names(yaml: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(branches) = yaml.get("branches").and_then(Value::as_sequence) {
        for b in branches {
            if let Some(name) = b.get("name").and_then(Value::as_str) {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

fn get_explicit_targets(yaml: &Value) -> BTreeSet<String> {
    yaml.get("solve")
        .and_then(|s| s.get("explicit_forms"))
        .and_then(Value::as_mapping)
        .map(|m| {
            m.keys()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default()
}

fn get_numerical_targets(yaml: &Value, vars: &[VariableMeta]) -> BTreeSet<String> {
    let all: BTreeSet<String> = vars.iter().map(|v| v.key.clone()).collect();
    let unsupported: BTreeSet<String> = yaml
        .get("solve")
        .and_then(|s| s.get("numerical"))
        .and_then(|n| n.get("unsupported_targets"))
        .and_then(Value::as_sequence)
        .map(|seq| {
            seq.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    all.difference(&unsupported).cloned().collect()
}

fn default_unit_for_dimension(dimension: &str) -> &'static str {
    match dimension.trim().to_ascii_lowercase().as_str() {
        "dimensionless" | "ratio" | "friction_factor" | "mach" => "1",
        "pressure" | "stress" => "Pa",
        "length" | "diameter" | "distance" | "roughness" => "m",
        "temperature" => "K",
        _ => "?",
    }
}

fn constructor_doc(eq: &EquationMeta, constant_identifiers: &BTreeSet<String>) -> String {
    let mut doc = String::new();
    doc.push_str(&eq.name);
    doc.push_str("\n\n");
    doc.push_str(&format!("**Equation:** `{}`\n", eq.display_line));
    doc.push_str(&format!("**Path ID:** `{}`\n\n", eq.path_id));
    doc.push_str("**Variables**\n\n");
    for v in &eq.variables {
        doc.push_str(&format!(
            "* `{}` — {} (`{}`, default `{}`)\n",
            v.key, v.name, v.dimension, v.default_unit
        ));
    }
    let used_constants: Vec<&VariableMeta> = eq
        .variables
        .iter()
        .filter(|v| is_auto_constant_var(&v.key, constant_identifiers))
        .collect();
    if !used_constants.is_empty() {
        doc.push_str("\n**Auto-resolved constants**\n\n");
        for c in &used_constants {
            doc.push_str(&format!("* `{}` ({})\n", c.key, c.name));
        }
        doc.push_str("* Override with `.override_constant(\"name\", value)` when needed.\n");
    }
    if !eq.assumptions.is_empty() {
        doc.push_str("\n**Assumptions**\n\n");
        for a in &eq.assumptions {
            doc.push_str(&format!("* {}\n", a));
        }
    }
    let targets = eq
        .variables
        .iter()
        .map(|v| format!("`{}`", v.key))
        .collect::<Vec<_>>()
        .join(", ");
    doc.push_str(&format!("\n**Supported targets:** {}\n", targets));
    if !eq.branches.is_empty() {
        let branches = eq
            .branches
            .iter()
            .map(|b| format!("`{}`", b))
            .collect::<Vec<_>>()
            .join(", ");
        doc.push_str(&format!("**Branches:** {}\n", branches));
    }
    doc.push_str("\n# Example\n\n```no_run\n");
    doc.push_str("use equations::{eq};\n\n");
    doc.push_str("let value = eq\n");
    doc.push_str(&format!(
        "    .solve(equations::{}::{}::equation())\n",
        module_path(eq),
        safe_ident(&eq.fn_name)
    ));
    let target = eq.default_target.as_deref().unwrap_or_else(|| {
        eq.variables
            .first()
            .map(|v| v.key.as_str())
            .unwrap_or("target")
    });
    doc.push_str(&format!(
        "    .target_{}()\n",
        safe_ident(&normalize_snake(target))
    ));
    for v in &eq.variables {
        if is_auto_constant_var(&v.key, constant_identifiers) {
            continue;
        }
        doc.push_str(&format!(
            "    .given_{}(\"1 {}\")\n",
            safe_ident(&normalize_snake(&v.key)),
            v.default_unit
        ));
    }
    if let Some(first_branch) = eq.branches.first() {
        doc.push_str(&format!(
            "    .branch_{}()\n",
            safe_ident(&normalize_snake(first_branch))
        ));
    }
    doc.push_str("    .value()\n");
    doc.push_str("    .expect(\"solve\");\n```\n");
    doc
}

fn target_method_doc(eq: &EquationMeta, var: &VariableMeta) -> String {
    let mut methods = Vec::new();
    if eq.explicit_targets.contains(&var.key) {
        methods.push("explicit");
    }
    if eq.numerical_targets.contains(&var.key) {
        methods.push("numerical");
    }
    let methods = if methods.is_empty() {
        "none".to_string()
    } else {
        methods.join(", ")
    };
    format!(
        "Solve **{}** for `{}`.\n\n**Variable:** {}\n**Dimension:** `{}`\n**Default unit:** `{}`\n**Supported methods:** {}\n\nThis selects the solve target.",
        eq.name, var.key, var.name, var.dimension, var.default_unit, methods
    )
}

fn given_method_doc(eq: &EquationMeta, var: &VariableMeta) -> String {
    format!(
        "Provide `{}` for **{}**.\n\n**Variable:** {}\n**Dimension:** `{}`\n**Default unit:** `{}`\n\nAccepts:\n\n* canonical SI numeric input\n* unit-tagged string input, e.g. `\"2.5 {}`\"",
        var.key, eq.name, var.name, var.dimension, var.default_unit, var.default_unit
    )
}

fn branch_method_doc(eq: &EquationMeta, branch: &str) -> String {
    format!("Select the `{}` branch for **{}**.", branch, eq.name)
}

fn convenience_solve_doc(eq: &EquationMeta, target: &str, inputs: &[&VariableMeta]) -> String {
    let inputs_line = inputs
        .iter()
        .map(|v| format!("`{}`", v.key))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Convenience solve for **{}** target `{}`.\n\n**Inputs:** {}\n\nAccepts SI numeric values or unit-tagged strings for each input.\n\nReturns SI scalar `f64`.",
        eq.name, target, inputs_line
    )
}

fn constant_identifiers(constants: &[ConstantMeta]) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for c in constants {
        ids.insert(c.key.to_ascii_lowercase());
        ids.insert(c.symbol_ascii.to_ascii_lowercase());
        for alias in &c.aliases {
            ids.insert(alias.to_ascii_lowercase());
        }
    }
    ids
}

fn is_auto_constant_var(key: &str, constant_identifiers: &BTreeSet<String>) -> bool {
    constant_identifiers.contains(&key.to_ascii_lowercase())
}

fn normalize_snake(input: &str) -> String {
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

fn to_pascal(s: &str) -> String {
    s.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(h) => h.to_ascii_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn safe_ident(name: &str) -> String {
    let n = normalize_snake(name);
    let keywords: BTreeSet<&str> = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ]
    .into_iter()
    .collect();
    if keywords.contains(n.as_str()) {
        format!("{n}_")
    } else {
        n
    }
}

fn unique_method_names(names: Vec<String>) -> Vec<String> {
    let mut used = BTreeSet::new();
    let mut out = Vec::with_capacity(names.len());
    for base in names {
        let mut candidate = safe_ident(&base);
        if !used.insert(candidate.clone()) {
            let mut i = 2usize;
            loop {
                let c = format!("{}_{}", candidate, i);
                if used.insert(c.clone()) {
                    candidate = c;
                    break;
                }
                i += 1;
            }
        }
        out.push(candidate);
    }
    out
}

fn push_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("    ");
    }
}

fn push_doc_line(out: &mut String, indent: usize, text: &str) {
    push_indent(out, indent);
    out.push_str(&format!("#[doc = \"{}\"]\n", escape_doc_line(text)));
}

fn push_doc_block(out: &mut String, indent: usize, text: &str) {
    let mut escaped = text.replace('\\', "\\\\");
    escaped = escaped.replace('\"', "\\\"");
    escaped = escaped.replace('\r', "");
    escaped = escaped.replace('\n', "\\n");
    push_indent(out, indent);
    out.push_str(&format!("#[doc = \"{}\"]\n", escaped));
}

fn escape_doc_line(text: &str) -> String {
    text.replace('\\', "\\\\").replace('\"', "\\\"")
}

fn module_path(eq: &EquationMeta) -> String {
    let mut parts = vec![safe_ident(&eq.category)];
    parts.extend(eq.subcategories.iter().map(|s| safe_ident(s)));
    parts.join("::")
}
