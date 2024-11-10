// SPARC lacks `FICLONE`.
#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[test]
fn test_ioctl_ficlone() {
    use rustix::io;

    let src = std::fs::File::open("Cargo.toml").unwrap();
    let dest = tempfile::tempfile().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir = std::fs::File::open(dir.path()).unwrap();

    // `src` isn't opened for writing, so passing it as the output fails.
    assert_eq!(rustix::fs::ioctl_ficlone(&src, &src), Err(io::Errno::BADF));

    // `FICLONE` operates on regular files, not directories.
    assert_eq!(rustix::fs::ioctl_ficlone(&dir, &dir), Err(io::Errno::ISDIR));

    // Now try something that might succeed, though be prepared for filesystems
    // that don't support this.
    match rustix::fs::ioctl_ficlone(&dest, &src) {
        Ok(()) | Err(io::Errno::OPNOTSUPP) => (),
        Err(e) if e == io::Errno::from_raw_os_error(0x12) => (),
        Err(err) => panic!("{:?}", err),
    }
}
