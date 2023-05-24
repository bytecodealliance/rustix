#[cfg(feature = "fs")]
#[cfg(linux_kernel)]
#[test]
fn test_tee() {
    use rustix::io::{read, write};
    use rustix::pipe::{pipe, tee, SpliceFlags};

    let message = b"Hello, tee!";
    assert!(message.len() <= rustix::pipe::PIPE_BUF);

    // Create two pipes.
    let (read_a, write_a) = pipe().unwrap();
    let (read_b, write_b) = pipe().unwrap();

    // Write a message into one of the pipes.
    let n = write(&write_a, message).unwrap();
    assert_eq!(n, message.len());

    // "Tee" the message into the other pipe.
    let n = tee(&read_a, &write_b, 256, SpliceFlags::empty()).unwrap();
    assert_eq!(n, message.len());

    // Check that the "tee" wrote our message to the other pipe.
    let mut buf = vec![0_u8; 256];
    let n = read(&read_b, &mut buf).unwrap();
    assert_eq!(n, message.len());
    assert_eq!(&buf[..n], message);

    // Check that the "tee" left our message in the first pipe.
    let mut buf = vec![0_u8; 256];
    let n = read(&read_a, &mut buf).unwrap();
    assert_eq!(n, message.len());
    assert_eq!(&buf[..n], message);
}
