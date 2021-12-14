use super::super::wasi_filesystem;
use super::FileType;
use crate::as_ptr;
#[cfg(feature = "std")]
use crate::fd::IntoFd;
use crate::fd::{AsFd, BorrowedFd};
use crate::ffi::{CStr, CString};
use crate::io::{self, OwnedFd};
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::mem::size_of;
//use wasi_filesystem::dirent;

/// `DIR*`
pub struct Dir {
    fd: OwnedFd,
    buf: Vec<u8>,
    pos: usize,
    next: Option<u64>,
}

impl Dir {
    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[cfg(not(any(io_lifetimes_use_std, not(feature = "std"))))]
    #[inline]
    pub fn from<F: IntoFd>(fd: F) -> io::Result<Self> {
        let fd = fd.into_fd();
        Self::_from(fd.into())
    }

    /// Construct a `Dir`, assuming ownership of the file descriptor.
    #[cfg(any(io_lifetimes_use_std, not(feature = "std")))]
    #[inline]
    pub fn from<F: Into<OwnedFd>>(fd: F) -> io::Result<Self> {
        let fd = fd.into();
        Self::_from(fd)
    }

    #[inline]
    fn _from(fd: OwnedFd) -> io::Result<Self> {
        Ok(Self {
            fd,
            buf: Vec::new(),
            pos: 0,
            next: None,
        })
    }

    /// `rewinddir(self)`
    #[inline]
    pub fn rewind(&mut self) {
        self.pos = self.buf.len();
        self.next = Some(0);
    }

    /// `readdir(self)`, where `None` means the end of the directory.
    pub fn read(&mut self) -> Option<io::Result<DirEntry>> {
        todo!("readdir")
    }

    fn read_more(&mut self) -> Option<io::Result<()>> {
        todo!("read_more")
    }
}

impl AsFd for Dir {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Iterator for Dir {
    type Item = io::Result<DirEntry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Self::read(self)
    }
}

/// `struct dirent`
#[derive(Debug)]
pub struct DirEntry {
    d_ino: u64,
    d_type: u8,
    name: CString,
}

impl DirEntry {
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
