//! I/O operations.
//!
//! If you're looking for [`SeekFrom`], that's in the [`fs`] module.
//!
//! [`SeekFrom`]: https://docs.rs/rustix/*/rustix/fs/enum.SeekFrom.html
//! [`fs`]: https://docs.rs/rustix/*/rustix/fs/index.html

mod close;
#[cfg(not(windows))]
mod dup;
mod errno;
#[cfg(not(windows))]
mod fcntl;
#[cfg(not(feature = "std"))]
pub(crate) mod fd;
mod ioctl;
#[cfg(not(any(windows, target_os = "redox")))]
#[cfg(all(feature = "fs", feature = "net"))]
mod is_read_write;
#[cfg(not(windows))]
mod read_write;

pub use close::close;
#[cfg(not(windows))]
pub use dup::*;
pub use errno::{retry_on_intr, Errno, Result};
#[cfg(not(windows))]
pub use fcntl::*;
pub use ioctl::*;
#[cfg(not(any(windows, target_os = "redox")))]
#[cfg(all(feature = "fs", feature = "net"))]
pub use is_read_write::*;
#[cfg(not(windows))]
pub use read_write::*;
