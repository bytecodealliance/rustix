// Implementation derived from `weak` in Rust's
// library/std/src/sys/unix/weak.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

//! Support for "weak linkage" to symbols on Unix
//!
//! Some I/O operations we do in libstd require newer versions of OSes but we
//! need to maintain binary compatibility with older releases for now. In order
//! to use the new functionality when available we use this module for
//! detection.
//!
//! One option to use here is weak linkage, but that is unfortunately only
//! really workable on Linux. Hence, use dlsym to get the symbol value at
//! runtime. This is also done for compatibility with older versions of glibc,
//! and to avoid creating dependencies on `GLIBC_PRIVATE` symbols. It assumes that
//! we've been dynamically linked to the library the symbol comes from, but that
//! is currently always the case for things like libpthread/libc.
//!
//! A long time ago this used weak linkage for the `__pthread_get_minstack`
//! symbol, but that caused Debian to detect an unnecessarily strict versioned
//! dependency on libc6 (#23628).

// There are a variety of `#[cfg]`s controlling which targets are involved in
// each instance of `weak!` and `syscall!`. Rather than trying to unify all of
// that, we'll just allow that some unix targets don't use this module at all.
#![allow(dead_code, unused_macros)]
#![allow(clippy::doc_markdown)]

use std::{
    ffi::CStr,
    marker, mem,
    sync::atomic::{AtomicUsize, Ordering},
};

macro_rules! weak {
    (fn $name:ident($($t:ty),*) -> $ret:ty) => (
        #[allow(non_upper_case_globals)]
        static $name: crate::weak::Weak<unsafe extern fn($($t),*) -> $ret> =
            crate::weak::Weak::new(concat!(stringify!($name), '\0'));
    )
}

pub(crate) struct Weak<F> {
    name: &'static str,
    addr: AtomicUsize,
    _marker: marker::PhantomData<F>,
}

impl<F> Weak<F> {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            addr: AtomicUsize::new(1),
            _marker: marker::PhantomData,
        }
    }

    pub(crate) fn get(&self) -> Option<F> {
        assert_eq!(mem::size_of::<F>(), mem::size_of::<usize>());
        unsafe {
            if self.addr.load(Ordering::SeqCst) == 1 {
                self.addr.store(fetch(self.name), Ordering::SeqCst);
            }
            match self.addr.load(Ordering::SeqCst) {
                0 => None,
                addr => Some(mem::transmute_copy::<usize, F>(&addr)),
            }
        }
    }
}

unsafe fn fetch(name: &str) -> usize {
    let name = match CStr::from_bytes_with_nul(name.as_bytes()) {
        Ok(c_str) => c_str,
        Err(..) => return 0,
    };
    libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) as usize
}

#[cfg(not(target_os = "linux"))]
macro_rules! syscall {
    (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
        unsafe fn $name($($arg_name: $t),*) -> $ret {
            weak! { fn $name($($t),*) -> $ret }

            if let Some(fun) = $name.get() {
                fun($($arg_name),*)
            } else {
                errno::set_errno(errno::Errno(libc::ENOSYS));
                -1
            }
        }
    )
}

#[cfg(target_os = "linux")]
macro_rules! syscall {
    (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
        unsafe fn $name($($arg_name:$t),*) -> $ret {
            // This looks like a hack, but concat_idents only accepts idents
            // (not paths).
            use libc::*;

            syscall(
                concat_idents!(SYS_, $name),
                $($arg_name as c_long),*
            ) as $ret
        }
    )
}

macro_rules! weakcall {
    (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
        unsafe fn $name($($arg_name: $t),*) -> $ret {
            weak! { fn $name($($t),*) -> $ret }

            if let Some(fun) = $name.get() {
                fun($($arg_name),*)
            } else {
                errno::set_errno(errno::Errno(libc::ENOSYS));
                -1
            }
        }
    )
}
