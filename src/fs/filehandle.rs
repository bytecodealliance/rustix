use core::mem::size_of;

use crate::{backend, ffi, io, path};
use backend::fd::{AsFd, OwnedFd};
use backend::fs::types::{HandleFlags, OFlags};

/// This maximum is more of a "guideline"; the man page for name_to_handle_at(2) indicates it could
/// increase in the future.
const MAX_HANDLE_SIZE: usize = 128;

/// The minimum size of a `struct file_handle` is the size of an int and an unsigned int, for the
/// length and type fields.
const HANDLE_STRUCT_SIZE: usize = size_of::<ffi::c_uint>() + size_of::<ffi::c_int>();

/// An opaque identifier for a file.
///
/// While the C struct definition in `fcntl.h` exposes fields like length and type, in reality,
/// user applications cannot usefully interpret (or modify) the separate fields of a file handle, so
/// this implementation treats the file handle as an entirely opaque sequence of bytes.
#[derive(Debug)]
pub struct FileHandle {
    raw: Box<[u8]>,
}

impl FileHandle {
    fn new(size: usize) -> Self {
        let handle_allocation_size: usize = HANDLE_STRUCT_SIZE + size;
        let bytes = vec![0; handle_allocation_size];

        let mut handle = Self {
            raw: Box::from(bytes),
        };
        handle.set_handle_bytes(size);

        handle
    }

    /// Create a file handle from a sequence of bytes.
    ///
    /// # Panics
    ///
    /// Panics if the given handle is malformed, suggesting that it did not originate from a
    /// previous call to name_to_handle_at().
    pub fn from_raw(raw: Box<[u8]>) -> Self {
        assert!(raw.len() >= HANDLE_STRUCT_SIZE);

        let handle = Self { raw };

        assert!(handle.raw.len() >= handle.get_handle_bytes() + HANDLE_STRUCT_SIZE);

        handle
    }

    /// Get the raw bytes of a file handle.
    pub fn into_raw(self) -> Box<[u8]> {
        self.raw
    }

    /// Set the `handle_bytes` field (first 4 bytes of the struct) to the given length.
    fn set_handle_bytes(&mut self, size: usize) {
        self.raw[0..size_of::<ffi::c_uint>()].copy_from_slice(&(size as ffi::c_uint).to_ne_bytes());
    }

    /// Get the length of the file handle data by reading the `handle_bytes` field
    fn get_handle_bytes(&self) -> usize {
        ffi::c_uint::from_ne_bytes(
            self.raw[0..size_of::<ffi::c_uint>()]
                .try_into()
                .expect("Vector should be long enough"),
        ) as usize
    }

    fn as_mut_ptr(&mut self) -> *mut ffi::c_void {
        self.raw.as_mut_ptr() as *mut _
    }

    fn as_ptr(&self) -> *const ffi::c_void {
        self.raw.as_ptr() as *const _
    }
}

/// An identifier for a mount that is returned by [`name_to_handle_at`].
///
/// [`name_to_handle_at`]: crate::fs::name_to_handle_at
#[derive(Debug)]
pub enum MountId {
    /// By default a MountId is a C int.
    Regular(ffi::c_int),
    /// When `AT_HANDLE_MNT_ID_UNIQUE` is passed in `HandleFlags`, MountId is a u64.
    Unique(u64),
}

/// `name_to_handle_at(dirfd, path, flags)` - Gets a filehandle given a path.
///
/// # References
///  - [Linux]
///
///  [Linux]: https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html
pub fn name_to_handle_at<Fd: AsFd, P: path::Arg>(
    dirfd: Fd,
    path: P,
    flags: HandleFlags,
) -> io::Result<(FileHandle, MountId)> {
    // name_to_handle_at(2) takes the mount_id parameter as either a 32-bit or 64-bit int pointer
    // depending on the flag AT_HANDLE_MNT_ID_UNIQUE
    let mount_id_unique: bool = flags.contains(HandleFlags::MNT_ID_UNIQUE);
    let mut mount_id_int: ffi::c_int = 0;
    let mut mount_id_64: u64 = 0;
    let mount_id_ptr: *mut ffi::c_void = if mount_id_unique {
        &mut mount_id_64 as *mut u64 as *mut _
    } else {
        &mut mount_id_int as *mut ffi::c_int as *mut _
    };

    // The MAX_HANDLE_SZ constant is not a fixed upper bound, because the kernel is permitted to
    // increase it in the future. So, the loop is needed in the rare case that MAX_HANDLE_SZ was
    // insufficient.
    let mut handle_size: usize = MAX_HANDLE_SIZE;
    path.into_with_c_str(|path| loop {
        let mut file_handle = FileHandle::new(handle_size);

        let ret = backend::fs::syscalls::name_to_handle_at(
            dirfd.as_fd(),
            path,
            file_handle.as_mut_ptr(),
            mount_id_ptr,
            flags,
        );

        // If EOVERFLOW was returned, and the handle size was increased, we need to try again with
        // a larger handle. If the handle size was not increased, EOVERFLOW was due to some other
        // cause, and should be returned to the user.
        if let Err(e) = ret {
            if e == io::Errno::OVERFLOW && file_handle.get_handle_bytes() > handle_size {
                handle_size = file_handle.get_handle_bytes();
                continue;
            }
        }

        let mount_id = if mount_id_unique {
            MountId::Unique(mount_id_64)
        } else {
            MountId::Regular(mount_id_int)
        };

        return ret.map(|_| (file_handle, mount_id));
    })
}

/// `open_by_handle_at(mount_fd, handle, flags)` - Open a file by filehandle.
///
/// # References
///  - [Linux]
///
///  [Linux]: https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html
pub fn open_by_handle_at<Fd: AsFd>(
    mount_fd: Fd,
    handle: &FileHandle,
    flags: OFlags,
) -> io::Result<OwnedFd> {
    backend::fs::syscalls::open_by_handle_at(mount_fd.as_fd(), handle.as_ptr(), flags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_to_handle() {
        let (_, mount_id) =
            name_to_handle_at(crate::fs::CWD, "Cargo.toml", HandleFlags::empty()).unwrap();
        assert!(matches!(mount_id, MountId::Regular(_)));

        match name_to_handle_at(crate::fs::CWD, "Cargo.toml", HandleFlags::MNT_ID_UNIQUE) {
            // On a new enough kernel, AT_HANDLE_MNT_ID_UNIQUE should succeed:
            Ok((_, mount_id)) => assert!(matches!(mount_id, MountId::Unique(_))),
            // But it should be rejected with -EINVAL on an older kernel:
            Err(e) => assert!(e == io::Errno::INVAL),
        }
    }
}
