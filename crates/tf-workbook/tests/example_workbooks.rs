use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;
use tf_workbook::{
    WorkbookRowResult, WorkbookRowState, execute_workbook, load_workbook_dir, rename_row_key,
    save_workbook_dir, validate_workbook,
};

fn examples_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("workbooks")
}

fn workbook_paths() -> Vec<PathBuf> {
    vec![
        examples_root().join("injector_orifice_sizer.engwb"),
        examples_root().join("pipe_flow_pump_power.engwb"),
        examples_root().join("nozzle_shock_backpressure.engwb"),
        examples_root().join("oblique_vs_cone_shock.engwb"),
        examples_root().join("engineering_logbook.engwb"),
    ]
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create dst dir");
    for entry in fs::read_dir(src).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let ty = entry.file_type().expect("file type");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).expect("copy file");
        }
    }
}

#[test]
fn examples_load_validate_and_roundtrip() {
    for wb_path in workbook_paths() {
        let doc = load_workbook_dir(&wb_path).expect("load workbook");
        let validation = validate_workbook(&doc);
        assert!(
            validation.ok,
            "validation failed for {}: {:?}",
            wb_path.display(),
            validation.messages
        );

        let td = tempdir().expect("tempdir");
        let copy_path = td
            .path()
            .join(wb_path.file_name().expect("workbook dir name"));
        copy_dir_recursive(&wb_path, &copy_path);

        let doc_copy = load_workbook_dir(&copy_path).expect("load copied workbook");
        let before_tabs = doc_copy.tabs.len();
        let before_rows: usize = doc_copy.tabs.iter().map(|t| t.rows.len()).sum();
        save_workbook_dir(&doc_copy).expect("save copied workbook");
        let doc_reloaded = load_workbook_dir(&copy_path).expect("reload copied workbook");
        let after_tabs = doc_reloaded.tabs.len();
        let after_rows: usize = doc_reloaded.tabs.iter().map(|t| t.rows.len()).sum();

        assert_eq!(
            before_tabs,
            after_tabs,
            "tab count drifted for {}",
            wb_path.display()
        );
        assert_eq!(
            before_rows,
            after_rows,
            "row count drifted for {}",
            wb_path.display()
        );
    }
}

#[test]
fn examples_execute_smoke_and_plot_study_shape() {
    for wb_path in workbook_paths() {
        let doc = load_workbook_dir(&wb_path).expect("load workbook");
        let run = execute_workbook(&doc, None).expect("execute workbook");

        let mut ok_rows = 0usize;
        let mut bad_rows = 0usize;
        let mut has_study = false;
        let mut has_plot = false;

        for tab in &run.tabs {
            for row in &tab.rows {
                match row.state {
                    WorkbookRowState::Ok => ok_rows += 1,
                    WorkbookRowState::Invalid | WorkbookRowState::Error => bad_rows += 1,
                    _ => {}
                }
                if let Some(result) = &row.result {
                    match result {
                        WorkbookRowResult::Study(study) => {
                            has_study |= study.table.rows.len() > 5;
                        }
                        WorkbookRowResult::Plot(plot) => {
                            has_plot |= plot.series.iter().any(|s| s.x.len() > 5 && s.y.len() > 5);
                        }
                        _ => {}
                    }
                }
            }
        }

        assert!(
            ok_rows > 0,
            "expected >=1 ok row for {}, got none",
            wb_path.display()
        );
        assert!(
            bad_rows > 0,
            "expected >=1 invalid/error row for {}, got none",
            wb_path.display()
        );
        assert!(
            has_study,
            "expected study with >5 samples for {}, got none",
            wb_path.display()
        );
        assert!(
            has_plot,
            "expected plot with >5 points for {}, got none",
            wb_path.display()
        );
    }
}

#[test]
fn injector_rename_rewrites_refs_and_still_executes() {
    let src = examples_root().join("injector_orifice_sizer.engwb");
    let td = tempdir().expect("tempdir");
    let dst = td.path().join("injector_orifice_sizer.engwb");
    copy_dir_recursive(&src, &dst);

    let mut doc = load_workbook_dir(&dst).expect("load copied workbook");
    let updates = rename_row_key(&mut doc, "dp_orifice", "dp_injector").expect("rename row key");
    assert!(updates > 0, "expected at least one rewritten reference");

    let validation = validate_workbook(&doc);
    assert!(
        validation.ok,
        "validation failed after rename: {:?}",
        validation.messages
    );
    let _run = execute_workbook(&doc, None).expect("execute after rename");

    // Check on-disk rewrite happened.
    let mut all_text = String::new();
    for tab in &doc.tabs {
        let p = dst.join("tabs").join(&tab.file);
        all_text.push_str(&fs::read_to_string(p).expect("read tab file"));
    }
    assert!(
        all_text.contains("ref:dp_injector"),
        "expected rewritten references to new key"
    );
    assert!(
        !all_text.contains("ref:dp_orifice"),
        "old reference key still present after rename"
    );
}
