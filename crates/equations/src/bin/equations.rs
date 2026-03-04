use std::{env, path::PathBuf, process};

use equations::{Registry, generate_schema_to_path, run_registry_tests};

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
        "export-docs" | "export-mdbook" | "export-html" | "serve-book" | "export-pdf" => Err(
            "Unified export commands are owned by the top-level `eng` CLI.\nRun: cargo run -p eng --bin eng -- <export-docs|export-mdbook|export-html|serve-book|export-pdf>".to_string(),
        ),
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
source: \"<primary source citation>\"
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

fn default_registry_path() -> PathBuf {
    crate_root().join("registry")
}

fn default_schema_path() -> PathBuf {
    crate_root().join("schemas").join("equation.schema.json")
}

fn usage() -> String {
    "usage:
  equations generate-schema [--schema-out PATH]
  equations validate [--registry-dir PATH] [--with-tests]
  equations test-registry [--registry-dir PATH]
  equations lint [--registry-dir PATH] [--strict]
  equations scaffold --key KEY --category CATEGORY [--name NAME] [--out PATH] [--force]

Unified docs/export commands moved to:
  cargo run -p eng --bin eng -- export-docs [--out-dir PATH]
  cargo run -p eng --bin eng -- export-mdbook [--out-dir PATH]
  cargo run -p eng --bin eng -- export-html [--out-dir PATH]
  cargo run -p eng --bin eng -- serve-book [--out-dir PATH] [--no-open]
  cargo run -p eng --bin eng -- export-pdf [--out-file PATH]"
        .to_string()
}
