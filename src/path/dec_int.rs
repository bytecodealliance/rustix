use itoa::{write, Integer};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::{ffi::OsStr, ops::Deref, path::Path};
use unsafe_io::os::posish::AsRawFd;

/// Format an integer into a decimal `Path` component, without constructing a
/// temporary `PathBuf` or `String`. This is used for opening paths such
/// as `/proc/self/fd/<fd>` on Linux.
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
    pub fn from_fd<Fd: AsRawFd>(fd: &Fd) -> Self {
        Self::new(fd.as_raw_fd())
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

#[test]
fn test_dec_int() {
    assert_eq!((*DecInt::new(0)).to_str().unwrap(), "0");
    assert_eq!((*DecInt::new(-1)).to_str().unwrap(), "-1");
    assert_eq!((*DecInt::new(789)).to_str().unwrap(), "789");
    assert_eq!(
        (*DecInt::new(i64::MIN)).to_str().unwrap(),
        i64::MIN.to_string()
    );
    assert_eq!(
        (*DecInt::new(i64::MAX)).to_str().unwrap(),
        i64::MAX.to_string()
    );
    assert_eq!(
        (*DecInt::new(u64::MAX)).to_str().unwrap(),
        u64::MAX.to_string()
    );
}
