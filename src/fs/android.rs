//! Work around Bionic (Android's libc) lacking `telldir` and `seekdir`. See
//! [xfstests-bld's telldir.c implementation] ([pinned version]) for details.
//!
//! [xfstests-bld's telldir.c implementation]: https://github.com/tytso/xfstests-bld/blob/master/android-compat/telldir.c
//! [pinned version]: https://github.com/tytso/xfstests-bld/blob/7ca4e52401a5e3376ee285cbb973f84559c12572/android-compat/telldir.c

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
