#[cfg(linux_kernel)]
#[test]
fn test_fcntl_getpipe_size() {
    use rustix::pipe::fcntl_getpipe_size;

    let (reader, writer) = rustix::pipe::pipe().unwrap();

    let reader_size = fcntl_getpipe_size(&reader).unwrap();
    let writer_size = fcntl_getpipe_size(&writer).unwrap();
    assert_eq!(reader_size, writer_size);
}

#[cfg(linux_kernel)]
#[test]
fn test_fcntl_setpipe_size() {
    use rustix::pipe::{fcntl_getpipe_size, fcntl_setpipe_size};

    let (reader, writer) = rustix::pipe::pipe().unwrap();

    let new_size = 4096 * 2;
    let reader_size = fcntl_setpipe_size(&reader, new_size).unwrap();
    let writer_size = fcntl_getpipe_size(&writer).unwrap();
    assert_eq!(reader_size, new_size);
    assert_eq!(reader_size, writer_size);

    let new_size = 4096 * 16;
    let reader_size = fcntl_setpipe_size(&reader, new_size).unwrap();
    let writer_size = fcntl_getpipe_size(&writer).unwrap();
    assert_eq!(reader_size, new_size);
    assert_eq!(reader_size, writer_size);
}

/// Test that we can write up to the pipe buffer size without blocking.
#[cfg(linux_kernel)]
#[test]
fn test_fcntl_pipe_sized_writes() {
    use rustix::io::{read, write};
    use rustix::pipe::{fcntl_getpipe_size, fcntl_setpipe_size};

    let (reader, writer) = rustix::pipe::pipe().unwrap();

    let size = fcntl_getpipe_size(&reader).unwrap();

    let ones = vec![1; size];
    assert_eq!(write(&writer, &ones), Ok(size));
    let mut buf = vec![2; size];
    assert_eq!(read(&reader, &mut buf), Ok(size));
    assert_eq!(buf, ones);

    let size = size * 2;
    let set_size = fcntl_setpipe_size(&reader, size).unwrap();
    let get_size = fcntl_getpipe_size(&reader).unwrap();
    assert_eq!(size, set_size);
    assert_eq!(size, get_size);

    let ones = vec![1; size];
    assert_eq!(write(&writer, &ones), Ok(size));
    let mut buf = vec![2; size];
    assert_eq!(read(&reader, &mut buf), Ok(size));
    assert_eq!(buf, ones);
}
