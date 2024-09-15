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
