//! Some register syscall related types or parameters.

use crate::fd::{AsFd, BorrowedFd};
use core::fmt;

use crate::io;

use crate::alluring::sys;

pub(crate) fn execute<Fd: AsFd>(
    fd: Fd,
    opcode: sys::IoringRegisterOp,
    arg: *const core::ffi::c_void,
    len: u32,
) -> io::Result<()> {
    unsafe {
        sys::io_uring_register(fd.as_fd(), opcode, arg, len)?;
    }
    Ok(())
}

/// Information about what `io_uring` features the kernel supports.
///
/// You can fill this in with [`register_probe`](crate::Submitter::register_probe).
pub struct Probe(ProbeAndOps);

#[repr(C)]
struct ProbeAndOps(sys::io_uring_probe, [sys::io_uring_probe_op; Probe::COUNT]);

impl Probe {
    pub(crate) const COUNT: usize = 256;

    /// Create a new probe with no features enabled.
    pub fn new() -> Probe {
        Probe(ProbeAndOps(
            sys::io_uring_probe::default(),
            [sys::io_uring_probe_op::default(); Probe::COUNT],
        ))
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut sys::io_uring_probe {
        &mut (self.0).0
    }

    /// Get whether a specific opcode is supported.
    pub fn is_supported(&self, opcode: sys::IoringOp) -> bool {
        unsafe {
            let probe = &(self.0).0;

            if opcode as u32 <= probe.last_op as u32 {
                let ops = probe.ops.as_slice(Self::COUNT);
                ops[opcode as usize]
                    .flags
                    .contains(sys::IoringOpFlags::SUPPORTED)
            } else {
                false
            }
        }
    }
}

impl Default for Probe {
    #[inline]
    fn default() -> Probe {
        Probe::new()
    }
}

impl fmt::Debug for Probe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Op<'a>(&'a sys::io_uring_probe_op);

        impl fmt::Debug for Op<'_> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Op").field("code", &self.0.op).finish()
            }
        }

        let probe = &(self.0).0;
        let list = unsafe { probe.ops.as_slice(probe.last_op as usize + 1) };
        let list = list
            .iter()
            .filter(|op| op.flags.contains(sys::IoringOpFlags::SUPPORTED))
            .map(Op);

        f.debug_set().entries(list).finish()
    }
}

/// An allowed feature of io_uring. You can set the allowed features with
/// [`register_restrictions`](crate::Submitter::register_restrictions).
#[repr(transparent)]
pub struct Restriction(sys::io_uring_restriction);

/// inline zeroed to improve codegen
#[inline(always)]
fn res_zeroed() -> sys::io_uring_restriction {
    Default::default()
}

impl Restriction {
    /// Allow an `io_uring_register` opcode.
    pub fn register_op(op: sys::IoringRegisterOp) -> Restriction {
        let mut res = res_zeroed();
        res.opcode = sys::IoringRestrictionOp::RegisterOp;
        res.register_or_sqe_op_or_sqe_flags.register_op = op;
        Restriction(res)
    }

    /// Allow a submission queue event opcode.
    pub fn sqe_op(op: sys::IoringOp) -> Restriction {
        let mut res = res_zeroed();
        res.opcode = sys::IoringRestrictionOp::SqeOp;
        res.register_or_sqe_op_or_sqe_flags.sqe_op = op;
        Restriction(res)
    }

    /// Allow the given [submission queue event flags](crate::squeue::Flags).
    pub fn sqe_flags_allowed(flags: sys::IoringSqeFlags) -> Restriction {
        let mut res = res_zeroed();
        res.opcode = sys::IoringRestrictionOp::SqeFlagsAllowed;
        res.register_or_sqe_op_or_sqe_flags.sqe_flags = flags;
        Restriction(res)
    }

    /// Require the given [submission queue event flags](crate::squeue::Flags). These flags must be
    /// set on every submission.
    pub fn sqe_flags_required(flags: sys::IoringSqeFlags) -> Restriction {
        let mut res = res_zeroed();
        res.opcode = sys::IoringRestrictionOp::SqeFlagsRequired;
        res.register_or_sqe_op_or_sqe_flags.sqe_flags = flags;
        Restriction(res)
    }
}

/// A RawFd, which can be used for
/// [register_files_update](crate::Submitter::register_files_update).
///
/// File descriptors can be skipped if they are set to `SKIP_FILE`.
/// Skipping an fd will not touch the file associated with the previous fd at that index.
pub const SKIP_FILE: BorrowedFd<'static> = crate::io_uring::io_uring_register_files_skip();

#[test]
fn test_probe_layout() {
    use std::alloc::Layout;
    use std::mem;

    let probe = Probe::new();
    assert_eq!(
        Layout::new::<sys::io_uring_probe>().size()
            + mem::size_of::<sys::io_uring_probe_op>() * 256,
        Layout::for_value(&probe.0).size()
    );
    assert_eq!(
        Layout::new::<sys::io_uring_probe>().align(),
        Layout::for_value(&probe.0).align()
    );
}
