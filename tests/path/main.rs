#![cfg(not(windows))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod arg;
#[cfg(feature = "itoa")]
mod dec_int;
