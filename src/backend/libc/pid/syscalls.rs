//! libc syscalls for PIDs

use crate::backend::c;
use crate::pid::{Pid, RawNonZeroPid};

#[cfg(not(target_os = "wasi"))]
#[inline]
#[must_use]
pub(crate) fn getpid() -> Pid {
    unsafe {
        let pid = c::getpid();
        debug_assert_ne!(pid, 0);
        Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(pid))
    }
}
