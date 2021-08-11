use crate::imp;
use crate::io::{self, OwnedFd};
use crate::net::{AcceptFlags, AddressFamily, Protocol, SocketType};

/// `socketpair(domain, type_ | accept_flags, protocol)`
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/socketpair.html
/// [Linux]: https://man7.org/linux/man-pages/man2/socketpair.2.html
#[inline]
pub fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    accept_flags: AcceptFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    imp::syscalls::socketpair(domain, type_, accept_flags, protocol)
}
