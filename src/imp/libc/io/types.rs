use bitflags::bitflags;
use std::os::raw::c_int;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    pub struct ReadWriteFlags: c_int {
        /// `RWF_DSYNC`
        const DSYNC = libc::RWF_DSYNC;
        /// `RWF_HIPRI`
        const HIPRI = libc::RWF_HIPRI;
        /// `RWF_SYNC`
        const SYNC = libc::RWF_SYNC;
        /// `RWF_NOWAIT`
        const NOWAIT = libc::RWF_NOWAIT;
        /// `RWF_APPEND`
        const APPEND = libc::RWF_APPEND;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    pub struct DupFlags: c_int {
        /// `O_CLOEXEC`
        #[cfg(not(any(target_os = "android", target_os = "macos", target_os = "ios", target_os = "redox")))] // Android 5.0 has dup3, but libc doesn't have bindings
        const CLOEXEC = libc::O_CLOEXEC;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `PROT_*` flags for use with [`mmap`].
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct ProtFlags: c_int {
        /// `PROT_READ`
        const READ = libc::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = libc::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = libc::PROT_EXEC;
        /// `PROT_NONE`
        const NONE = libc::PROT_NONE;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `MAP_*` flags for use with [`mmap`].
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct MapFlags: c_int {
        /// `MAP_SHARED`
        const SHARED = libc::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const SHARED_VALIDATE = libc::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = libc::MAP_PRIVATE;
        /// `MAP_ANONYMOUS`, aka `MAP_ANON`
        const ANONYMOUS = libc::MAP_ANONYMOUS;
        /// `MAP_DENYWRITE`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const DENYWRITE = libc::MAP_DENYWRITE;
        /// `MAP_FIXED`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const FIXED_NOREPLACE = libc::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const GROWSDOWN = libc::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const HUGETLB = libc::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const HUGE_2MB = libc::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const HUGE_1GB = libc::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const LOCKED = libc::MAP_LOCKED;
        /// `MAP_NORESERVE`
        #[cfg(not(any(target_os = "freebsd", target_os = "redox")))]
        const NORESERVE = libc::MAP_NORESERVE;
        /// `MAP_POPULATE`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "redox")))]
        const POPULATE = libc::MAP_POPULATE;
        /// `MAP_STACK`
        #[cfg(not(any(target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "redox")))]
        const STACK = libc::MAP_STACK;
        /// `MAP_SYNC`
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "emscripten", target_os = "fuchsia", target_os = "redox")))]
        const SYNC = libc::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        #[cfg(any())]
        const UNINITIALIZED = libc::MAP_UNINITIALIZED;
    }
}

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
bitflags! {
    /// `O_*` constants for use with [`pipe_with`].
    ///
    /// [`pipe_with`]: crate::io::pipe_with
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

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// The `O_*` flags accepted by [`userfaultfd`].
    ///
    /// [`userfaultfd`]: crate::io::userfaultfd
    pub struct UserFaultFdFlags: c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = libc::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = libc::O_NONBLOCK;
    }
}

/// `struct termios`, for use with [`ioctl_tcgets`].
///
/// [`ioctl_tcgets`]: crate::io::ioctl_tcgets
#[cfg(not(target_os = "wasi"))]
pub type Termios = libc::termios;

/// `struct winsize`
#[cfg(not(target_os = "wasi"))]
pub type Winsize = libc::winsize;

#[cfg(not(target_os = "wasi"))]
pub type Tcflag = libc::tcflag_t;

#[cfg(not(target_os = "wasi"))]
pub const ICANON: Tcflag = libc::ICANON;

#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
pub const PIPE_BUF: usize = libc::PIPE_BUF;
