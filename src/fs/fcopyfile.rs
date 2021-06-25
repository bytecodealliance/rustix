use crate::{imp, io};
use imp::fs::CopyfileFlags;
use io_lifetimes::AsFd;

/// `copyfile_state_t`
pub use imp::fs::copyfile_state_t;

/// `fcopyfile(from, to, state, flags)`
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
#[inline]
pub fn copyfile_state_alloc() -> io::Result<copyfile_state_t> {
    imp::syscalls::copyfile_state_alloc()
}

/// `copyfile_state_free(state)`
#[inline]
pub unsafe fn copyfile_state_free(state: copyfile_state_t) -> io::Result<()> {
    imp::syscalls::copyfile_state_free(state)
}

/// `copyfile_state_get(state, COPYFILE_STATE_COPIED)`
#[inline]
pub unsafe fn copyfile_state_get_copied(state: copyfile_state_t) -> io::Result<u64> {
    imp::syscalls::copyfile_state_get_copied(state)
}

/// `copyfile_state_get(state, flags, dst)`
#[inline]
pub unsafe fn copyfile_state_get(
    state: copyfile_state_t,
    flag: u32,
    dst: *mut std::os::raw::c_void,
) -> io::Result<()> {
    imp::syscalls::copyfile_state_get(state, flag, dst)
}
