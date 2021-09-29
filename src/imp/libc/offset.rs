//! Automatically enable "large file" support features.

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
)))]
pub(super) use libc::{
    fstat as libc_fstat, fstatat as libc_fstatat, lseek as libc_lseek, off_t as libc_off_t,
};

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use libc::{
    fstat64 as libc_fstat, fstatat64 as libc_fstatat, lseek64 as libc_lseek, off64_t as libc_off_t,
    rlimit64 as libc_rlimit,
};

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "wasi",
)))]
pub(super) use libc::mmap as libc_mmap;

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "l4re",
    target_os = "redox",
    target_os = "wasi",
)))]
pub(super) use libc::{rlimit as libc_rlimit, RLIM_INFINITY as LIBC_RLIM_INFINITY};

#[cfg(not(any(
    target_os = "android",
    target_os = "fuchsia",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux",
    target_os = "wasi",
)))]
pub(super) use libc::getrlimit as libc_getrlimit;

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
pub(super) use libc::{getrlimit64 as libc_getrlimit, mmap64 as libc_mmap};

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "redox",
)))]
pub(super) use libc::openat as libc_openat;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use libc::openat64 as libc_openat;

#[cfg(target_os = "fuchsia")]
pub(super) use libc::fallocate as libc_fallocate;
#[cfg(any(target_os = "android", target_os = "linux",))]
pub(super) use libc::fallocate64 as libc_fallocate;
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "ios",
    target_os = "linux",
    target_os = "l4re",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
)))]
pub(super) use libc::posix_fadvise as libc_posix_fadvise;
#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    target_os = "l4re",
))]
pub(super) use libc::posix_fadvise64 as libc_posix_fadvise;

#[cfg(all(not(any(target_os = "android", target_os = "linux", target_os = "emscripten"))))]
pub(super) use libc::{pread as libc_pread, pwrite as libc_pwrite};
#[cfg(any(target_os = "android", target_os = "linux", target_os = "emscripten"))]
pub(super) use libc::{
    pread64 as libc_pread, preadv64 as libc_preadv, pwrite64 as libc_pwrite,
    pwritev64 as libc_pwritev,
};
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "redox",
)))]
pub(super) use libc::{preadv as libc_preadv, pwritev as libc_pwritev};
// macOS added preadv and pwritev in version 11.0
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod readwrite_pv {
    weakcall! {
        pub(in crate::imp::libc) fn preadv(
            fd: libc::c_int,
            iov: *const libc::iovec,
            iovcnt: libc::c_int,
            offset: libc::off_t
        ) -> libc::ssize_t
    }
    weakcall! {
        pub(in crate::imp::libc) fn pwritev(
            fd: libc::c_int,
            iov: *const libc::iovec,
            iovcnt: libc::c_int, offset: libc::off_t
        ) -> libc::ssize_t
    }
}
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) use libc::{preadv64v2 as libc_preadv2, pwritev64v2 as libc_pwritev2};
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(super) use readwrite_pv::{preadv as libc_preadv, pwritev as libc_pwritev};

#[cfg(not(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "netbsd",
    target_os = "redox",
    target_os = "wasi",
)))]
pub(super) use libc::fstatfs as libc_fstatfs;
#[cfg(not(any(
    target_os = "android",
    target_os = "fuchsia",
    target_os = "ios",
    target_os = "linux",
    target_os = "l4re",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
)))]
pub(super) use libc::posix_fallocate as libc_posix_fallocate;
#[cfg(any(target_os = "l4re",))]
pub(super) use libc::posix_fallocate64 as libc_posix_fallocate;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "emscripten",
    target_os = "l4re",
))]
pub(super) use libc::fstatfs64 as libc_fstatfs;
