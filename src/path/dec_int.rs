use crate::io::AsRawFd;
use io_lifetimes::AsFd;
use itoa::{write, Integer};
use std::ffi::OsStr;
use std::ops::Deref;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::path::Path;

/// Format an integer into a decimal `Path` component, without constructing a
/// temporary `PathBuf` or `String`.
///
/// This is used for opening paths such as `/proc/self/fd/<fd>` on Linux.
///
/// # Example
///
/// ```rust
/// use posish::path::DecInt;
///
/// assert_eq!(
///     format!("hello {}", DecInt::new(9876).display()),
///     "hello 9876"
/// );
/// ```
#[derive(Clone)]
pub struct DecInt {
    buf: [u8; 20],
    len: usize,
}

impl DecInt {
    /// Construct a new path component from an integer.
    #[inline]
    pub fn new<Int: Integer>(i: Int) -> Self {
        let mut me = Self {
            buf: [0; 20],
            len: 0,
        };
        me.len = write(&mut me.buf[..], i).unwrap();
        me
    }

    /// Construct a new path component from a file descriptor.
    #[inline]
    pub fn from_fd<Fd: AsFd>(fd: &Fd) -> Self {
        Self::new(fd.as_fd().as_raw_fd())
    }

    /// Return the raw byte buffer.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl Deref for DecInt {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let as_os_str: &OsStr = OsStrExt::from_bytes(&self.buf[..self.len]);
        Path::new(as_os_str)
    }
}

impl AsRef<Path> for DecInt {
    #[inline]
    fn as_ref(&self) -> &Path {
        &*self
    }
}
