//! Unsafe `ioctl` API.
//!
//! Unix systems expose a number of `ioctl`'s. Although they were originally meant to modify the
//! behavior of files, they've now been adopted as a general purpose system call for making calls
//! into the kernel. In addition to the wide variety of system calls that are included by default
//! in the kernel, many drivers expose their own `ioctl`'s for controlling their behavior, some
//! of which are proprietary.  Therefore it is impossible to make a safe interface for every `ioctl`
//! call, as they all have wildly varying semantics.
//!
//! This module provides an unsafe interface to write your own `ioctl` API.

#![allow(unsafe_code)]

use crate::backend::c;
use crate::fd::{AsFd, BorrowedFd};
use crate::io::Result;

/// Perform an `ioctl` call.
#[inline]
pub fn ioctl<F: AsFd, I: Ioctl>(fd: F, mut ioctl: I) -> Result<I::Output> {
    let fd = fd.as_fd();
    let request = I::OPCODE.raw();
    let arg = ioctl.as_ptr();

    // SAFETY: The variant of `Ioctl` asserts that this is a valid IOCTL call to make.
    let output = if I::IS_MUTATING {
        unsafe { _ioctl(fd, request, arg) }?
    } else {
        unsafe { _ioctl_readonly(fd, request, arg) }?
    };

    // SAFETY: The variant of `Ioctl` asserts that this is a valid pointer to the output data.
    unsafe { I::output_from_ptr(output, arg) }
}

unsafe fn _ioctl(
    fd: BorrowedFd<'_>,
    request: RawOpcode,
    arg: *mut c::c_void,
) -> Result<IoctlOutput> {
    crate::backend::io::syscalls::ioctl(fd, request, arg)
}

unsafe fn _ioctl_readonly(
    fd: BorrowedFd<'_>,
    request: RawOpcode,
    arg: *mut c::c_void,
) -> Result<IoctlOutput> {
    crate::backend::io::syscalls::ioctl_readonly(fd, request, arg)
}

/// A trait defining the properties of an `ioctl` command.
///
/// # Safety
///
///
pub unsafe trait Ioctl {
    /// The type of the output data.
    type Output;

    /// The opcode used by this `ioctl` command.
    const OPCODE: Opcode;

    /// Does the `ioctl` mutate any data in the userspace?
    const IS_MUTATING: bool;

    /// Get a pointer to the data to be passed to the `ioctl` command.
    fn as_ptr(&mut self) -> *mut c::c_void;

    /// Cast the output data to the correct type.
    ///
    /// # Safety
    ///
    /// The `ptr` must be the resulting value after a successful `ioctl` call.
    unsafe fn output_from_ptr(out: IoctlOutput, ptr: *mut c::c_void) -> Result<Self::Output>;
}

/// The opcode used by an `Ioctl`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Opcode {
    /// A raw, "bad" opcode.
    Bad(RawOpcode),
}

impl Opcode {
    /// Get the raw opcode.
    #[inline]
    pub fn raw(self) -> RawOpcode {
        match self {
            Self::Bad(raw) => raw,
        }
    }
}

/// The type used by the `ioctl` to signify the output.
pub type IoctlOutput = c::c_int;

/// The type used by the `ioctl` to signify the command.
pub type RawOpcode = _RawOpcode;

// Under raw Linux, this is an unsigned int.
#[cfg(linux_raw)]
type _RawOpcode = c::c_uint;

// On libc Linux with GNU libc, this is an unsigned long.
#[cfg(all(not(linux_raw), target_os = "linux", target_env = "gnu"))]
type _RawOpcode = c::c_ulong;

// Musl uses a c_int
#[cfg(all(not(linux_raw), target_os = "linux", not(target_env = "gnu")))]
type _RawOpcode = c::c_int;

// Android uses c_int
#[cfg(all(not(linux_raw), target_os = "android"))]
type _RawOpcode = c::c_int;

// Every BSD I looked at, Haiku and Redox uses an unsigned long.
#[cfg(any(apple, bsd, target_os = "redox", target_os = "haiku"))]
type _RawOpcode = c::c_ulong;

// Solaris, Fuchsia, Emscripten and WASI use an int
#[cfg(any(
    target_os = "solaris",
    target_os = "illumos",
    target_os = "fuchsia",
    target_os = "emscripten",
    target_os = "wasi",
    target_os = "nto"
))]
type _RawOpcode = c::c_int;

// ESP-IDF uses a c_uint
#[cfg(target_os = "espidf")]
type _RawOpcode = c::c_uint;

// Windows has ioctlsocket, which uses i32
#[cfg(windows)]
type _RawOpcode = i32;
