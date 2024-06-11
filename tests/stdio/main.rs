//! Tests for [`rustix::stdio`].

#![cfg(feature = "stdio")]

#[cfg(not(feature = "rustc-dep-of-std"))]
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod dup2_stdio;
#[cfg(not(feature = "rustc-dep-of-std"))]
#[cfg(not(windows))]
#[cfg(not(target_os = "wasi"))]
mod dup2_to_replace_stdio;
