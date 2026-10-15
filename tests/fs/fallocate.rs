use rustix::fs::{fallocate, fstat, fsync, open, FallocateFlags, Mode, OFlags};
use rustix::io::{write, Errno};

/// Test that `fallocate` reserves the number of bytes requested.
///
/// On Apple platforms, `F_PREALLOCATE` without `F_ALLOCATEALL` can succeed
/// while allocating fewer bytes than requested.
#[test]
fn allocate_full_length_even_if_large() {
    let tmp = tempfile::tempdir().unwrap();
    let file = open(
        tmp.path().join("file"),
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // Large enough to potentially evoke a partial allocation.
    let requested = 16 * 1024 * 1024;
    match fallocate(&file, FallocateFlags::empty(), 0, requested) {
        Ok(()) => {}
        Err(Errno::OPNOTSUPP) => return,
        Err(err) => Err(err).unwrap(),
    }
    fsync(&file).unwrap();

    let allocated = fstat(&file).unwrap().st_blocks as u64 * 512;
    assert!(
        allocated >= requested,
        "fallocate reported success after reserving {allocated} of {requested} bytes"
    );
}

/// Test that `fallocate` increases the file size.
#[test]
fn increase_file_size() {
    let tmp = tempfile::tempdir().unwrap();
    let file = open(
        tmp.path().join("file"),
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    let stat = fstat(&file).unwrap();
    assert_eq!(stat.st_size, 0);

    // `fallocate` increases the file size.
    match fallocate(&file, FallocateFlags::empty(), 7, 11) {
        Ok(()) => {}
        Err(Errno::OPNOTSUPP) => return,
        Err(err) => Err(err).unwrap(),
    }
    let stat = fstat(&file).unwrap();
    assert_eq!(stat.st_size, 18);
}

/// Test that `fallocate` doesn't decrease the file size.
///
/// Except on Apple platforms, where it does decrase the file size.
#[test]
fn dont_decrease_file_size() {
    let tmp = tempfile::tempdir().unwrap();
    let file = open(
        tmp.path().join("file"),
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    let stat = fstat(&file).unwrap();
    assert_eq!(stat.st_size, 0);

    assert_eq!(write(&file, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap(), 26);

    let stat = fstat(&file).unwrap();
    assert_eq!(stat.st_size, 26);

    // `fallocate` doesn't decrease the file size, except on Apple.
    match fallocate(&file, FallocateFlags::empty(), 7, 3) {
        Ok(()) => {}
        Err(Errno::OPNOTSUPP) => return,
        Err(err) => Err(err).unwrap(),
    }

    #[cfg(not(apple))]
    {
        let stat = fstat(&file).unwrap();
        assert_eq!(stat.st_size, 26);
        fallocate(&file, FallocateFlags::empty(), 0, 1).unwrap();
        let stat = fstat(&file).unwrap();
        assert_eq!(stat.st_size, 26);
    }

    #[cfg(apple)]
    {
        let stat = fstat(&file).unwrap();
        assert_eq!(stat.st_size, 10);
        fallocate(&file, FallocateFlags::empty(), 0, 1).unwrap();
        let stat = fstat(&file).unwrap();
        assert_eq!(stat.st_size, 1);
    }
}
