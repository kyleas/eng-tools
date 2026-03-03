$ErrorActionPreference = "Stop"

Write-Host "Running equations CI hardening checks..."

cargo test -p equations

cargo run -p equations --bin equations -- generate-schema
cargo run -p equations --bin equations -- validate --with-tests
cargo run -p equations --bin equations -- export-docs
cargo run -p equations --bin equations -- export-mdbook
cargo run -p equations --bin equations -- export-html

$required = @(
  "generated/catalog.json",
  "generated/search_index.json",
  "generated/page_models.json",
  "generated/navigation.json",
  "generated/examples_index.json",
  "generated/constants.json",
  "generated/families.json",
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

git diff --exit-code -- `
  crates/equations/schemas/equation.schema.json `
  generated/catalog.json `
  generated/search_index.json `
  generated/page_models.json `
  generated/navigation.json `
  generated/examples_index.json `
  generated/constants.json `
  generated/families.json `
  generated/architecture_spec.json

Write-Host "Equations CI checks passed."
