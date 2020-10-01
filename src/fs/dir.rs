//! `Dir`, `Entry`, and `SeekLoc`.

#[cfg(target_os = "android")]
use crate::fs::android::{seekdir as libc_seekdir, telldir as libc_telldir};
use crate::fs::FileType;
use errno::{set_errno, Errno};
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux"
)))]
use libc::{dirent as libc_dirent, readdir as libc_readdir};
#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux"
))]
use libc::{dirent64 as libc_dirent, readdir64 as libc_readdir};
#[cfg(not(target_os = "android"))]
use libc::{seekdir as libc_seekdir, telldir as libc_telldir};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::{convert::TryInto, ffi::CStr, io, ptr};
#[cfg(target_os = "wasi")]
use std::{
    ffi::CString,
    mem::MaybeUninit,
    os::wasi::io::{AsRawFd, IntoRawFd, RawFd},
};

/// `DIR*`
pub struct Dir(ptr::NonNull<libc::DIR>);

impl Dir {
    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[inline]
    pub fn from<F: IntoRawFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_raw_fd();
        unsafe { Self::_from(fd) }
    }

    unsafe fn _from(fd: RawFd) -> io::Result<Self> {
        let d = libc::fdopendir(fd as libc::c_int);
        if let Some(d) = ptr::NonNull::new(d) {
            Ok(Self(d))
        } else {
            let e = io::Error::last_os_error();
            libc::close(fd as libc::c_int);
            Err(e)
        }
    }

    /// `seekdir(self, loc)`
    #[inline]
    pub fn seek(&self, loc: SeekLoc) {
        unsafe { libc_seekdir(self.0.as_ptr(), loc.0) }
    }

    /// `telldir(self)`
    #[inline]
    pub fn tell(&self) -> SeekLoc {
        SeekLoc(unsafe { libc_telldir(self.0.as_ptr()) })
    }

    /// `rewinddir(self)`
    #[inline]
    pub fn rewind(&self) {
        unsafe { libc::rewinddir(self.0.as_ptr()) }
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    pub fn read(&self) -> Option<io::Result<Entry>> {
        set_errno(Errno(0));
        let dirent = unsafe { libc_readdir(self.0.as_ptr()) };
        if dirent.is_null() {
            let curr_errno = io::Error::last_os_error();
            if curr_errno.raw_os_error() == Some(0) {
                // We successfully reached the end of the stream.
                None
            } else {
                // `errno` is unknown or non-zero, so an error occurred.
                Some(Err(curr_errno))
            }
        } else {
            // We successfully read an entry.
            Some(Ok(unsafe {
                Entry {
                    #[cfg(not(target_os = "wasi"))]
                    dirent: *dirent,

                    // TODO: When WASI gains a `d_loc` field, update `Entry::seek_loc`.
                    #[cfg(target_os = "wasi")]
                    dirent: libc_dirent {
                        d_ino: (*dirent).d_ino,
                        d_type: (*dirent).d_type,
                        d_name: MaybeUninit::uninit().assume_init(),
                    },

                    #[cfg(target_os = "wasi")]
                    name: CStr::from_ptr((*dirent).d_name.as_ptr()).to_owned(),
                }
            }))
        }
    }
}

unsafe impl Send for Dir {}
unsafe impl Sync for Dir {}

impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        unsafe { libc::dirfd(self.0.as_ptr()) as RawFd }
    }
}

impl Drop for Dir {
    #[inline]
    fn drop(&mut self) {
        unsafe { libc::closedir(self.0.as_ptr()) };
    }
}

impl Iterator for Dir {
    type Item = io::Result<Entry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Self::read(self)
    }
}

/// `struct dirent`
#[derive(Debug)]
pub struct Entry {
    dirent: libc_dirent,

    #[cfg(target_os = "wasi")]
    name: CString,
}

impl Entry {
    /// Returns the file name of this directory entry.
    #[inline]
    pub fn file_name(&self) -> &CStr {
        #[cfg(not(target_os = "wasi"))]
        unsafe {
            CStr::from_ptr(self.dirent.d_name.as_ptr())
        }

        #[cfg(target_os = "wasi")]
        &self.name
    }

    /// Returns the type of this directory entry.
    #[inline]
    pub fn file_type(&self) -> FileType {
        FileType::from_dirent_d_type(self.dirent.d_type)
    }

    /// Return the inode number of this directory entry.
    #[cfg(not(any(target_os = "freebsd", target_os = "netbsd")))]
    #[inline]
    pub fn ino(&self) -> u64 {
        self.dirent.d_ino
    }

    /// Return the inode number of this directory entry.
    #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
    #[inline]
    pub fn ino(&self) -> u64 {
        #[allow(clippy::useless_conversion)]
        self.dirent.d_fileno.into()
    }

    /// Return a cookie indicating the location of this entry, for use with [`Dir::seek`].
    ///
    /// [`Dir::seek`]: struct.Dir.html#method.seek
    ///
    /// TODO: Use `d_loc` on WASI once we have libc support.
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    #[inline]
    pub fn seek_loc(&self) -> io::Result<SeekLoc> {
        let off_i64: i64 = self.dirent.d_off;
        unsafe { SeekLoc::from_raw(off_i64 as u64) }
    }
}

/// A location for use with [`Dir::seek`].
///
/// [`Dir::seek`]: struct.Dir.html#method.seek
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SeekLoc(libc::c_long);

impl SeekLoc {
    /// Return the location encoded as a `u64`. Note that this value is meant to
    /// be opaque, and applications shouldn't do anything with it except call
    /// `SeekLoc::from_raw`.
    #[inline]
    #[allow(clippy::useless_conversion)]
    pub fn to_raw(&self) -> u64 {
        i64::from(self.0) as u64
    }

    /// Construct a new `SeekLoc` from a value returned by `SeekLoc::to_raw`.
    ///
    /// # Safety
    ///
    /// The passed-in `loc` value must be a value returned from
    /// `SeekLoc::to_raw`.
    #[inline]
    pub unsafe fn from_raw(loc: u64) -> io::Result<Self> {
        Ok(Self(
            loc.try_into()
                .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?,
        ))
    }
}
