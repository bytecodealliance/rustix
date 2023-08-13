//! Terminal-related `ioctl` functions.

use crate::fd::AsFd;
use crate::{backend, io, ioctl};
use backend::c;

/// `ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal.
///
/// # References
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4
/// [NetBSD]: https://man.netbsd.org/tty.4
/// [OpenBSD]: https://man.openbsd.org/tty.4
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
#[inline]
#[doc(alias = "TIOCEXCL")]
pub fn ioctl_tiocexcl<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    ioctl::ioctl(fd, Tiocexcl)
}

/// `ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.
///
/// # References
///  - [Linux]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/tty_ioctl.4.html
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4
/// [NetBSD]: https://man.netbsd.org/tty.4
/// [OpenBSD]: https://man.openbsd.org/tty.4
#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
#[inline]
#[doc(alias = "TIOCNXCL")]
pub fn ioctl_tiocnxcl<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    ioctl::ioctl(fd, Tiocnxcl)
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
struct Tiocexcl;

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Tiocexcl {
    type Output = ();
    const OPCODE: ioctl::Opcode = ioctl::Opcode::bad(c::TIOCEXCL as ioctl::RawOpcode);
    const IS_MUTATING: bool = false;

    fn as_ptr(&mut self) -> *mut c::c_void {
        core::ptr::null_mut()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(())
    }
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
struct Tiocnxcl;

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Tiocnxcl {
    type Output = ();
    const OPCODE: ioctl::Opcode = ioctl::Opcode::bad(c::TIOCNXCL as ioctl::RawOpcode);
    const IS_MUTATING: bool = false;

    fn as_ptr(&mut self) -> *mut c::c_void {
        core::ptr::null_mut()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(())
    }
}
