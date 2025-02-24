//! Utilities to help with buffering.

#![allow(unsafe_code)]

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::slice;

/// A memory buffer that may be uninitialized.
///
/// If you see errors like "move occurs because `x` has type `&mut [u8]`,
/// which does not implement the `Copy` trait", replace `x` with `&mut *x`.
pub trait Buffer<T>: private::Sealed<T> {}

// Implement `Buffer` for all the types that implement `Sealed`.
impl<T> Buffer<T> for &mut [T] {}
impl<T, const N: usize> Buffer<T> for &mut [T; N] {}
#[cfg(feature = "alloc")]
impl<T> Buffer<T> for &mut Vec<T> {}
impl<T> Buffer<T> for &mut [MaybeUninit<T>] {}
impl<T, const N: usize> Buffer<T> for &mut [MaybeUninit<T>; N] {}
#[cfg(feature = "alloc")]
impl<T> Buffer<T> for &mut Vec<MaybeUninit<T>> {}
#[cfg(feature = "alloc")]
impl<'a, T> Buffer<T> for Extend<'a, T> {}

impl<T> private::Sealed<T> for &mut [T] {
    type Output = usize;

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        len
    }
}

impl<T, const N: usize> private::Sealed<T> for &mut [T; N] {
    type Output = usize;

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), N)
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        len
    }
}

// `Vec` implements `DerefMut` to `&mut [T]`, however it doesn't get
// auto-derefed in a `impl Buffer<u8>`, so we add this `impl` so that our users
// don't have to add an extra `*` in these situations.
#[cfg(feature = "alloc")]
impl<T> private::Sealed<T> for &mut Vec<T> {
    type Output = usize;

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr(), self.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        len
    }
}

impl<'a, T> private::Sealed<T> for &'a mut [MaybeUninit<T>] {
    type Output = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

impl<'a, T, const N: usize> private::Sealed<T> for &'a mut [MaybeUninit<T>; N] {
    type Output = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> private::Sealed<T> for &'a mut Vec<MaybeUninit<T>> {
    type Output = (&'a mut [T], &'a mut [MaybeUninit<T>]);

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        (self.as_mut_ptr().cast(), self.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        let (init, uninit) = self.split_at_mut(len);

        // SAFETY: The user asserts that the slice is now initialized.
        let init = slice::from_raw_parts_mut(init.as_mut_ptr().cast::<T>(), init.len());

        (init, uninit)
    }
}

/// A type that implements [`Buffer`] by appending to a `Vec`, up to its
/// capacity.
///
/// Because this uses the capacity, and never reallocates, the `Vec` should
/// have some non-empty spare capacity.
#[cfg(feature = "alloc")]
pub struct Extend<'a, T>(&'a mut Vec<T>);

/// Construct an [`Extend`], which implements [`Buffer`].
///
/// This wraps a `Vec` and uses the spare capacity of the `Vec` as the buffer
/// to receive data in, automaically resizing the `Vec` to include the
/// received elements.
///
/// This uses the existing capacity, and never allocates, so the `Vec` should
/// have some non-empty spare capacity!
///
/// # Examples
///
/// ```
/// # fn test(input: &std::fs::File) -> rustix::io::Result<()> {
/// use rustix::io::{read, Errno};
/// use rustix::buffer::extend;
///
/// let mut buf = Vec::with_capacity(1024);
/// match read(input, extend(&mut buf)) {
///     Ok(0) => { /* end of stream */ }
///     Ok(n) => { /* `buf` is now `n` bytes longer */ }
///     Err(Errno::INTR) => { /* `buf` is unmodified */ }
///     Err(e) => { return Err(e); }
/// }
///
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
pub fn extend<'a, T>(v: &'a mut Vec<T>) -> Extend<'a, T> {
    Extend(v)
}

#[cfg(feature = "alloc")]
impl<'a, T> private::Sealed<T> for Extend<'a, T> {
    /// The mutated `Vec` reflects the number of bytes read. We also return
    /// this number, and a value of 0 indicates the end of the stream has
    /// been reached.
    type Output = usize;

    #[inline]
    fn parts_mut(&mut self) -> (*mut T, usize) {
        let spare = self.0.spare_capacity_mut();

        debug_assert!(!spare.is_empty(), "`extend` uses spare capacity, and never allocates new memory, so the `Vec` passed to it should have some spare capacity.");

        (spare.as_mut_ptr().cast(), spare.len())
    }

    #[inline]
    unsafe fn assume_init(self, len: usize) -> Self::Output {
        // We initialized `len` elements; extend the `Vec` to include them.
        self.0.set_len(self.0.len() + len);
        len
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
        type Output;

        /// Return a pointer and length for this buffer.
        fn parts_mut(&mut self) -> (*mut T, usize);

        /// Convert a finished buffer pointer into its result.
        ///
        /// # Safety
        ///
        /// At least `len` bytes of the buffer must now be initialized.
        unsafe fn assume_init(self, len: usize) -> Self::Output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(windows))]
    fn test_buffer() {
        use crate::io::read;
        use core::mem::MaybeUninit;

        let input = std::fs::File::open("Cargo.toml").unwrap();

        let mut buf = vec![0_u8; 3];
        buf.reserve(32);
        let _x: usize = read(&input, extend(&mut buf)).unwrap();
        let _x: (&mut [u8], &mut [MaybeUninit<u8>]) =
            read(&input, buf.spare_capacity_mut()).unwrap();
        let _x: usize = read(&input, &mut buf).unwrap();
        let _x: usize = read(&input, &mut *buf).unwrap();
        let _x: usize = read(&input, &mut buf[..]).unwrap();
        let _x: usize = read(&input, &mut (*buf)[..]).unwrap();

        let mut buf = [0, 0, 0];
        let _x: usize = read(&input, &mut buf).unwrap();
        let _x: usize = read(&input, &mut buf[..]).unwrap();

        let mut buf = [
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(&input, &mut buf).unwrap();
        let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(&input, &mut buf[..]).unwrap();

        let mut buf = vec![
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(&input, &mut buf).unwrap();
        let _x: (&mut [u8], &mut [MaybeUninit<u8>]) = read(&input, &mut buf[..]).unwrap();

        // This is reduced from src/fs/inotify.rs line 177.
        struct Wrapper<'a>(&'a mut [u8]);
        impl<'a> Wrapper<'a> {
            fn read(&mut self) {
                let input = std::fs::File::open("Cargo.toml").unwrap();

                // Ideally we'd write this.
                //let _x: usize = read(&input, self.0).unwrap();
                // But we need to write this instead.
                let _x: usize = read(&input, &mut *self.0).unwrap();
            }
        }
        let mut buf = vec![0_u8; 3];
        let mut wrapper = Wrapper(&mut buf);
        wrapper.read();

        // Why does this get two error messages?
        //let mut buf = [0, 0, 0];
        //let _x = read(&input, buf).unwrap();
    }

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
