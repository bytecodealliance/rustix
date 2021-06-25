//! `posish` provides efficient memory-safe and [I/O-safe] wrappers to
//! "POSIX-ish" `libc` APIs and syscalls.
//!
//! The wrappers perform the following tasks:
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
//! [`std`]: https://doc.rust-lang.org/std/
//! [`getrandom`]: https://crates.io/crates/getrandom
//! [`bitflags`]: https://crates.io/crates/bitflags
//! [`AsFd`]: https://docs.rs/io-lifetimes/latest/io_lifetimes/trait.AsFd.html
//! [`OwnedFd`]: https://docs.rs/io-lifetimes/latest/io_lifetimes/struct.OwnedFd.html
//! [io-lifetimes crate]: https://crates.io/crates/io-lifetimes
//! [I/O-safe]: https://github.com/rust-lang/rfcs/pull/3128
//! [`Result`]: https://docs.rs/posish/latest/posish/io/type.Result.html
//! [`Arg`]: https://docs.rs/posish/latest/posish/path/trait.Arg.html

#![deny(missing_docs)]
#![cfg_attr(linux_raw, deny(unsafe_code))]
#![cfg_attr(linux_raw, feature(asm))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

/// Re-export `io_lifetimes` since we use its types in our public API, so
/// that our users don't need to do anything special to use the same version.
pub use io_lifetimes;

#[cfg(all(libc, not(target_os = "wasi")))]
#[macro_use]
mod weak;

pub mod fs;
pub mod io;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))] // WASI doesn't support `net` yet.
pub mod net;
pub mod path;
pub mod process;
pub mod rand;
pub mod time;

#[cfg(libc)]
mod libc;
#[cfg(linux_raw)]
mod linux_raw;

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
