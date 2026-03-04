use eng::docs;

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

    let isentropic_page =
        std::fs::read_to_string(book_root.join("src").join("devices").join("isentropic_calc.md"))
            .expect("read generated isentropic device page");
    assert!(isentropic_page.contains("# Isentropic Calculator"));
    assert!(isentropic_page.contains("ENG_ISENTROPIC("));
    assert!(isentropic_page.contains("ENG_ISENTROPIC_FROM_A_ASTAR_TO_M("));

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
