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

    use_feature_or_nothing("vec_into_raw_parts");
    use_feature_or_nothing("toowned_clone_into");
    use_feature_or_nothing("rustc_attrs");
    use_feature_or_nothing("specialization");
    use_feature_or_nothing("slice_internals");
    use_feature_or_nothing("const_raw_ptr_deref");

    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let asm_name = format!("{}/{}.s", OUTLINE_PATH, arch);
    let os_name = var("CARGO_CFG_TARGET_OS").unwrap();
    let is_x32 = arch == "x86_64" && var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32";
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    // If rustix_use_libc is set, or if we're on an architecture/OS that doesn't
    // have raw syscall support, use libc.
    if var("CARGO_CFG_RUSTIX_USE_LIBC").is_ok()
        || os_name != "linux"
        || std::fs::metadata(&asm_name).is_err()
        || is_x32
    {
        use_feature("libc");
    } else {
        use_feature("linux_raw");

        use_feature_or_nothing("core_intrinsics");
        use_feature_or_nothing("doc_cfg");

        use_feature_or_else("asm", || {
            link_in_librustix_outline(&arch, &asm_name);
        });
    }
    println!("cargo:rerun-if-env-changed=CARGO_CFG_RUSTIX_USE_LIBC");
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

fn use_feature_or_else<F: FnOnce()>(feature: &str, or_else: F) {
    if has_feature(feature) {
        use_feature(feature);
    } else {
        or_else();
    }
}

fn use_feature(feature: &str) {
    println!("cargo:rustc-cfg={}", feature);
}

/// Test whether the rustc at `var("RUSTC")` supports the given feature.
fn has_feature(feature: &str) -> bool {
    let out_dir = var("OUT_DIR").unwrap();
    let rustc = var("RUSTC").unwrap();

    let mut child = std::process::Command::new(rustc)
        .arg("--crate-type=rlib") // Don't require `main`.
        .arg("--emit=metadata") // Do as little as possible but still parse.
        .arg("--out-dir")
        .arg(out_dir) // Put the output somewhere inconsequential.
        .arg("-") // Read from stdin.
        .stdin(std::process::Stdio::piped()) // Stdin is a pipe.
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
