//! The WASI backend.
//!
//! This uses wit-bindgen with interface-types-based WASI API proposals.

wit_bindgen_rust::import!("src/imp/wasi/wasi-filesystem.wit.md");

pub(crate) mod fs;
pub(crate) mod io;
pub(crate) mod process;
pub(crate) mod thread;
pub(crate) mod time;

#[cfg(not(feature = "std"))]
pub(crate) mod fd {
    pub use crate::io::fd::*;
    pub(crate) use i32 as LibcFd;
}
#[cfg(feature = "std")]
pub(crate) mod fd {
    pub use io_lifetimes::*;

    #[allow(unused_imports)]
    pub(crate) use i32 as LibcFd;
    #[allow(unused_imports)]
    pub use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
}

impl fd::FromRawFd for wasi_filesystem::Descriptor {
    #[inline]
    unsafe fn from_raw_fd(raw: i32) -> Self {
        Self::from_raw(raw)
    }
}

impl fd::IntoRawFd for wasi_filesystem::Descriptor {
    #[inline]
    fn into_raw_fd(self) -> i32 {
        self.into_raw()
    }
}

impl fd::AsRawFd for wasi_filesystem::Descriptor {
    #[inline]
    fn as_raw_fd(&self) -> i32 {
        self.as_raw()
    }
}

impl fd::FromFd for wasi_filesystem::Descriptor {
    #[inline]
    fn from_fd(fd: fd::OwnedFd) -> Self {
        unsafe { Self::from_raw(fd::IntoRawFd::into_raw_fd(fd)) }
    }
}

impl fd::IntoFd for wasi_filesystem::Descriptor {
    #[inline]
    fn into_fd(self) -> fd::OwnedFd {
        let raw = self.as_raw();
        core::mem::forget(self);
        unsafe { fd::FromRawFd::from_raw_fd(raw) }
    }
}

impl fd::AsFd for wasi_filesystem::Descriptor {
    #[inline]
    fn as_fd(&self) -> fd::BorrowedFd<'_> {
        unsafe { fd::BorrowedFd::borrow_raw(self.as_raw()) }
    }
}
