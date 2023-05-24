#![allow(unused_imports)]

// Import everything from libc, but we'll add some stuff and override some
// things below.
pub(crate) use libc::*;

/// `PROC_SUPER_MAGIC`—The magic number for the procfs filesystem.
#[cfg(all(linux_kernel, target_env = "musl"))]
pub(crate) const PROC_SUPER_MAGIC: u32 = 0x0000_9fa0;

/// `NFS_SUPER_MAGIC`—The magic number for the NFS filesystem.
#[cfg(all(linux_kernel, target_env = "musl"))]
pub(crate) const NFS_SUPER_MAGIC: u32 = 0x0000_6969;

#[cfg(all(
    linux_kernel,
    any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "sparc",
        target_arch = "sparc64"
    )
))]
pub(crate) const SIGEMT: c_int = linux_raw_sys::general::SIGEMT as _;

// Automatically enable “large file” support (LFS) features.

#[cfg(target_os = "vxworks")]
pub(super) use libc::_Vx_ticks64_t as _Vx_ticks_t;
#[cfg(target_os = "aix")]
pub(super) use libc::blksize64_t as blksize_t;
#[cfg(linux_kernel)]
pub(super) use libc::fallocate64 as fallocate;
#[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
#[cfg(any(linux_like, target_os = "aix"))]
pub(super) use libc::open64 as open;
#[cfg(any(linux_kernel, target_os = "aix", target_os = "l4re"))]
pub(super) use libc::posix_fallocate64 as posix_fallocate;
#[cfg(any(all(linux_like, not(target_os = "android")), target_os = "aix"))]
pub(super) use libc::{blkcnt64_t as blkcnt_t, rlim64_t as rlim_t};
#[cfg(any(linux_like, target_os = "aix"))]
pub(super) use libc::{
    fstat64 as fstat, fstatat64 as fstatat, fstatfs64 as fstatfs, fstatvfs64 as fstatvfs,
    ftruncate64 as ftruncate, getrlimit64 as getrlimit, ino64_t as ino_t, lseek64 as lseek,
    mmap64 as mmap, off64_t as off_t, openat64 as openat, posix_fadvise64 as posix_fadvise,
    rlimit64 as rlimit, setrlimit64 as setrlimit, statfs64 as statfs, statvfs64 as statvfs,
    RLIM64_INFINITY as RLIM_INFINITY,
};
#[cfg(apple)]
pub(super) use libc::{
    host_info64_t as host_info_t, host_statistics64 as host_statistics,
    vm_statistics64_t as vm_statistics_t,
};
#[cfg(not(all(linux_kernel, any(target_pointer_width = "32", target_arch = "mips64"))))]
#[cfg(any(linux_like, target_os = "aix"))]
pub(super) use libc::{lstat64 as lstat, stat64 as stat};
#[cfg(any(linux_kernel, target_os = "aix", target_os = "emscripten"))]
pub(super) use libc::{pread64 as pread, pwrite64 as pwrite};
#[cfg(any(target_os = "aix", target_os = "linux", target_os = "emscripten"))]
pub(super) use libc::{preadv64 as preadv, pwritev64 as pwritev};

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) unsafe fn prlimit(
    pid: libc::pid_t,
    resource: libc::__rlimit_resource_t,
    new_limit: *const libc::rlimit64,
    old_limit: *mut libc::rlimit64,
) -> libc::c_int {
    // `prlimit64` wasn't supported in glibc until 2.13.
    weak_or_syscall! {
        fn prlimit64(
            pid: libc::pid_t,
            resource: libc::__rlimit_resource_t,
            new_limit: *const libc::rlimit64,
            old_limit: *mut libc::rlimit64
        ) via SYS_prlimit64 -> libc::c_int
    }

    prlimit64(pid, resource, new_limit, old_limit)
}

#[cfg(all(target_os = "linux", target_env = "musl"))]
pub(super) unsafe fn prlimit(
    pid: libc::pid_t,
    resource: libc::c_int,
    new_limit: *const libc::rlimit64,
    old_limit: *mut libc::rlimit64,
) -> libc::c_int {
    weak_or_syscall! {
        fn prlimit64(
            pid: libc::pid_t,
            resource: libc::c_int,
            new_limit: *const libc::rlimit64,
            old_limit: *mut libc::rlimit64
        ) via SYS_prlimit64 -> libc::c_int
    }

    prlimit64(pid, resource, new_limit, old_limit)
}

#[cfg(target_os = "android")]
pub(super) unsafe fn prlimit(
    pid: libc::pid_t,
    resource: libc::c_int,
    new_limit: *const libc::rlimit64,
    old_limit: *mut libc::rlimit64,
) -> libc::c_int {
    weak_or_syscall! {
        fn prlimit64(
            pid: libc::pid_t,
            resource: libc::c_int,
            new_limit: *const libc::rlimit64,
            old_limit: *mut libc::rlimit64
        ) via SYS_prlimit64 -> libc::c_int
    }

    prlimit64(pid, resource, new_limit, old_limit)
}

#[cfg(target_os = "android")]
mod readwrite_pv64 {
    // 64-bit offsets on 32-bit platforms are passed in endianness-specific
    // lo/hi pairs. See src/backend/linux_raw/conv.rs for details.
    #[cfg(all(target_endian = "little", target_pointer_width = "32"))]
    fn lo(x: u64) -> usize {
        (x >> 32) as usize
    }
    #[cfg(all(target_endian = "little", target_pointer_width = "32"))]
    fn hi(x: u64) -> usize {
        (x & 0xffff_ffff) as usize
    }
    #[cfg(all(target_endian = "big", target_pointer_width = "32"))]
    fn lo(x: u64) -> usize {
        (x & 0xffff_ffff) as usize
    }
    #[cfg(all(target_endian = "big", target_pointer_width = "32"))]
    fn hi(x: u64) -> usize {
        (x >> 32) as usize
    }

    pub(in super::super) unsafe fn preadv64(
        fd: libc::c_int,
        iov: *const libc::iovec,
        iovcnt: libc::c_int,
        offset: libc::off64_t,
    ) -> libc::ssize_t {
        // Older Android libc lacks `preadv64`, so use the `weak!` mechanism to
        // test for it, and call back to `libc::syscall`. We don't use
        // `weak_or_syscall` here because we need to pass the 64-bit offset
        // specially.
        weak! {
            fn preadv64(libc::c_int, *const libc::iovec, libc::c_int, libc::off64_t) -> libc::ssize_t
        }
        if let Some(fun) = preadv64.get() {
            fun(fd, iov, iovcnt, offset)
        } else {
            #[cfg(target_pointer_width = "32")]
            {
                libc::syscall(
                    libc::SYS_preadv,
                    fd,
                    iov,
                    iovcnt,
                    hi(offset as u64),
                    lo(offset as u64),
                ) as libc::ssize_t
            }
            #[cfg(target_pointer_width = "64")]
            {
                libc::syscall(libc::SYS_preadv, fd, iov, iovcnt, offset) as libc::ssize_t
            }
        }
    }
    pub(in super::super) unsafe fn pwritev64(
        fd: libc::c_int,
        iov: *const libc::iovec,
        iovcnt: libc::c_int,
        offset: libc::off64_t,
    ) -> libc::ssize_t {
        // See the comments in `preadv64`.
        weak! {
            fn pwritev64(libc::c_int, *const libc::iovec, libc::c_int, libc::off64_t) -> libc::ssize_t
        }
        if let Some(fun) = pwritev64.get() {
            fun(fd, iov, iovcnt, offset)
        } else {
            #[cfg(target_pointer_width = "32")]
            {
                libc::syscall(
                    libc::SYS_pwritev,
                    fd,
                    iov,
                    iovcnt,
                    hi(offset as u64),
                    lo(offset as u64),
                ) as libc::ssize_t
            }
            #[cfg(target_pointer_width = "64")]
            {
                libc::syscall(libc::SYS_pwritev, fd, iov, iovcnt, offset) as libc::ssize_t
            }
        }
    }
}
#[cfg(target_os = "android")]
pub(super) use readwrite_pv64::{preadv64 as preadv, pwritev64 as pwritev};

// macOS added preadv and pwritev in version 11.0
#[cfg(apple)]
mod readwrite_pv {
    weakcall! {
        pub(in super::super) fn preadv(
            fd: libc::c_int,
            iov: *const libc::iovec,
            iovcnt: libc::c_int,
            offset: libc::off_t
        ) -> libc::ssize_t
    }
    weakcall! {
        pub(in super::super) fn pwritev(
            fd: libc::c_int,
            iov: *const libc::iovec,
            iovcnt: libc::c_int, offset: libc::off_t
        ) -> libc::ssize_t
    }
}
#[cfg(apple)]
pub(super) use readwrite_pv::{preadv, pwritev};

// glibc added `preadv64v2` and `pwritev64v2` in version 2.26.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod readwrite_pv64v2 {
    // 64-bit offsets on 32-bit platforms are passed in endianness-specific
    // lo/hi pairs. See src/backend/linux_raw/conv.rs for details.
    #[cfg(all(target_endian = "little", target_pointer_width = "32"))]
    fn lo(x: u64) -> usize {
        (x >> 32) as usize
    }
    #[cfg(all(target_endian = "little", target_pointer_width = "32"))]
    fn hi(x: u64) -> usize {
        (x & 0xffff_ffff) as usize
    }
    #[cfg(all(target_endian = "big", target_pointer_width = "32"))]
    fn lo(x: u64) -> usize {
        (x & 0xffff_ffff) as usize
    }
    #[cfg(all(target_endian = "big", target_pointer_width = "32"))]
    fn hi(x: u64) -> usize {
        (x >> 32) as usize
    }

    pub(in super::super) unsafe fn preadv64v2(
        fd: libc::c_int,
        iov: *const libc::iovec,
        iovcnt: libc::c_int,
        offset: libc::off64_t,
        flags: libc::c_int,
    ) -> libc::ssize_t {
        // Older glibc lacks `preadv64v2`, so use the `weak!` mechanism to
        // test for it, and call back to `libc::syscall`. We don't use
        // `weak_or_syscall` here because we need to pass the 64-bit offset
        // specially.
        weak! {
            fn preadv64v2(libc::c_int, *const libc::iovec, libc::c_int, libc::off64_t, libc::c_int) -> libc::ssize_t
        }
        if let Some(fun) = preadv64v2.get() {
            fun(fd, iov, iovcnt, offset, flags)
        } else {
            #[cfg(target_pointer_width = "32")]
            {
                libc::syscall(
                    libc::SYS_preadv,
                    fd,
                    iov,
                    iovcnt,
                    hi(offset as u64),
                    lo(offset as u64),
                    flags,
                ) as libc::ssize_t
            }
            #[cfg(target_pointer_width = "64")]
            {
                libc::syscall(libc::SYS_preadv2, fd, iov, iovcnt, offset, flags) as libc::ssize_t
            }
        }
    }
    pub(in super::super) unsafe fn pwritev64v2(
        fd: libc::c_int,
        iov: *const libc::iovec,
        iovcnt: libc::c_int,
        offset: libc::off64_t,
        flags: libc::c_int,
    ) -> libc::ssize_t {
        // See the comments in `preadv64v2`.
        weak! {
            fn pwritev64v2(libc::c_int, *const libc::iovec, libc::c_int, libc::off64_t, libc::c_int) -> libc::ssize_t
        }
        if let Some(fun) = pwritev64v2.get() {
            fun(fd, iov, iovcnt, offset, flags)
        } else {
            #[cfg(target_pointer_width = "32")]
            {
                libc::syscall(
                    libc::SYS_pwritev,
                    fd,
                    iov,
                    iovcnt,
                    hi(offset as u64),
                    lo(offset as u64),
                    flags,
                ) as libc::ssize_t
            }
            #[cfg(target_pointer_width = "64")]
            {
                libc::syscall(libc::SYS_pwritev2, fd, iov, iovcnt, offset, flags) as libc::ssize_t
            }
        }
    }
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) use readwrite_pv64v2::{preadv64v2 as preadv2, pwritev64v2 as pwritev2};
