#[cfg(feature = "cc")]
use cc::Build;
use std::env::var;

const OUTLINE_PATH: &str = "src/imp/linux_raw/arch/outline";

fn main() {
    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let asm_name = format!("{}/{}.S", OUTLINE_PATH, arch);
    let os_name = var("CARGO_CFG_TARGET_OS").unwrap();
    let is_x32 = arch == "x86_64" && var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32";
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    // If rsix_use_libc is set, or if we're on an architecture/OS that doesn't
    // have raw syscall support, use libc.
    if var("CARGO_CFG_RSIX_USE_LIBC").is_ok()
        || os_name != "linux"
        || std::fs::metadata(&asm_name).is_err()
        || is_x32
    {
        println!("cargo:rustc-cfg=libc");
    } else {
        println!("cargo:rustc-cfg=linux_raw");

        if let rustc_version::Channel::Nightly = rustc_version::version_meta()
            .expect("query rustc release channel")
            .channel
        {
            println!("cargo:rustc-cfg=linux_raw_inline_asm");
            println!("cargo:rustc-cfg=rustc_attrs");
        } else {
            link_in_librsix_outline(&arch, &asm_name);
        }
        if rustc_version::version().unwrap() >= rustc_version::Version::parse("1.56.0").unwrap() {
            println!("cargo:rustc-cfg=const_fn_union");
        }
    }
    println!("cargo:rerun-if-env-changed=CARGO_CFG_RSIX_USE_LIBC");
}

fn link_in_librsix_outline(arch: &str, asm_name: &str) {
    let name = format!("rsix_outline_{}", arch);
    let to = format!("{}/lib{}.a", OUTLINE_PATH, name);
    println!("cargo:rerun-if-changed={}", to);

    // If "cc" is not enabled, use a pre-built library.
    #[cfg(not(feature = "cc"))]
    {
        let _ = asm_name;
        println!("cargo:rustc-link-search={}", OUTLINE_PATH);
        println!("cargo:rustc-link-lib=static={}", name);
    }

    // If "cc" is enabled, build the library from source, update the pre-built
    // version, and assert that the pre-built version is checked in.
    #[cfg(feature = "cc")]
    {
        let out_dir = var("OUT_DIR").unwrap();
        Build::new().file(&asm_name).compile(&name);
        println!("cargo:rerun-if-changed={}", asm_name);
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
