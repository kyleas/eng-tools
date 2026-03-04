$ErrorActionPreference = "Stop"

Write-Host "Running full unified verification (code + docs exports)..."

cargo check
cargo test

cargo run -p eng --bin eng -- export-docs
cargo run -p eng --bin eng -- export-mdbook
cargo run -p eng --bin eng -- export-html

$required = @(
  "generated/catalog.json",
  "generated/search_index.json",
  "generated/page_models.json",
  "generated/navigation.json",
  "generated/examples_index.json",
  "generated/constants.json",
  "generated/families.json",
  "generated/devices.json",
  "generated/bindings/binding_spec.json",
  "generated/bindings/invoke_protocol.json",
  "generated/bindings/python/engpy/__init__.py",
  "generated/bindings/excel/eng_xloil.py",
  "generated/bindings/excel/eng_pyxll.py",
  "generated/architecture_spec.json",
  "generated/book/book.toml",
  "generated/book/src/SUMMARY.md",
  "generated/book/src/architecture/index.md"
)

foreach ($f in $required) {
  if (-not (Test-Path $f)) {
    throw "Missing required generated artifact: $f"
  }
}

if (Get-Command mdbook -ErrorAction SilentlyContinue) {
  if (-not (Test-Path "generated/html/index.html")) {
    throw "mdbook is installed but generated/html/index.html is missing after export-html"
  }
}

Write-Host "Unified verification passed."
