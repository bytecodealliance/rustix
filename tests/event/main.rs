//! Tests for [`rustix::event`].

#![cfg(feature = "event")]

#[cfg(not(feature = "rustc-dep-of-std"))] // TODO
#[cfg(feature = "net")]
#[cfg(any(linux_kernel, target_os = "illumos", target_os = "redox"))]
mod epoll;
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod eventfd;
mod poll;
#[cfg(any(bsd, linux_kernel, windows, target_os = "wasi"))]
mod select;

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
