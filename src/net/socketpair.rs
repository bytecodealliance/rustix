use crate::{
    io,
    net::{AcceptFlags, AddressFamily, Protocol, SocketType},
};
use io_lifetimes::OwnedFd;
#[cfg(libc)]
use {crate::libc::conv::ret, std::mem::MaybeUninit, std::os::raw::c_int};

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
        ret(libc::socketpair(
            domain.0 as c_int,
            type_.0 as c_int | accept_flags.bits(),
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
        u32::from(domain.0),
        type_.0 | accept_flags.bits(),
        protocol as u32,
    )
}
