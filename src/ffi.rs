//! Utilities related to FFI bindings.

#[cfg(not(windows))]
#[cfg(feature = "alloc")]
pub use alloc::ffi::{CString, NulError};
#[cfg(not(windows))]
pub use core::ffi::{CStr, FromBytesWithNulError};
pub use core::ffi::{
    c_char, c_int, c_long, c_longlong, c_short, c_uint, c_ulong, c_ulonglong, c_ushort, c_void,
};
