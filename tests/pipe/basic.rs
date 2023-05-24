#[test]
fn test_basic_pipes() {
    use rustix::io::{read, write};
    use rustix::pipe::pipe;

    let message = b"Hello, tee!";
    assert!(message.len() <= rustix::pipe::PIPE_BUF);

    let (reader, writer) = pipe().unwrap();

    let n = write(&writer, message).unwrap();
    assert_eq!(n, message.len());

    let mut buf = vec![0_u8; 256];
    let n = read(&reader, &mut buf).unwrap();
    assert_eq!(n, message.len());
    assert_eq!(&buf[..n], message);
}
