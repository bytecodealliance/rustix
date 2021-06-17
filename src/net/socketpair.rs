use crate::io;
use crate::net::{AcceptFlags, AddressFamily, Protocol, SocketType};
use io_lifetimes::OwnedFd;
#[cfg(libc)]
use {crate::zero_ok, std::mem::MaybeUninit, std::os::raw::c_int};

/// `socketpair(domain, type_ | accept_flags, protocol)`
#[inline]
pub fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    _socketpair(domain, type_, accept_flags, protocol)
}

#[cfg(libc)]
fn _socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        zero_ok(libc::socketpair(
            domain as c_int,
            type_ as c_int | accept_flags.bits(),
            protocol as c_int,
            fds.as_mut_ptr().cast::<c_int>(),
        ))?;

        let [fd0, fd1] = fds.assume_init();
        Ok((fd0, fd1))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    crate::linux_raw::socketpair(
        domain as u32,
        type_ as u32 | accept_flags.bits(),
        protocol as u32,
    )
}
