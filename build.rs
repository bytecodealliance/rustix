use cc::Build;
use std::env::var;

fn main() {
    let asm_name = format!(
        "src/imp/linux_raw/arch/outline/{}.S",
        var("CARGO_CFG_TARGET_ARCH").unwrap()
    );
    let os_name = var("CARGO_CFG_TARGET_OS").unwrap();
    let is_x32 = var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64"
        && var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32";

    // If posish_use_libc is set, or if we're on an architecture/OS that doesn't
    // have raw syscall support, use libc.
    if var("CARGO_CFG_POSISH_USE_LIBC").is_ok()
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
            println!("cargo:rustc-cfg=const_fn_union");
        } else {
            Build::new().file(&asm_name).compile("asm");
            println!("cargo:rerun-if-changed={}", asm_name);
            println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
        }
    }
    println!("cargo:rerun-if-env-changed=CARGO_CFG_POSISH_USE_LIBC");
}
