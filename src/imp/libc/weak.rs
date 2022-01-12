// Implementation derived from `weak` in Rust's
// library/std/src/sys/unix/weak.rs at revision
// fd0cb0cdc21dd9c06025277d772108f8d42cb25f.

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
//! and to avoid creating dependencies on `GLIBC_PRIVATE` symbols. It assumes
//! that we've been dynamically linked to the library the symbol comes from,
//! but that is currently always the case for things like libpthread/libc.
//!
//! A long time ago this used weak linkage for the `__pthread_get_minstack`
//! symbol, but that caused Debian to detect an unnecessarily strict versioned
//! dependency on libc6 (#23628).

// There are a variety of `#[cfg]`s controlling which targets are involved in
// each instance of `weak!` and `syscall!`. Rather than trying to unify all of
// that, we'll just allow that some unix targets don't use this module at all.
#![allow(dead_code, unused_macros)]
#![allow(clippy::doc_markdown)]

use crate::ffi::ZStr;
use core::sync::atomic::{self, AtomicUsize, Ordering};
use core::{marker, mem};

macro_rules! weak {
    (fn $name:ident($($t:ty),*) -> $ret:ty) => (
        #[allow(non_upper_case_globals)]
        static $name: crate::imp::weak::Weak<unsafe extern fn($($t),*) -> $ret> =
            crate::imp::weak::Weak::new(concat!(stringify!($name), '\0'));
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
            // Relaxed is fine here because we fence before reading through the
            // pointer (see the comment below).
            match self.addr.load(Ordering::Relaxed) {
                1 => self.initialize(),
                0 => None,
                addr => {
                    let func = mem::transmute_copy::<usize, F>(&addr);
                    // The caller is presumably going to read through this value
                    // (by calling the function we've dlsymed). This means we'd
                    // need to have loaded it with at least C11's consume
                    // ordering in order to be guaranteed that the data we read
                    // from the pointer isn't from before the pointer was
                    // stored. Rust has no equivalent to memory_order_consume,
                    // so we use an acquire fence (sorry, ARM).
                    //
                    // Now, in practice this likely isn't needed even on CPUs
                    // where relaxed and consume mean different things. The
                    // symbols we're loading are probably present (or not) at
                    // init, and even if they aren't the runtime dynamic loader
                    // is extremely likely have sufficient barriers internally
                    // (possibly implicitly, for example the ones provided by
                    // invoking `mprotect`).
                    //
                    // That said, none of that's *guaranteed*, and so we fence.
                    atomic::fence(Ordering::Acquire);
                    Some(func)
                }
            }
        }
    }

    // Cold because it should only happen during first-time initialization.
    #[cold]
    unsafe fn initialize(&self) -> Option<F> {
        let val = fetch(self.name);
        // This synchronizes with the acquire fence in `get`.
        self.addr.store(val, Ordering::Release);

        match val {
            0 => None,
            addr => Some(mem::transmute_copy::<usize, F>(&addr)),
        }
    }
}

unsafe fn fetch(name: &str) -> usize {
    let name = match ZStr::from_bytes_with_nul(name.as_bytes()) {
        Ok(c_str) => c_str,
        Err(..) => return 0,
    };
    libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr().cast()) as usize
}

#[cfg(not(any(target_os = "android", target_os = "linux")))]
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

#[cfg(any(target_os = "android", target_os = "linux"))]
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
    ($vis:vis fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
        $vis unsafe fn $name($($arg_name: $t),*) -> $ret {
            weak! { fn $name($($t),*) -> $ret }

            // Use a weak symbol from libc when possible, allowing `LD_PRELOAD`
            // interposition, but if it's not found just fail.
            if let Some(fun) = $name.get() {
                fun($($arg_name),*)
            } else {
                errno::set_errno(errno::Errno(libc::ENOSYS));
                -1
            }
        }
    )
}
