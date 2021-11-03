//! Utilities related to FFI bindings.

/// Minimal and unoptimized `strlen` implementation. TODO: Replace
/// this with a real `strlen` implementation.
#[cfg(feature = "rustc-dep-of-std")]
#[allow(unsafe_code)]
unsafe fn strlen(mut s: *const u8) -> usize {
    let mut len = 0;
    while *s != b'\0' {
        len += 1;
        s = s.add(1);
    }
    len
}

#[cfg(feature = "rustc-dep-of-std")]
mod z_str;

#[cfg(feature = "rustc-dep-of-std")]
pub use z_str::{FromBytesWithNulError, FromVecWithNulError, NulError, ZStr, ZString};

#[cfg(not(feature = "rustc-dep-of-std"))]
pub use std::ffi::{CStr as ZStr, CString as ZString, FromBytesWithNulError, NulError};
