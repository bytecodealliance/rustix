use rustix::fs::{sync_file_range, SyncFileRangeFlags};
use std::io::Write as _;

#[test]
fn test_sync_file_range() {
    let tmp = tempfile::tempdir().unwrap();
    let mut file = std::fs::File::create(tmp.path().join("file")).unwrap();
    file.write_all(&[7_u8; 3 * 4096]).unwrap();

    // Start write-back of the first page, then wait for the whole file.
    sync_file_range(&file, 0, 4096, SyncFileRangeFlags::WRITE).unwrap();
    sync_file_range(
        &file,
        0,
        0,
        SyncFileRangeFlags::WAIT_BEFORE
            | SyncFileRangeFlags::WRITE
            | SyncFileRangeFlags::WAIT_AFTER,
    )
    .unwrap();

    // No flags does nothing, and a range past the end is fine.
    sync_file_range(&file, 4096, 4096, SyncFileRangeFlags::empty()).unwrap();
    sync_file_range(&file, 1 << 40, 4096, SyncFileRangeFlags::WRITE).unwrap();
}

#[test]
fn test_sync_file_range_rejects_a_pipe() {
    let (reader, _writer) = rustix::pipe::pipe().unwrap();
    assert_eq!(
        sync_file_range(&reader, 0, 0, SyncFileRangeFlags::WRITE),
        Err(rustix::io::Errno::SPIPE)
    );
}
