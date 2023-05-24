pub(crate) mod errno;
#[cfg(not(windows))]
#[cfg(not(feature = "std"))]
pub(crate) mod io_slice;
#[cfg(not(windows))]
pub(crate) mod types;

#[cfg_attr(windows, path = "windows_syscalls.rs")]
pub(crate) mod syscalls;
