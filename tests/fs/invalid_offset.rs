//! Posix-ish interfaces tend to use signed integers for file offsets, while
//! Rust APIs tend to use `u64`. Test that exreme `u64` values in APIs that
//! take file offsets are properly diagnosed.
//!
//! These tests are disabled on ios/macos since those platforms kill the
//! process with SIGXFSZ instead of returning an error.

#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

use std::io::SeekFrom;

#[test]
fn invalid_offset_seek() {
    use posish::fs::{cwd, openat, seek, Mode, OFlags};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
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
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "ios",
    target_os = "macos"
)))]
#[test]
fn invalid_offset_fallocate() {
    use posish::fs::{cwd, fallocate, openat, FallocateFlags, Mode, OFlags};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    fallocate(&file, FallocateFlags::empty(), u64::MAX, 1).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), i64::MAX as u64 + 1, 1).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), 0, u64::MAX).unwrap_err();
    fallocate(&file, FallocateFlags::empty(), 0, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(not(any(
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "ios",
    target_os = "macos",
)))]
#[test]
fn invalid_offset_fadvise() {
    use posish::fs::{cwd, fadvise, openat, Advice, Mode, OFlags};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    // `fadvise` never fails on invalid offsets.
    fadvise(&file, i64::MAX as u64, i64::MAX as u64, Advice::Normal).unwrap();
    fadvise(&file, u64::MAX, 0, Advice::Normal).unwrap();
    // `fadvise` fails on invalid lengths.
    fadvise(&file, u64::MAX, u64::MAX, Advice::Normal).unwrap_err();
    fadvise(&file, i64::MAX as u64, u64::MAX, Advice::Normal).unwrap_err();
    fadvise(&file, 0, u64::MAX, Advice::Normal).unwrap_err();
    fadvise(&file, u64::MAX, i64::MAX as u64 + 1, Advice::Normal).unwrap_err();
    fadvise(&file, i64::MAX as u64, i64::MAX as u64 + 1, Advice::Normal).unwrap_err();
    fadvise(&file, 0, i64::MAX as u64 + 1, Advice::Normal).unwrap_err();
}

#[test]
fn invalid_offset_pread() {
    use posish::{
        fs::{cwd, openat, Mode, OFlags},
        io::pread,
    };
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    let mut buf = [0_u8; 1_usize];
    pread(&file, &mut buf, u64::MAX).unwrap_err();
    pread(&file, &mut buf, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
#[test]
fn invalid_offset_pwrite() {
    use posish::{
        fs::{cwd, openat, Mode, OFlags},
        io::pwrite,
    };
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    let buf = [0_u8; 1_usize];
    pwrite(&file, &buf, u64::MAX).unwrap_err();
    pwrite(&file, &buf, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn invalid_offset_copy_file_range() {
    use posish::{
        fs::{copy_file_range, cwd, openat, Mode, OFlags},
        io::write,
    };
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let foo = openat(
        &dir,
        "foo",
        OFlags::RDWR | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();
    let bar = openat(
        &dir,
        "bar",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();
    write(&foo, b"a").unwrap();

    let mut off_in = u64::MAX;
    let mut off_out = 0;
    copy_file_range(&foo, Some(&mut off_in), &bar, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = i64::MAX as u64 + 1;
    let mut off_out = 0;
    copy_file_range(&foo, Some(&mut off_in), &bar, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = u64::MAX;
    copy_file_range(&foo, Some(&mut off_in), &bar, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = i64::MAX as u64;
    copy_file_range(&foo, Some(&mut off_in), &bar, Some(&mut off_out), 1).unwrap_err();

    let mut off_in = 0;
    let mut off_out = i64::MAX as u64 + 1;
    copy_file_range(&foo, Some(&mut off_in), &bar, Some(&mut off_out), 1).unwrap_err();
}
