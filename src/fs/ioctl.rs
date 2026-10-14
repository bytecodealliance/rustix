//! Filesystem-oriented `ioctl` functions.

#![allow(unsafe_code)]

#[cfg(linux_kernel)]
use {
    crate::backend::c,
    crate::fd::AsFd,
    crate::{backend, ffi, io, ioctl},
};

use bitflags::bitflags;

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
use crate::fd::{AsRawFd as _, BorrowedFd};

/// `ioctl(fd, BLKSSZGET)`—Returns the logical block size of a block device.
///
/// This is mentioned in the [Linux `openat` manual page].
///
/// [Linux `openat` manual page]: https://man7.org/linux/man-pages/man2/openat.2.html
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "BLKSSZGET")]
pub fn ioctl_blksszget<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    // SAFETY: `BLZSSZGET` is a getter opcode that gets a `u32`.
    unsafe {
        let ctl = ioctl::Getter::<{ c::BLKSSZGET }, c::c_uint>::new();
        ioctl::ioctl(fd, ctl)
    }
}

/// `ioctl(fd, BLKPBSZGET)`—Returns the physical block size of a block device.
#[cfg(linux_kernel)]
#[inline]
#[doc(alias = "BLKPBSZGET")]
pub fn ioctl_blkpbszget<Fd: AsFd>(fd: Fd) -> io::Result<u32> {
    // SAFETY: `BLKPBSZGET` is a getter opcode that gets a `u32`.
    unsafe {
        let ctl = ioctl::Getter::<{ c::BLKPBSZGET }, c::c_uint>::new();
        ioctl::ioctl(fd, ctl)
    }
}

/// `ioctl(fd, FICLONE, src_fd)`—Share data between open files.
///
/// This ioctl is not available on SPARC platforms.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/ioctl_ficlone.2.html
#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[inline]
#[doc(alias = "FICLONE")]
pub fn ioctl_ficlone<Fd: AsFd, SrcFd: AsFd>(fd: Fd, src_fd: SrcFd) -> io::Result<()> {
    unsafe { ioctl::ioctl(fd, Ficlone(src_fd.as_fd())) }
}

/// `ioctl(fd, EXT4_IOC_RESIZE_FS, blocks)`—Resize ext4 filesystem on fd.
#[cfg(linux_raw_dep)]
#[inline]
#[doc(alias = "EXT4_IOC_RESIZE_FS")]
pub fn ext4_ioc_resize_fs<Fd: AsFd>(fd: Fd, blocks: u64) -> io::Result<()> {
    // SAFETY: `EXT4_IOC_RESIZE_FS` is a pointer setter opcode.
    unsafe {
        let ctl = ioctl::Setter::<{ backend::fs::EXT4_IOC_RESIZE_FS }, u64>::new(blocks);
        ioctl::ioctl(fd, ctl)
    }
}

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
struct Ficlone<'a>(BorrowedFd<'a>);

#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
unsafe impl ioctl::Ioctl for Ficlone<'_> {
    type Output = ();

    const IS_MUTATING: bool = false;

    fn opcode(&self) -> ioctl::Opcode {
        c::FICLONE as ioctl::Opcode
    }

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

#[cfg(linux_raw_dep)]
bitflags! {
    /// `FS_*` constants for use with [`ioctl_getflags`] and [`ioctl_setflags`].
    ///
    /// [`ioctl_getflags`]: crate::fs::ioctl::ioctl_getflags
    /// [`ioctl_setflags`]: crate::fs::ioctl::ioctl_setflags
    ///
    /// Not every flag returned by [`ioctl_getflags`] can be changed with
    /// [`ioctl_setflags`]. The kernel validates which changes the filesystem
    /// and calling process permit.
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct IFlags: ffi::c_uint {
        /// `FS_APPEND_FL`
        const APPEND = linux_raw_sys::general::FS_APPEND_FL;
        /// `FS_CASEFOLD_FL`
        const CASEFOLD = linux_raw_sys::general::FS_CASEFOLD_FL;
        /// `FS_COMPR_FL`
        const COMPRESSED = linux_raw_sys::general::FS_COMPR_FL;
        /// `FS_DIRSYNC_FL`
        const DIRSYNC = linux_raw_sys::general::FS_DIRSYNC_FL;
        /// `FS_ENCRYPT_FL`
        const ENCRYPTED = linux_raw_sys::general::FS_ENCRYPT_FL;
        /// `FS_IMMUTABLE_FL`
        const IMMUTABLE = linux_raw_sys::general::FS_IMMUTABLE_FL;
        /// `FS_JOURNAL_DATA_FL`
        const JOURNALING = linux_raw_sys::general::FS_JOURNAL_DATA_FL;
        /// `FS_NOATIME_FL`
        const NOATIME = linux_raw_sys::general::FS_NOATIME_FL;
        /// `FS_NOCOW_FL`
        const NOCOW = linux_raw_sys::general::FS_NOCOW_FL;
        /// `FS_NODUMP_FL`
        const NODUMP = linux_raw_sys::general::FS_NODUMP_FL;
        /// `FS_NOTAIL_FL`
        const NOTAIL = linux_raw_sys::general::FS_NOTAIL_FL;
        /// `FS_PROJINHERIT_FL`
        const PROJECT_INHERIT = linux_raw_sys::general::FS_PROJINHERIT_FL;
        /// `FS_SECRM_FL`
        const SECURE_REMOVAL = linux_raw_sys::general::FS_SECRM_FL;
        /// `FS_SYNC_FL`
        const SYNC = linux_raw_sys::general::FS_SYNC_FL;
        /// `FS_TOPDIR_FL`
        const TOPDIR = linux_raw_sys::general::FS_TOPDIR_FL;
        /// `FS_UNRM_FL`
        const UNRM = linux_raw_sys::general::FS_UNRM_FL;
        /// `FS_VERITY_FL`
        const VERITY = linux_raw_sys::general::FS_VERITY_FL;
    }
}

/// `ioctl(fd, FS_IOC_GETFLAGS)`—Returns the [inode flags] attributes
///
/// [inode flags]: https://man7.org/linux/man-pages/man2/ioctl_iflags.2.html
#[cfg(linux_raw_dep)]
#[inline]
#[doc(alias = "FS_IOC_GETFLAGS")]
pub fn ioctl_getflags<Fd: AsFd>(fd: Fd) -> io::Result<IFlags> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let ctl = ioctl::Getter::<{ c::FS_IOC32_GETFLAGS }, u32>::new();
        #[cfg(target_pointer_width = "64")]
        let ctl = ioctl::Getter::<{ c::FS_IOC_GETFLAGS }, u32>::new();

        ioctl::ioctl(fd, ctl).map(IFlags::from_bits_retain)
    }
}

/// `ioctl(fd, FS_IOC_SETFLAGS)`—Modify the [inode flags] attributes
///
/// [inode flags]: https://man7.org/linux/man-pages/man2/ioctl_iflags.2.html
#[cfg(linux_raw_dep)]
#[inline]
#[doc(alias = "FS_IOC_SETFLAGS")]
pub fn ioctl_setflags<Fd: AsFd>(fd: Fd, flags: IFlags) -> io::Result<()> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let ctl = ioctl::Setter::<{ c::FS_IOC32_SETFLAGS }, u32>::new(flags.bits());

        #[cfg(target_pointer_width = "64")]
        let ctl = ioctl::Setter::<{ c::FS_IOC_SETFLAGS }, u32>::new(flags.bits());

        ioctl::ioctl(fd, ctl)
    }
}

#[cfg(linux_raw_dep)]
bitflags! {
    /// `FS_XFLAG_*` constants for [`Fsxattr::fsx_xflags`].
    ///
    /// Values returned by [`ioctl_fsgetxattr`] retain any bits that are not
    /// represented by a named constant.
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FsxattrFlags: u32 {
        /// `FS_XFLAG_REALTIME`
        const REALTIME = linux_raw_sys::general::FS_XFLAG_REALTIME;
        /// `FS_XFLAG_PREALLOC`
        const PREALLOC = linux_raw_sys::general::FS_XFLAG_PREALLOC;
        /// `FS_XFLAG_IMMUTABLE`
        const IMMUTABLE = linux_raw_sys::general::FS_XFLAG_IMMUTABLE;
        /// `FS_XFLAG_APPEND`
        const APPEND = linux_raw_sys::general::FS_XFLAG_APPEND;
        /// `FS_XFLAG_SYNC`
        const SYNC = linux_raw_sys::general::FS_XFLAG_SYNC;
        /// `FS_XFLAG_NOATIME`
        const NOATIME = linux_raw_sys::general::FS_XFLAG_NOATIME;
        /// `FS_XFLAG_NODUMP`
        const NODUMP = linux_raw_sys::general::FS_XFLAG_NODUMP;
        /// `FS_XFLAG_RTINHERIT`
        const RTINHERIT = linux_raw_sys::general::FS_XFLAG_RTINHERIT;
        /// `FS_XFLAG_PROJINHERIT`
        const PROJINHERIT = linux_raw_sys::general::FS_XFLAG_PROJINHERIT;
        /// `FS_XFLAG_NOSYMLINKS`
        const NOSYMLINKS = linux_raw_sys::general::FS_XFLAG_NOSYMLINKS;
        /// `FS_XFLAG_EXTSIZE`
        const EXTSIZE = linux_raw_sys::general::FS_XFLAG_EXTSIZE;
        /// `FS_XFLAG_EXTSZINHERIT`
        const EXTSZINHERIT = linux_raw_sys::general::FS_XFLAG_EXTSZINHERIT;
        /// `FS_XFLAG_NODEFRAG`
        const NODEFRAG = linux_raw_sys::general::FS_XFLAG_NODEFRAG;
        /// `FS_XFLAG_FILESTREAM`
        const FILESTREAM = linux_raw_sys::general::FS_XFLAG_FILESTREAM;
        /// `FS_XFLAG_DAX`
        const DAX = linux_raw_sys::general::FS_XFLAG_DAX;
        /// `FS_XFLAG_COWEXTSIZE`
        const COWEXTSIZE = linux_raw_sys::general::FS_XFLAG_COWEXTSIZE;
        /// `FS_XFLAG_HASATTR`
        const HASATTR = linux_raw_sys::general::FS_XFLAG_HASATTR;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// Filesystem attributes returned by [`ioctl_fsgetxattr`].
#[cfg(linux_raw_dep)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Fsxattr {
    /// Extended filesystem flags.
    pub fsx_xflags: FsxattrFlags,
    /// Preferred extent size.
    pub fsx_extsize: u32,
    /// Number of extents.
    pub fsx_nextents: u32,
    /// Project identifier.
    pub fsx_projid: u32,
    /// Preferred copy-on-write extent size.
    pub fsx_cowextsize: u32,
}

#[cfg(linux_raw_dep)]
const FS_IOC_FSGETXATTR: ioctl::Opcode =
    ioctl::opcode::read::<linux_raw_sys::general::fsxattr>(b'X', 31);

#[cfg(linux_raw_dep)]
#[inline]
fn fsxattr_from_raw(raw: linux_raw_sys::general::fsxattr) -> Fsxattr {
    Fsxattr {
        fsx_xflags: FsxattrFlags::from_bits_retain(raw.fsx_xflags),
        fsx_extsize: raw.fsx_extsize,
        fsx_nextents: raw.fsx_nextents,
        fsx_projid: raw.fsx_projid,
        fsx_cowextsize: raw.fsx_cowextsize,
    }
}

/// `ioctl(fd, FS_IOC_FSGETXATTR)`—Returns extended filesystem attributes.
#[cfg(linux_raw_dep)]
#[inline]
#[doc(alias = "FS_IOC_FSGETXATTR")]
pub fn ioctl_fsgetxattr<Fd: AsFd>(fd: Fd) -> io::Result<Fsxattr> {
    // SAFETY: `fsxattr` consists entirely of integers and bytes, so all-zero
    // is a valid value. This also initializes the reserved padding before the
    // kernel may read it.
    let mut raw: linux_raw_sys::general::fsxattr = unsafe { core::mem::zeroed() };

    // SAFETY: `FS_IOC_FSGETXATTR` is `_IOR('X', 31, struct fsxattr)`. `raw`
    // has the exact kernel layout and is fully initialized, and `Updater`
    // permits the kernel to read and write it.
    unsafe {
        let ctl =
            ioctl::Updater::<{ FS_IOC_FSGETXATTR }, linux_raw_sys::general::fsxattr>::new(&mut raw);
        ioctl::ioctl(fd, ctl)?;
    }

    Ok(fsxattr_from_raw(raw))
}

#[cfg(all(test, linux_raw_dep))]
mod tests {
    use super::*;

    #[test]
    fn test_fsgetxattr_from_raw_fields_and_unknown_bits() {
        const UNKNOWN_XFLAG: u32 = 0x0000_0004;

        let attrs = fsxattr_from_raw(linux_raw_sys::general::fsxattr {
            fsx_xflags: FsxattrFlags::APPEND.bits() | UNKNOWN_XFLAG,
            fsx_extsize: 11,
            fsx_nextents: 22,
            fsx_projid: 33,
            fsx_cowextsize: 44,
            fsx_pad: [0; 8],
        });

        assert!(attrs.fsx_xflags.contains(FsxattrFlags::APPEND));
        assert_eq!(
            (attrs.fsx_xflags & !FsxattrFlags::APPEND).bits(),
            UNKNOWN_XFLAG
        );
        assert_eq!(attrs.fsx_extsize, 11);
        assert_eq!(attrs.fsx_nextents, 22);
        assert_eq!(attrs.fsx_projid, 33);
        assert_eq!(attrs.fsx_cowextsize, 44);
    }

    #[cfg(not(any(
        target_arch = "hexagon",
        target_arch = "sparc",
        target_arch = "sparc64"
    )))]
    #[test]
    fn test_fsgetxattr_opcode_matches_linux_raw_sys() {
        assert_eq!(
            FS_IOC_FSGETXATTR,
            linux_raw_sys::ioctl::FS_IOC_FSGETXATTR as ioctl::Opcode
        );
    }

    #[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
    #[test]
    fn test_fsgetxattr_opcode_matches_sparc_uapi() {
        assert_eq!(FS_IOC_FSGETXATTR, 0x401c_581f);
    }

    #[cfg(target_arch = "hexagon")]
    #[test]
    fn test_fsgetxattr_opcode_matches_hexagon_uapi() {
        assert_eq!(FS_IOC_FSGETXATTR, 0x801c_581f_u32 as ioctl::Opcode);
    }
}
