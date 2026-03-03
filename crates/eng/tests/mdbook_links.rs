use std::{
    fs,
    path::{Path, PathBuf},
};

use eng::docs;

#[test]
fn unified_mdbook_links_resolve() {
    let temp = tempfile::tempdir().expect("tempdir");
    let book_root = temp.path().join("book");
    docs::export_unified_mdbook_to(&book_root).expect("export unified mdbook");
    let src_root = book_root.join("src");

    for file in collect_markdown(&src_root) {
        let text = fs::read_to_string(&file).expect("read markdown");
        for link in markdown_links(&text) {
            if is_external_or_anchor(&link) {
                continue;
            }
            let target = resolve_link_target(&file, &link);
            assert!(
                target.exists(),
                "broken mdbook link in {} -> {}",
                file.display(),
                link
            );
        }
    }
}

fn collect_markdown(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.expect("walkdir entry");
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            out.push(entry.path().to_path_buf());
        }
    }
    out
}

fn markdown_links(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'('
            && i > 0
            && bytes[i - 1] == b']'
            && let Some(end) = text[i + 1..].find(')')
        {
            let raw = text[i + 1..i + 1 + end].trim();
            if !raw.is_empty() {
                out.push(raw.to_string());
            }
            i += end + 1;
        }
        i += 1;
    }
    out
}

fn is_external_or_anchor(link: &str) -> bool {
    link.starts_with('#')
        || link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("mailto:")
}

fn resolve_link_target(from_file: &Path, link: &str) -> PathBuf {
    let clean = link.split('#').next().unwrap_or(link);
    from_file
        .parent()
        .expect("parent dir")
        .join(clean.replace('/', std::path::MAIN_SEPARATOR_STR))
}
