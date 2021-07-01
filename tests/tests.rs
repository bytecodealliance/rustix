#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod fs;
mod io;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))] // WASI doesn't support `net` yet.
mod net;
mod path;
mod process;
mod rand;
mod time;
