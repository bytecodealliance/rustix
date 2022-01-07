pub use imp::*;

#[cfg(not(windows))]
mod imp {
    //! The following is derived from Rust's
    //! library/std/src/sys/unix/io.rs
    //! dca3f1b786efd27be3b325ed1e01e247aa589c3b.

    use super::super::super::c;
    use core::marker::PhantomData;
    use core::slice;

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct IoSlice<'a> {
        vec: c::iovec,
        _p: PhantomData<&'a [u8]>,
    }

    impl<'a> IoSlice<'a> {
        #[inline]
        pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
            IoSlice {
                vec: c::iovec {
                    iov_base: buf.as_ptr() as *mut u8 as *mut c::c_void,
                    iov_len: buf.len(),
                },
                _p: PhantomData,
            }
        }

        #[inline]
        pub fn advance(&mut self, n: usize) {
            if self.vec.iov_len < n {
                panic!("advancing IoSlice beyond its length");
            }

            unsafe {
                self.vec.iov_len -= n;
                self.vec.iov_base = self.vec.iov_base.add(n);
            }
        }

        #[inline]
        pub fn as_slice(&self) -> &[u8] {
            unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
        }
    }

    #[repr(transparent)]
    pub struct IoSliceMut<'a> {
        vec: c::iovec,
        _p: PhantomData<&'a mut [u8]>,
    }

    impl<'a> IoSliceMut<'a> {
        #[inline]
        pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
            IoSliceMut {
                vec: c::iovec {
                    iov_base: buf.as_mut_ptr() as *mut c::c_void,
                    iov_len: buf.len(),
                },
                _p: PhantomData,
            }
        }

        #[inline]
        pub fn advance(&mut self, n: usize) {
            if self.vec.iov_len < n {
                panic!("advancing IoSliceMut beyond its length");
            }

            unsafe {
                self.vec.iov_len -= n;
                self.vec.iov_base = self.vec.iov_base.add(n);
            }
        }

        #[inline]
        pub fn as_slice(&self) -> &[u8] {
            unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
        }

        #[inline]
        pub fn as_mut_slice(&mut self) -> &mut [u8] {
            unsafe { slice::from_raw_parts_mut(self.vec.iov_base as *mut u8, self.vec.iov_len) }
        }
    }
}

#[cfg(windows)]
mod imp {
    use super::super::super::c;
    use core::marker::PhantomData;
    use core::slice;

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct IoSlice<'a> {
        vec: c::WSABUF,
        _p: PhantomData<&'a [u8]>,
    }

    impl<'a> IoSlice<'a> {
        #[inline]
        pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
            assert!(buf.len() <= c::ULONG::MAX as usize);
            IoSlice {
                vec: c::WSABUF {
                    len: buf.len() as c::ULONG,
                    buf: buf.as_ptr() as *mut u8 as *mut c::CHAR,
                },
                _p: PhantomData,
            }
        }

        #[inline]
        pub fn advance(&mut self, n: usize) {
            if (self.vec.len as usize) < n {
                panic!("advancing IoSlice beyond its length");
            }

            unsafe {
                self.vec.len -= n as c::ULONG;
                self.vec.buf = self.vec.buf.add(n);
            }
        }

        #[inline]
        pub fn as_slice(&self) -> &[u8] {
            unsafe { slice::from_raw_parts(self.vec.buf as *mut u8, self.vec.len as usize) }
        }
    }

    #[repr(transparent)]
    pub struct IoSliceMut<'a> {
        vec: c::WSABUF,
        _p: PhantomData<&'a mut [u8]>,
    }

    impl<'a> IoSliceMut<'a> {
        #[inline]
        pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
            assert!(buf.len() <= c::ULONG::MAX as usize);
            IoSliceMut {
                vec: c::WSABUF {
                    len: buf.len() as c::ULONG,
                    buf: buf.as_mut_ptr() as *mut c::CHAR,
                },
                _p: PhantomData,
            }
        }

        #[inline]
        pub fn advance(&mut self, n: usize) {
            if (self.vec.len as usize) < n {
                panic!("advancing IoSliceMut beyond its length");
            }

            unsafe {
                self.vec.len -= n as c::ULONG;
                self.vec.buf = self.vec.buf.add(n);
            }
        }

        #[inline]
        pub fn as_slice(&self) -> &[u8] {
            unsafe { slice::from_raw_parts(self.vec.buf as *mut u8, self.vec.len as usize) }
        }

        #[inline]
        pub fn as_mut_slice(&mut self) -> &mut [u8] {
            unsafe { slice::from_raw_parts_mut(self.vec.buf as *mut u8, self.vec.len as usize) }
        }
    }
}
