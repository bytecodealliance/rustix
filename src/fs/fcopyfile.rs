use crate::{imp, io};
use imp::fd::AsFd;
use imp::fs::CopyfileFlags;

/// `copyfile_state_t`
pub use imp::fs::copyfile_state_t;

/// `fcopyfile(from, to, state, flags)`
///
/// # Safety
///
/// The `state` operand must be allocated with `copyfile_state_alloc` and not
/// yet freed with `copyfile_state_free`.
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html
#[inline]
pub unsafe fn fcopyfile<FromFd: AsFd, ToFd: AsFd>(
    from: &FromFd,
    to: &ToFd,
    state: copyfile_state_t,
    flags: CopyfileFlags,
) -> io::Result<()> {
    let from = from.as_fd();
    let to = to.as_fd();
    imp::syscalls::fcopyfile(from, to, state, flags)
}

/// `copyfile_state_alloc()`
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html
#[inline]
pub fn copyfile_state_alloc() -> io::Result<copyfile_state_t> {
    imp::syscalls::copyfile_state_alloc()
}

/// `copyfile_state_free(state)`
///
/// # Safety
///
/// The `state` operand must be allocated with `copyfile_state_alloc` and not
/// yet freed with `copyfile_state_free`.
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html
#[inline]
pub unsafe fn copyfile_state_free(state: copyfile_state_t) -> io::Result<()> {
    imp::syscalls::copyfile_state_free(state)
}

/// `copyfile_state_get(state, COPYFILE_STATE_COPIED)`
///
/// # Safety
///
/// The `state` operand must be allocated with `copyfile_state_alloc` and not
/// yet freed with `copyfile_state_free`.
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html
#[inline]
pub unsafe fn copyfile_state_get_copied(state: copyfile_state_t) -> io::Result<u64> {
    imp::syscalls::copyfile_state_get_copied(state)
}

/// `copyfile_state_get(state, flags, dst)`
///
/// # Safety
///
/// The `state` operand must be allocated with `copyfile_state_alloc` and not
/// yet freed with `copyfile_state_free`.
///
/// # References
///  - [Apple]
///
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html
#[inline]
pub unsafe fn copyfile_state_get(
    state: copyfile_state_t,
    flag: u32,
    dst: *mut core::ffi::c_void,
) -> io::Result<()> {
    imp::syscalls::copyfile_state_get(state, flag, dst)
}
