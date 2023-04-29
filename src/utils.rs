#![allow(dead_code)]

/// Convert a `&T` into a `*const T` without using an `as`.
#[inline]
pub(crate) const fn as_ptr<T>(t: &T) -> *const T {
    t
}

/// Convert a `&mut T` into a `*mut T` without using an `as`.
#[inline]
pub(crate) fn as_mut_ptr<T>(t: &mut T) -> *mut T {
    t
}

/// Convert an `Option<&T>` into a possibly-null `*const T`.
#[inline]
pub(crate) const fn optional_as_ptr<T>(t: Option<&T>) -> *const T {
    match t {
        Some(t) => t,
        None => core::ptr::null(),
    }
}

/// Convert an `Option<&mut T>` into a possibly-null `*mut T`.
#[inline]
pub(crate) fn optional_as_mut_ptr<T>(t: Option<&mut T>) -> *mut T {
    match t {
        Some(t) => t,
        None => core::ptr::null_mut(),
    }
}

/// Convert a `*mut c_void` to a `*mut T`, checking that it is not null,
/// misaligned, or pointing to a region of memory that wraps around the address
/// space.
pub(crate) fn check_raw_pointer<T>(value: *mut core::ffi::c_void) -> Option<core::ptr::NonNull<T>> {
    if (value as usize)
        .checked_add(core::mem::size_of::<T>())
        .is_none()
        || (value as usize) % core::mem::align_of::<T>() != 0
    {
        return None;
    }

    core::ptr::NonNull::new(value.cast())
}
