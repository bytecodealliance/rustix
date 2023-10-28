//! Code for handling buffers to write into.

#![allow(unsafe_code)]

use core::mem::MaybeUninit;
use core::slice;

/// A buffer that the system can write into.
pub unsafe trait Buffer<T: AnyBitPattern>: Sized {
    /// The result of the process operation.
    type Result;

    /// Convert this buffer into a pointer to a buffer and its capacity.
    fn as_buffer(&mut self) -> (*mut T, usize);

    /// Convert a finished buffer pointer into its result.
    unsafe fn finish(self, len: usize) -> Self::Result;
}

unsafe impl<T: AnyBitPattern> Buffer<T> for &mut [T] {
    type Result = usize;

    #[inline]
    fn as_buffer(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        len
    }
}

unsafe impl<'a, T: AnyBitPattern> Buffer<T> for &'a mut [MaybeUninit<T>] {
    type Result = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn as_buffer(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn finish(self, len: usize) -> Self::Result {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(
            init.as_mut_ptr().cast(),
            init.len()
        );

        (init, uninit)
    }
}

/// Implements [`Buffer`] around the `Vec` type.
///
/// This implementation fills the buffer with data and sets the length.
#[cfg(feature = "alloc")]
unsafe impl<T: AnyBitPattern> Buffer<T> for alloc::vec::Vec<T> {
    type Result = alloc::vec::Vec<T>;

    #[inline]
    fn as_buffer(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn finish(mut self, len: usize) -> Self::Result {
        self.set_len(len);
        self
    }
}

/// Types made up of plain-old-data.
///
/// # Safety
///
/// - The OS can write any byte pattern to this structure.
/// - This type does not implement `Drop`.
pub unsafe trait AnyBitPattern {}

macro_rules! impl_pod {
    ($($ty:ty),*) => {
        $(
            unsafe impl AnyBitPattern for $ty {}
        )*
    }
}

impl_pod! {
    u8, i8, u16, i16, u32, i32, u64, i64,
    usize, isize
}
