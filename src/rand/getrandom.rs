#![allow(unsafe_code)]

use crate::{backend, buffer, io};
use buffer::{Buffer, with_buffer};

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
pub fn getrandom<Buf: Buffer<u8>>(buf: Buf, flags: GetRandomFlags) -> io::Result<Buf::Result> {
    unsafe {
        with_buffer(buf, |ptr, cap| backend::rand::syscalls::getrandom(ptr, cap, flags))
    }
}
