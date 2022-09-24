#[cfg(feature = "fs")]
#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_splice_cursor() {
    use rustix::io::{pipe, splice, SpliceFlags};
    use std::io::{Read, Seek, SeekFrom, Write};

    let mut src = tempfile::tempfile().unwrap();
    let mut dest = tempfile::tempfile().unwrap();
    let (read_p, write_p) = pipe().unwrap();
    let mut buff = vec![];

    writeln!(src, "hello world").unwrap();

    src.seek(SeekFrom::Start(6)).unwrap();

    splice(&src, None, &write_p, None, 5, SpliceFlags::empty()).unwrap();
    splice(&read_p, None, &dest, None, 5, SpliceFlags::empty()).unwrap();

    dest.rewind().unwrap();

    dest.read_to_end(&mut buff).unwrap();
    assert_eq!(buff, b"world");
}

#[cfg(feature = "fs")]
#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_splice_offset() {
    use rustix::io::{pipe, splice, SpliceFlags};
    use std::io::{Read, Write};

    let mut src = tempfile::tempfile().unwrap();
    let mut dest = tempfile::tempfile().unwrap();
    let (read_p, write_p) = pipe().unwrap();
    let mut buff = vec![];

    writeln!(src, "hello world").unwrap();

    splice(&src, Some(0), &write_p, None, 5, SpliceFlags::empty()).unwrap();
    splice(&read_p, None, &dest, Some(0), 5, SpliceFlags::empty()).unwrap();

    dest.read_to_end(&mut buff).unwrap();
    assert_eq!(buff, b"hello");
}

#[cfg(feature = "fs")]
#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn test_splice_pipe2pipe() {
    use rustix::io::{pipe, read, splice, write, SpliceFlags};

    let (read_p1, write_p1) = pipe().unwrap();
    let (read_p2, write_p2) = pipe().unwrap();
    let mut buff = [0; 5];

    write(&write_p1, b"hello").unwrap();
    splice(&read_p1, None, write_p2, None, 5, SpliceFlags::empty()).unwrap();
    read(&read_p2, &mut buff).unwrap();

    assert_eq!(&buff, b"hello");
}
