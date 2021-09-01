#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod auxv;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched_yield;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have uname.
mod uname;
