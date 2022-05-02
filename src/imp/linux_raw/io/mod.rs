pub mod epoll;
pub(crate) mod error;
#[cfg(not(feature = "std"))]
pub(crate) mod io_slice;
pub(crate) mod poll_fd;
pub(crate) mod syscalls;
pub(crate) mod types;
