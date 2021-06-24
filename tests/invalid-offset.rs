//! Posix-ish interfaces tend to use signed integers for file offsets, while
//! Rust APIs tend to use `u64`. Test that exreme `u64` values in APIs that
//! take file offsets are properly diagnosed.

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

#[test]
fn invalid_offset_posix_fallocate() {
    use posish::fs::{cwd, openat, posix_fallocate, Mode, OFlags};
    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(&cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();
    let file = openat(
        &dir,
        "foo",
        OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        Mode::IRUSR | Mode::IWUSR,
    )
    .unwrap();

    posix_fallocate(&file, u64::MAX, 1).unwrap_err();
    posix_fallocate(&file, i64::MAX as u64, 1).unwrap_err();
    posix_fallocate(&file, i64::MAX as u64 + 1, 1).unwrap_err();
    posix_fallocate(&file, 0, u64::MAX).unwrap_err();
    posix_fallocate(&file, 0, i64::MAX as u64 + 1).unwrap_err();
}

#[test]
fn invalid_offset_pread() {
    use posish::fs::{cwd, openat, Mode, OFlags};
    use posish::io::pread;
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
    pread(&file, &mut buf, i64::MAX as u64).unwrap_err();
    pread(&file, &mut buf, i64::MAX as u64 + 1).unwrap_err();
}

#[test]
fn invalid_offset_pwrite() {
    use posish::fs::{cwd, openat, Mode, OFlags};
    use posish::io::pwrite;
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
    pwrite(&file, &buf, i64::MAX as u64).unwrap_err();
    pwrite(&file, &buf, i64::MAX as u64 + 1).unwrap_err();
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn invalid_offset_copy_file_range() {
    use posish::fs::{copy_file_range, cwd, openat, Mode, OFlags};
    use posish::io::write;
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
