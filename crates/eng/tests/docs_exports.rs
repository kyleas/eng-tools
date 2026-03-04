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
}
