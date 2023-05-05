// `is_read_write` is not yet implemented on Windows. And `ioctl_fionread`
// on Windows doesn't work on files.
#[cfg(not(windows))]
#[test]
fn test_ioctls() {
    let file = std::fs::File::open("Cargo.toml").unwrap();

    #[cfg(all(feature = "fs", feature = "net"))]
    assert_eq!(rustix::io::is_read_write(&file).unwrap(), (true, false));

    assert_eq!(
        rustix::io::ioctl_fionread(&file).unwrap(),
        file.metadata().unwrap().len()
    );
}

// TODO: Enable this on mips and power once libc is updated.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "sparc64"
)))]
#[test]
fn test_ioctl_ficlone() {
    use rustix::io;

    let src = std::fs::File::open("Cargo.toml").unwrap();
    let dest = tempfile::tempfile().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir = std::fs::File::open(dir.path()).unwrap();

    // `src` isn't opened for writing, so passing it as the output fails.
    assert_eq!(rustix::io::ioctl_ficlone(&src, &src), Err(io::Errno::BADF));

    // `FICLONE` operates on regular files, not directories.
    assert_eq!(rustix::io::ioctl_ficlone(&dir, &dir), Err(io::Errno::ISDIR));

    // Now try something that might succeed, though be prepared for filesystems
    // that don't support this.
    match rustix::io::ioctl_ficlone(&dest, &src) {
        Ok(()) | Err(io::Errno::OPNOTSUPP) => (),
        Err(err) => Err(err).unwrap(),
    }
}
