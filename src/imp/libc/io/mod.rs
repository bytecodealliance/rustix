pub(crate) mod errno;
#[cfg(not(feature = "std"))]
#[cfg_attr(windows, path = "io_slice_windows.rs")]
pub(crate) mod io_slice;
pub(crate) mod poll_fd;
#[cfg(not(windows))]
pub(crate) mod types;

#[cfg_attr(windows, path = "windows_syscalls.rs")]
pub(crate) mod syscalls;

#[cfg(not(feature = "std"))]
pub use io_slice::{IoSlice, IoSliceMut};
#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod epoll;
