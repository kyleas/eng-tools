use eng::docs;
use serde_json::Value;

#[test]
fn unified_docs_export_api_writes_catalog_artifacts() {
    let temp = tempfile::tempdir().expect("tempdir");
    docs::export_unified_docs_to(temp.path()).expect("export unified docs");
    for rel in [
        "catalog.json",
        "search_index.json",
        "page_models.json",
        "navigation.json",
        "examples_index.json",
        "constants.json",
        "families.json",
        "devices.json",
        "bindings/invoke_protocol.json",
        "bindings/python/pyproject.toml",
        "bindings/python/README.md",
        "fluids.json",
        "materials.json",
        "architecture_spec.json",
    ] {
        assert!(
            temp.path().join(rel).exists(),
            "missing generated artifact {rel}"
        );
    }
}

#[test]
fn unified_mdbook_export_api_writes_book_structure() {
    let temp = tempfile::tempdir().expect("tempdir");
    let book_root = temp.path().join("book");
    let paths = docs::export_unified_mdbook_to(&book_root).expect("export unified mdbook");
    assert!(book_root.join("book.toml").exists());
    assert!(book_root.join("src").join("SUMMARY.md").exists());
    assert!(book_root.join("src").join("index.md").exists());
    assert!(
        book_root
            .join("src")
            .join("architecture")
            .join("index.md")
            .exists()
    );
    assert_eq!(paths.source_dir, book_root);

    let isentropic_page = std::fs::read_to_string(
        book_root
            .join("src")
            .join("devices")
            .join("isentropic_calc.md"),
    )
    .expect("read generated isentropic device page");
    assert!(isentropic_page.contains("# Isentropic Calculator"));
    assert!(isentropic_page.contains("ENG_ISENTROPIC("));
    assert!(isentropic_page.contains("ENG_ISENTROPIC_FROM_A_ASTAR_TO_M("));
    assert!(isentropic_page.contains("ENG_ISENTROPIC_FROM_NU_DEG_TO_M("));
    assert!(isentropic_page.contains("ENG_ISENTROPIC_FROM_M_TO_NU_DEG("));

    let nozzle_page = std::fs::read_to_string(
        book_root
            .join("src")
            .join("devices")
            .join("nozzle_flow_calc.md"),
    )
    .expect("read generated nozzle-flow device page");
    assert!(nozzle_page.contains("# Nozzle Flow Calculator"));
    assert!(nozzle_page.contains("ENG_NOZZLE_FLOW("));
    assert!(nozzle_page.contains("ENG_NOZZLE_FLOW_FROM_A_ASTAR_TO_M("));

    let area_mach = std::fs::read_to_string(
        book_root
            .join("src")
            .join("equations")
            .join("compressible")
            .join("area_mach.md"),
    )
    .expect("read generated area_mach page");
    assert!(area_mach.contains("Branch behavior"));
    assert!(area_mach.contains("branch=\""));
    assert!(area_mach.contains("ENG_COMPRESSIBLE_AREA_MACH_M("));
    assert!(area_mach.contains("ENG_EQUATION_BRANCHES_TEXT(\"compressible.area_mach\")"));

    let prandtl_meyer = std::fs::read_to_string(
        book_root
            .join("src")
            .join("equations")
            .join("compressible")
            .join("prandtl_meyer.md"),
    )
    .expect("read generated prandtl_meyer page");
    assert!(prandtl_meyer.contains("Prandtl-Meyer"));
    assert!(prandtl_meyer.contains("ENG_COMPRESSIBLE_PRANDTL_MEYER_M("));

    let continuity = std::fs::read_to_string(
        book_root
            .join("src")
            .join("equations")
            .join("fluids")
            .join("continuity_mass_flow.md"),
    )
    .expect("read continuity_mass_flow page");
    assert!(
        continuity.contains("$$\n\\dot{m} = \\rho A V\n$$"),
        "expected block math with exact latex, got:\n{continuity}"
    );
    assert!(
        continuity.contains("\\(\\dot{m}\\)"),
        "expected inline symbol math wrapper for m_dot"
    );
    assert!(
        !continuity.contains("[\\dot{m} = \\rho A V]"),
        "literal bracketed latex regression detected"
    );

    let book_toml = std::fs::read_to_string(book_root.join("book.toml")).expect("read book.toml");
    assert!(
        book_toml.contains("mathjax-support = true"),
        "MathJax support must stay enabled"
    );
}

#[test]
fn all_registered_devices_generate_pages_and_catalog_links() {
    let temp = tempfile::tempdir().expect("tempdir");
    let out_root = temp.path();
    docs::export_unified_docs_to(out_root).expect("export unified docs");
    let book_root = out_root.join("book");
    docs::export_unified_mdbook_to(&book_root).expect("export unified mdbook");

    let catalog_text =
        std::fs::read_to_string(out_root.join("catalog.json")).expect("read generated catalog");
    let catalog: Value = serde_json::from_str(&catalog_text).expect("parse generated catalog");
    let links = catalog["items"]["links"]
        .as_array()
        .expect("catalog links array");

    for spec in eng::devices::generation_specs() {
        let page_path = book_root
            .join("src")
            .join("devices")
            .join(format!("{}.md", spec.key));
        assert!(
            page_path.exists(),
            "missing generated device page {}",
            page_path.display()
        );
        let page = std::fs::read_to_string(&page_path).expect("read generated device page");
        assert!(
            page.contains(spec.name),
            "device page {} missing heading/name '{}'",
            page_path.display(),
            spec.name
        );
        assert!(
            page.contains(spec.summary),
            "device page {} missing summary '{}'",
            page_path.display(),
            spec.summary
        );

        for dep in spec.equation_dependencies {
            let has_link = links.iter().any(|link| {
                link["relation"] == "device_uses_equation"
                    && link["from"] == spec.key
                    && link["to"] == *dep
            });
            assert!(
                has_link,
                "catalog missing device_uses_equation link for {} -> {}",
                spec.key, dep
            );
        }
    }
}

#[test]
fn category_pages_include_autogenerated_equation_summary_tables() {
    let temp = tempfile::tempdir().expect("tempdir");
    docs::export_unified_docs_to(temp.path()).expect("export unified docs");
    let book_root = temp.path().join("book");
    docs::export_unified_mdbook_to(&book_root).expect("export unified mdbook");

    let compressible_index = std::fs::read_to_string(
        book_root
            .join("src")
            .join("equations")
            .join("compressible")
            .join("index.md"),
    )
    .expect("read compressible category page");
    assert!(compressible_index.contains("## Equation Summary"));
    assert!(compressible_index.contains("<table><thead><tr><th>Equation</th>"));
    assert!(
        compressible_index.contains("href=\"./area_mach.md\""),
        "expected known equation link in category table"
    );
    assert!(
        compressible_index.contains("<code>compressible.area_mach</code>"),
        "expected known path id in category table"
    );
    assert!(
        compressible_index.contains("<td>\\(") && compressible_index.contains("\\)</td>"),
        "expected MathJax inline delimiters in LaTeX table cells"
    );

    let fluids_index = std::fs::read_to_string(
        book_root
            .join("src")
            .join("equations")
            .join("fluids")
            .join("index.md"),
    )
    .expect("read fluids category page");
    assert!(
        fluids_index.contains("\\dot{m}"),
        "expected known LaTeX from continuity equation in fluids category table"
    );

    let page_models_text = std::fs::read_to_string(temp.path().join("page_models.json"))
        .expect("read generated page_models artifact");
    let page_models: Value =
        serde_json::from_str(&page_models_text).expect("parse generated page_models");
    let models = page_models["items"]
        .as_array()
        .expect("page_models.items array");
    for model in models
        .iter()
        .filter(|m| m["category"].as_str() == Some("compressible"))
    {
        let path_id = model["path_id"]
            .as_str()
            .expect("compressible model path_id");
        assert!(
            compressible_index.contains(path_id),
            "category table should auto-include all compressible equations; missing {path_id}"
        );
    }
}
