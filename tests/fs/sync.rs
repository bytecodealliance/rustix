#[test]
fn test_sync() {
    rustix::fs::sync();
}

#[cfg(linux_kernel)]
#[test]
fn test_syncfs() {
    let f = std::fs::File::open("Cargo.toml").unwrap();
    rustix::fs::syncfs(&f).unwrap();
}
