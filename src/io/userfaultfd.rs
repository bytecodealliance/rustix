//! The Linux `userfaultfd` API.
//!
//! # Safety
//!
//! Calling `userfaultfd` is safe, but the returned file descriptor lets users
//! observe and manipulate process memory in magical ways.
#![allow(unsafe_code)]

use crate::fd::AsFd;
use crate::imp;
use crate::io::{self, OwnedFd};

pub use imp::io::{
    UffdEvent, UffdFeatureFlags, UffdMsg, UffdPagefaultFlags, UffdioApi, UffdioCopy,
    UffdioCopyModeFlags, UffdioIoctlFlags, UffdioRange, UffdioRegister, UffdioRegisterModeFlags,
    UffdioWriteprotect, UffdioZeropage, UffdioZeropageModeFlags, UserfaultfdFlags, UFFD_API,
};

/// `userfaultfd(flags)` (since Linux 4.3)
///
/// # Safety
///
/// The call itself is safe, but the returned file descriptor lets users
/// observe and manipulate process memory in magical ways.
///
/// # References
///  - [Linux]
///  - [Linux userfaultfd]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/userfaultfd.2.html
/// [Linux userfaultfd]: https://www.kernel.org/doc/Documentation/vm/userfaultfd.txt
#[inline]
pub unsafe fn userfaultfd(flags: UserfaultfdFlags) -> io::Result<OwnedFd> {
    imp::io::syscalls::userfaultfd(flags)
}

/// `ioctl(fd, UFFDIO_API, api)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[inline]
#[doc(alias = "UFFDIO_API")]
pub fn ioctl_uffdio_api<Fd: AsFd>(fd: &Fd, api: &mut UffdioApi) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_api(fd, api)
}

/// `ioctl(fd, UFFDIO_REGISTER, register)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_REGISTER")]
#[inline]
pub fn ioctl_uffdio_register<Fd: AsFd>(fd: &Fd, register: &mut UffdioRegister) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_register(fd, register)
}

/// `ioctl(fd, UFFDIO_UNREGISTER, unregister)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_UNREGISTER")]
#[inline]
pub fn ioctl_uffdio_unregister<Fd: AsFd>(fd: &Fd, range: &UffdioRange) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_unregister(fd, range)
}

/// `ioctl(fd, UFFDIO_WAKE, range)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_WAKE")]
#[inline]
pub fn ioctl_uffdio_wake<Fd: AsFd>(fd: &Fd, range: &UffdioRange) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_wake(fd, range)
}

/// `ioctl(fd, UFFDIO_COPY, copy)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_COPY")]
#[inline]
pub fn ioctl_uffdio_copy<Fd: AsFd>(fd: &Fd, copy: &mut UffdioCopy) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_copy(fd, copy)
}

/// `ioctl(fd, UFFDIO_ZEROPAGE, zeropage)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_ZEROPAGE")]
#[inline]
pub fn ioctl_uffdio_zeropage<Fd: AsFd>(fd: &Fd, zeropage: &mut UffdioZeropage) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_zeropage(fd, zeropage)
}

/// `ioctl(fd, UFFDIO_WRITEPROTECT, writeprotect)` (since Linux 5.7)
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_userfaultfd.2.html
#[doc(alias = "UFFDIO_WRITEPROTECT")]
#[inline]
pub fn ioctl_uffdio_writeprotect<Fd: AsFd>(
    fd: &Fd,
    writeprotect: &mut UffdioWriteprotect,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::io::syscalls::ioctl_uffdio_writeprotect(fd, writeprotect)
}
