use crate::zero_ok;
#[cfg(unix)]
use std::os::unix::io::{FromRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{FromRawFd, RawFd};
use std::{convert::TryInto, io, mem::MaybeUninit, net::TcpStream};

/// `socketpair(domain, SOCK_STREAM, protocol)`
pub fn socketpair_stream(domain: i32, protocol: i32) -> io::Result<(TcpStream, TcpStream)> {
    let mut fds = MaybeUninit::<[RawFd; 2]>::uninit();
    unsafe {
        zero_ok(libc::socketpair(
            domain.try_into().unwrap(),
            libc::SOCK_STREAM,
            protocol.try_into().unwrap(),
            fds.as_mut_ptr() as *mut RawFd,
        ))?;
        let fds = fds.assume_init();
        Ok((
            TcpStream::from_raw_fd(fds[0]),
            TcpStream::from_raw_fd(fds[1]),
        ))
    }
}
