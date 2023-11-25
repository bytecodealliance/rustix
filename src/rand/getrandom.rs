#![allow(unsafe_code)]

use crate::{backend, io};
use core::mem::MaybeUninit;
use core::slice;

pub use backend::rand::types::GetRandomFlags;

/// `getrandom(buf, flags)`—Reads a sequence of random bytes.
///
/// This is a very low-level API which may be difficult to use correctly. Most
/// users should prefer to use [`getrandom`] or [`rand`] APIs instead.
///
/// [`getrandom`]: https://crates.io/crates/getrandom
/// [`rand`]: https://crates.io/crates/rand
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/getrandom.2.html
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    unsafe { backend::rand::syscalls::getrandom(buf.as_mut_ptr(), buf.len(), flags) }
}

/// `getrandom(buf, flags)`—Reads a sequence of random bytes.
///
/// This is a very low-level API which may be difficult to use correctly. Most
/// users should prefer to use [`getrandom`] or [`rand`] APIs instead.
///
/// This is identical to [`getn`]
///
/// [`getrandom`]: https://crates.io/crates/getrandom
/// [`rand`]: https://crates.io/crates/rand
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/getrandom.2.html
#[inline]
pub fn getrandom_uninit(
    buf: &mut [MaybeUninit<u8>],
    flags: GetRandomFlags,
) -> io::Result<(&mut [u8], &mut [MaybeUninit<u8>])> {
    // Get number of initialized bytes.
    let length = unsafe {
        backend::rand::syscalls::getrandom(buf.as_mut_ptr() as *mut u8, buf.len(), flags)
    };

    // Split into the initialized and uninitialized portions.
    let (init, uninit) = buf.split_at_mut(length?);
    let init = unsafe { slice::from_raw_parts_mut(init.as_mut_ptr() as *mut u8, init.len()) };

    Ok((init, uninit))
}
