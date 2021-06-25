use crate::{imp, io};

/// `GRND_*`
pub use imp::rand::GetRandomFlags;

/// `getrandom(buf.as_mut_ptr(), buf.len(), flags)`
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    imp::syscalls::getrandom(buf, flags)
}
