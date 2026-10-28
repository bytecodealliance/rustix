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

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[test]
fn test_ioctl_ficlonerange() {
    use rustix::io;

    let src = std::fs::File::open("Cargo.toml").unwrap();
    let dest = tempfile::tempfile().unwrap();

    // Often the temporary directory is on a different filesystem (like tmpfs),
    // which means the test to clone some data doesn't do anything interesting,
    // singe the ioctl simply returns failure.
    // Uncomment the line below to use a file in the same directory as the source,
    // which guarantees they are on the same filesystem.
    // let dest = std::fs::File::options()
    //     .create(true)
    //     .truncate(true)
    //     .read(true)
    //     .write(true)
    //     .open("test_ficlonerange").unwrap();

    let dir = tempfile::tempdir().unwrap();
    let dir = std::fs::File::open(dir.path()).unwrap();

    // `src` isn't opened for writing, so passing it as the output fails.
    assert_eq!(
        rustix::fs::ioctl_ficlonerange(&src, &src, 0, 4096, 0),
        Err(io::Errno::BADF)
    );

    // `FICLONERANGE` operates on regular files, not directories.
    assert_eq!(
        rustix::fs::ioctl_ficlonerange(&dir, &dir, 0, 4096, 0),
        Err(io::Errno::ISDIR)
    );

    // Now try something that might succeed, though be prepared for filesystems
    // that don't support this.
    // Copy 4096 bytes from offset 4096 in src to offset 8192 in dest.
    match rustix::fs::ioctl_ficlonerange(&dest, &src, 4096, 4096, 8192) {
        Ok(()) => {
            use std::os::unix::fs::FileExt;

            let mut expected_buf = vec![0u8; 4096];
            let mut actual_buf = vec![0u8; 4096];
            src.read_exact_at(expected_buf.as_mut_slice(), 4096)
                .unwrap();
            dest.read_exact_at(actual_buf.as_mut_slice(), 8192).unwrap();

            assert_eq!(expected_buf, actual_buf);
        }

        Err(io::Errno::OPNOTSUPP) => (),
        Err(e) if e == io::Errno::from_raw_os_error(0x12) => (),
        Err(err) => panic!("{:?}", err),
    }
}
