#[cfg(not(any(windows, target_os = "wasi")))]
#[macro_use]
mod weak;

mod conv;
mod offset;

#[cfg(windows)]
pub(crate) mod io_lifetimes;
#[cfg(not(windows))]
pub(crate) use io_lifetimes;

#[cfg(windows)]
pub(crate) mod libc;
#[cfg(not(windows))]
pub(crate) use libc;

#[cfg(not(windows))]
pub(crate) mod fs;
pub(crate) mod io;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.
pub(crate) mod net;
#[cfg(not(windows))]
pub(crate) mod process;
#[cfg(not(windows))]
pub(crate) mod rand;
pub(crate) mod syscalls;
#[cfg(not(windows))]
pub(crate) mod time;
