#[cfg(linux_raw)]
use io_lifetimes::FromFd;
use std::{io, net::TcpStream};
#[cfg(libc)]
use {
    crate::zero_ok,
    std::mem::MaybeUninit,
    unsafe_io::os::posish::{FromRawFd, RawFd},
};

/// `socketpair(domain, SOCK_STREAM | SOCK_CLOEXEC, protocol)`
#[cfg(libc)]
pub fn socketpair_stream(domain: i32, protocol: i32) -> io::Result<(TcpStream, TcpStream)> {
    let mut fds = MaybeUninit::<[RawFd; 2]>::uninit();
    unsafe {
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        let flags = libc::SOCK_CLOEXEC;

        // Darwin lacks `SOCK_CLOEXEC`.
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        let flags = 0;

        zero_ok(libc::socketpair(
            domain,
            libc::SOCK_STREAM | flags,
            protocol,
            fds.as_mut_ptr().cast::<RawFd>(),
        ))?;

        let fds = fds.assume_init();

        // Darwin lacks `SOCK_CLOEXEC`, so set it manually.
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        for fd in &fds {
            zero_ok(libc::ioctl(*fd, libc::FIOCLEX))?;
        }

        Ok((
            TcpStream::from_raw_fd(fds[0]),
            TcpStream::from_raw_fd(fds[1]),
        ))
    }
}

/// `socketpair(domain, SOCK_STREAM | SOCK_CLOEXEC, protocol)`
#[cfg(linux_raw)]
pub fn socketpair_stream(domain: i32, protocol: i32) -> io::Result<(TcpStream, TcpStream)> {
    crate::linux_raw::socketpair(
        domain,
        linux_raw_sys::general::SOCK_STREAM | linux_raw_sys::general::SOCK_CLOEXEC,
        protocol,
    )
    .map(|fds| (TcpStream::from_fd(fds.0), TcpStream::from_fd(fds.1)))
}
