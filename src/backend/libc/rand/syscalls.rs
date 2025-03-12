//! libc syscalls supporting `rustix::rand`.

#[cfg(linux_kernel)]
use {
    crate::backend::c, crate::backend::conv::ret_usize, crate::io, crate::rand::GetRandomFlags,
    core::mem::MaybeUninit,
};

#[cfg(linux_kernel)]
pub(crate) unsafe fn getrandom(
    buf: *mut [MaybeUninit<u8>],
    flags: GetRandomFlags,
) -> io::Result<usize> {
    // `getrandom` wasn't supported in glibc until 2.25.
    weak_or_syscall! {
        fn getrandom(buf: *mut c::c_void, buflen: c::size_t, flags: c::c_uint) via SYS_getrandom -> c::ssize_t
    }

    ret_usize(getrandom(buf.cast(), buf.len(), flags.bits()))
}
