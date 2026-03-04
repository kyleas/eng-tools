use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    process,
};

use eng::bindings::INVOKE_PROTOCOL_VERSION;
use eng::docs::{
    HtmlBuildStatus, default_book_root, default_generated_root, export_unified_catalog,
    export_unified_docs_to, export_unified_html_to, export_unified_mdbook_to,
    export_unified_pdf_to,
};
use eng::invoke::process_invoke_json;

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
        "export-docs" => {
            let out_dir =
                parse_flag_path(&args, "--out-dir").unwrap_or_else(default_generated_root);
            export_unified_docs_to(&out_dir).map_err(|e| e.to_string())?;
            println!(
                "wrote unified docs/catalog artifacts: {}",
                out_dir.display()
            );
            println!("catalog: {}", out_dir.join("catalog.json").display());
            Ok(())
        }
        "export-mdbook" => {
            let out_dir = parse_flag_path(&args, "--out-dir").unwrap_or_else(default_book_root);
            let paths = export_unified_mdbook_to(&out_dir).map_err(|e| e.to_string())?;
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
            let out_dir = parse_flag_path(&args, "--out-dir").unwrap_or_else(default_book_root);
            let report = export_unified_html_to(&out_dir).map_err(|e| e.to_string())?;
            println!("unified mdBook source generated.");
            print_mdbook_paths(&report.paths);
            match report.status {
                HtmlBuildStatus::Built => {
                    let html_dir = default_generated_root().join("html");
                    println!("HTML build completed via `mdbook build`.");
                    println!("Unified HTML output: {}", html_dir.display());
                    println!("Open: {}", html_dir.join("index.html").display());
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
            let out_dir = parse_flag_path(&args, "--out-dir").unwrap_or_else(default_book_root);
            let no_open = args.iter().any(|a| a == "--no-open");
            let paths = export_unified_mdbook_to(&out_dir).map_err(|e| e.to_string())?;
            println!("unified mdBook source generated.");
            print_mdbook_paths(&paths);
            let mut cmd = process::Command::new("mdbook");
            cmd.arg("serve").arg(&paths.source_dir);
            if !no_open {
                cmd.arg("--open");
            }
            let status = cmd.status().map_err(|_| {
                "mdbook executable not found in PATH. Install with: cargo install mdbook"
                    .to_string()
            })?;
            if status.success() {
                Ok(())
            } else {
                Err(format!("mdbook serve failed with status {status}"))
            }
        }
        "export-pdf" => {
            let out_file = parse_flag_path(&args, "--out-file")
                .unwrap_or_else(|| default_generated_root().join("engineering_handbook.pdf"));
            export_unified_pdf_to(&out_file).map_err(|e| e.to_string())?;
            println!("wrote unified pdf handbook: {}", out_file.display());
            Ok(())
        }
        "export-catalog" => {
            let path = export_unified_catalog().map_err(|e| e.to_string())?;
            println!("catalog: {}", path.display());
            Ok(())
        }
        "invoke" => {
            let req_json = parse_flag_value(&args, "--request-json")
                .ok_or_else(|| "invoke requires --request-json '{...}'".to_string())?;
            let resp = process_invoke_json(&req_json);
            println!(
                "{}",
                serde_json::to_string(&resp).map_err(|e| format!("serialize response: {e}"))?
            );
            Ok(())
        }
        "worker" => run_worker(),
        _ => Err(usage()),
    }
}

fn run_worker() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .map_err(|e| format!("worker read error: {e}"))?;
        if n == 0 {
            break;
        }
        let req_line = line.trim();
        if req_line.is_empty() {
            continue;
        }
        let resp = process_invoke_json(req_line);
        let out =
            serde_json::to_string(&resp).map_err(|e| format!("worker serialize response: {e}"))?;
        stdout
            .write_all(out.as_bytes())
            .map_err(|e| format!("worker write error: {e}"))?;
        stdout
            .write_all(b"\n")
            .map_err(|e| format!("worker write error: {e}"))?;
        stdout
            .flush()
            .map_err(|e| format!("worker flush error: {e}"))?;
    }
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

fn usage() -> String {
    format!(
        "usage:
  eng export-docs [--out-dir PATH]
  eng export-mdbook [--out-dir PATH]
  eng export-html [--out-dir PATH]
  eng serve-book [--out-dir PATH] [--no-open]
  eng export-pdf [--out-file PATH]
  eng export-catalog
  eng invoke --request-json '{{\"protocol_version\":\"{}\",\"op\":\"...\",\"request_id\":\"...\",\"args\":{{...}}}}'
  eng worker  # persistent JSON-lines invoke worker on stdin/stdout",
        INVOKE_PROTOCOL_VERSION
    )
}

fn print_mdbook_paths(paths: &eng::docs::MdBookPaths) {
    println!("Unified mdBook source: {}", paths.source_dir.display());
    println!("Unified HTML output: {}", paths.html_dir.display());
    println!("Open: {}", paths.html_index.display());
}
