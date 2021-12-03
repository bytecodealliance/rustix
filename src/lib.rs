//! `rustix` provides efficient memory-safe and [I/O-safe] wrappers to
//! POSIX-like, Unix-like, Linux, and Winsock2 syscall-like APIs, with
//! configurable backends.
//!
//! For example, instead of calling libc directly, using an `unsafe` block,
//! passing a raw file descriptor, passing a raw pointer, passing a buffer
//! length, manually checking the return value for errors, and manually
//! converting the return value to the intended type:
//!
//! ```rust
//! # fn read(sock: std::net::TcpStream, buf: &mut [u8]) -> std::io::Result<()> {
//! # use std::convert::TryInto;
//! # use rustix::fd::AsRawFd;
//! # #[cfg(windows)]
//! # use winapi::um::winsock2 as libc;
//! # const MSG_PEEK: i32 = libc::MSG_PEEK;
//! let nread: usize = unsafe {
//!     match libc::recv(
//!         sock.as_raw_fd() as _,
//!         buf.as_mut_ptr().cast(),
//!         buf.len().try_into().unwrap_or(i32::MAX as _),
//!         MSG_PEEK,
//!     ) {
//!         -1 => return Err(std::io::Error::last_os_error()),
//!         nread => nread as usize,
//!     }
//! };
//! # let _ = nread;
//! # Ok(())
//! # }
//! ```
//!
//! With rustix you can call a safe function, pass it any type that implements
//! [`AsFd`], pass it a slice, and get a `Result` carrying the intended type:
//!
//! ```rust
//! # fn read(sock: std::net::TcpStream, buf: &mut [u8]) -> std::io::Result<()> {
//! # use rustix::net::RecvFlags;
//! let nread: usize = rustix::net::recv(&sock, buf, RecvFlags::PEEK)?;
//! # let _ = nread;
//! # Ok(())
//! # }
//! ```
//!
//! rustix's APIs perform the following tasks:
//!  - Error values are translated to [`Result`]s.
//!  - Buffers are passed as Rust slices.
//!  - Out-parameters are presented as return values.
//!  - Path arguments use [`Arg`], so they accept any string type.
//!  - File descriptors are passed and returned via [`AsFd`] and [`OwnedFd`]
//!    instead of bare integers, ensuring I/O safety.
//!  - Constants use `enum`s and [`bitflags`] types.
//!  - Multiplexed functions (eg. `fcntl`, `ioctl`, etc.) are de-multiplexed.
//!  - Variadic functions (eg. `openat`, etc.) are presented as non-variadic.
//!  - Functions and types which need `l` prefixes or `64` suffixes to enable
//!    large-file support are used automatically, and file sizes and offsets
//!    are presented as `i64` and `u64`.
//!  - Behaviors that depend on the sizes of C types like `long` are hidden.
//!  - In some places, more human-friendly and less historical-accident names
//!    are used.
//!
//! Things they don't do include:
//!  - Detecting whether functions are supported at runtime.
//!  - Hiding significant differences between platforms.
//!  - Restricting ambient authorities.
//!  - Imposing sandboxing features such as filesystem path or network address
//!    sandboxing.
//!
//! See [`cap-std`], [`system-interface`], and [`io-streams`] for libraries
//! which do hide significant differences between platforms, and [`cap-std`]
//! which does perform sandboxing and restricts ambient authorities.
//!
//! [`cap-std`]: https://crates.io/crates/cap-std
//! [`system-interface`]: https://crates.io/crates/system-interface
//! [`io-streams`]: https://crates.io/crates/io-streams
//! [`getrandom`]: https://crates.io/crates/getrandom
//! [`bitflags`]: https://crates.io/crates/bitflags
//! [`AsFd`]: https://doc.rust-lang.org/stable/std/os/unix/io/trait.AsFd.html
//! [`OwnedFd`]: https://docs.rs/io-lifetimes/latest/io_lifetimes/struct.OwnedFd.html
//! [io-lifetimes crate]: https://crates.io/crates/io-lifetimes
//! [I/O-safe]: https://github.com/rust-lang/rfcs/blob/master/text/3128-io-safety.md
//! [`Result`]: https://docs.rs/rustix/latest/rustix/io/type.Result.html
//! [`Arg`]: https://docs.rs/rustix/latest/rustix/path/trait.Arg.html

#![deny(missing_docs)]
#![cfg_attr(linux_raw, deny(unsafe_code))]
#![cfg_attr(asm, feature(asm))]
#![cfg_attr(any(rustc_attrs, not(feature = "std")), feature(rustc_attrs))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(all(target_os = "wasi", feature = "std"), feature(wasi_ext))]
#![cfg_attr(all(linux_raw, asm, target_arch = "x86"), feature(naked_functions))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), allow(incomplete_features))]
#![cfg_attr(not(feature = "std"), feature(specialization))]
#![cfg_attr(not(feature = "std"), feature(slice_internals))]
#![cfg_attr(not(feature = "std"), feature(toowned_clone_into))]
#![cfg_attr(not(feature = "std"), feature(vec_into_raw_parts))]
#![cfg_attr(feature = "rustc-dep-of-std", feature(const_raw_ptr_deref))]
#![cfg_attr(feature = "rustc-dep-of-std", feature(core_intrinsics))]
#![cfg_attr(
    all(not(feature = "rustc-dep-of-std"), core_intrinsics),
    feature(core_intrinsics)
)]

#[cfg(not(feature = "rustc-dep-of-std"))]
extern crate alloc;

/// Export `*Fd*` types and traits that used in rustix's public API.
///
/// Users can use this to avoid needing to import anything else to use the same
/// versions of these types and traits.
///
/// Note that `OwnedFd` lives at [`rustix::io::OwnedFd`].
///
/// [`rustix::io::OwnedFd`]: crate::io::OwnedFd
pub mod fd {
    use super::imp;
    pub use imp::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
    #[cfg(feature = "std")]
    pub use imp::fd::{FromFd, IntoFd};
}

#[cfg(not(windows))]
#[macro_use]
pub(crate) mod zstr;
#[macro_use]
pub(crate) mod const_assert;

mod imp;

#[cfg(not(windows))]
pub mod ffi;
#[cfg(not(windows))]
pub mod fs;
pub mod io;
#[cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.
pub mod net;
#[cfg(not(windows))]
pub mod path;
#[cfg(not(windows))]
pub mod process;
#[cfg(not(windows))]
pub mod rand;
#[cfg(not(windows))]
pub mod thread;
#[cfg(not(windows))]
pub mod time;

#[cfg(not(windows))]
#[doc(hidden)]
pub mod runtime;

/// Convert a `&T` into a `*const T` without using an `as`.
#[inline]
#[allow(dead_code)]
const fn as_ptr<T>(t: &T) -> *const T {
    t
}

/// Convert a `&mut T` into a `*mut T` without using an `as`.
#[inline]
#[allow(dead_code)]
fn as_mut_ptr<T>(t: &mut T) -> *mut T {
    t
}
