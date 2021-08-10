#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
mod getrandom;
