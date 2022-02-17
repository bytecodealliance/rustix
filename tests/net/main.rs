//! Tests for [`rustix::net`].

#![cfg(feature = "net")]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "redox", target_os = "wasi")))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod addr;
mod connect_bind_send;
mod poll;
mod sockopt;
#[cfg(unix)]
mod unix;
#[cfg(not(windows))]
mod unix_msg;
mod v4;
mod v4msg_tcp;
mod v4msg_udp;
mod v6;
mod v6msg_tcp;
mod v6msg_udp;

/// Windows requires us to call a setup function before using any of the
/// socket APIs.
#[cfg(windows)]
#[ctor::ctor]
fn windows_startup() {
    let _ = rustix::net::wsa_startup().unwrap();
}

/// Windows requires us to call a cleanup function after using any of the
/// socket APIs.
#[cfg(windows)]
#[ctor::dtor]
fn windows_shutdown() {
    rustix::net::wsa_cleanup().unwrap();
}
