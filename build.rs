fn main() {
    if std::env::var("CARGO_CFG_LINUX_RAW").is_err() {
        println!("cargo:rustc-cfg=libc");
    }
    println!("cargo:cargo:rerun-if-env-changed=CARGO_CFG_LINUX_RAW");
}
