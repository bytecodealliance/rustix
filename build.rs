#[cfg(feature = "cc")]
use cc::Build;
use std::env::var;
use std::io::Write;

/// The directory for out-of-line ("outline") libraries.
const OUTLINE_PATH: &str = "src/imp/linux_raw/arch/outline";

fn main() {
    // Don't rerun this on changes other than build.rs, as we only depend on
    // the rustc version.
    println!("cargo:rerun-if-changed=build.rs");

    use_feature_or_nothing("rustc_attrs");

    // Features only used in no-std configurations.
    #[cfg(not(feature = "std"))]
    {
        use_feature_or_nothing("vec_into_raw_parts");
        use_feature_or_nothing("toowned_clone_into");
        use_feature_or_nothing("specialization");
        use_feature_or_nothing("slice_internals");
        use_feature_or_nothing("const_raw_ptr_deref");
    }

    // Gather target information.
    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let asm_name = format!("{}/{}.s", OUTLINE_PATH, arch);
    let asm_name_present = std::fs::metadata(&asm_name).is_ok();
    let os_name = var("CARGO_CFG_TARGET_OS").unwrap();
    let pointer_width = var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    let endian = var("CARGO_CFG_TARGET_ENDIAN").unwrap();

    // Check for special target variants.
    let is_x32 = arch == "x86_64" && pointer_width == "32";
    let is_arm64_ilp32 = arch == "aarch64" && pointer_width == "32";
    let is_powerpc64be = arch == "powerpc64" && endian == "big";
    let is_unsupported_abi = is_x32 || is_arm64_ilp32 || is_powerpc64be;

    // Check for `--features=use-libc`. This allows crate users to enable the
    // libc backend.
    let feature_use_libc = var("CARGO_FEATURE_USE_LIBC").is_ok();

    // Check for `RUSTFLAGS=--cfg=rustix_use_libc`. This allows end users to
    // enable the libc backend even if rustix is depended on transitively.
    let cfg_use_libc = var("CARGO_CFG_RUSTIX_USE_LIBC").is_ok();

    // Check for eg. `RUSTFLAGS=--cfg=rustix_use_experimental_asm`. This is a
    // rustc flag rather than a cargo feature flag because it's experimental
    // and not something we want accidentally enabled via --all-features.
    let rustix_use_experimental_asm = var("CARGO_CFG_RUSTIX_USE_EXPERIMENTAL_ASM").is_ok();

    // If the libc backend is requested, or if we're not on a platform for
    // which we have linux-raw support, use the libc backend.
    //
    // For now Android uses the libc backend; in theory it could use the
    // linux-raw backend, but to do that we'll need to figure out how to
    // install the toolchain for it.
    if feature_use_libc
        || cfg_use_libc
        || os_name != "linux"
        || !asm_name_present
        || is_unsupported_abi
    {
        // Use the libc backend.
        use_feature("libc");
    } else {
        // Use the linux-raw backend.
        use_feature("linux_raw");
        use_feature_or_nothing("core_intrinsics");

        // Use inline asm if we have it, or outline asm otherwise. On PowerPC,
        // Rust's inline asm is considered experimental, so only use it if
        // `--cfg=rustix_use_experimental_asm` is given.
        if has_feature("asm") && (arch != "powerpc64" || rustix_use_experimental_asm) {
            use_feature("asm");
            if rustix_use_experimental_asm {
                use_feature("asm_experimental_arch");
            }
        } else {
            link_in_librustix_outline(&arch, &asm_name);
        }
    }

    println!("cargo:rerun-if-env-changed=CARGO_CFG_RUSTIX_USE_EXPERIMENTAL_ASM");
}

fn link_in_librustix_outline(arch: &str, asm_name: &str) {
    let name = format!("rustix_outline_{}", arch);
    let profile = var("PROFILE").unwrap();
    let to = format!("{}/{}/lib{}.a", OUTLINE_PATH, profile, name);
    println!("cargo:rerun-if-changed={}", to);

    // If "cc" is not enabled, use a pre-built library.
    #[cfg(not(feature = "cc"))]
    {
        let _ = asm_name;
        println!("cargo:rustc-link-search={}/{}", OUTLINE_PATH, profile);
        println!("cargo:rustc-link-lib=static={}", name);
    }

    // If "cc" is enabled, build the library from source, update the pre-built
    // version, and assert that the pre-built version is checked in.
    #[cfg(feature = "cc")]
    {
        let out_dir = var("OUT_DIR").unwrap();
        Build::new().file(&asm_name).compile(&name);
        println!("cargo:rerun-if-changed={}", asm_name);
        if std::fs::metadata(".git").is_ok() {
            let from = format!("{}/lib{}.a", out_dir, name);
            let prev_metadata = std::fs::metadata(&to);
            std::fs::copy(&from, &to).unwrap();
            assert!(
                prev_metadata.is_ok(),
                "{} didn't previously exist; please inspect the new file and `git add` it",
                to
            );
            assert!(
                std::process::Command::new("git")
                    .arg("diff")
                    .arg("--quiet")
                    .arg(&to)
                    .status()
                    .unwrap()
                    .success(),
                "{} changed; please inspect the change and `git commit` it",
                to
            );
        }
    }
}

fn use_feature_or_nothing(feature: &str) {
    if has_feature(feature) {
        use_feature(feature);
    }
}

fn use_feature(feature: &str) {
    println!("cargo:rustc-cfg={}", feature);
}

/// Test whether the rustc at `var("RUSTC")` supports the given feature.
fn has_feature(feature: &str) -> bool {
    use std::process::Stdio;
    let out_dir = var("OUT_DIR").unwrap();
    let rustc = var("RUSTC").unwrap();

    let mut child = std::process::Command::new(rustc)
        .arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=metadata") // Do as little as possible but still parse.
        .arg("--out-dir")
        .arg(out_dir) // Put the output somewhere inconsequential.
        .arg("-") // Read from stdin.
        .stdin(Stdio::piped()) // Stdin is a pipe.
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    writeln!(
        child.stdin.take().unwrap(),
        "#![allow(stable_features)]\n#![feature({})]",
        feature
    )
    .unwrap();

    child.wait().unwrap().success()
}
