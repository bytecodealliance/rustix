//! Tests for [`rustix::net`].

#![cfg(feature = "net")]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "redox", target_os = "wasi")))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod addr;
#[cfg(unix)]
mod cmsg;
mod connect_bind_send;
mod dgram;
#[cfg(feature = "event")]
mod poll;
mod sockopt;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
mod unix_alloc;
mod v4;
mod v6;

#[cfg(windows)]
mod windows {
    use std::sync::OnceLock;

    pub struct Thing;

    impl Thing {
        pub fn new() -> Self {
            let _ = rustix::net::wsa_startup().unwrap();
            Self
        }
    }

    impl Drop for Thing {
        fn drop(&mut self) {
            rustix::net::wsa_cleanup().unwrap();
        }
    }

    pub static CLEANUP: OnceLock<Thing> = OnceLock::new();
}

/// Checks whether the Windows socket interface has been started already, and
/// if not, starts it.
pub fn init() {
    #[cfg(windows)]
    let _ = windows::CLEANUP.get_or_init(|| windows::Thing::new());
}
