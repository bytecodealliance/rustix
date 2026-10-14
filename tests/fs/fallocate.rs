//! Apple `F_PREALLOCATE` without `F_ALLOCATEALL` can return success after
//! allocating only a fraction of the requested length.

#![cfg(apple)]

use rustix::fs::{fallocate, fstat, fsync, openat, FallocateFlags, Mode, OFlags, CWD};

#[test]
fn apple_fallocate_reserves_requested_space() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // Large enough that a partial `F_ALLOCATECONTIG` allocation (a few MiB)
    // would fall short of the request.
    let requested = 16 * 1024 * 1024;
    fallocate(&file, FallocateFlags::empty(), 0, requested).unwrap();
    fsync(&file).unwrap();

    let allocated = fstat(&file).unwrap().st_blocks as u64 * 512;
    assert!(
        allocated >= requested,
        "fallocate reported success after reserving {allocated} of {requested} bytes"
    );
}
