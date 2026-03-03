use std::{env, fs, path::PathBuf, process};

use equations::{
    Registry,
    docs::{
        HtmlBuildStatus, export_docs_artifacts, export_html_docs, export_mdbook_source,
        export_pdf_handbook, serve_mdbook,
    },
    generate_schema_to_path, run_registry_tests,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(usage());
    }
    let cmd = args.remove(0);

    match cmd.as_str() {
        "generate-schema" => {
            let schema_out =
                parse_flag_path(&args, "--schema-out").unwrap_or_else(default_schema_path);
            generate_schema_to_path(&schema_out).map_err(|e| e.to_string())?;
            println!("wrote schema: {}", schema_out.display());
            Ok(())
        }
        "export-docs" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let out_dir =
                parse_flag_path(&args, "--out-dir").unwrap_or_else(default_generated_path);
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            export_docs_artifacts(registry.equations(), &out_dir).map_err(|e| e.to_string())?;
            println!(
                "wrote unified docs/catalog artifacts: {}",
                out_dir.display()
            );
            println!("catalog: {}", out_dir.join("catalog.json").display());
            Ok(())
        }
        "export-mdbook" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let out_dir = parse_flag_path(&args, "--out-dir")
                .unwrap_or_else(|| default_generated_path().join("book"));
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            let paths =
                export_mdbook_source(registry.equations(), &out_dir).map_err(|e| e.to_string())?;
            println!("unified mdBook source generated.");
            print_mdbook_paths(&paths);
            println!(
                "Preview locally: run `mdbook serve --open` in {}",
                paths.source_dir.display()
            );
            println!(
                "Build static HTML: run `mdbook build` in {}",
                paths.source_dir.display()
            );
            Ok(())
        }
        "export-html" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let out_dir = parse_flag_path(&args, "--out-dir")
                .unwrap_or_else(|| default_generated_path().join("book"));
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            let report =
                export_html_docs(registry.equations(), &out_dir).map_err(|e| e.to_string())?;
            println!("unified mdBook source generated.");
            print_mdbook_paths(&report.paths);
            match report.status {
                HtmlBuildStatus::Built => {
                    let compat_html_dir = default_generated_path().join("html");
                    mirror_directory(&report.paths.html_dir, &compat_html_dir)?;
                    println!("HTML build completed via `mdbook build`.");
                    println!("Unified HTML output: {}", compat_html_dir.display());
                    println!("Open: {}", compat_html_dir.join("index.html").display());
                }
                HtmlBuildStatus::MdBookNotInstalled { message } => {
                    println!("HTML build skipped: mdbook not found in PATH.");
                    println!();
                    println!("{message}");
                    println!("Install: cargo install mdbook");
                    println!(
                        "Then run in {}: mdbook build  (or mdbook serve --open)",
                        report.paths.source_dir.display()
                    );
                }
            }
            Ok(())
        }
        "serve-book" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let out_dir = parse_flag_path(&args, "--out-dir")
                .unwrap_or_else(|| default_generated_path().join("book"));
            let no_open = args.iter().any(|a| a == "--no-open");
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            let paths =
                export_mdbook_source(registry.equations(), &out_dir).map_err(|e| e.to_string())?;
            println!("unified mdBook source generated.");
            print_mdbook_paths(&paths);
            println!(
                "Starting `mdbook serve{}` from {}",
                if no_open { "" } else { " --open" },
                paths.source_dir.display()
            );
            serve_mdbook(&paths.source_dir, !no_open).map_err(|e| e.to_string())
        }
        "export-pdf" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let out_file = parse_flag_path(&args, "--out-file")
                .unwrap_or_else(|| default_generated_path().join("engineering_handbook.pdf"));
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            export_pdf_handbook(registry.equations(), &out_file).map_err(|e| e.to_string())?;
            println!("wrote unified pdf handbook: {}", out_file.display());
            Ok(())
        }
        "validate" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let with_tests = args.iter().any(|a| a == "--with-tests");
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            if with_tests {
                registry.validate_with_tests().map_err(|e| e.to_string())?;
            } else {
                registry.validate().map_err(|e| e.to_string())?;
            }
            println!("registry valid: {}", registry_dir.display());
            Ok(())
        }
        "test-registry" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            let summary = run_registry_tests(&registry).map_err(|e| e.to_string())?;
            println!(
                "registry tests passed: {} passed, {} failed",
                summary.passed, summary.failed
            );
            Ok(())
        }
        "lint" => {
            let registry_dir =
                parse_flag_path(&args, "--registry-dir").unwrap_or_else(default_registry_path);
            let strict = args.iter().any(|a| a == "--strict");
            let registry = Registry::load_from_dir(&registry_dir).map_err(|e| e.to_string())?;
            registry.validate().map_err(|e| e.to_string())?;
            let warnings = registry.lint().map_err(|e| e.to_string())?;
            for w in &warnings {
                println!("[{}] {}: {}", w.code, w.equation, w.message);
            }
            println!("lint warnings: {}", warnings.len());
            if strict && !warnings.is_empty() {
                return Err("lint strict mode failed".to_string());
            }
            Ok(())
        }
        "scaffold" => {
            let key = parse_required_flag(&args, "--key")?;
            let category = parse_required_flag(&args, "--category")?;
            let name = parse_flag_value(&args, "--name")
                .unwrap_or_else(|| title_case_from_key(&key).to_string());
            let force = args.iter().any(|a| a == "--force");
            let out_path = parse_flag_path(&args, "--out").unwrap_or_else(|| {
                default_registry_path()
                    .join(&category)
                    .join(format!("{key}.yaml"))
            });
            scaffold_equation(&out_path, &key, &category, &name, force)?;
            println!("wrote scaffold: {}", out_path.display());
            Ok(())
        }
        _ => Err(usage()),
    }
}

fn mirror_directory(source: &PathBuf, target: &PathBuf) -> Result<(), String> {
    if !source.exists() {
        return Err(format!(
            "html source directory does not exist: {}",
            source.display()
        ));
    }
    if target.exists() {
        fs::remove_dir_all(target)
            .map_err(|e| format!("failed to clear {}: {e}", target.display()))?;
    }
    copy_dir_recursive(source, target)
}

fn copy_dir_recursive(source: &PathBuf, target: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(target)
        .map_err(|e| format!("failed to create {}: {e}", target.display()))?;
    for entry in
        fs::read_dir(source).map_err(|e| format!("failed to read {}: {e}", source.display()))?
    {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = target.join(entry.file_name());
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| {
                format!(
                    "failed to copy {} -> {}: {e}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn scaffold_equation(
    out_path: &PathBuf,
    key: &str,
    category: &str,
    name: &str,
    force: bool,
) -> Result<(), String> {
    if out_path.exists() && !force {
        return Err(format!(
            "refusing to overwrite existing file {} (use --force to overwrite)",
            out_path.display()
        ));
    }
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create {}: {e}", parent.display()))?;
    }
    let template = format!(
        "key: {key}
taxonomy:
  category: {category}
name: {name}
display:
  latex: \"<equation latex>\"
variables:
  x:
    name: Variable x
    dimension: dimensionless
  y:
    name: Variable y
    dimension: dimensionless
residual: \"x - y\"
solve:
  explicit_forms:
    x: y
assumptions:
  - Add equation assumptions.
tests:
  - full_state:
      x: \"1\"
      y: \"1\"
"
    );
    std::fs::write(out_path, template)
        .map_err(|e| format!("failed to write {}: {e}", out_path.display()))?;
    Ok(())
}

fn parse_flag_path(args: &[String], flag: &str) -> Option<PathBuf> {
    args.windows(2).find_map(|pair| {
        if pair[0] == flag {
            Some(PathBuf::from(pair[1].clone()))
        } else {
            None
        }
    })
}

fn parse_flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find_map(|pair| {
        if pair[0] == flag {
            Some(pair[1].clone())
        } else {
            None
        }
    })
}

fn parse_required_flag(args: &[String], flag: &str) -> Result<String, String> {
    parse_flag_value(args, flag).ok_or_else(|| format!("missing required flag {flag}"))
}

fn title_case_from_key(key: &str) -> String {
    key.split(['_', '-'])
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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

fn default_registry_path() -> PathBuf {
    crate_root().join("registry")
}

fn default_schema_path() -> PathBuf {
    crate_root().join("schemas").join("equation.schema.json")
}

fn default_generated_path() -> PathBuf {
    workspace_root().join("generated")
}

fn usage() -> String {
    "usage:
  equations generate-schema [--schema-out PATH]
  equations export-docs [--registry-dir PATH] [--out-dir PATH]
  equations export-mdbook [--registry-dir PATH] [--out-dir PATH]
  equations export-html [--registry-dir PATH] [--out-dir PATH]
  equations serve-book [--registry-dir PATH] [--out-dir PATH] [--no-open]
  equations export-pdf [--registry-dir PATH] [--out-file PATH]
  equations validate [--registry-dir PATH] [--with-tests]
  equations test-registry [--registry-dir PATH]
  equations lint [--registry-dir PATH] [--strict]
  equations scaffold --key KEY --category CATEGORY [--name NAME] [--out PATH] [--force]"
        .to_string()
}

fn print_mdbook_paths(paths: &equations::docs::MdBookPaths) {
    println!("Unified mdBook source: {}", paths.source_dir.display());
    println!("Unified HTML output: {}", paths.html_dir.display());
    println!("Open: {}", paths.html_index.display());
}
