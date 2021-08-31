use bitflags::bitflags;

bitflags! {
    /// `RWF_*` constants for use with [`preadv2`] and [`pwritev2`].
    ///
    /// [`preadv2`]: crate::io::preadv2
    /// [`pwritev2`]: crate::io::pwritev
    pub struct ReadWriteFlags: std::os::raw::c_uint {
        /// `RWF_DSYNC`
        const DSYNC = linux_raw_sys::general::RWF_DSYNC;
        /// `RWF_HIPRI`
        const HIPRI = linux_raw_sys::general::RWF_HIPRI;
        /// `RWF_SYNC`
        const SYNC = linux_raw_sys::general::RWF_SYNC;
        /// `RWF_NOWAIT`
        const NOWAIT = linux_raw_sys::general::RWF_NOWAIT;
        /// `RWF_APPEND`
        const APPEND = linux_raw_sys::general::RWF_APPEND;
    }
}

bitflags! {
    /// `O_*` constants for use with [`dup2`].
    ///
    /// [`dup2`]: crate::io::dup2
    pub struct DupFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
    }
}

bitflags! {
    /// `PROT_*` flags for use with [`mmap`].
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct ProtFlags: u32 {
        /// `PROT_READ`
        const READ = linux_raw_sys::general::PROT_READ;
        /// `PROT_WRITE`
        const WRITE = linux_raw_sys::general::PROT_WRITE;
        /// `PROT_EXEC`
        const EXEC = linux_raw_sys::general::PROT_EXEC;
        /// `PROT_NONE`
        const NONE = linux_raw_sys::general::PROT_NONE;
    }
}

bitflags! {
    /// `MAP_*` flags for use with [`mmap`].
    ///
    /// [`mmap`]: crate::io::mmap
    pub struct MapFlags: u32 {
        /// `MAP_SHARED`
        const SHARED = linux_raw_sys::general::MAP_SHARED;
        /// `MAP_SHARED_VALIDATE`
        const SHARED_VALIDATE = linux_raw_sys::v5_4::general::MAP_SHARED_VALIDATE;
        /// `MAP_PRIVATE`
        const PRIVATE = linux_raw_sys::general::MAP_PRIVATE;
        /// `MAP_ANONYMOUS`, aka `MAP_ANON`
        const ANONYMOUS = linux_raw_sys::general::MAP_ANONYMOUS;
        /// `MAP_DENYWRITE`
        const DENYWRITE = linux_raw_sys::general::MAP_DENYWRITE;
        /// `MAP_FIXED`
        const FIXED = linux_raw_sys::v5_4::general::MAP_FIXED;
        /// `MAP_FIXED_NOREPLACE`
        const FIXED_NOREPLACE = linux_raw_sys::v5_4::general::MAP_FIXED_NOREPLACE;
        /// `MAP_GROWSDOWN`
        const GROWSDOWN = linux_raw_sys::general::MAP_GROWSDOWN;
        /// `MAP_HUGETLB`
        const HUGETLB = linux_raw_sys::general::MAP_HUGETLB;
        /// `MAP_HUGE_2MB`
        const HUGE_2MB = linux_raw_sys::v5_4::general::MAP_HUGE_2MB;
        /// `MAP_HUGE_1GB`
        const HUGE_1GB = linux_raw_sys::v5_4::general::MAP_HUGE_1GB;
        /// `MAP_LOCKED`
        const LOCKED = linux_raw_sys::general::MAP_LOCKED;
        /// `MAP_NORESERVE`
        const NORESERVE = linux_raw_sys::general::MAP_NORESERVE;
        /// `MAP_POPULATE`
        const POPULATE = linux_raw_sys::general::MAP_POPULATE;
        /// `MAP_STACK`
        const STACK = linux_raw_sys::general::MAP_STACK;
        /// `MAP_SYNC`
        const SYNC = linux_raw_sys::v5_4::general::MAP_SYNC;
        /// `MAP_UNINITIALIZED`
        const UNINITIALIZED = linux_raw_sys::v5_4::general::MAP_UNINITIALIZED;
    }
}

bitflags! {
    /// `O_*` constants for use with [`pipe_with`].
    ///
    /// [`pipe_with`]: crate::io::pipe_with
    pub struct PipeFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_DIRECT`
        const DIRECT = linux_raw_sys::general::O_DIRECT;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
}

bitflags! {
    /// The `O_*` flags accepted by [`userfaultfd`].
    ///
    /// [`userfaultfd`]: crate::io::userfaultfd
    pub struct UserfaultfdFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
}

bitflags! {
    /// The `EFD_*` flags accepted by [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    pub struct EventfdFlags: std::os::raw::c_uint {
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

    /// `MADV_FREE`
    LinuxFree = linux_raw_sys::v5_4::general::MADV_FREE,
    /// `MADV_REMOVE`
    LinuxRemove = linux_raw_sys::general::MADV_REMOVE,
    /// `MADV_DONTFORK`
    LinuxDontFork = linux_raw_sys::general::MADV_DONTFORK,
    /// `MADV_DOFORK`
    LinuxDoFork = linux_raw_sys::v5_4::general::MADV_DOFORK,
    /// `MADV_HWPOISON`
    LinuxHwPoison = linux_raw_sys::general::MADV_HWPOISON,
    /// `MADV_SOFT_OFFLINE`
    LinuxSoftOffline = linux_raw_sys::v5_4::general::MADV_SOFT_OFFLINE,
    /// `MADV_MERGEABLE`
    LinuxMergeable = linux_raw_sys::general::MADV_MERGEABLE,
    /// `MADV_UNMERGEABLE`
    LinuxUnmergeable = linux_raw_sys::general::MADV_UNMERGEABLE,
    /// `MADV_HUGEPAGE`
    LinuxHugepage = linux_raw_sys::v5_4::general::MADV_HUGEPAGE,
    /// `MADV_NOHUGEPAGE`
    LinuxNoHugepage = linux_raw_sys::v5_4::general::MADV_NOHUGEPAGE,
    /// `MADV_DONTDUMP`
    LinuxDontDump = linux_raw_sys::v5_4::general::MADV_DONTDUMP,
    /// `MADV_DODUMP`
    LinuxDoDump = linux_raw_sys::v5_4::general::MADV_DODUMP,
    /// `MADV_WIPEONFORK`
    LinuxWipeOnFork = linux_raw_sys::v5_4::general::MADV_WIPEONFORK,
    /// `MADV_KEEPONFORK`
    LinuxKeepOnFork = linux_raw_sys::v5_4::general::MADV_KEEPONFORK,
    /// `MADV_COLD`
    LinuxCold = linux_raw_sys::v5_4::general::MADV_COLD,
    /// `MADV_PAGEOUT`
    LinuxPageOut = linux_raw_sys::v5_4::general::MADV_PAGEOUT,
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

/// `struct termios`, for use with [`ioctl_tcgets`].
///
/// [`ioctl_tcgets`]: crate::io::ioctl_tcgets
pub type Termios = linux_raw_sys::general::termios;

/// `struct winsize`
pub type Winsize = linux_raw_sys::general::winsize;

pub type Tcflag = linux_raw_sys::general::tcflag_t;

pub const ICANON: std::os::raw::c_uint = linux_raw_sys::general::ICANON;

pub const PIPE_BUF: usize = linux_raw_sys::general::PIPE_BUF as usize;
