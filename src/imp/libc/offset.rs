//! Automatically enable "large file" support features.

#[cfg(not(windows))]
use super::c;

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
)))]
pub(super) use c::{
    fstat as libc_fstat, fstatat as libc_fstatat, ftruncate as libc_ftruncate, lseek as libc_lseek,
    off_t as libc_off_t,
};

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use c::{
    fstat64 as libc_fstat, fstatat64 as libc_fstatat, ftruncate64 as libc_ftruncate,
    lseek64 as libc_lseek, off64_t as libc_off_t, rlimit64 as libc_rlimit,
};

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "wasi",
)))]
pub(super) use c::mmap as libc_mmap;

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "l4re",
    target_os = "redox",
    target_os = "wasi",
)))]
pub(super) use c::{rlimit as libc_rlimit, RLIM_INFINITY as LIBC_RLIM_INFINITY};

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "fuchsia",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux",
    target_os = "wasi",
)))]
pub(super) use c::{getrlimit as libc_getrlimit, setrlimit as libc_setrlimit};

// TODO: Add `RLIM64_INFINITY` to upstream libc.
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) const LIBC_RLIM_INFINITY: u64 = !0u64;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use c::{
    getrlimit64 as libc_getrlimit, mmap64 as libc_mmap, setrlimit64 as libc_setrlimit,
};

#[cfg(any(target_os = "android", target_os = "linux",))]
pub(super) use c::prlimit64 as libc_prlimit;

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "redox",
)))]
pub(super) use c::openat as libc_openat;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use c::openat64 as libc_openat;

#[cfg(target_os = "fuchsia")]
pub(super) use c::fallocate as libc_fallocate;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub(super) use c::fallocate64 as libc_fallocate;
#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "illumos",
    target_os = "ios",
    target_os = "linux",
    target_os = "l4re",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
)))]
pub(super) use c::posix_fadvise as libc_posix_fadvise;
#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    target_os = "l4re",
))]
pub(super) use c::posix_fadvise64 as libc_posix_fadvise;

#[cfg(all(not(any(
    windows,
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten"
))))]
pub(super) use c::{pread as libc_pread, pwrite as libc_pwrite};
#[cfg(any(target_os = "android", target_os = "linux", target_os = "emscripten"))]
pub(super) use c::{
    pread64 as libc_pread, preadv64 as libc_preadv, pwrite64 as libc_pwrite,
    pwritev64 as libc_pwritev,
};
#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "emscripten",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "redox",
)))]
pub(super) use c::{preadv as libc_preadv, pwritev as libc_pwritev};
// macOS added preadv and pwritev in version 11.0
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod readwrite_pv {
    use super::c;

    weakcall! {
        pub(in super::super) fn preadv(
            fd: c::c_int,
            iov: *const c::iovec,
            iovcnt: c::c_int,
            offset: c::off_t
        ) -> c::ssize_t
    }
    weakcall! {
        pub(in super::super) fn pwritev(
            fd: c::c_int,
            iov: *const c::iovec,
            iovcnt: c::c_int, offset: c::off_t
        ) -> c::ssize_t
    }
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) use c::{preadv64v2 as libc_preadv2, pwritev64v2 as libc_pwritev2};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(super) use readwrite_pv::{preadv as libc_preadv, pwritev as libc_pwritev};

#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "illumos",
    target_os = "ios",
    target_os = "linux",
    target_os = "l4re",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
)))]
pub(super) use c::posix_fallocate as libc_posix_fallocate;
#[cfg(any(target_os = "l4re"))]
pub(super) use c::posix_fallocate64 as libc_posix_fallocate;
#[cfg(not(any(
    windows,
    target_os = "android",
    target_os = "emscripten",
    target_os = "illumos",
    target_os = "linux",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub(super) use {c::fstatfs as libc_fstatfs, c::statfs as libc_statfs};

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use {c::fstatfs64 as libc_fstatfs, c::statfs64 as libc_statfs};
