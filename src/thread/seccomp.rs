use core::marker::PhantomData;
use core::ptr::null_mut;

use bitflags::bitflags;

use crate::backend::c;
use crate::backend::thread::syscalls;
use crate::backend::thread::types::SeccompOperation;
use crate::io;

bitflags! {
    /// fixme
    pub struct SetSecureComputingFilterFlags: u32 {
        /// fixme
        const TSYNC = c::SECCOMP_FILTER_FLAG_TSYNC;
        /// fixme
        const LOG = c::SECCOMP_FILTER_FLAG_LOG;
        /// fixme
        const SPEC_ALLOW = c::SECCOMP_FILTER_FLAG_SPEC_ALLOW;
        /// fixme
        const NEW_LISTENER = c::SECCOMP_FILTER_FLAG_NEW_LISTENER;
        /// fixme
        const TSYNC_ESRCH = c::SECCOMP_FILTER_FLAG_TSYNC_ESRCH;
        /// fixme
        const WAIT_KILLABLE_RECV = c::SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV;

        const _ = !0;
    }
}

/// fixme
#[derive(Debug)]
#[repr(transparent)]
pub struct SecureComputingFilterLine(c::sock_filter);

/// fixme
#[derive(Debug)]
#[repr(transparent)]
pub struct SecureComputingFilter<'a>(c::sock_fprog, PhantomData<&'a [SecureComputingFilterLine]>);
impl<'a> From<&'a [SecureComputingFilterLine]> for SecureComputingFilter<'a> {
    fn from(filter_lines: &'a [SecureComputingFilterLine]) -> Self {
        Self(
            c::sock_fprog {
                // usize as u16 is lossy. However filter programs with more than BPF_MAXINSNS (4096)
                // will be rejected by the kernel with EINVAL.
                len: filter_lines.len() as u16,
                filter: filter_lines.as_ptr() as *mut c::sock_filter,
            },
            PhantomData,
        )
    }
}

/// fixme
pub fn set_secure_computing_mode_strict() -> io::Result<()> {
    syscalls::seccomp(SeccompOperation::SetModeStrict, None, null_mut()).map(|_| ())
}

/// fixme
///
/// ... low level interface ... you likely want to use a library like libseccomp.
///
/// ... return value can be anything .... no I/O Safety ...
pub fn set_secure_computing_mode_filter(
    filter: &SecureComputingFilter,
    flags: SetSecureComputingFilterFlags,
) -> io::Result<i32> {
    syscalls::seccomp(
        SeccompOperation::SetModeFilter,
        Some(flags),
        filter as *const _ as *mut c::c_void,
    )
}
