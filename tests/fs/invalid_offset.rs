//! Tests for extreme `u64` file offsets.
//!
//! POSIX-ish interfaces tend to use signed integers for file offsets, while
//! Rust APIs tend to use `u64`. Test that extreme `u64` values in APIs that
//! take file offsets are properly diagnosed.
//!
//! These tests are disabled on iOS/macOS since those platforms kill the
//! process with `Signal::XFSZ` instead of returning an error.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use rustix::fs::SeekFrom;

#[test]
fn invalid_offset_seek() {
    use rustix::fs::{openat, seek, Mode, OFlags, CWD};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    seek(&file, SeekFrom::Start(u64::MAX)).unwrap_err();
    seek(&file, SeekFrom::Start(i64::MAX as u64 + 1)).unwrap_err();
    seek(&file, SeekFrom::End(-1)).unwrap_err();
    seek(&file, SeekFrom::End(i64::MIN)).unwrap_err();
    seek(&file, SeekFrom::Current(-1)).unwrap_err();
    seek(&file, SeekFrom::Current(i64::MIN)).unwrap_err();
}

#[cfg(not(any(
    netbsdlike,
    target_os = "dragonfly",
    target_os = "nto",
    target_os = "redox"
)))]
#[test]
fn invalid_offset_fallocate() {
    use rustix::fs::{fallocate, openat, FallocateFlags, Mode, OFlags, CWD};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    fallocate(&file, FallocateFlags::empty(), u64::MAX, 1).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), i64::MAX as u64 + 1, 1).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), 0, u64::MAX).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), 0, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(not(any(
    apple,
    netbsdlike,
    target_os = "cygwin",
    target_os = "dragonfly",
    target_os = "haiku",
    target_os = "redox",
    target_os = "solaris",
)))]
#[test]
fn invalid_offset_fadvise() {
    use core::num::NonZeroU64;
    use rustix::fs::{fadvise, openat, Advice, Mode, OFlags, CWD};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    // `fadvise` never fails on invalid offsets.
    fadvise(
        &file,
        i64::MAX as u64,
        NonZeroU64::new(i64::MAX as u64),
        Advice::Normal,
    )
    .unwrap();
    fadvise(&file, u64::MAX, None, Advice::Normal).unwrap();
    fadvise(&file, i64::MAX as u64, NonZeroU64::new(1), Advice::Normal).unwrap();
    fadvise(&file, 1, NonZeroU64::new(i64::MAX as u64), Advice::Normal).unwrap();
    fadvise(&file, i64::MAX as u64 + 1, None, Advice::Normal).unwrap();
    fadvise(
        &file,
        u64::MAX,
        NonZeroU64::new(i64::MAX as u64),
        Advice::Normal,
    )
    .unwrap();

    // `fadvise` fails on invalid lengths.
    fadvise(&file, u64::MAX, NonZeroU64::new(u64::MAX), Advice::Normal).unwrap_err();
    fadvise(
        &file,
        i64::MAX as u64,
        NonZeroU64::new(u64::MAX),
        Advice::Normal,
    )
    .unwrap_err();
    fadvise(&file, 0, NonZeroU64::new(u64::MAX), Advice::Normal).unwrap_err();
    fadvise(
        &file,
        u64::MAX,
        NonZeroU64::new(i64::MAX as u64 + 1),
        Advice::Normal,
    )
    .unwrap_err();
    fadvise(
        &file,
        i64::MAX as u64 + 1,
        NonZeroU64::new(u64::MAX),
        Advice::Normal,
    )
    .unwrap_err();
    fadvise(
        &file,
        i64::MAX as u64,
        NonZeroU64::new(i64::MAX as u64 + 1),
        Advice::Normal,
    )
    .unwrap_err();
    fadvise(
        &file,
        0,
        NonZeroU64::new(i64::MAX as u64 + 1),
        Advice::Normal,
    )
    .unwrap_err();
}

#[test]
fn invalid_offset_pread() {
    use rustix::fs::{openat, Mode, OFlags, CWD};
    use rustix::io::pread;
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    let mut buf = [0_u8; 1];
    pread(&file, &mut buf, u64::MAX).unwrap_err();
    pread(&file, &mut buf, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(not(any(apple, target_os = "cygwin")))]
#[test]
fn invalid_offset_pwrite() {
    use rustix::fs::{openat, Mode, OFlags, CWD};
    use rustix::io::pwrite;
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "file",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();

    let buf = [0_u8; 1];
    pwrite(&file, &buf, u64::MAX).unwrap_err();
    pwrite(&file, &buf, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(linux_kernel)]
#[test]
fn invalid_offset_copy_file_range() {
    use rustix::fs::{copy_file_range, openat, Mode, OFlags, CWD};
    use rustix::io::write;
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let src = openat(
        &dir,
        "src",
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();
    let dst = openat(
        &dir,
        "dst",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();
    write(&src, b"a").unwrap();

    let mut off_in = u64::MAX;
    let mut off_out = 0;
    copy_file_range(&src, Some(&mut off_in), &dst, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = i64::MAX as u64 + 1;
    let mut off_out = 0;
    copy_file_range(&src, Some(&mut off_in), &dst, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = u64::MAX;
    copy_file_range(&src, Some(&mut off_in), &dst, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = i64::MAX as u64;
    copy_file_range(&src, Some(&mut off_in), &dst, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = i64::MAX as u64 + 1;
    copy_file_range(&src, Some(&mut off_in), &dst, Some(&mut off_out), 1).unwrap_err();
}
