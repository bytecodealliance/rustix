//! Linux loop block device operations.
//!
//! see: https://man7.org/linux/man-pages/man4/loop.4.html

#![allow(unsafe_code)]

use crate::backend::c;
use crate::fd::{AsFd, BorrowedFd};
use crate::{io, ioctl};
use bitflags::bitflags;
use linux_raw_sys::loop_device as lo;

/// Size of the name fields in [`LoopInfo`].
pub const LO_NAME_SIZE: usize = lo::LO_NAME_SIZE as usize;

/// Size of the [`LoopInfo::lo_encrypt_key`] field.
pub const LO_KEY_SIZE: usize = lo::LO_KEY_SIZE as usize;

bitflags! {
    /// `LO_FLAGS_*` constants for use with [`LoopInfo`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct LoopFlags: u32 {
        /// `LO_FLAGS_READ_ONLY`—The loop device is read-only.
        const READ_ONLY = lo::LO_FLAGS_READ_ONLY as u32;

        /// `LO_FLAGS_AUTOCLEAR`—The loop device detaches its backing file
        /// on last close (since Linux 2.6.25).
        const AUTOCLEAR = lo::LO_FLAGS_AUTOCLEAR as u32;

        /// `LO_FLAGS_PARTSCAN`—Allow automatic partition scanning (since
        /// Linux 3.2).
        const PARTSCAN = lo::LO_FLAGS_PARTSCAN as u32;

        /// `LO_FLAGS_DIRECT_IO`—Use direct I/O mode to access the backing
        /// file (since Linux 4.10).
        const DIRECT_IO = lo::LO_FLAGS_DIRECT_IO as u32;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

/// Status information of a loop device.
///
/// Fields documented as read-only are filled in by the kernel and ignored on
/// input.
///
/// The `Default` value is all zeros, which is a valid configuration that
/// maps the whole backing file starting at offset zero.
// <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/loop.h>
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[doc(alias = "loop_info64")]
pub struct LoopInfo {
    /// The device number of the backing file's device (read-only).
    pub lo_device: u64,

    /// The inode number of the backing file (read-only).
    pub lo_inode: u64,

    /// The device number of the loop device itself (read-only).
    pub lo_rdevice: u64,

    /// The offset in bytes into the backing file at which the loop device
    /// starts.
    pub lo_offset: u64,

    /// The size limit of the loop device in bytes; 0 means the maximum
    /// available.
    pub lo_sizelimit: u64,

    /// The number `N` of the loop device `/dev/loopN` (read-only).
    pub lo_number: u32,

    /// The encryption type (`LO_CRYPT_*`).
    ///
    /// Linux 5.19 removed support for encryption; this should be 0.
    #[deprecated]
    pub lo_encrypt_type: u32,

    /// The size of [`Self::lo_encrypt_key`] in use.
    ///
    /// Linux 5.19 removed support for encryption; this should be 0.
    #[deprecated]
    pub lo_encrypt_key_size: u32,

    /// Flags for the loop device.
    pub lo_flags: LoopFlags,

    /// The name of the backing file, as a NUL-padded byte string.
    ///
    /// This is informational only; the kernel does not use it to find the
    /// backing file.
    pub lo_file_name: [u8; LO_NAME_SIZE],

    /// The name of the encryption algorithm.
    ///
    /// Linux 5.19 removed support for encryption; this should be zeroed.
    #[deprecated]
    pub lo_crypt_name: [u8; LO_NAME_SIZE],

    /// The encryption key.
    ///
    /// Linux 5.19 removed support for encryption; this should be zeroed.
    #[deprecated]
    pub lo_encrypt_key: [u8; LO_KEY_SIZE],

    /// Initialization values for the encryption.
    ///
    /// Linux 5.19 removed support for encryption; this should be zeroed.
    #[deprecated]
    pub lo_init: [u64; 2],
}

impl Default for LoopInfo {
    #[inline]
    #[allow(deprecated)] // The obsolete encryption fields still need a value.
    fn default() -> Self {
        Self {
            lo_device: 0,
            lo_inode: 0,
            lo_rdevice: 0,
            lo_offset: 0,
            lo_sizelimit: 0,
            lo_number: 0,
            lo_encrypt_type: 0,
            lo_encrypt_key_size: 0,
            lo_flags: LoopFlags::empty(),
            lo_file_name: [0; LO_NAME_SIZE],
            lo_crypt_name: [0; LO_NAME_SIZE],
            lo_encrypt_key: [0; LO_KEY_SIZE],
            lo_init: [0; 2],
        }
    }
}

/// Configuration struct for [`ioctl_loop_configure`].
///
/// Create one with [`LoopConfig::new`], then adjust its public fields.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc(alias = "loop_config")]
pub struct LoopConfig<'fd> {
    /// The backing file or block device to attach to the loop device.
    pub fd: BorrowedFd<'fd>,

    /// The logical block size of the loop device, in bytes.
    ///
    /// This must be a power of two between 512 and the page size, or 0 to let
    /// the kernel choose a default.
    pub block_size: u32,

    /// The initial status of the loop device.
    pub info: LoopInfo,

    __reserved: [u64; 8],
}

impl<'fd> LoopConfig<'fd> {
    /// Create a new `LoopConfig` attaching `fd`, with the default block size
    /// and a zeroed [`LoopInfo`].
    #[inline]
    #[allow(deprecated)] // The obsolete encryption fields still need a value.
    pub const fn new(fd: BorrowedFd<'fd>) -> Self {
        Self {
            fd,
            block_size: 0,
            info: LoopInfo {
                lo_device: 0,
                lo_inode: 0,
                lo_rdevice: 0,
                lo_offset: 0,
                lo_sizelimit: 0,
                lo_number: 0,
                lo_encrypt_type: 0,
                lo_encrypt_key_size: 0,
                lo_flags: LoopFlags::empty(),
                lo_file_name: [0; LO_NAME_SIZE],
                lo_crypt_name: [0; LO_NAME_SIZE],
                lo_encrypt_key: [0; LO_KEY_SIZE],
                lo_init: [0; 2],
            },
            __reserved: [0; 8],
        }
    }
}

/// `ioctl(control, LOOP_CTL_GET_FREE)`—Find or allocate a free loop device.
///
/// `control` must be an open file descriptor for `/dev/loop-control`. On
/// success, returns the number `N` of a free loop device `/dev/loopN`,
/// allocating a new device if no free device exists.
///
/// The returned device is not reserved: another process may claim it before
/// the caller attaches a backing file. In that case
/// [`ioctl_loop_configure`] fails with [`io::Errno::BUSY`], and the caller
/// should request another free device.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/loop.4.html
#[inline]
#[doc(alias = "LOOP_CTL_GET_FREE")]
pub fn ioctl_loop_ctl_get_free<Fd: AsFd>(control: Fd) -> io::Result<u32> {
    // SAFETY: `LOOP_CTL_GET_FREE` takes no argument and returns the device
    // number.
    unsafe { ioctl::ioctl(control, LoopCtlGetFree) }
}

/// `ioctl(fd, LOOP_CONFIGURE, config)`—Attach a backing file to a loop
/// device and configure it.
///
/// `fd` must be an open file descriptor for a loop device `/dev/loopN`.
/// Attaches `config.fd` as the backing file, and applies `config.block_size`
/// and `config.info` in a single step.
///
/// The loop device keeps its own reference to the backing file, so
/// `config.fd` may be closed once this returns.
///
/// Fails with [`io::Errno::BUSY`] if the loop device already has a backing
/// file attached.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man4/loop.4.html
#[inline]
#[doc(alias = "LOOP_CONFIGURE")]
pub fn ioctl_loop_configure<Fd: AsFd>(fd: Fd, config: &LoopConfig<'_>) -> io::Result<()> {
    // SAFETY: `LOOP_CONFIGURE` is a pointer setter opcode that takes a
    // `struct loop_config`, which `LoopConfig` is layout-compatible with.
    unsafe {
        let ctl =
            ioctl::Setter::<{ lo::LOOP_CONFIGURE as ioctl::Opcode }, LoopConfig<'_>>::new(*config);
        ioctl::ioctl(fd, ctl)
    }
}

struct LoopCtlGetFree;

unsafe impl ioctl::Ioctl for LoopCtlGetFree {
    type Output = u32;

    const IS_MUTATING: bool = false;

    fn opcode(&self) -> ioctl::Opcode {
        lo::LOOP_CTL_GET_FREE as ioctl::Opcode
    }

    fn as_ptr(&mut self) -> *mut c::c_void {
        core::ptr::null_mut()
    }

    unsafe fn output_from_ptr(
        ret: ioctl::IoctlOutput,
        _: *mut c::c_void,
    ) -> io::Result<Self::Output> {
        // The kernel returns the non-negative device number on success.
        Ok(ret as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_layouts() {
        use linux_raw_sys::loop_device as c;

        check_renamed_struct!(
            LoopInfo,
            loop_info64,
            lo_device,
            lo_inode,
            lo_rdevice,
            lo_offset,
            lo_sizelimit,
            lo_number,
            lo_encrypt_type,
            lo_encrypt_key_size,
            lo_flags,
            lo_file_name,
            lo_crypt_name,
            lo_encrypt_key,
            lo_init
        );
        // The layout macros take a bare type name, which would elide
        // `LoopConfig`'s lifetime parameter; name it explicitly instead.
        type LoopConfig = super::LoopConfig<'static>;
        check_renamed_struct!(LoopConfig, loop_config, fd, block_size, info, __reserved);
    }
}
