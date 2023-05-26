//! Tests for [`rustix::param`].

#![cfg(feature = "param")]
#![cfg(not(windows))]
#![cfg_attr(core_c_str, feature(core_c_str))]

#[cfg(not(target_os = "wasi"))]
#[macro_use]
mod weak;

#[cfg(not(target_os = "wasi"))]
mod auxv;
