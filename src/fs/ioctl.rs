//! Filesystem-oriented `ioctl` functions.

#[cfg(linux_kernel)]
use {
    crate::fd::{AsFd, AsRawFd, BorrowedFd},
    crate::{backend, io, ioctl},
    backend::c,
    core::mem::MaybeUninit,
};

/// `ioctl(fd, BLKSSZGET)`—Returns the logical block size of a block device.
///
/// This is mentioned in the [Linux `openat` manual page].
///
/// [Linux `openat` manual page]: https://man7.org/linux/man-pages/man2/openat.2.html
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "BLKSSZGET")]
pub fn ioctl_blksszget<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    ioctl::ioctl(fd, Blksszget(MaybeUninit::uninit()))
}

/// `ioctl(fd, BLKPBSZGET)`—Returns the physical block size of a block device.
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "BLKPBSZGET")]
pub fn ioctl_blkpbszget<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    ioctl::ioctl(fd, Blkpbszget(MaybeUninit::uninit()))
}

/// `ioctl(fd, FICLONE, src_fd)`—Share data between open files.
///
/// This ioctl is not available on Sparc platforms
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_ficlone.2.html
#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[inline]
#[doc(alias = "FICLONE")]
pub fn ioctl_ficlone<Fd: AsFd, SrcFd: AsFd>(fd: Fd, src_fd: SrcFd) -> io::Result<()> {
    ioctl::ioctl(fd, Ficlone(src_fd.as_fd()))
}

/// `ioctl(fd, EXT4_IOC_RESIZE_FS, blocks)`—Resize ext4 filesystem on fd.
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "EXT4_IOC_RESIZE_FS")]
pub fn ext4_ioc_resize_fs<Fd: AsFd>(fd: Fd, blocks: u64) -> io::Result<()> {
    ioctl::ioctl(fd, Ext4IocResizeFs(blocks))
}

#[cfg(linux_kernel)]
struct Blksszget(MaybeUninit<c::c_uint>);

#[cfg(linux_kernel)]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Blksszget {
    type Output = u32;

    const OPCODE: ioctl::Opcode = ioctl::Opcode::Bad(c::BLKSSZGET);
    const IS_MUTATING: bool = true;

    fn as_ptr(&mut self) -> *mut c::c_void {
        self.0.as_mut_ptr().cast()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        arg: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        let ptr: *mut MaybeUninit<c::c_uint> = arg.cast();
        let value = ptr.read().assume_init();
        Ok(value)
    }
}

#[cfg(linux_kernel)]
struct Blkpbszget(MaybeUninit<c::c_uint>);

#[cfg(linux_kernel)]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Blkpbszget {
    type Output = u32;

    const OPCODE: ioctl::Opcode = ioctl::Opcode::Bad(c::BLKPBSZGET);
    const IS_MUTATING: bool = true;

    fn as_ptr(&mut self) -> *mut c::c_void {
        self.0.as_mut_ptr().cast()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        arg: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        let ptr: *mut MaybeUninit<c::c_uint> = arg.cast();
        let value = ptr.read().assume_init();
        Ok(value)
    }
}

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
struct Ficlone<'a>(BorrowedFd<'a>);

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Ficlone<'_> {
    type Output = ();

    const OPCODE: ioctl::Opcode = ioctl::Opcode::Bad(c::FICLONE);
    const IS_MUTATING: bool = false;

    fn as_ptr(&mut self) -> *mut c::c_void {
        self.0.as_raw_fd() as *mut c::c_void
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(())
    }
}

#[cfg(linux_kernel)]
struct Ext4IocResizeFs(u64);

#[cfg(linux_kernel)]
#[allow(unsafe_code)]
unsafe impl ioctl::Ioctl for Ext4IocResizeFs {
    type Output = ();

    const OPCODE: ioctl::Opcode = ioctl::Opcode::Bad(backend::fs::EXT4_IOC_RESIZE_FS);
    const IS_MUTATING: bool = false;

    fn as_ptr(&mut self) -> *mut c::c_void {
        (&mut self.0 as *mut u64).cast()
    }

    unsafe fn output_from_ptr(
        _: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        Ok(())
    }
}
