use bitflags::bitflags;
use std::os::raw::c_int;

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    pub struct ReadWriteFlags: c_int {
        /// `RWF_DSYNC`
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const DSYNC = libc::RWF_DSYNC;
        /// `RWF_HIPRI`
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const HIPRI = libc::RWF_HIPRI;
        /// `RWF_SYNC`
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const SYNC = libc::RWF_SYNC;
        /// `RWF_NOWAIT`
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const NOWAIT = libc::RWF_NOWAIT;
        /// `RWF_APPEND`
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
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
        #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos", target_os = "redox")))] // Android 5.0 has dup3, but libc doesn't have bindings
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
    /// `PROT_*` flags for use with [`mprotect`].
    ///
    /// [`mprotect`]: crate::io::mprotect
    pub struct MprotectFlags: c_int {
        /// `PROT_READ`
        const READ = libc::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = libc::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = libc::PROT_EXEC;
        /// `PROT_NONE`
        const NONE = libc::PROT_NONE;
        /// `PROT_GROWSUP`
        #[cfg(any(target_os = "android", target_os = "linux"))]
        const GROWSUP = libc::PROT_GROWSUP;
        /// `PROT_GROWSDOWN`
        #[cfg(any(target_os = "android", target_os = "linux"))]
        const GROWSDOWN = libc::PROT_GROWSDOWN;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `MAP_*` flags for use with [`mmap`].
    ///
    /// For `MAP_ANONYMOUS` (aka `MAP_ANON`), see [`mmap_anonymous`].
    ///
    /// [`mmap`]: crate::io::mmap
    /// [`mmap_anonymous`]: crates::io::mmap_anonymous
    pub struct MapFlags: c_int {
        /// `MAP_SHARED`
        const SHARED = libc::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const SHARED_VALIDATE = libc::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = libc::MAP_PRIVATE;
        /// `MAP_DENYWRITE`
        #[cfg(not(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "freebsd",
            target_os = "redox"
        )))]
        const DENYWRITE = libc::MAP_DENYWRITE;
        /// `MAP_FIXED`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const FIXED = libc::MAP_FIXED;
        /// `MAP_FIXED_NOREPLACE`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const FIXED_NOREPLACE = libc::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        #[cfg(not(any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "netbsd",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "redox"
        )))]
        const GROWSDOWN = libc::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        #[cfg(not(any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGETLB = libc::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGE_2MB = libc::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGE_1GB = libc::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        #[cfg(not(any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const LOCKED = libc::MAP_LOCKED;
        /// `MAP_NORESERVE`
        #[cfg(not(any(target_os = "freebsd", target_os = "redox")))]
        const NORESERVE = libc::MAP_NORESERVE;
        /// `MAP_POPULATE`
        #[cfg(not(any(
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const POPULATE = libc::MAP_POPULATE;
        /// `MAP_STACK`
        #[cfg(not(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "redox",
        )))]
        const STACK = libc::MAP_STACK;
        /// `MAP_SYNC`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const SYNC = libc::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        #[cfg(any())]
        const UNINITIALIZED = libc::MAP_UNINITIALIZED;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `MLOCK_*` flags for use with [`mlock_with`].
    ///
    /// [`mlock_with`]: crate::io::mlock_with
    pub struct MlockFlags: i32 {
        // libc doesn't define `MLOCK_ONFAULT` yet.
        // /// `MLOCK_ONFAULT`
        // const ONFAULT = libc::MLOCK_ONFAULT;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `MCL_*` flags for use with [`mlockall`].
    ///
    /// [`mlockall`]: crate::io::mlockall
    pub struct MlockallFlags: i32 {
        // libc doesn't define `MCL_ONFAULT` yet.
        // const ONFAULT = libc::MCL_ONFAULT;
        /// Lock all pages which will become mapped into the address
        /// space of the process in the future.  These could be, for
        /// instance, new pages required by a growing heap and stack
        /// as well as new memory-mapped files or shared memory
        /// regions.
        const FUTURE = libc::MCL_FUTURE;
        /// Lock all pages which are currently mapped into the address
        /// space of the process.
        const CURRENT = libc::MCL_CURRENT;
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
        #[cfg(not(any(target_os = "openbsd", target_os = "redox")))]
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
    pub struct UserfaultfdFlags: c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = libc::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = libc::O_NONBLOCK;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// The `EFD_*` flags accepted by [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    pub struct EventfdFlags: std::os::raw::c_int {
        /// `EFD_CLOEXEC`
        const CLOEXEC = libc::EFD_CLOEXEC;
        /// `EFD_NONBLOCK`
        const NONBLOCK = libc::EFD_NONBLOCK;
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = libc::EFD_SEMAPHORE;
    }
}

/// `POSIX_MADV_*` constants for use with [`madvise`].
///
/// Note that there is no `LinuxDontNeed` in the libc configuration because
/// `libc` implementations don't provide a way to access it.
///
/// [`madvise`]: crate::io::madvise
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum Advice {
    /// `POSIX_MADV_NORMAL`
    #[cfg(not(target_os = "android"))]
    Normal = libc::POSIX_MADV_NORMAL,

    /// `POSIX_MADV_NORMAL`
    #[cfg(target_os = "android")]
    Normal = libc::MADV_NORMAL,

    /// `POSIX_MADV_SEQUENTIAL`
    #[cfg(not(target_os = "android"))]
    Sequential = libc::POSIX_MADV_SEQUENTIAL,

    /// `POSIX_MADV_SEQUENTIAL`
    #[cfg(target_os = "android")]
    Sequential = libc::MADV_SEQUENTIAL,

    /// `POSIX_MADV_RANDOM`
    #[cfg(not(target_os = "android"))]
    Random = libc::POSIX_MADV_RANDOM,

    /// `POSIX_MADV_RANDOM`
    #[cfg(target_os = "android")]
    Random = libc::MADV_RANDOM,

    /// `POSIX_MADV_WILLNEED`
    #[cfg(not(target_os = "android"))]
    WillNeed = libc::POSIX_MADV_WILLNEED,

    /// `POSIX_MADV_WILLNEED`
    #[cfg(target_os = "android")]
    WillNeed = libc::MADV_WILLNEED,

    /// `POSIX_MADV_DONTNEED`
    #[cfg(not(any(target_os = "android", target_os = "emscripten")))]
    DontNeed = libc::POSIX_MADV_DONTNEED,

    /// `POSIX_MADV_DONTNEED`
    #[cfg(target_os = "android")]
    DontNeed = libc::MADV_DONTNEED,

    /// `MADV_FREE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxFree = libc::MADV_FREE,
    /// `MADV_REMOVE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxRemove = libc::MADV_REMOVE,
    /// `MADV_DONTFORK`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDontFork = libc::MADV_DONTFORK,
    /// `MADV_DOFORK`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDoFork = libc::MADV_DOFORK,
    /// `MADV_HWPOISON`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxHwPoison = libc::MADV_HWPOISON,
    /// `MADV_SOFT_OFFLINE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxSoftOffline = libc::MADV_SOFT_OFFLINE,
    /// `MADV_MERGEABLE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxMergeable = libc::MADV_MERGEABLE,
    /// `MADV_UNMERGEABLE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxUnmergeable = libc::MADV_UNMERGEABLE,
    /// `MADV_HUGEPAGE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxHugepage = libc::MADV_HUGEPAGE,
    /// `MADV_NOHUGEPAGE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxNoHugepage = libc::MADV_NOHUGEPAGE,
    /// `MADV_DONTDUMP`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDontDump = libc::MADV_DONTDUMP,
    /// `MADV_DODUMP`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDoDump = libc::MADV_DODUMP,
}

#[cfg(target_os = "emscripten")]
impl Advice {
    /// `POSIX_MADV_DONTNEED`
    #[allow(non_upper_case_globals)]
    pub const DontNeed: Self = Self::Normal;
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

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub const PIPE_BUF: usize = libc::PIPE_BUF;
