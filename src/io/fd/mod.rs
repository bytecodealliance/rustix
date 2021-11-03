//! Owned and borrowed Unix-like file descriptors.

#![cfg_attr(staged_api, unstable(feature = "io_safety", issue = "87074"))]
#![deny(unsafe_op_in_unsafe_fn)]

// `RawFd`, `AsRawFd`, etc.
pub mod raw;

// `OwnedFd`, `AsFd`, etc.
pub mod owned;
