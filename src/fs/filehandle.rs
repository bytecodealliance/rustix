use alloc::{boxed::Box, vec};
use core::mem::size_of;

use crate::{backend, ffi, io, path};
use backend::fd::{AsFd, OwnedFd};
use backend::fs::types::{HandleFlags, OFlags};

/// This maximum is more of a "guideline"; the man page for name_to_handle_at(2) indicates it could
/// increase in the future. This value is defined in libc `fcntl.h`.
const MAX_HANDLE_SIZE: usize = 128;

/// The minimum size of a `struct file_handle` is the size of an int and an unsigned int, for the
/// length and type fields.
const HANDLE_STRUCT_SIZE: usize = size_of::<ffi::c_uint>() + size_of::<ffi::c_int>();

/// An opaque identifier for a file.
///
/// While the C struct definition in `fcntl.h` exposes the length and type fields,
/// user applications cannot usefully interpret (or modify) those fields of a file handle, so
/// this implementation does not expose them.
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
        handle.set_handle_len(size);

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

        assert!(handle.raw.len() >= handle.get_handle_len() + HANDLE_STRUCT_SIZE);

        handle
    }

    /// Get the raw bytes of a file handle.
    pub fn into_raw(self) -> Box<[u8]> {
        self.raw
    }

    /// Borrow the raw bytes of a file handle.
    pub fn as_raw(&self) -> &[u8] {
        &self.raw
    }

    /// Get the `f_handle` field, i.e. the actual file handle contents, as a byte slice.
    pub fn get_handle_contents(&self) -> &[u8] {
        &self.raw[HANDLE_STRUCT_SIZE..]
    }

    /// Set the `handle_bytes` field (first 4 bytes of the struct) to the given length.
    fn set_handle_len(&mut self, size: usize) {
        self.raw[0..size_of::<ffi::c_uint>()].copy_from_slice(&(size as ffi::c_uint).to_ne_bytes());
    }

    /// Get the length of the file handle data by reading the `handle_bytes` field
    fn get_handle_len(&self) -> usize {
        ffi::c_uint::from_ne_bytes(
            self.raw[0..size_of::<ffi::c_uint>()]
                .try_into()
                .expect("Vector should be long enough"),
        ) as usize
    }

    fn as_mut_ptr(&mut self) -> *mut ffi::c_void {
        self.raw.as_mut_ptr() as *mut _
    }
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
) -> io::Result<(FileHandle, u64)> {
    // name_to_handle_at(2) takes the mount_id parameter as either a 32-bit or 64-bit int pointer
    // depending on the flag AT_HANDLE_MNT_ID_UNIQUE
    let mount_id_unique: bool = flags.contains(HandleFlags::MNT_ID_UNIQUE);
    let mut mount_id_64: u64 = 0;
    let mut mount_id_int: ffi::c_int = 0;

    let mount_id_ptr = if mount_id_unique {
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
            if e == io::Errno::OVERFLOW && file_handle.get_handle_len() > handle_size {
                handle_size = file_handle.get_handle_len();
                continue;
            }
        }

        let mount_id: u64 = if mount_id_unique {
            mount_id_64
        } else {
            mount_id_int as u64
        };

        return ret.map(|_| (file_handle, mount_id));
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_to_handle() {
        // On a new enough kernel, AT_HANDLE_MNT_ID_UNIQUE should succeed, but it should be rejected
        // with -EINVAL on an older kernel:
        if let Err(e) = name_to_handle_at(crate::fs::CWD, "Cargo.toml", HandleFlags::MNT_ID_UNIQUE)
        {
            assert!(e == io::Errno::INVAL);
        }
    }
}
