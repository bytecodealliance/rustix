use cc::Build;
use std::env::var;

fn main() {
    if var("CARGO_CFG_LINUX_RAW").is_err() {
        println!("cargo:rustc-cfg=libc");
    } else {
        if let rustc_version::Channel::Nightly = rustc_version::version_meta()
            .expect("query rustc release channel")
            .channel
        {
            println!("cargo:rustc-cfg=linux_raw_inline_asm");
        } else {
            let asm = format!(
                "src/imp/linux_raw/arch/{}.S",
                var("CARGO_CFG_TARGET_ARCH").unwrap()
            );
            Build::new().file(&asm).compile("asm");
            println!("cargo:rerun-if-changed={}", asm);
            println!("cargo:cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
        }
    }
    println!("cargo:cargo:rerun-if-env-changed=CARGO_CFG_LINUX_RAW");
}
