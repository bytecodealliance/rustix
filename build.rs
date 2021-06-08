fn main() {
    #[cfg(not(linux_raw))]
    println!("cargo:rustc-cfg=libc");
}
