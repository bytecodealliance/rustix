//! Tests for [`rustix::system`].

#![cfg(feature = "system")]
#![cfg(not(target_os = "wasi"))]

mod uname;
