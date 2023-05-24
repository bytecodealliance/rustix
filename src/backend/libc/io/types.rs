use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `FD_*` constants for use with [`fcntl_getfd`] and [`fcntl_setfd`].
    ///
    /// [`fcntl_getfd`]: crate::io::fcntl_getfd
    /// [`fcntl_setfd`]: crate::io::fcntl_setfd
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FdFlags: c::c_int {
        /// `FD_CLOEXEC`
        const CLOEXEC = c::FD_CLOEXEC;
    }
}

#[cfg(linux_kernel)]
bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ReadWriteFlags: c::c_int {
        /// `RWF_DSYNC` (since Linux 4.7)
        const DSYNC = linux_raw_sys::general::RWF_DSYNC as c::c_int;
        /// `RWF_HIPRI` (since Linux 4.6)
        const HIPRI = linux_raw_sys::general::RWF_HIPRI as c::c_int;
        /// `RWF_SYNC` (since Linux 4.7)
        const SYNC = linux_raw_sys::general::RWF_SYNC as c::c_int;
        /// `RWF_NOWAIT` (since Linux 4.14)
        const NOWAIT = linux_raw_sys::general::RWF_NOWAIT as c::c_int;
        /// `RWF_APPEND` (since Linux 4.16)
        const APPEND = linux_raw_sys::general::RWF_APPEND as c::c_int;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct DupFlags: c::c_int {
        /// `O_CLOEXEC`
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "android",
            target_os = "redox",
        )))] // Android 5.0 has dup3, but libc doesn't have bindings
        const CLOEXEC = c::O_CLOEXEC;
    }
}
