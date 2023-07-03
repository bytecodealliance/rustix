//! Declare functions defined in out-of-line ("outline") asm files.
//!
//! Kernel calling conventions differ from userspace calling conventions, so we
//! also define inline function wrappers which reorder the arguments so that
//! they match with the kernel convention as closely as possible, to minimize
//! the amount of out-of-line code we need.
//!
//! This is needed because as of our MSRV of 1.63, inline asm are experimental
//! for some architectures.

// For these architectures, pass the `nr` argument last.
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc64",
))]
mod nr_last;

#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc64",
))]
pub(in crate::backend) use nr_last::*;
