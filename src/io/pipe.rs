use crate::io;
#[cfg(any(
    linux_raw,
    all(
        libc,
        not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
    )
))]
use bitflags::bitflags;
use io_lifetimes::OwnedFd;
#[cfg(libc)]
use {crate::libc::conv::ret, std::mem::MaybeUninit};

#[cfg(all(
    libc,
    not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
))]
bitflags! {
    /// `O_*` constants for use with `pipe2`.
    pub struct PipeFlags: libc::c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = libc::O_CLOEXEC;
        /// `O_DIRECT`
        #[cfg(not(any(target_os = "redox")))]
        const DIRECT = libc::O_DIRECT;
        /// `O_NONBLOCK`
        const NONBLOCK = libc::O_NONBLOCK;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `O_*` constants for use with `pipe2`.
    pub struct PipeFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_DIRECT`
        const DIRECT = linux_raw_sys::general::O_DIRECT;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
}

/// `pipe()`
#[cfg(any(target_os = "ios", target_os = "macos"))]
#[inline]
pub fn pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    _pipe()
}

#[cfg(all(libc, any(target_os = "ios", target_os = "macos")))]
fn _pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::pipe(result.as_mut_ptr().cast::<i32>()))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    crate::linux_raw::pipe()
}

/// `pipe2(flags)`
#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
#[inline]
pub fn pipe2(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    _pipe2(flags)
}

#[cfg(all(
    libc,
    not(any(target_os = "ios", target_os = "macos", target_os = "wasi"))
))]
fn _pipe2(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut result = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(libc::pipe2(result.as_mut_ptr().cast::<i32>(), flags.bits()))?;
        let [p0, p1] = result.assume_init();
        Ok((p0, p1))
    }
}

#[cfg(linux_raw)]
#[inline]
fn _pipe2(flags: PipeFlags) -> io::Result<(OwnedFd, OwnedFd)> {
    crate::linux_raw::pipe2(flags.bits())
}
