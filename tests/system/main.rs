//! Tests for [`rustix::system`].

#![cfg(feature = "system")]
#![cfg(not(any(windows, target_os = "wasi")))]

mod uname;
mod sysinfo;
