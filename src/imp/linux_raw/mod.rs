//! The linux_raw backend.
//!
//! This makes Linux syscalls directly, without going through libc.
//!
//! # Safety
//!
//! These files performs raw system calls, and sometimes passes them
//! uninitialized memory buffers. The signatures in this file are currently
//! manually maintained and must correspond with the signatures of the actual
//! Linux syscalls.
//!
//! Some of this could be auto-generated from the Linux header file
//! <linux/syscalls.h>, but we often need more information than it provides,
//! such as which pointers are array slices, out parameters, or in-out
//! parameters, which integers are owned or borrowed file descriptors, etc.

mod arch;
mod conv;
mod elf;
mod reg;
mod vdso;
mod vdso_wrappers;

pub(crate) mod fs;
pub(crate) mod io;
#[cfg(feature = "io_uring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "io_uring")))]
pub(crate) mod io_uring;
#[cfg(feature = "net")]
pub(crate) mod net;
pub(crate) mod process;
pub(crate) mod rand;
#[cfg(feature = "runtime")]
pub(crate) mod runtime;
#[cfg(feature = "termios")]
pub(crate) mod termios;
#[cfg(feature = "thread")]
pub(crate) mod thread;
pub(crate) mod time;

#[cfg(feature = "std")]
pub(crate) mod fd {
    pub use io_lifetimes::*;

    #[allow(unused_imports)]
    pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

    #[allow(unused_imports)]
    pub(crate) use std::os::unix::io::RawFd as LibcFd;
}

#[cfg(not(feature = "std"))]
pub(crate) use crate::io::fd;

// The linux_raw backend doesn't use actual libc, so we define selected
// libc-like definitions in a module called `c`.
pub(crate) mod c;
