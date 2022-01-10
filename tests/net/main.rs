#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "redox", target_os = "wasi")))] // WASI doesn't support `net` yet.
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod addr;
#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
mod unix_msg;
mod v4;
mod v4msg_tcp;
mod v4msg_udp;
mod v6;
mod v6msg_tcp;
mod v6msg_udp;
