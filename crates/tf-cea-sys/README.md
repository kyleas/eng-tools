# tf-cea-sys

Raw FFI bindings to NASA CEA (Chemical Equilibrium with Applications).

## Overview

This crate provides low-level unsafe bindings to the NASA CEA C interface. It is **not intended for direct use** by application code. Instead, use the safe, high-level `tf-cea` crate.

## Architecture

- **Vendored Source**: NASA CEA Fortran/C code in `third_party/nasa-cea/`
- **Build Integration**: CMake invoked via `build.rs` to compile static library
- **FFI Layer**: Minimal unsafe bindings to C interface
- **License**: Apache 2.0 (NASA CEA) + MIT/Apache-2.0 dual (this crate)

## Build Requirements

Before building this crate, you must vendor the NASA CEA source:

```bash
cd ../../third_party
git submodule add https://github.com/nasa/cea nasa-cea
git submodule update --init --recursive
cd nasa-cea
git checkout v3.0.4
```

See `third_party/nasa-cea/VENDOR_INSTRUCTIONS.md` for detailed instructions.

### Prerequisites

- **CMake 3.19+**: `cmake --version`
- **Fortran compiler**: `gfortran --version` (or Intel ifx)
- **C compiler**: `gcc --version` (or clang/MSVC)

#### Platform-Specific

**Linux:**
```bash
sudo apt-get install cmake gfortran gcc
```

**macOS:**
```bash
brew install cmake gcc
```

**Windows:**
- Install MinGW-w64 or Intel Fortran Compiler
- Ensure CMake, gfortran, gcc are on PATH

## Usage

This crate is not meant to be used directly. Use `tf-cea` instead:

```rust
// ❌ Don't do this
use tf_cea_sys::*;
unsafe {
    cea_init(...);
}

// ✅ Do this instead
use tf_cea::{CeaBackend, CeaBackendConfig};
let backend = tf_cea::create_backend(config)?;
```

## Safety

All functions in this crate are `unsafe` and require careful handling of:

- **Memory management**: CEA may allocate memory that must be freed
- **Null termination**: String parameters must be properly null-terminated
- **Thread safety**: CEA likely has global state (not thread-safe)
- **Error codes**: Check return values for CEA_SUCCESS
- **Initialization**: Must call `cea_init()` before other functions

## FFI Interface

The C interface wraps NASA CEA's Fortran core. Key function categories:

- **Initialization**: `cea_init()`, `cea_finalize()`
- **Equilibrium**: `cea_equilibrium()`, `cea_free_equilibrium_result()`
- **Rocket**: `cea_rocket()`
- **Utilities**: `cea_version()`, `cea_last_error()`

See `src/lib.rs` for complete FFI declarations.

## NASA CEA Information

- **Repository**: https://github.com/nasa/cea
- **Documentation**: https://nasa.github.io/cea/
- **License**: Apache 2.0
- **Version**: v3.0.4 (2 days ago as of March 2026)

## Development

### Building

```bash
cargo build -p tf-cea-sys
```

This will:
1. Check for vendored NASA CEA source
2. Invoke CMake to configure and build CEA with C bindings
3. Link the static library into the Rust crate
4. Verify FFI declarations compile

### Testing

```bash
cargo test -p tf-cea-sys
```

NOTE: These are minimal compile-time tests. Integration tests belong in `tf-cea`.

### Updating NASA CEA

To update to a newer NASA CEA version:

1. Update the submodule/vendored source:
   ```bash
   cd ../../third_party/nasa-cea
   git fetch
   git checkout v3.1.0  # or desired version
   ```

2. Rebuild and test:
   ```bash
   cargo clean -p tf-cea-sys
   cargo test -p tf-cea-sys
   cargo test -p tf-cea
   ```

3. Update version references in:
   - `third_party/README.md`
   - `third_party/nasa-cea/VENDOR_INSTRUCTIONS.md`
   - `crates/tf-cea-sys/Cargo.toml` (check if FFI changed)
   - `docs/CEA_BACKEND_DECISION.md`

## Troubleshooting

**Build Error: "NASA CEA source not found"**
- Follow vendoring instructions in `third_party/nasa-cea/VENDOR_INSTRUCTIONS.md`

**Build Error: "CMake not found"**
- Install CMake 3.19+ and ensure it's on PATH

**Build Error: "gfortran not found"**
- Install GCC/gfortran toolchain

**Link Error: undefined reference to CEA functions**
- Check that NASA CEA C bindings are enabled in CMake
- Verify `build.rs` CMake configuration includes `-DCEA_ENABLE_BIND_C=ON`

## License

This crate is dual-licensed MIT/Apache-2.0 (same as Thermoflow).

NASA CEA source code (vendored in `third_party/nasa-cea/`) is licensed under Apache 2.0.
