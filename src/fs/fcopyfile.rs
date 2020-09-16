use crate::{fs::CopyfileFlags, negative_err};
use std::{
    io,
    mem::MaybeUninit,
    os::unix::io::{AsRawFd, RawFd},
};

/// `copyfile_state_t`
#[allow(non_camel_case_types)]
pub type copyfile_state_t = *mut libc::c_void;

/// `copyfile_flags_t`
#[allow(non_camel_case_types)]
type copyfile_flags_t = u32;

/// `fcopyfile(from, to, state, flags)`
#[inline]
pub fn fcopyfile<FromFd: AsRawFd, ToFd: AsRawFd>(
    from: &FromFd,
    to: &ToFd,
    state: copyfile_state_t,
    flags: CopyfileFlags,
) -> io::Result<()> {
    let from = from.as_raw_fd();
    let to = to.as_raw_fd();
    unsafe { _fcopyfile(from, to, state, flags) }
}

unsafe fn _fcopyfile(
    from: RawFd,
    to: RawFd,
    state: copyfile_state_t,
    flags: CopyfileFlags,
) -> io::Result<()> {
    extern "C" {
        fn fcopyfile(
            from: libc::c_int,
            to: libc::c_int,
            state: copyfile_state_t,
            flags: copyfile_flags_t,
        ) -> libc::c_int;
    }

    negative_err(fcopyfile(from, to, state, flags.bits()))
}

/// `copyfile_state_alloc()`
pub fn copyfile_state_alloc() -> copyfile_state_t {
    extern "C" {
        fn copyfile_state_alloc() -> copyfile_state_t;
    }

    unsafe { copyfile_state_alloc() }
}

/// `copyfile_state_free(state)`
pub fn copyfile_state_free(state: copyfile_state_t) -> io::Result<()> {
    extern "C" {
        fn copyfile_state_free(state: copyfile_state_t) -> libc::c_int;
    }

    negative_err(unsafe { copyfile_state_free(state) })
}

const COPYFILE_STATE_COPIED: u32 = 8;

/// `copyfile_state_get(state, COPYFILE_STATE_COPIED)`
pub fn copyfile_state_get_copied(state: copyfile_state_t) -> io::Result<u64> {
    let mut copied = MaybeUninit::<u64>::uninit();
    unsafe { copyfile_state_get(state, COPYFILE_STATE_COPIED, copied.as_mut_ptr() as *mut _) }?;
    Ok(unsafe { copied.assume_init() })
}

/// `copyfile_state_get(state, flags, dst)`
unsafe fn copyfile_state_get(
    state: copyfile_state_t,
    flag: u32,
    dst: *mut libc::c_void,
) -> io::Result<()> {
    extern "C" {
        fn copyfile_state_get(
            state: copyfile_state_t,
            flag: u32,
            dst: *mut libc::c_void,
        ) -> libc::c_int;
    }

    negative_err(copyfile_state_get(state, flag, dst))
}
