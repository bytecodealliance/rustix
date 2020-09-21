use crate::path::DecInt;
#[cfg(target_os = "hermit")]
use std::os::hermit::ext::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(target_os = "vxworks")]
use std::os::vxworks::ext::ffi::OsStrExt;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    io,
    path::{Component, Components, Iter, Path, PathBuf},
    str,
};

/// A trait for passing path arguments. This is similar to
/// [`AsRef`]`<`[`Path`]`>`, but is implemented for more kinds of strings and
/// can convert into more kinds of strings.
///
/// [`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
/// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
pub trait Arg {
    /// Return a view of this string as a string slice.
    fn as_str(&self) -> io::Result<&str>;

    /// Return a potentially-lossy rendering of this string as a `Cow<str>`.
    fn to_string_lossy(&self) -> Cow<str>;

    /// Return a view of this string as a maybe-owend [`CStr`].
    ///
    /// [`CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
    #[cfg(not(windows))]
    fn as_cstr(&self) -> io::Result<Cow<CStr>>;

    /// Return a view of this string as a byte slice.
    #[cfg(not(windows))]
    fn as_maybe_utf8_bytes(&self) -> &[u8];

    /// Return a view of this string as a maybe-owend [`OsStr`].
    ///
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>>;
}

impl Arg for &str {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        Ok(self)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for String {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        Ok(self)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for &OsStr {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        OsStr::to_string_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_maybe_utf8_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for OsString {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_os_str().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_os_str().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_maybe_utf8_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for &Path {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_os_str().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        Path::to_string_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_os_str().as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for PathBuf {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_os_str().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_os_str().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_os_str().as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for &CStr {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.to_str().map_err(|_| utf8_error())
    }

    fn to_string_lossy(&self) -> Cow<str> {
        CStr::to_string_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Borrowed(self))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.to_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for CString {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_c_str().to_str().map_err(|_| utf8_error())
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_c_str().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Borrowed(self.as_c_str()))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl<'a> Arg for Cow<'a, OsStr> {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        (**self).to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        (**self).to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl<'a> Arg for Component<'a> {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_os_str().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_os_str().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_os_str().as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl<'a> Arg for Components<'a> {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_path().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_path().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(
            self.as_path().as_os_str().as_bytes(),
        )?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_path().as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl<'a> Arg for Iter<'a> {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_path().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        self.as_path().to_string_lossy()
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(
            self.as_path().as_os_str().as_bytes(),
        )?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_path().as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for &[u8] {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        str::from_utf8(self).map_err(|_| utf8_error())
    }

    fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(*self)?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for Vec<u8> {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        str::from_utf8(self).map_err(|_| utf8_error())
    }

    fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.clone())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

impl Arg for DecInt {
    #[inline]
    fn as_str(&self) -> io::Result<&str> {
        self.as_os_str().to_str().ok_or_else(utf8_error)
    }

    fn to_string_lossy(&self) -> Cow<str> {
        Path::to_string_lossy(self)
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_cstr(&self) -> io::Result<Cow<CStr>> {
        Ok(Cow::Owned(CString::new(self.as_os_str().as_bytes())?))
    }

    #[cfg(not(windows))]
    #[inline]
    fn as_maybe_utf8_bytes(&self) -> &[u8] {
        self.as_os_str().as_bytes()
    }

    #[cfg(windows)]
    fn as_os_str(&self) -> io::Result<Cow<OsStr>> {
        self.as_ref()
    }
}

fn utf8_error() -> io::Error {
    io::Error::from_raw_os_error(libc::EILSEQ)
}
