# Third-Party Dependencies

This directory contains vendored third-party source code used by Thermoflow.

## NASA CEA (Chemical Equilibrium with Applications)

**Location:** `nasa-cea/`

**Source:** https://github.com/nasa/cea

**Version:** v3.0.4 (or later)

**License:** Apache 2.0

### Vendoring Process

To update the vendored NASA CEA source:

```bash
# Option 1: Git submodule (recommended)
git submodule add https://github.com/nasa/cea third_party/nasa-cea
git submodule update --init --recursive

# Option 2: Direct clone for vendoring
cd third_party
git clone https://github.com/nasa/cea nasa-cea
cd nasa-cea
git checkout v3.0.4  # or desired version
rm -rf .git  # if you want to fully vendor rather than submodule
```

### Build Integration

The `tf-cea-sys` crate build script (`crates/tf-cea-sys/build.rs`) automatically:
1. Invokes CMake to configure NASA CEA with C bindings enabled
2. Builds the static library (`libcea.a` or `cea.lib`)
3. Links the library into the Rust build
4. Bundles the thermodynamic and transport databases

### Build Prerequisites

**All platforms:**
- CMake 3.19+ (`cmake --version`)
- Fortran compiler (gfortran recommended)
- C compiler (gcc/clang/MSVC)

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
- Ensure CMake, gfortran, and gcc are on PATH

### Directory Structure

```
third_party/nasa-cea/
├── CMakeLists.txt          # NASA CEA build configuration
├── source/                 # Fortran core + C bindings
│   ├── bind/c/            # C interface layer
│   └── *.f90              # Fortran solver
├── data/                   # Thermodynamic/transport databases
│   ├── thermo.inp
│   └── trans.inp
├── cmake/                  # CMake modules
├── samples/                # Example problems
└── README.md              # NASA's documentation
```

### License Compliance

NASA CEA is licensed under Apache 2.0, which is compatible with Thermoflow's MIT/Apache-2.0 dual license.

Full license text: `nasa-cea/LICENSE.txt`

### Update Strategy

1. Monitor https://github.com/nasa/cea/releases for new versions
2. Test new version against Thermoflow's regression suite
3. Update version pin in this README and in `tf-cea-sys/build.rs`
4. Document any API changes in `tf-cea-sys/CHANGELOG.md`
