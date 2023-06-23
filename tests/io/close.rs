use rustix::fd::IntoRawFd;

#[cfg(any(unix, target_os = "wasi"))]
#[test]
fn test_close_file() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    let raw = file.into_raw_fd();
    unsafe {
        rustix::io::close(raw);
    }
}

#[test]
fn test_close_socket() {
    let socket = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let raw = socket.into_raw_fd();
    unsafe {
        rustix::io::close(raw);
    }
}
