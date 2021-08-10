#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.

mod unix;
mod v4;
mod v6;
