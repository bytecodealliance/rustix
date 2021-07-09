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
        /// `MAP_DENYWRITE`, aka `MAP_DENYWRITE`
        const DENYWRITE = linux_raw_sys::general::MAP_DENYWRITE;
        /// `MAP_FIXED`, aka `MAP_FIXED`
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
    pub struct UserFaultFdFlags: std::os::raw::c_uint {
        /// `O_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::O_CLOEXEC;
        /// `O_NONBLOCK`
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;
    }
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
