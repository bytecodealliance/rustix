//! Utilities related to FFI bindings.

#[cfg(not(feature = "std"))]
pub use alloc::ffi::{CString, NulError};
#[cfg(not(feature = "std"))]
pub use core::ffi::{CStr, FromBytesWithNulError};

#[cfg(feature = "std")]
pub use std::ffi::{CStr, CString, FromBytesWithNulError, NulError};
