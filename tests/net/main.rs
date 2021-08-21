#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod unix;
mod v4;
mod v6;
