//! build.rs for tf-cea-sys
//!
//! Links Thermoflow to the prebuilt official NASA CEA release binaries.
//!
//! The vendored binaries are located in:
//! - `third_party/cea/include/` - C headers
//! - `third_party/cea/lib/<platform>/` - Platform-specific shared libraries
//! - `third_party/cea/data/` - Runtime data files (thermo.lib)

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../third_party/cea");

    link_vendored_cea();
}

fn link_vendored_cea() {
    // Locate vendored NASA CEA in third_party/cea/
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("failed to find workspace root");

    let cea_root = workspace_root.join("third_party").join("cea");

    // Verify prebuilt assets exist
    let include_dir = cea_root.join("include");
    let data_dir = cea_root.join("data");

    if !include_dir.exists() {
        panic!(
            "NASA CEA include directory not found at {:?}\n\
             Please ensure third_party/cea/include/ contains cea.h and cea_enum.h\n\
             See docs/BUNDLED_NATIVE_CEA.md for setup instructions.",
            include_dir
        );
    }

    if !data_dir.exists() {
        panic!(
            "NASA CEA data directory not found at {:?}\n\
             Please ensure third_party/cea/data/ contains thermo.lib\n\
             See docs/BUNDLED_NATIVE_CEA.md for setup instructions.",
            data_dir
        );
    }

    // Determine platform-specific library path
    let lib_dir = determine_lib_dir(&cea_root);

    if !lib_dir.exists() {
        panic!(
            "NASA CEA library directory not found at {:?}\n\
             Current target: {}\n\
             Available platforms: windows-msvc, linux-x86_64, macos\n\
             See docs/BUNDLED_NATIVE_CEA.md for setup instructions.",
            lib_dir,
            env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
        );
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Link to the prebuilt CEA library
    // Use dylib (shared library) linking
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=dylib=cea_bindc");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=cea_bindc");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=cea_bindc");

    // Copy thermo.lib to output directory for runtime access
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    copy_thermo_lib(&data_dir, &out_dir);

    // Copy DLL/shared library to output directories for runtime discovery
    copy_shared_library(&lib_dir, &out_dir);

    // Embed the data directory path for runtime discovery
    println!("cargo:rustc-env=CEA_DATA_DIR={}", data_dir.display());

    println!("cargo:rustc-env=CEA_LIBRARY_DIR={}", lib_dir.display());

    println!(
        "cargo:warning=Linked to prebuilt NASA CEA at {}",
        cea_root.display()
    );
}

/// Determine the correct platform/vendor-specific library directory
fn determine_lib_dir(cea_root: &Path) -> PathBuf {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown".to_string());

    // Map Rust target to CEA platform directory
    match target_os.as_str() {
        "windows" => cea_root.join("windows-msvc"),
        "linux" => cea_root.join("linux-x86_64"),
        "macos" => cea_root.join("macos"),
        unknown => {
            eprintln!(
                "Warning: Unknown target OS '{}', assuming linux-x86_64",
                unknown
            );
            cea_root.join("linux-x86_64")
        }
    }
}

/// Copy thermo.lib to a location where the runtime can find it easily.
/// This is critical because CEA requires this file to initialize.
fn copy_thermo_lib(data_dir: &Path, out_dir: &Path) {
    let thermo_src = data_dir.join("thermo.lib");

    if !thermo_src.exists() {
        eprintln!(
            "Warning: thermo.lib not found at {}\n\
             CEA runtime initialization will fail unless this file is available.",
            thermo_src.display()
        );
        return;
    }

    // Copy to output directory (target/debug or target/release)
    let thermo_dst = out_dir.join("thermo.lib");
    if let Err(e) = std::fs::copy(&thermo_src, &thermo_dst) {
        eprintln!(
            "Warning: Failed to copy thermo.lib: {}\n\
             Runtime may fail to initialize.",
            e
        );
    }

    // Also try to copy to the binary output directory
    // This helps when running tests/binaries directly
    if let Ok(profile) = env::var("PROFILE") {
        let bin_dir = out_dir
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join(&profile))
            .map(|p| p.join("..").join(&profile));

        if let Some(dir) = bin_dir {
            let bin_thermo = dir.join("thermo.lib");
            if let Err(e) = std::fs::copy(&thermo_src, &bin_thermo) {
                // Silently ignore if this fails; the OUT_DIR copy is the important one
                let _ = e;
            }
        }
    }
}

/// Copy shared library (DLL/.so/.dylib) to runtime directories for automatic discovery.
/// On Windows, the DLL must be in the same directory as the executable or on PATH.
/// This function ensures tests and examples can find the library without manual setup.
fn copy_shared_library(lib_dir: &Path, out_dir: &Path) {
    // Determine shared library filename based on platform
    let lib_filename = if cfg!(target_os = "windows") {
        "cea_bindc.dll"
    } else if cfg!(target_os = "macos") {
        "libcea_bindc.dylib"
    } else {
        "libcea_bindc.so"
    };

    let lib_src = lib_dir.join(lib_filename);

    if !lib_src.exists() {
        eprintln!(
            "Warning: {} not found at {}\n\
             Runtime library loading will fail.",
            lib_filename,
            lib_src.display()
        );
        return;
    }

    // Strategy: Copy to multiple output directories to cover tests, examples, and binaries
    // 1. Copy to OUT_DIR (where thermo.lib goes)
    let _ = std::fs::copy(&lib_src, out_dir.join(lib_filename));

    // 2. Copy to target/{debug,release}/ for main binaries
    // 3. Copy to target/{debug,release}/deps/ for test binaries
    if let Ok(_profile) = env::var("PROFILE") {
        // Navigate from OUT_DIR (target/{profile}/build/tf-cea-sys-{hash}/out)
        // up to target/{profile}/
        if let Some(target_profile_dir) = out_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        {
            // Copy to target/{profile}/ (for main binaries and examples)
            let main_dst = target_profile_dir.join(lib_filename);
            if let Err(e) = std::fs::copy(&lib_src, &main_dst) {
                eprintln!(
                    "Warning: Failed to copy {} to {}: {}",
                    lib_filename,
                    main_dst.display(),
                    e
                );
            } else {
                println!(
                    "cargo:warning=Copied {} to {}",
                    lib_filename,
                    main_dst.display()
                );
            }

            // Copy to target/{profile}/deps/ (for test binaries)
            let deps_dir = target_profile_dir.join("deps");
            if deps_dir.exists() || std::fs::create_dir_all(&deps_dir).is_ok() {
                let deps_dst = deps_dir.join(lib_filename);
                if let Err(e) = std::fs::copy(&lib_src, &deps_dst) {
                    eprintln!(
                        "Warning: Failed to copy {} to {}: {}",
                        lib_filename,
                        deps_dst.display(),
                        e
                    );
                } else {
                    println!(
                        "cargo:warning=Copied {} to {}",
                        lib_filename,
                        deps_dst.display()
                    );
                }
            }
        }
    }
}
