//! Utilities related to FFI bindings.

#[cfg(not(feature = "std"))]
pub use core::ffi::{CStr, CString, FromBytesWithNulError, NulError};

#[cfg(feature = "std")]
pub use std::ffi::{CStr, CString, FromBytesWithNulError, NulError};
