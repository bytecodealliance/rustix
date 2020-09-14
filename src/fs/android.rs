//! Work around Bionic (Android's libc) lacking `telldir` and `seekdir`. See
//!
//! https://kernel.googlesource.com/pub/scm/fs/ext2/xfstests-bld/+/HEAD/android-compat/telldir.c
//! https://kernel.googlesource.com/pub/scm/fs/ext2/xfstests-bld/+/7ca4e52401a5e3376ee285cbb973f84559c12572/android-compat/telldir.c
//!
//! for details.

use crate::zero_ok;

#[repr(C)]
struct DIR_INTERNALS {
    fd: libc::c_int,
}

pub(crate) unsafe fn telldir(dir: *mut libc::DIR) -> libc::c_long {
    let dir = &*(dir as *mut DIR_INTERNALS);
    libc::lseek(dir.fd, 0, libc::SEEK_CUR)
}

pub(crate) unsafe fn seekdir(dir: *mut libc::DIR, loc: libc::c_long) {
    let dir = &*(dir as *mut DIR_INTERNALS);
    zero_ok(libc::lseek(dir.fd, loc, libc::SEEK_SET)).unwrap()
}
