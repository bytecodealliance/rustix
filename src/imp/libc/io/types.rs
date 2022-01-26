use super::super::c;
#[cfg(not(target_os = "wasi"))]
use bitflags::bitflags;

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    pub struct ReadWriteFlags: c::c_int {
        /// `RWF_DSYNC` (since Linux 4.7)
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const DSYNC = c::RWF_DSYNC;
        /// `RWF_HIPRI` (since Linux 4.6)
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const HIPRI = c::RWF_HIPRI;
        /// `RWF_SYNC` (since Linux 4.7)
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const SYNC = c::RWF_SYNC;
        /// `RWF_NOWAIT` (since Linux 4.14)
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const NOWAIT = c::RWF_NOWAIT;
        /// `RWF_APPEND` (since Linux 4.16)
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        const APPEND = c::RWF_APPEND;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    pub struct DupFlags: c::c_int {
        /// `O_CLOEXEC`
        #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos", target_os = "redox")))] // Android 5.0 has dup3, but libc doesn't have bindings
        const CLOEXEC = c::O_CLOEXEC;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `PROT_*` flags for use with [`mmap`].
    ///
    /// For `PROT_NONE`, use `ProtFlags::empty()`.
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct ProtFlags: c::c_int {
        /// `PROT_READ`
        const READ = c::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = c::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = c::PROT_EXEC;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `PROT_*` flags for use with [`mprotect`].
    ///
    /// For `PROT_NONE`, use `MprotectFlags::empty()`.
    ///
    /// [`mprotect`]: crate::io::mprotect
    pub struct MprotectFlags: c::c_int {
        /// `PROT_READ`
        const READ = c::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = c::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = c::PROT_EXEC;
        /// `PROT_GROWSUP`
        #[cfg(any(target_os = "android", target_os = "linux"))]
        const GROWSUP = c::PROT_GROWSUP;
        /// `PROT_GROWSDOWN`
        #[cfg(any(target_os = "android", target_os = "linux"))]
        const GROWSDOWN = c::PROT_GROWSDOWN;
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
    pub struct MapFlags: c::c_int {
        /// `MAP_SHARED`
        const SHARED = c::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        #[cfg(not(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const SHARED_VALIDATE = c::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = c::MAP_PRIVATE;
        /// `MAP_DENYWRITE`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "freebsd",
            target_os = "redox"
        )))]
        const DENYWRITE = c::MAP_DENYWRITE;
        /// `MAP_FIXED`
        #[cfg(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const FIXED = c::MAP_FIXED;
        /// `MAP_FIXED_NOREPLACE`
        #[cfg(not(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const FIXED_NOREPLACE = c::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
            target_os = "ios",
            target_os = "netbsd",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "redox"
        )))]
        const GROWSDOWN = c::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGETLB = c::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        #[cfg(not(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGE_2MB = c::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        #[cfg(not(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const HUGE_1GB = c::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const LOCKED = c::MAP_LOCKED;
        /// `MAP_NORESERVE`
        #[cfg(not(any(target_os = "dragonfly", target_os = "freebsd", target_os = "redox")))]
        const NORESERVE = c::MAP_NORESERVE;
        /// `MAP_POPULATE`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const POPULATE = c::MAP_POPULATE;
        /// `MAP_STACK`
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "redox",
        )))]
        const STACK = c::MAP_STACK;
        /// `MAP_SYNC`
        #[cfg(not(any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "redox",
        )))]
        const SYNC = c::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        #[cfg(any())]
        const UNINITIALIZED = c::MAP_UNINITIALIZED;
    }
}

#[cfg(target_os = "linux")]
bitflags! {
    /// `MREMAP_*` flags for use with [`mremap`].
    ///
    /// For `MREMAP_FIXED`, see [`mremap_fixed`].
    ///
    /// [`mremap`]: crate::io::mremap
    /// [`mremap_fixed`]: crate::io::mremap_fixed
    pub struct MremapFlags: i32 {
        /// `MREMAP_MAYMOVE`
        const MAYMOVE = c::MREMAP_MAYMOVE;
    }
}

#[cfg(not(target_os = "wasi"))]
bitflags! {
    /// `MS_*` flags for use with [`msync`].
    ///
    /// [`msync`]: crate::io::msync
    pub struct MsyncFlags: i32 {
        /// `MS_SYNC` Requests an update and waits for it to complete.
        const SYNC = c::MS_SYNC;
        /// `MS_ASYNC` Specifies that an update be scheduled,
        /// but the call returns immediately.
        const ASYNC = c::MS_ASYNC;
        /// `MS_INVALIDATE` Asks to invalidate other mappings of the same file (so
        /// that they can be updated with the fresh values just written).
        const INVALIDATE = c::MS_INVALIDATE;
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
        // const ONFAULT = c::MLOCK_ONFAULT;
    }
}

#[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "wasi")))]
bitflags! {
    /// `O_*` constants for use with [`pipe_with`].
    ///
    /// [`pipe_with`]: crate::io::pipe_with
    pub struct PipeFlags: c::c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = c::O_CLOEXEC;
        /// `O_DIRECT`
        #[cfg(not(any(target_os = "illumos", target_os = "openbsd", target_os = "redox")))]
        const DIRECT = c::O_DIRECT;
        /// `O_NONBLOCK`
        const NONBLOCK = c::O_NONBLOCK;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// The `O_*` and `UFFD_*` flags accepted by [`userfaultfd`].
    ///
    /// [`userfaultfd`]: crate::io::userfaultfd
    pub struct UserfaultfdFlags: c::c_int {
        /// `O_CLOEXEC`
        const CLOEXEC = c::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = c::O_NONBLOCK;
        /// `UFFD_USER_MODE_ONLY` (since Linux 5.11)
        const USER_MODE_ONLY = userfaultfd_sys::UFFD_USER_MODE_ONLY as _;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// The `UFFD_FEATURE_*` flags for use in [`ioctl_uffdio_api`].
    ///
    /// [`ioctl_uffdio_api`]: crate::io::ioctl_uffdio_api
    pub struct UffdFeatureFlags: u64 {
       /// `UFFD_FEATURE_EVENT_FORK` (since Linux 4.11)
       const EVENT_FORK = userfaultfd_sys::UFFD_FEATURE_EVENT_FORK;
       /// `UFFD_FEATURE_EVENT_REMAP` (since Linux 4.11)
       const EVENT_REMAP = userfaultfd_sys::UFFD_FEATURE_EVENT_REMAP;
       /// `UFFD_FEATURE_EVENT_REMOVE` (since Linux 4.11)
       const EVENT_REMOVE = userfaultfd_sys::UFFD_FEATURE_EVENT_REMOVE;
       /// `UFFD_FEATURE_EVENT_UNMAP` (since Linux 4.11)
       const EVENT_UNMAP = userfaultfd_sys::UFFD_FEATURE_EVENT_UNMAP;
       /// `UFFD_FEATURE_MISSING_HUGETLBFS` (since Linux 4.11)
       const MISSING_HUGETLBFS = userfaultfd_sys::UFFD_FEATURE_MISSING_HUGETLBFS;
       /// `UFFD_FEATURE_MISSING_SHMEM` (since Linux 4.11)
       const MISSING_SHMEM = userfaultfd_sys::UFFD_FEATURE_MISSING_SHMEM;
       /// `UFFD_FEATURE_SIGBUS` (since Linux 4.14)
       const SIGBUS = userfaultfd_sys::UFFD_FEATURE_SIGBUS;
       /// `UFFD_FEATURE_THREAD_ID` (since Linux 4.14)
       const THREAD_ID = userfaultfd_sys::UFFD_FEATURE_THREAD_ID;
       /// `UFFD_FEATURE_PAGEFAULT_FLAG_WP` (since Linux 5.7)
       const PAGEFAULT_FLAG_WP = userfaultfd_sys::UFFD_FEATURE_PAGEFAULT_FLAG_WP;
    }
}

/// The `UFFD_EVENT_*` flags for use in [`UffdMsg`].
///
/// [`UffdMsg`]: crate::io::UffdMsg
#[cfg(any(target_os = "android", target_os = "linux"))]
#[repr(u8)]
pub enum UffdEvent {
    /// `UFFD_EVENT_PAGEFAULT` (since Linux 4.3)
    Pagefault = userfaultfd_sys::UFFD_EVENT_PAGEFAULT as _,
    /// `UFFD_EVENT_FORK` (since Linux 4.11)
    Fork = userfaultfd_sys::UFFD_EVENT_FORK as _,
    /// `UFFD_EVENT_REMAP` (since Linux 4.11)
    Remap = userfaultfd_sys::UFFD_EVENT_REMAP as _,
    /// `UFFD_EVENT_REMOVE` (since Linux 4.11)
    Remove = userfaultfd_sys::UFFD_EVENT_REMOVE as _,
    /// `UFFD_EVENT_UNMAP` (since Linux 4.11)
    Unmap = userfaultfd_sys::UFFD_EVENT_UNMAP as _,
}

#[cfg(any(target_os = "android", target_os = "linux"))]
impl UffdEvent {
    /// Convert a raw uffd event number into a `UffdEvent`, if possible.
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw as _ {
            userfaultfd_sys::UFFD_EVENT_PAGEFAULT => Some(Self::Pagefault),
            userfaultfd_sys::UFFD_EVENT_FORK => Some(Self::Fork),
            userfaultfd_sys::UFFD_EVENT_REMAP => Some(Self::Remap),
            userfaultfd_sys::UFFD_EVENT_REMOVE => Some(Self::Remove),
            userfaultfd_sys::UFFD_EVENT_UNMAP => Some(Self::Unmap),
            _ => None,
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `UFFD_PAGEFAULT_FLAG_*` flags for use in [`UffdMsg`].
    ///
    /// [`UffdMsg`]: crate::io::UffdMsg
    pub struct UffdPagefaultFlags: u64 {
        /// `UFFD_PAGEFAULT_FLAG_WRITE`
        const WRITE = userfaultfd_sys::UFFD_PAGEFAULT_FLAG_WRITE;
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// `UFFDIO_REGISTER_MODE_*` flags for use in [`UffdRegister`].
    ///
    /// [`UffdRegister`]: crate::io::UffdRegister
    pub struct UffdioRegisterModeFlags: u64 {
       /// `UFFDIO_REGISTER_MODE_MISSING`
       const MISSING = userfaultfd_sys::UFFDIO_REGISTER_MODE_MISSING;
       /// `UFFDIO_REGISTER_MODE_WP`
       const WP = userfaultfd_sys::UFFDIO_REGISTER_MODE_WP;
    }
}

bitflags! {
    /// `UFFDIO_COPY_MODE_*` flags for use in [`ioctl_uffdio_copy`].
    ///
    /// [`ioctl_uffdio_copy`]: crate::io::ioctl_uffdio_copy
    pub struct UffdioCopyModeFlags: u64 {
        /// `UFFDIO_COPY_MODE_DONTWAKE`
        const DONTWAKE = userfaultfd_sys::UFFDIO_COPY_MODE_DONTWAKE;
        /// `UFFDIO_COPY_MODE_WP`
        const WP = userfaultfd_sys::UFFDIO_COPY_MODE_WP;
    }
}

bitflags! {
    /// `UFFDIO_ZEROPAGE_MODE_*` flags for use in [`ioctl_uffdio_zeropage`].
    ///
    /// [`ioctl_uffdio_zeropage`]: crate::io::ioctl_uffdio_zeropage
    pub struct UffdioZeropageModeFlags: u64 {
        /// `UFFDIO_ZEROPAGE_MODE_DONTWAKE`
        const DONTWAKE = userfaultfd_sys::UFFDIO_ZEROPAGE_MODE_DONTWAKE;
    }
}

bitflags! {
    /// `_UFFDIO_*` flags for use with [`ioctl_uffdio_register`].
    ///
    /// [`ioctl_uffdio_register`]: crate::io::ioctl_uffdio_register
    pub struct UffdioIoctlFlags: u64 {
        /// `_UFFDIO_REGISTER`
        const REGISTER = 1 << userfaultfd_sys::_UFFDIO_REGISTER;
        /// `_UFFDIO_UNREGISTER`
        const UNREGISTER = 1 << userfaultfd_sys::_UFFDIO_UNREGISTER;
        /// `_UFFDIO_WAKE`
        const WAKE = 1 << userfaultfd_sys::_UFFDIO_WAKE;
        /// `_UFFDIO_COPY`
        const COPY = 1 << userfaultfd_sys::_UFFDIO_COPY;
        /// `_UFFDIO_ZEROPAGE`
        const ZEROPAGE = 1 << userfaultfd_sys::_UFFDIO_ZEROPAGE;
        /// `_UFFDIO_API`
        const API = 1 << userfaultfd_sys::_UFFDIO_API;
    }
}

/// `UFFD_API` for use with [`ioctl_uffdio_api`].
pub const UFFD_API: u64 = userfaultfd_sys::UFFD_API;

#[cfg(any(target_os = "android", target_os = "linux"))]
bitflags! {
    /// The `EFD_*` flags accepted by [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    pub struct EventfdFlags: c::c_int {
        /// `EFD_CLOEXEC`
        const CLOEXEC = c::EFD_CLOEXEC;
        /// `EFD_NONBLOCK`
        const NONBLOCK = c::EFD_NONBLOCK;
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = c::EFD_SEMAPHORE;
    }
}

/// `POSIX_MADV_*` constants for use with [`madvise`].
///
/// [`madvise`]: crate::io::madvise
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum Advice {
    /// `POSIX_MADV_NORMAL`
    #[cfg(not(target_os = "android"))]
    Normal = c::POSIX_MADV_NORMAL,

    /// `POSIX_MADV_NORMAL`
    #[cfg(target_os = "android")]
    Normal = c::MADV_NORMAL,

    /// `POSIX_MADV_SEQUENTIAL`
    #[cfg(not(target_os = "android"))]
    Sequential = c::POSIX_MADV_SEQUENTIAL,

    /// `POSIX_MADV_SEQUENTIAL`
    #[cfg(target_os = "android")]
    Sequential = c::MADV_SEQUENTIAL,

    /// `POSIX_MADV_RANDOM`
    #[cfg(not(target_os = "android"))]
    Random = c::POSIX_MADV_RANDOM,

    /// `POSIX_MADV_RANDOM`
    #[cfg(target_os = "android")]
    Random = c::MADV_RANDOM,

    /// `POSIX_MADV_WILLNEED`
    #[cfg(not(target_os = "android"))]
    WillNeed = c::POSIX_MADV_WILLNEED,

    /// `POSIX_MADV_WILLNEED`
    #[cfg(target_os = "android")]
    WillNeed = c::MADV_WILLNEED,

    /// `POSIX_MADV_DONTNEED`
    #[cfg(not(any(target_os = "android", target_os = "emscripten")))]
    DontNeed = c::POSIX_MADV_DONTNEED,

    /// `POSIX_MADV_DONTNEED`
    #[cfg(target_os = "android")]
    DontNeed = i32::MAX - 1,

    /// `MADV_DONTNEED`
    // `MADV_DONTNEED` has the same value as `POSIX_MADV_DONTNEED`. We don't
    // have a separate `posix_madvise` from `madvise`, so we expose a special
    // value which we special-case.
    #[cfg(target_os = "linux")]
    LinuxDontNeed = i32::MAX,

    /// `MADV_DONTNEED`
    #[cfg(target_os = "android")]
    LinuxDontNeed = c::MADV_DONTNEED,
    /// `MADV_FREE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxFree = c::MADV_FREE,
    /// `MADV_REMOVE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxRemove = c::MADV_REMOVE,
    /// `MADV_DONTFORK`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDontFork = c::MADV_DONTFORK,
    /// `MADV_DOFORK`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDoFork = c::MADV_DOFORK,
    /// `MADV_HWPOISON`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxHwPoison = c::MADV_HWPOISON,
    /// `MADV_SOFT_OFFLINE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxSoftOffline = c::MADV_SOFT_OFFLINE,
    /// `MADV_MERGEABLE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxMergeable = c::MADV_MERGEABLE,
    /// `MADV_UNMERGEABLE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxUnmergeable = c::MADV_UNMERGEABLE,
    /// `MADV_HUGEPAGE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxHugepage = c::MADV_HUGEPAGE,
    /// `MADV_NOHUGEPAGE`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxNoHugepage = c::MADV_NOHUGEPAGE,
    /// `MADV_DONTDUMP`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDontDump = c::MADV_DONTDUMP,
    /// `MADV_DODUMP`
    #[cfg(any(target_os = "android", target_os = "linux"))]
    LinuxDoDump = c::MADV_DODUMP,
}

#[cfg(target_os = "emscripten")]
impl Advice {
    /// `POSIX_MADV_DONTNEED`
    #[allow(non_upper_case_globals)]
    pub const DontNeed: Self = Self::Normal;
}

/// `struct termios` for use with [`ioctl_tcgets`].
///
/// [`ioctl_tcgets`]: crate::io::ioctl_tcgets
#[cfg(not(target_os = "wasi"))]
pub type Termios = c::termios;

/// `struct winsize` for use with [`ioctl_tiocgwinsz`].
///
/// [`ioctl_tiocgwinsz`]: crate::io::ioctl_tiocgwinsz
#[cfg(not(target_os = "wasi"))]
pub type Winsize = c::winsize;

/// `tcflag_t`—A type for the flags fields of [`Termios`].
#[cfg(not(target_os = "wasi"))]
pub type Tcflag = c::tcflag_t;

/// `struct uffd_msg` for use with [`read`] from a [`userfaultfd`] file descriptor.
///
/// [`read`]: crate::io::read
/// [`userfaultfd`]: crate::io::userfaultfd
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdMsg = userfaultfd_sys::uffd_msg;

/// `struct uffd_api` for use with [`ioctl_uffdio_api`].
///
/// [`ioctl_uffdio_api`]: crate::io::ioctl_uffdio_api
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioApi = userfaultfd_sys::uffdio_api;

/// `struct uffd_register` for use with [`ioctl_uffdio_register`].
///
/// [`ioctl_uffdio_register`]: crate::io::ioctl_uffdio_register
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioRegister = userfaultfd_sys::uffdio_register;

/// `struct uffd_range` for use with [`ioctl_uffdio_unregister`] and [`ioctl_uffdio_wake`].
///
/// [`ioctl_uffdio_unregister`]: crate::io::ioctl_uffdio_unregister
/// [`ioctl_uffdio_wake`]: crate::io::ioctl_uffdio_wake
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioRange = userfaultfd_sys::uffdio_range;

/// `struct uffd_copy` for use with [`ioctl_uffdio_copy`].
///
/// [`ioctl_uffdio_copy`]: crate::io::ioctl_uffdio_copy
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioCopy = userfaultfd_sys::uffdio_copy;

/// `struct uffd_zeropage` for use with [`ioctl_uffdio_zeropage`].
///
/// [`ioctl_uffdio_zeropage`]: crate::io::ioctl_uffdio_zeropage
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioZeropage = userfaultfd_sys::uffdio_zeropage;

/// `struct uffd_writeprotect` for use with [`ioctl_uffdio_writeprotect`] (as of Linux 5.7).
///
/// [`ioctl_uffdio_writeprotect`]: crate::io::ioctl_uffdio_writeprotect
#[cfg(any(target_os = "android", target_os = "linux"))]
pub type UffdioWriteprotect = userfaultfd_sys::uffdio_writeprotect;

/// `ICANON`—A flag for the `c_lflag` field of [`Termios`] indicating
/// canonical mode.
#[cfg(not(target_os = "wasi"))]
pub const ICANON: Tcflag = c::ICANON;

/// `PIPE_BUF`—The maximum size of a write to a pipe guaranteed to be atomic.
#[cfg(not(any(target_os = "illumos", target_os = "redox", target_os = "wasi")))]
pub const PIPE_BUF: usize = c::PIPE_BUF;
