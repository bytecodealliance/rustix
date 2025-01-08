//! Tests for [`rustix::path`].

#![cfg(any(feature = "fs", feature = "net"))]
#![cfg(not(windows))]
#![cfg_attr(core_c_str, feature(core_c_str))]
#![cfg_attr(alloc_c_string, feature(alloc_c_string))]

#[cfg(not(feature = "rustc-dep-of-std"))]
mod arg;
mod dec_int;
