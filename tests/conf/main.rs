//! Tests for [`rustix::conf`].

#![cfg(feature = "conf")]
#![cfg(not(windows))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

#[cfg(not(target_os = "wasi"))]
#[macro_use]
mod weak;

#[cfg(not(target_os = "wasi"))]
mod auxv;
