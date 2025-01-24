//! Utilities to help with buffering.

#![allow(unsafe_code)]

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::slice;

/// A memory buffer that may be uninitialized.
pub trait Buffer<T> {
    /// The result of the process operation.
    type Result;

    /// Convert this buffer into a maybe-unitiailized view.
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>];

    /// Convert a finished buffer pointer into its result.
    ///
    /// # Safety
    ///
    /// At least `len` bytes of the buffer must now be initialized.
    unsafe fn finish(self, len: usize) -> Self::Result;
}

/// Implements [`Buffer`] around the a slice of bytes.
///
/// `Result` is a `usize` indicating how many bytes were written.
impl<T> Buffer<T> for &mut [T] {
    type Result = usize;

    #[inline]
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>] {
        // SAFETY: This just casts away the knowledge that the elements are
        // initialized.
        unsafe { core::mem::transmute::<&mut [T], &mut [MaybeUninit<T>]>(self) }
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

/// Implements [`Buffer`] around the a slice of bytes.
///
/// `Result` is a `usize` indicating how many bytes were written.
impl<T, const N: usize> Buffer<T> for &mut [T; N] {
    type Result = usize;

    #[inline]
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>] {
        // SAFETY: This just casts away the knowledge that the elements are
        // initialized.
        unsafe { core::mem::transmute::<&mut [T], &mut [MaybeUninit<T>]>(*self) }
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

/// Implements [`Buffer`] around the a slice of bytes.
///
/// `Result` is a `usize` indicating how many bytes were written.
impl<T> Buffer<T> for &mut Vec<T> {
    type Result = usize;

    #[inline]
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>] {
        // SAFETY: This just casts away the knowledge that the elements are
        // initialized.
        unsafe { core::mem::transmute::<&mut [T], &mut [MaybeUninit<T>]>(self) }
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

/// Implements [`Buffer`] around the a slice of uninitialized bytes.
///
/// `Result` is a pair of slices giving the initialized and uninitialized
/// subslices after the new data is written.
impl<'a, T> Buffer<T> for &'a mut [MaybeUninit<T>] {
    type Result = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>] {
        self
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

/// Implements [`Buffer`] around the `Vec` type.
///
/// This implementation fills the buffer, overwriting any previous data, with
/// the new data data and sets the length.
#[cfg(feature = "alloc")]
impl<T> Buffer<T> for Vec<T> {
    type Result = Vec<T>;

    #[inline]
    fn as_maybe_uninitialized(&mut self) -> &mut [MaybeUninit<T>] {
        self.clear();
        self.spare_capacity_mut()
    }

    #[inline]
    unsafe fn finish(mut self, len: usize) -> Self::Result {
        self.set_len(len);
        self
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
