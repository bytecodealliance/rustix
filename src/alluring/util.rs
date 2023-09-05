use crate::fd::OwnedFd;
use crate::io;
use crate::mm::{Advice, MapFlags, ProtFlags};
use core::num::NonZeroU32;
use core::ptr;
use core::sync::atomic;

pub(crate) mod private {
    /// Private trait that we use as a supertrait of `EntryMarker` to prevent it from being
    /// implemented from outside this crate: https://jack.wrenn.fyi/blog/private-trait-methods/
    pub trait Sealed {}
}

/// A region of memory mapped using `mmap(2)`.
pub(crate) struct Mmap {
    addr: ptr::NonNull<core::ffi::c_void>,
    len: usize,
}

impl Mmap {
    /// Map `len` bytes starting from the offset `offset` in the file descriptor `fd` into memory.
    pub fn new(fd: &OwnedFd, offset: u64, len: usize) -> io::Result<Mmap> {
        unsafe {
            let addr = crate::mm::mmap(
                ptr::null_mut(),
                len,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED | MapFlags::POPULATE,
                fd,
                offset,
            )?;
            // here, `mmap` will never return null
            let addr = ptr::NonNull::new_unchecked(addr);
            Ok(Mmap { addr, len })
        }
    }

    /// Do not make the stored memory accessible by child processes after a `fork`.
    pub fn dontfork(&self) -> io::Result<()> {
        unsafe {
            crate::mm::madvise(self.addr.as_ptr(), self.len, Advice::LinuxDontFork)?;
        }
        Ok(())
    }

    /// Get a pointer to the memory.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut core::ffi::c_void {
        self.addr.as_ptr()
    }

    /// Get a pointer to the data at the given offset.
    #[inline]
    pub unsafe fn offset(&self, offset: u32) -> *mut core::ffi::c_void {
        self.as_mut_ptr().add(offset as usize)
    }
}

impl Drop for Mmap {
    fn drop(&mut self) {
        unsafe {
            crate::mm::munmap(self.addr.as_ptr(), self.len).unwrap();
        }
    }
}

#[inline(always)]
pub(crate) unsafe fn unsync_load(u: *const atomic::AtomicU32) -> u32 {
    *u.cast::<u32>()
}

#[inline]
pub(crate) const fn cast_ptr<T>(n: &T) -> *const T {
    n
}

/// Convert a valid `u32` constant.
///
/// This is a workaround for the lack of panic-in-const in older
/// toolchains.
#[allow(unconditional_panic)]
pub(crate) const fn unwrap_u32(t: Option<u32>) -> u32 {
    match t {
        Some(v) => v,
        None => [][1],
    }
}

/// Convert a valid `NonZeroU32` constant.
///
/// This is a workaround for the lack of panic-in-const in older
/// toolchains.
#[allow(unconditional_panic)]
pub(crate) const fn unwrap_nonzero(t: Option<NonZeroU32>) -> NonZeroU32 {
    match t {
        Some(v) => v,
        None => [][1],
    }
}
