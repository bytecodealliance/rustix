//! Utilities to help with buffering.

#![allow(unsafe_code)]

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::slice;

/// A memory buffer that may be uninitialized.
pub trait Buffer<T>: private::Sealed<T> {}

// Implement `Buffer` for all the types that implement `Sealed`.
impl<T> Buffer<T> for &mut [T] {}
impl<T, const N: usize> Buffer<T> for &mut [T; N] {}
#[cfg(feature = "alloc")]
impl<T> Buffer<T> for &mut Vec<T> {}
impl<'a, T> Buffer<T> for &'a mut [MaybeUninit<T>] {}
impl<'a, T, const N: usize> Buffer<T> for &'a mut [MaybeUninit<T>; N] {}
#[cfg(feature = "alloc")]
impl<'a, T> Buffer<T> for Extend<'a, T> {}

impl<T> private::Sealed<T> for &mut [T] {
    type Result = usize;

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

impl<T, const N: usize> private::Sealed<T> for &mut [T; N] {
    type Result = usize;

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), N)
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

// `Vec` implements `DerefMut` to `&mut [T]`, however it doesn't get
// auto-derefed in a `impl Buffer<u8>`, so we add this `impl` so that our users
// don't have to add an extra `*` in these situations.
#[cfg(feature = "alloc")]
impl<T> private::Sealed<T> for &mut Vec<T> {
    type Result = usize;

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

impl<'a, T> private::Sealed<T> for &'a mut [MaybeUninit<T>] {
    type Result = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

impl<'a, T, const N: usize> private::Sealed<T> for &'a mut [MaybeUninit<T>; N] {
    type Result = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

/// A type that implements [`Buffer`] by appending to a `Vec`, up to its
/// capacity.
///
/// Because this uses the capacity, and never reallocates, it's a good idea to
/// reserve some space in a `Vec` before using this!
#[cfg(feature = "alloc")]
pub struct Extend<'a, T>(&'a mut Vec<T>);

/// Construct an [`Extend`].
#[cfg(feature = "alloc")]
pub fn extend<T>(v: &mut Vec<T>) -> Extend<T> {
    Extend(v)
}

#[cfg(feature = "alloc")]
impl<'a, T> private::Sealed<T> for Extend<'a, T> {
    /// The mutated `Vec` reflects the number of bytes read.
    type Result = ();

    #[inline]
    fn as_raw_parts_mut(&mut self) -> (*mut T, usize) {
        let spare = self.0.spare_capacity_mut();
        (spare.as_mut_ptr().cast(), spare.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        self.0.set_len(self.0.len() + len);
    }
}

/// Split an uninitialized byte slice into initialized and uninitialized parts.
///
/// # Safety
///
/// `init_len` must not be greater than `buf.len()`, and at least `init_len`
/// bytes must be initialized.
#[inline]
pub(super) unsafe fn split_init(
    buf: &mut [MaybeUninit<u8>],
    init_len: usize,
) -> (&mut [u8], &mut [MaybeUninit<u8>]) {
    debug_assert!(init_len <= buf.len());
    let buf_ptr = buf.as_mut_ptr();
    let uninit_len = buf.len() - init_len;
    let init = slice::from_raw_parts_mut(buf_ptr.cast::<u8>(), init_len);
    let uninit = slice::from_raw_parts_mut(buf_ptr.add(init_len), uninit_len);
    (init, uninit)
}

mod private {
    pub trait Sealed<T> {
        /// The result of the process operation.
        type Result;

        /// Return a pointer and length for this buffer.
        fn as_raw_parts_mut(&mut self) -> (*mut T, usize);

        /// Convert a finished buffer pointer into its result.
        ///
        /// # Safety
        ///
        /// At least `len` bytes of the buffer must now be initialized.
        unsafe fn finish(self, len: usize) -> Self::Result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_init() {
        let mut input_array = [
            MaybeUninit::new(0_u8),
            MaybeUninit::new(1_u8),
            MaybeUninit::new(2_u8),
            MaybeUninit::new(3_u8),
        ];
        let input_array_clone = input_array.clone();
        let input_array_ptr = input_array.as_ptr();
        let output_array = [0_u8, 1_u8, 2_u8, 3_u8];

        unsafe {
            let (init, uninit) = split_init(&mut input_array, 0);
            assert_eq!(init, &[]);
            assert_eq!(uninit.len(), input_array_clone.len());
            assert_eq!(uninit.as_ptr(), input_array_ptr);

            let (init, uninit) = split_init(&mut input_array, input_array_clone.len());
            assert_eq!(init, &output_array[..]);
            assert_eq!(init.as_ptr(), input_array_ptr.cast());
            assert_eq!(uninit.len(), 0);
            assert_eq!(
                uninit.as_ptr(),
                input_array_ptr.add(input_array_clone.len())
            );

            let (init, uninit) = split_init(&mut input_array, 2);
            assert_eq!(init, &output_array[..2]);
            assert_eq!(init.as_ptr(), input_array_ptr.cast());
            assert_eq!(uninit.len(), 2);
            assert_eq!(uninit.as_ptr(), input_array_ptr.add(2));
        }
    }

    #[test]
    fn test_split_init_empty() {
        unsafe {
            let (init, uninit) = split_init(&mut [], 0);
            assert!(init.is_empty());
            assert!(uninit.is_empty());
        }
    }
}
