//! `Dir` and `Entry`.

use crate::{fs::FileType, io};
use io_lifetimes::{IntoFd, OwnedFd};
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "l4re",
    target_os = "linux"
)))]
use libc::{dirent as libc_dirent, readdir as libc_readdir};
#[cfg(all(
    libc,
    any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "l4re",
        target_os = "linux"
    )
))]
use libc::{dirent64 as libc_dirent, readdir64 as libc_readdir};
use std::ffi::CStr;
#[cfg(target_os = "wasi")]
use std::ffi::CString;
use unsafe_io::os::posish::{AsRawFd, RawFd};
#[cfg(linux_raw)]
use {
    crate::as_ptr, io_lifetimes::AsFd, linux_raw_sys::general::linux_dirent64, std::ffi::CString,
    std::mem::size_of,
};
#[cfg(libc)]
use {
    errno::{errno, set_errno, Errno},
    std::{mem::zeroed, ptr::NonNull},
    unsafe_io::{os::posish::IntoRawFd, OwnsRaw},
};

/// `DIR*`
#[cfg(libc)]
#[repr(transparent)]
pub struct Dir(NonNull<libc::DIR>);

/// `DIR*`
#[cfg(linux_raw)]
pub struct Dir {
    fd: OwnedFd,
    buf: Vec<u8>,
    pos: usize,
    next: Option<u64>,
}

impl Dir {
    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[inline]
    pub fn from<F: IntoFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_fd();
        Self::_from(fd)
    }

    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[inline]
    pub fn from_into_fd<F: IntoFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_fd();
        Self::_from(fd)
    }

    #[cfg(libc)]
    fn _from(fd: OwnedFd) -> io::Result<Self> {
        let raw = fd.into_raw_fd() as libc::c_int;
        unsafe {
            let d = libc::fdopendir(raw);
            if let Some(d) = NonNull::new(d) {
                Ok(Self(d))
            } else {
                let e = io::Error::last_os_error();
                let _ = libc::close(raw);
                Err(e)
            }
        }
    }

    #[cfg(linux_raw)]
    fn _from(fd: OwnedFd) -> io::Result<Self> {
        // Buffer size chosen by wild guess.
        let buf = vec![0; 1024];
        let pos = buf.len();
        Ok(Self {
            fd,
            buf,
            pos,
            next: None,
        })
    }

    /// `rewinddir(self)`
    #[cfg(libc)]
    #[inline]
    pub fn rewind(&mut self) {
        unsafe { libc::rewinddir(self.0.as_ptr()) }
    }

    /// `rewinddir(self)`
    #[cfg(linux_raw)]
    #[inline]
    pub fn rewind(&mut self) {
        self.pos = self.buf.len();
        self.next = Some(0);
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    #[cfg(libc)]
    pub fn read(&mut self) -> Option<io::Result<Entry>> {
        set_errno(Errno(0));
        let dirent_ptr = unsafe { libc_readdir(self.0.as_ptr()) };
        if dirent_ptr.is_null() {
            let curr_errno = errno().0;
            if curr_errno == 0 {
                // We successfully reached the end of the stream.
                None
            } else {
                // `errno` is unknown or non-zero, so an error occurred.
                Some(Err(io::Error(curr_errno)))
            }
        } else {
            // We successfully read an entry.
            unsafe {
                let result = Entry {
                    dirent: read_dirent(dirent_ptr),

                    #[cfg(target_os = "wasi")]
                    name: CStr::from_ptr((*dirent_ptr).d_name.as_ptr()).to_owned(),
                };

                Some(Ok(result))
            }
        }
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    #[cfg(linux_raw)]
    pub fn read(&mut self) -> Option<io::Result<Entry>> {
        if let Some(next) = self.next.take() {
            match crate::linux_raw::seek(
                self.fd.as_fd(),
                next as i64,
                linux_raw_sys::general::SEEK_SET,
            ) {
                Ok(_) => (),
                Err(err) => return Some(Err(err)),
            }
        }

        // Compute linux_dirent64 field offsets.
        let z = linux_dirent64 {
            d_ino: 0_u64,
            d_off: 0_i64,
            d_type: 0_u8,
            d_reclen: 0_u16,
            d_name: Default::default(),
        };
        let base = as_ptr(&z) as usize;
        let offsetof_d_reclen = (as_ptr(&z.d_reclen) as usize) - base;
        let offsetof_d_name = (as_ptr(&z.d_name) as usize) - base;
        let offsetof_d_ino = (as_ptr(&z.d_ino) as usize) - base;
        let offsetof_d_type = (as_ptr(&z.d_type) as usize) - base;

        // Test if we need more entries, and if so, read more.
        if self.buf.len() - self.pos < size_of::<linux_dirent64>() {
            match self.read_more()? {
                Ok(()) => (),
                Err(e) => return Some(Err(e)),
            }
        }

        // We successfully read an entry. Extract the fields.
        let pos = self.pos;

        // Do an unaligned u16 load.
        let d_reclen = u16::from_ne_bytes([
            self.buf[pos + offsetof_d_reclen + 0],
            self.buf[pos + offsetof_d_reclen + 1],
        ]);
        assert!(self.buf.len() - pos >= d_reclen as usize);
        self.pos += d_reclen as usize;

        // Read the NUL-terminated name from the `d_name` field. Without
        // `unsafe`, we need to scan for the NUL twice: once to obtain a size
        // for the slice, and then once within `CStr::from_bytes_with_nul`.
        let name_start = pos + offsetof_d_name;
        let name_end = self.buf[name_start..].iter().position(|x| *x == b'\0').unwrap();
        let name = CStr::from_bytes_with_nul(&self.buf[name_start..name_end + 1]).unwrap();
        let name = name.to_owned();
        assert!(name.as_bytes().len() <= self.buf.len() - name_start);

        // Do an unaligned u64 load.
        let d_ino = u64::from_ne_bytes([
            self.buf[pos + offsetof_d_ino + 0],
            self.buf[pos + offsetof_d_ino + 1],
            self.buf[pos + offsetof_d_ino + 2],
            self.buf[pos + offsetof_d_ino + 3],
            self.buf[pos + offsetof_d_ino + 4],
            self.buf[pos + offsetof_d_ino + 5],
            self.buf[pos + offsetof_d_ino + 6],
            self.buf[pos + offsetof_d_ino + 7],
        ]);

        let d_type = self.buf[pos + offsetof_d_type];

        // Check that our types correspond to the `linux_dirent64` types.
        let _ = linux_dirent64 {
            d_ino,
            d_off: 0,
            d_type,
            d_reclen,
            d_name: Default::default(),
        };

        Some(Ok(Entry {
            d_ino,
            d_type,
            name: name.to_owned(),
        }))
    }

    #[cfg(linux_raw)]
    fn read_more(&mut self) -> Option<io::Result<()>> {
        self.buf.resize(self.buf.capacity() + 1, 0);
        self.pos = 0;
        let nread = match crate::linux_raw::getdents(self.fd.as_fd(), &mut self.buf) {
            Ok(nread) => nread,
            Err(err) => return Some(Err(err)),
        };
        self.buf.resize(nread, 0);
        if nread == 0 {
            None
        } else {
            Some(Ok(()))
        }
    }
}

// A `dirent` pointer returned from `readdir` may not point to a full `dirent`
// struct, as the name is NUL-terminated and memory may not be allocated for
// the full extent of the struct. Copy the fields one at a time.
#[cfg(libc)]
unsafe fn read_dirent(dirent_ptr: *const libc_dirent) -> libc_dirent {
    let d_type = (*dirent_ptr).d_type;

    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    let d_off = (*dirent_ptr).d_off;

    #[cfg(not(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
    let d_ino = (*dirent_ptr).d_ino;

    #[cfg(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    let d_fileno = (*dirent_ptr).d_fileno;

    #[cfg(not(target_os = "wasi"))]
    let d_reclen = (*dirent_ptr).d_reclen;

    #[cfg(any(
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "ios",
        target_os = "macos"
    ))]
    let d_namlen = (*dirent_ptr).d_namlen;

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let d_seekoff = (*dirent_ptr).d_seekoff;

    // Construct the dirent. Rust will give us an error if any OS has a dirent
    // with a field that we missed here. And we can avoid blindly copying the
    // whole `d_name` field, which may not be entirely allocated.
    #[cfg_attr(target_os = "wasi", allow(unused_mut))]
    let mut dirent = libc_dirent {
        d_type,
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "wasi",
        )))]
        d_off,
        #[cfg(not(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
        d_ino,
        #[cfg(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        d_fileno,
        #[cfg(not(target_os = "wasi"))]
        d_reclen,
        #[cfg(any(
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "ios",
            target_os = "macos"
        ))]
        d_namlen,
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        d_seekoff,
        // The `d_name` field is NUL-terminated, and we need to be careful not
        // to read bytes past the NUL, even though they're within the nominal
        // extent of the `struct dirent`, because they may not be allocated. So
        // don't read it from `dirent_ptr`.
        //
        // In theory this could use `MaybeUninit::uninit().assume_init()`, but
        // that [invokes undefined behavior].
        //
        // [invokes undefined behavior]: https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#initialization-invariant
        d_name: zeroed(),
    };

    // Copy from d_name, reading up to and including the first NUL.
    #[cfg(not(target_os = "wasi"))]
    {
        let name_len = CStr::from_ptr((*dirent_ptr).d_name.as_ptr())
            .to_bytes()
            .len()
            + 1;
        dirent.d_name[..name_len].copy_from_slice(&(*dirent_ptr).d_name[..name_len]);
    }

    dirent
}

/// `Dir` implements `Send` but not `Sync`, because we use `readdir` which is
/// not guaranteed to be thread-safe. Users can wrap this in a `Mutex` if they
/// need `Sync`, which is effectively what'd need to do to implement `Sync`
/// ourselves.
#[cfg(libc)]
unsafe impl Send for Dir {}

#[cfg(libc)]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        unsafe { libc::dirfd(self.0.as_ptr()) as RawFd }
    }
}

#[cfg(linux_raw)]
impl AsRawFd for Dir {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

// Safety: `Dir` owns its handle.
#[cfg(libc)]
unsafe impl OwnsRaw for Dir {}

#[cfg(libc)]
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
#[cfg(libc)]
#[derive(Debug)]
pub struct Entry {
    dirent: libc_dirent,

    #[cfg(target_os = "wasi")]
    name: CString,
}

/// `struct dirent`
#[cfg(linux_raw)]
#[derive(Debug)]
pub struct Entry {
    d_ino: u64,
    d_type: u8,
    name: CString,
}

#[cfg(libc)]
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
    #[cfg(not(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
    #[inline]
    pub fn ino(&self) -> u64 {
        self.dirent.d_ino
    }

    /// Return the inode number of this directory entry.
    #[cfg(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    #[inline]
    pub fn ino(&self) -> u64 {
        #[allow(clippy::useless_conversion)]
        self.dirent.d_fileno.into()
    }
}

#[cfg(linux_raw)]
impl Entry {
    /// Returns the file name of this directory entry.
    #[inline]
    pub fn file_name(&self) -> &CStr {
        &self.name
    }

    /// Returns the type of this directory entry.
    #[inline]
    pub fn file_type(&self) -> FileType {
        FileType::from_dirent_d_type(self.d_type)
    }

    /// Return the inode number of this directory entry.
    #[inline]
    pub fn ino(&self) -> u64 {
        self.d_ino
    }
}
