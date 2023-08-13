//! The Unix `ioctl` function is effectively lots of different functions hidden
//! behind a single dynamic dispatch interface. In order to provide a type-safe
//! API, rustix makes them all separate functions so that they can have
//! dedicated static type signatures.
//!
//! Some ioctls, such as those related to filesystems, terminals, and
//! processes, live in other top-level API modules.

use crate::{backend, io, ioctl};
use backend::{c, fd::AsFd};

#[cfg(not(target_os = "espidf"))]
use core::mem::MaybeUninit;

/// `ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag.
///
/// Also known as `fcntl(fd, F_SETFD, FD_CLOEXEC)`.
///
/// # References
///  - [Winsock2]
///  - [NetBSD]
///  - [OpenBSD]
///
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-ioctlsocket
/// [NetBSD]: https://man.netbsd.org/ioctl.2#GENERIC%20IOCTLS
/// [OpenBSD]: https://man.openbsd.org/ioctl.2#GENERIC_IOCTLS
#[cfg(apple)]
#[inline]
#[doc(alias = "FIOCLEX")]
#[doc(alias = "FD_CLOEXEC")]
pub fn ioctl_fioclex<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    ioctl::ioctl(fd, Fioclex)
}

/// `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode.
///
/// # References
///  - [Winsock2]
///  - [NetBSD]
///  - [OpenBSD]
///
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/winsock/winsock-ioctls#unix-ioctl-codes
/// [NetBSD]: https://man.netbsd.org/ioctl.2#GENERIC%20IOCTLS
/// [OpenBSD]: https://man.openbsd.org/ioctl.2#GENERIC_IOCTLS
#[inline]
#[doc(alias = "FIONBIO")]
pub fn ioctl_fionbio<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()> {
    ioctl::ioctl(fd, Fionbio(c::c_int::from(value)))
}

/// `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read.
///
/// The result of this function gets silently coerced into a C `int`
/// by the OS, so it may contain a wrapped value.
///
/// # References
///  - [Linux]
///  - [Winsock2]
///  - [FreeBSD]
///  - [NetBSD]
///  - [OpenBSD]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_tty.2.html
/// [Winsock2]: https://docs.microsoft.com/en-us/windows/win32/winsock/winsock-ioctls#unix-ioctl-codes
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2#GENERIC%09IOCTLS
/// [NetBSD]: https://man.netbsd.org/ioctl.2#GENERIC%20IOCTLS
/// [OpenBSD]: https://man.openbsd.org/ioctl.2#GENERIC_IOCTLS
#[cfg(not(target_os = "espidf"))]
#[inline]
#[doc(alias = "FIONREAD")]
pub fn ioctl_fionread<Fd: AsFd>(fd: Fd) -> io::Result<u64> {
    ioctl::ioctl(fd, Fionread(MaybeUninit::uninit()))
}

#[cfg(apple)]
struct Fioclex;

#[cfg(apple)]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Fioclex {
    type Output = ();

    const IS_MUTATING: bool = false;
    const OPCODE: ioctl::Opcode = ioctl::Opcode::bad(c::FIOCLEX);

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

struct Fionbio(c::c_int);

#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Fionbio {
    type Output = ();

    const IS_MUTATING: bool = false;
    const OPCODE: ioctl::Opcode = ioctl::Opcode::bad(c::FIONBIO);

    fn as_ptr(&mut self) -> *mut c::c_void {
        (&mut self.0 as *mut c::c_int).cast()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(())
    }
}

#[cfg(not(target_os = "espidf"))]
struct Fionread(MaybeUninit<c::c_int>);

#[cfg(not(target_os = "espidf"))]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Fionread {
    type Output = u64;
    const IS_MUTATING: bool = true;
    const OPCODE: ioctl::Opcode = ioctl::Opcode::bad(c::FIONREAD);

    fn as_ptr(&mut self) -> *mut c::c_void {
        self.0.as_mut_ptr().cast()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        ptr: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(ptr.cast::<c::c_int>().read() as u64)
    }
}
