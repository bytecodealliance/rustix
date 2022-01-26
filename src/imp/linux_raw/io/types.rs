use super::super::c;
use bitflags::bitflags;

bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    pub struct ReadWriteFlags: c::c_uint {
        /// `RWF_DSYNC` (since Linux 4.7)
        const DSYNC = linux_raw_sys::general::RWF_DSYNC;
        /// `RWF_HIPRI` (since Linux 4.6)
        const HIPRI = linux_raw_sys::general::RWF_HIPRI;
        /// `RWF_SYNC` (since Linux 4.7)
        const SYNC = linux_raw_sys::general::RWF_SYNC;
        /// `RWF_NOWAIT` (since Linux 4.14)
        const NOWAIT = linux_raw_sys::general::RWF_NOWAIT;
        /// `RWF_APPEND` (since Linux 4.16)
        const APPEND = linux_raw_sys::general::RWF_APPEND;
    }
}

bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    pub struct DupFlags: c::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
    }
}

bitflags! {
    /// `PROT_*` flags for use with [`mmap`].
    ///
    /// For `PROT_NONE`, use `ProtFlags::empty()`.
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct ProtFlags: u32 {
        /// `PROT_READ`
        const READ = linux_raw_sys::general::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = linux_raw_sys::general::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = linux_raw_sys::general::PROT_EXEC;
    }
}

bitflags! {
    /// `PROT_*` flags for use with [`mprotect`].
    ///
    /// For `PROT_NONE`, use `MprotectFlags::empty()`.
    ///
    /// [`mprotect`]: crate::io::mprotect
    pub struct MprotectFlags: u32 {
        /// `PROT_READ`
        const READ = linux_raw_sys::general::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = linux_raw_sys::general::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = linux_raw_sys::general::PROT_EXEC;
        /// `PROT_GROWSUP`
        const GROWSUP = linux_raw_sys::general::PROT_GROWSUP;
        /// `PROT_GROWSDOWN`
        const GROWSDOWN = linux_raw_sys::general::PROT_GROWSDOWN;
    }
}

bitflags! {
    /// `MAP_*` flags for use with [`mmap`].
    ///
    /// For `MAP_ANONYMOUS` (aka `MAP_ANON`), see [`mmap_anonymous`].
    ///
    /// [`mmap`]: crate::io::mmap
    /// [`mmap_anonymous`]: crates::io::mmap_anonymous
    pub struct MapFlags: u32 {
        /// `MAP_SHARED`
        const SHARED = linux_raw_sys::general::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE` (since Linux 4.15)
        const SHARED_VALIDATE = linux_raw_sys::general::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = linux_raw_sys::general::MAP_PRIVATE;
        /// `MAP_DENYWRITE`
        const DENYWRITE = linux_raw_sys::general::MAP_DENYWRITE;
        /// `MAP_FIXED`
        const FIXED = linux_raw_sys::general::MAP_FIXED;
        /// `MAP_FIXED_NOREPLACE` (since Linux 4.17)
        const FIXED_NOREPLACE = linux_raw_sys::general::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        const GROWSDOWN = linux_raw_sys::general::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        const HUGETLB = linux_raw_sys::general::MAP_HUGETLB;
        /// `MAP_HUGE_2MB` (since Linux 3.8)
        const HUGE_2MB = linux_raw_sys::general::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB` (since Linux 3.8)
        const HUGE_1GB = linux_raw_sys::general::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        const LOCKED = linux_raw_sys::general::MAP_LOCKED;
        /// `MAP_NORESERVE`
        const NORESERVE = linux_raw_sys::general::MAP_NORESERVE;
        /// `MAP_POPULATE`
        const POPULATE = linux_raw_sys::general::MAP_POPULATE;
        /// `MAP_STACK`
        const STACK = linux_raw_sys::general::MAP_STACK;
        /// `MAP_SYNC` (since Linux 4.15)
        const SYNC = linux_raw_sys::general::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        const UNINITIALIZED = linux_raw_sys::general::MAP_UNINITIALIZED;
    }
}

bitflags! {
    /// `MREMAP_*` flags for use with [`mremap`].
    ///
    /// For `MREMAP_FIXED`, see [`mremap_fixed`].
    ///
    /// [`mremap`]: crate::io::mremap
    /// [`mremap_fixed`]: crate::io::mremap_fixed
    pub struct MremapFlags: u32 {
        /// `MREMAP_MAYMOVE`
        const MAYMOVE = linux_raw_sys::general::MREMAP_MAYMOVE;
        /// `MREMAP_DONTUNMAP` (since Linux 5.7)
        const DONTUNMAP = linux_raw_sys::general::MREMAP_DONTUNMAP;
    }
}

bitflags! {
    /// `MLOCK_*` flags for use with [`mlock_with`].
    ///
    /// [`mlock_with`]: crate::io::mlock_with
    pub struct MlockFlags: u32 {
        /// `MLOCK_ONFAULT`
        const ONFAULT = linux_raw_sys::general::MLOCK_ONFAULT;
    }
}

bitflags! {
    /// `MS_*` flags for use with [`msync`].
    ///
    /// [`msync`]: crate::io::msync
    pub struct MsyncFlags: u32 {
        /// `MS_SYNC` Requests an update and waits for it to complete.
        const SYNC = linux_raw_sys::general::MS_SYNC;
        /// `MS_ASYNC` Specifies that an update be scheduled,
        /// but the call returns immediately.
        const ASYNC = linux_raw_sys::general::MS_ASYNC;
        /// `MS_INVALIDATE` Asks to invalidate other mappings of the same file (so
        /// that they can be updated with the fresh values just written).
        const INVALIDATE = linux_raw_sys::general::MS_INVALIDATE;
    }
}

bitflags! {
    /// `O_*` constants for use with [`pipe_with`].
    ///
    /// [`pipe_with`]: crate::io::pipe_with
    pub struct PipeFlags: c::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_DIRECT`
        const DIRECT = linux_raw_sys::general::O_DIRECT;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
}

bitflags! {
    /// The `O_*` and `UFFD_*` flags accepted by [`userfaultfd`].
    ///
    /// [`userfaultfd`]: crate::io::userfaultfd
    pub struct UserfaultfdFlags: c::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
        /// `UFFD_USER_MODE_ONLY` (since Linux 5.11)
        const USER_MODE_ONLY = linux_raw_sys::general::UFFD_USER_MODE_ONLY;
    }
}

bitflags! {
    /// The `UFFD_FEATURE_*` flags for use in [`ioctl_uffdio_api`].
    ///
    /// [`ioctl_uffdio_api`]: crate::io::ioctl_uffdio_api
    pub struct UffdFeatureFlags: u64 {
       /// `UFFD_FEATURE_EVENT_FORK` (since Linux 4.11)
       const EVENT_FORK = linux_raw_sys::general::UFFD_FEATURE_EVENT_FORK as _;
       /// `UFFD_FEATURE_EVENT_REMAP` (since Linux 4.11)
       const EVENT_REMAP = linux_raw_sys::general::UFFD_FEATURE_EVENT_REMAP as _;
       /// `UFFD_FEATURE_EVENT_REMOVE` (since Linux 4.11)
       const EVENT_REMOVE = linux_raw_sys::general::UFFD_FEATURE_EVENT_REMOVE as _;
       /// `UFFD_FEATURE_EVENT_UNMAP` (since Linux 4.11)
       const EVENT_UNMAP = linux_raw_sys::general::UFFD_FEATURE_EVENT_UNMAP as _;
       /// `UFFD_FEATURE_MISSING_HUGETLBFS` (since Linux 4.11)
       const MISSING_HUGETLBFS = linux_raw_sys::general::UFFD_FEATURE_MISSING_HUGETLBFS as _;
       /// `UFFD_FEATURE_MISSING_SHMEM` (since Linux 4.11)
       const MISSING_SHMEM = linux_raw_sys::general::UFFD_FEATURE_MISSING_SHMEM as _;
       /// `UFFD_FEATURE_SIGBUS` (since Linux 4.14)
       const SIGBUS = linux_raw_sys::general::UFFD_FEATURE_SIGBUS as _;
       /// `UFFD_FEATURE_THREAD_ID` (since Linux 4.14)
       const THREAD_ID = linux_raw_sys::general::UFFD_FEATURE_THREAD_ID as _;
       /// `UFFD_FEATURE_PAGEFAULT_FLAG_WP` (since Linux 5.7)
       const PAGEFAULT_FLAG_WP = linux_raw_sys::general::UFFD_FEATURE_PAGEFAULT_FLAG_WP as _;
    }
}

/// The `UFFD_EVENT_*` flags for use in [`UffdMsg`].
///
/// [`UffdMsg`]: crate::io::UffdMsg
#[repr(u8)]
pub enum UffdEvent {
    /// `UFFD_EVENT_PAGEFAULT` (since Linux 4.3)
    Pagefault = linux_raw_sys::general::UFFD_EVENT_PAGEFAULT as _,
    /// `UFFD_EVENT_FORK` (since Linux 4.11)
    Fork = linux_raw_sys::general::UFFD_EVENT_FORK as _,
    /// `UFFD_EVENT_REMAP` (since Linux 4.11)
    Remap = linux_raw_sys::general::UFFD_EVENT_REMAP as _,
    /// `UFFD_EVENT_REMOVE` (since Linux 4.11)
    Remove = linux_raw_sys::general::UFFD_EVENT_REMOVE as _,
    /// `UFFD_EVENT_UNMAP` (since Linux 4.11)
    Unmap = linux_raw_sys::general::UFFD_EVENT_UNMAP as _,
}

impl UffdEvent {
    /// Convert a raw uffd event number into a `UffdEvent`, if possible.
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw as _ {
            linux_raw_sys::general::UFFD_EVENT_PAGEFAULT => Some(Self::Pagefault),
            linux_raw_sys::general::UFFD_EVENT_FORK => Some(Self::Fork),
            linux_raw_sys::general::UFFD_EVENT_REMAP => Some(Self::Remap),
            linux_raw_sys::general::UFFD_EVENT_REMOVE => Some(Self::Remove),
            linux_raw_sys::general::UFFD_EVENT_UNMAP => Some(Self::Unmap),
            _ => None,
        }
    }
}

bitflags! {
    /// `UFFD_PAGEFAULT_FLAG_*` flags for use in [`UffdMsg`].
    ///
    /// [`UffdMsg`]: crate::io::UffdMsg
    pub struct UffdPagefaultFlags: u64 {
        /// `UFFD_PAGEFAULT_FLAG_WRITE`
        const WRITE = linux_raw_sys::general::UFFD_PAGEFAULT_FLAG_WRITE as _;
    }
}

bitflags! {
    /// `UFFDIO_REGISTER_MODE_*` flags for use in [`UffdRegister`].
    ///
    /// [`UffdRegister`]: crate::io::UffdRegister
    pub struct UffdioRegisterModeFlags: u64 {
       /// `UFFDIO_REGISTER_MODE_MISSING`
       const MISSING = linux_raw_sys::general::UFFDIO_REGISTER_MODE_MISSING as _;
       /// `UFFDIO_REGISTER_MODE_WP`
       const WP = linux_raw_sys::general::UFFDIO_REGISTER_MODE_WP as _;
    }
}

bitflags! {
    /// `UFFDIO_COPY_MODE_*` flags for use in [`ioctl_uffdio_copy`].
    ///
    /// [`ioctl_uffdio_copy`]: crate::io::ioctl_uffdio_copy
    pub struct UffdioCopyModeFlags: u64 {
        /// `UFFDIO_COPY_MODE_DONTWAKE`
        const DONTWAKE = linux_raw_sys::general::UFFDIO_COPY_MODE_DONTWAKE as _;
        /// `UFFDIO_COPY_MODE_WP`
        const WP = linux_raw_sys::general::UFFDIO_COPY_MODE_WP as _;
    }
}

bitflags! {
    /// `UFFDIO_ZEROPAGE_MODE_*` flags for use in [`ioctl_uffdio_zeropage`].
    ///
    /// [`ioctl_uffdio_zeropage`]: crate::io::ioctl_uffdio_zeropage
    pub struct UffdioZeropageModeFlags: u64 {
        /// `UFFDIO_ZEROPAGE_MODE_DONTWAKE`
        const DONTWAKE = linux_raw_sys::general::UFFDIO_ZEROPAGE_MODE_DONTWAKE as _;
    }
}

bitflags! {
    /// `_UFFDIO_*` flags for use with [`ioctl_uffdio_register`].
    ///
    /// [`ioctl_uffdio_register`]: crate::io::ioctl_uffdio_register
    pub struct UffdioIoctlFlags: u64 {
        /// `_UFFDIO_REGISTER`
        const REGISTER = 1 << linux_raw_sys::general::_UFFDIO_REGISTER;
        /// `_UFFDIO_UNREGISTER`
        const UNREGISTER = 1 << linux_raw_sys::general::_UFFDIO_UNREGISTER;
        /// `_UFFDIO_WAKE`
        const WAKE = 1 << linux_raw_sys::general::_UFFDIO_WAKE;
        /// `_UFFDIO_COPY`
        const COPY = 1 << linux_raw_sys::general::_UFFDIO_COPY;
        /// `_UFFDIO_ZEROPAGE`
        const ZEROPAGE = 1 << linux_raw_sys::general::_UFFDIO_ZEROPAGE;
        /// `_UFFDIO_API`
        const API = 1 << linux_raw_sys::general::_UFFDIO_API;
    }
}

/// `UFFD_API` for use with [`ioctl_uffdio_api`].
pub const UFFD_API: u64 = linux_raw_sys::general::UFFD_API as _;

bitflags! {
    /// The `EFD_*` flags accepted by [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    pub struct EventfdFlags: c::c_uint {
        /// `EFD_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::EFD_CLOEXEC;
        /// `EFD_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::EFD_NONBLOCK;
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = linux_raw_sys::general::EFD_SEMAPHORE;
    }
}

/// `POSIX_MADV_*` constants for use with [`madvise`].
///
/// [`madvise`]: crate::io::madvise
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Advice {
    /// `POSIX_MADV_NORMAL`
    Normal = linux_raw_sys::general::MADV_NORMAL,

    /// `POSIX_MADV_SEQUENTIAL`
    Sequential = linux_raw_sys::general::MADV_SEQUENTIAL,

    /// `POSIX_MADV_RANDOM`
    Random = linux_raw_sys::general::MADV_RANDOM,

    /// `POSIX_MADV_WILLNEED`
    WillNeed = linux_raw_sys::general::MADV_WILLNEED,

    /// `MADV_DONTNEED`
    LinuxDontNeed = linux_raw_sys::general::MADV_DONTNEED,

    /// `MADV_FREE` (since Linux 4.5)
    LinuxFree = linux_raw_sys::general::MADV_FREE,
    /// `MADV_REMOVE`
    LinuxRemove = linux_raw_sys::general::MADV_REMOVE,
    /// `MADV_DONTFORK`
    LinuxDontFork = linux_raw_sys::general::MADV_DONTFORK,
    /// `MADV_DOFORK`
    LinuxDoFork = linux_raw_sys::general::MADV_DOFORK,
    /// `MADV_HWPOISON`
    LinuxHwPoison = linux_raw_sys::general::MADV_HWPOISON,
    /// `MADV_SOFT_OFFLINE`
    LinuxSoftOffline = linux_raw_sys::general::MADV_SOFT_OFFLINE,
    /// `MADV_MERGEABLE`
    LinuxMergeable = linux_raw_sys::general::MADV_MERGEABLE,
    /// `MADV_UNMERGEABLE`
    LinuxUnmergeable = linux_raw_sys::general::MADV_UNMERGEABLE,
    /// `MADV_HUGEPAGE` (since Linux 2.6.38)
    LinuxHugepage = linux_raw_sys::general::MADV_HUGEPAGE,
    /// `MADV_NOHUGEPAGE` (since Linux 2.6.38)
    LinuxNoHugepage = linux_raw_sys::general::MADV_NOHUGEPAGE,
    /// `MADV_DONTDUMP` (since Linux 3.4)
    LinuxDontDump = linux_raw_sys::general::MADV_DONTDUMP,
    /// `MADV_DODUMP` (since Linux 3.4)
    LinuxDoDump = linux_raw_sys::general::MADV_DODUMP,
    /// `MADV_WIPEONFORK` (since Linux 4.14)
    LinuxWipeOnFork = linux_raw_sys::general::MADV_WIPEONFORK,
    /// `MADV_KEEPONFORK` (since Linux 4.14)
    LinuxKeepOnFork = linux_raw_sys::general::MADV_KEEPONFORK,
    /// `MADV_COLD` (since Linux 5.4)
    LinuxCold = linux_raw_sys::general::MADV_COLD,
    /// `MADV_PAGEOUT` (since Linux 5.4)
    LinuxPageOut = linux_raw_sys::general::MADV_PAGEOUT,
}

impl Advice {
    /// `POSIX_MADV_DONTNEED`
    ///
    /// On Linux, this is mapped to `POSIX_MADV_NORMAL` because
    /// Linux's `MADV_DONTNEED` differs from `POSIX_MADV_DONTNEED`. See
    /// `LinuxDontNeed` for the Linux behavior.
    #[allow(non_upper_case_globals)]
    pub const DontNeed: Self = Self::Normal;
}

/// `struct termios` for use with [`ioctl_tcgets`].
///
/// [`ioctl_tcgets`]: crate::io::ioctl_tcgets
pub type Termios = linux_raw_sys::general::termios;

/// `struct winsize` for use with [`ioctl_tiocgwinsz`].
///
/// [`ioctl_tiocgwinsz`]: crate::io::ioctl_tiocgwinsz
pub type Winsize = linux_raw_sys::general::winsize;

/// `tcflag_t`—A type for the flags fields of [`Termios`].
pub type Tcflag = linux_raw_sys::general::tcflag_t;

/// `struct uffd_msg` for use with [`read`] from a [`userfaultfd`] file descriptor.
///
/// [`read`]: crate::io::read
/// [`userfaultfd`]: crate::io::userfaultfd
pub type UffdMsg = linux_raw_sys::general::uffd_msg;

/// `struct uffd_api` for use with [`ioctl_uffdio_api`].
///
/// [`ioctl_uffdio_api`]: crate::io::ioctl_uffdio_api
pub type UffdioApi = linux_raw_sys::general::uffdio_api;

/// `struct uffd_register` for use with [`ioctl_uffdio_register`].
///
/// [`ioctl_uffdio_register`]: crate::io::ioctl_uffdio_register
pub type UffdioRegister = linux_raw_sys::general::uffdio_register;

/// `struct uffd_range` for use with [`ioctl_uffdio_unregister`] and [`ioctl_uffdio_wake`].
///
/// [`ioctl_uffdio_unregister`]: crate::io::ioctl_uffdio_unregister
/// [`ioctl_uffdio_wake`]: crate::io::ioctl_uffdio_wake
pub type UffdioRange = linux_raw_sys::general::uffdio_range;

/// `struct uffd_copy` for use with [`ioctl_uffdio_copy`].
///
/// [`ioctl_uffdio_copy`]: crate::io::ioctl_uffdio_copy
pub type UffdioCopy = linux_raw_sys::general::uffdio_copy;

/// `struct uffd_zeropage` for use with [`ioctl_uffdio_zeropage`].
///
/// [`ioctl_uffdio_zeropage`]: crate::io::ioctl_uffdio_zeropage
pub type UffdioZeropage = linux_raw_sys::general::uffdio_zeropage;

/// `struct uffd_writeprotect` for use with [`ioctl_uffdio_writeprotect`] (as of Linux 5.7).
///
/// [`ioctl_uffdio_writeprotect`]: crate::io::ioctl_uffdio_writeprotect
pub type UffdioWriteprotect = linux_raw_sys::general::uffdio_writeprotect;

/// `ICANON`—A flag for the `c_lflag` field of [`Termios`] indicating
/// canonical mode.
pub const ICANON: c::c_uint = linux_raw_sys::general::ICANON;

/// `PIPE_BUF`—The maximum size of a write to a pipe guaranteed to be atomic.
pub const PIPE_BUF: usize = linux_raw_sys::general::PIPE_BUF as usize;
