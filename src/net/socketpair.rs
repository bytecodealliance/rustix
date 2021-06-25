use crate::{
    imp, io,
    net::{AcceptFlags, AddressFamily, Protocol, SocketType},
};
use io_lifetimes::OwnedFd;

/// `socketpair(domain, type_ | accept_flags, protocol)`
#[inline]
pub fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::socketpair(domain, type_, accept_flags, protocol)
}
