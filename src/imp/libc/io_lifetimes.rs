//! io_lifetimes types for Windows assuming that Fd is Socket.
//!
//! We can make this assumption since rsix supports only std::net on Windows.

pub use io_lifetimes::BorrowedSocket as BorrowedFd;
pub(crate) use io_lifetimes::OwnedSocket as OwnedFd;
#[cfg(not(feature = "rustc-dep-of-std"))]
pub(crate) use std::os::windows::io::RawSocket as RawFd;
pub(crate) use winapi::um::winsock2::SOCKET as LibcFd;

pub(crate) trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}
#[cfg(not(feature = "rustc-dep-of-std"))]
impl<T: std::os::windows::io::AsRawSocket> AsRawFd for T {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_raw_socket()
    }
}

pub(crate) trait IntoRawFd {
    fn into_raw_fd(self) -> RawFd;
}
#[cfg(not(feature = "rustc-dep-of-std"))]
impl<T: std::os::windows::io::IntoRawSocket> IntoRawFd for T {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.into_raw_socket()
    }
}

pub(crate) trait FromRawFd {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self;
}
#[cfg(not(feature = "rustc-dep-of-std"))]
impl<T: std::os::windows::io::FromRawSocket> FromRawFd for T {
    #[inline]
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self::from_raw_socket(raw_fd)
    }
}

pub use io_lifetimes::AsSocket as AsFd;

/// We define `AsFd` as an alias for `AsSocket`, but that doesn't provide
/// an `as_fd` function. This trait adapts an `AsSocket` implementation to
/// provide `as_fd` using a blanket implementation.
pub(crate) trait AsSocketAsFd {
    fn as_fd(&self) -> BorrowedFd;
}
impl<T: io_lifetimes::AsSocket> AsSocketAsFd for T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd {
        self.as_socket()
    }
}

pub(crate) trait IntoFd {
    fn into_fd(self) -> OwnedFd;
}
impl<T: io_lifetimes::IntoSocket> IntoFd for T {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.into_socket()
    }
}

pub(crate) trait FromFd {
    fn from_fd(fd: OwnedFd) -> Self;
}
impl<T: io_lifetimes::FromSocket> FromFd for T {
    #[inline]
    fn from_fd(fd: OwnedFd) -> Self {
        Self::from_socket(fd)
    }
}
