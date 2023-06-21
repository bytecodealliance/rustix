#[test]
fn test_basic_pipes() {
    use rustix::io::{read, write};
    use rustix::pipe::pipe;

    let message = b"Hello, tee!";

    #[cfg(not(any(
        solarish,
        windows,
        target_os = "haiku",
        target_os = "redox",
        target_os = "wasi",
    )))]
    assert!(message.len() <= rustix::pipe::PIPE_BUF);

    let (reader, writer) = pipe().unwrap();

    let n = write(&writer, message).unwrap();
    assert_eq!(n, message.len());

    let mut buf = vec![0_u8; 256];
    let n = read(&reader, &mut buf).unwrap();
    assert_eq!(n, message.len());
    assert_eq!(&buf[..n], message);
}

#[test]
fn test_basic_pipes_with() {
    use rustix::io::{read, write};
    use rustix::pipe::{PipeFlags, pipe_with};

    let message = b"Hello, tee!";

    #[cfg(not(any(
        solarish,
        windows,
        target_os = "haiku",
        target_os = "redox",
        target_os = "wasi",
    )))]
    assert!(message.len() <= rustix::pipe::PIPE_BUF);

    let (reader, writer) = pipe_with(PipeFlags::CLOEXEC).unwrap();

    let n = write(&writer, message).unwrap();
    assert_eq!(n, message.len());

    let mut buf = vec![0_u8; 256];
    let n = read(&reader, &mut buf).unwrap();
    assert_eq!(n, message.len());
    assert_eq!(&buf[..n], message);
}
