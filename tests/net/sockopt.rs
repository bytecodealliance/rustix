#[test]
fn test_sockopt_timeout() {
    use rustix::net::{AddressFamily, Protocol, SocketType};
    use std::time::Duration;

    let s =
        rustix::net::socket(AddressFamily::INET, SocketType::STREAM, Protocol::default()).unwrap();

    // On a new socket we shouldn't have a timeout yet.
    assert!(
        rustix::net::sockopt::get_socket_timeout(&s, rustix::net::sockopt::Timeout::Recv)
            .unwrap()
            .is_none()
    );

    // Set a timeout.
    rustix::net::sockopt::set_socket_timeout(
        &s,
        rustix::net::sockopt::Timeout::Recv,
        Some(Duration::new(1, 1)),
    )
    .unwrap();

    // Check that we have a timeout of at least the time we set.
    assert!(
        rustix::net::sockopt::get_socket_timeout(&s, rustix::net::sockopt::Timeout::Recv)
            .unwrap()
            .unwrap()
            >= Duration::new(1, 1)
    );
}
