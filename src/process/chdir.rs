#[cfg(not(target_os = "fuchsia"))]
use crate::backend::fd::AsFd;
#[cfg(feature = "fs")]
use crate::path;
#[cfg(any(feature = "fs", not(target_os = "fuchsia")))]
use crate::{backend, io};
#[cfg(all(feature = "alloc", feature = "fs"))]
use {
    crate::ffi::{CStr, CString},
    crate::path::SMALL_PATH_BUFFER_SIZE,
    alloc::vec::Vec,
};

/// `chdir(path)`—Change the current working directory.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/chdir.html
/// [Linux]: https://man7.org/linux/man-pages/man2/chdir.2.html
#[inline]
#[cfg(feature = "fs")]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
pub fn chdir<P: path::Arg>(path: P) -> io::Result<()> {
    path.into_with_c_str(backend::process::syscalls::chdir)
}

/// `fchdir(fd)`—Change the current working directory.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchdir.html
/// [Linux]: https://man7.org/linux/man-pages/man2/fchdir.2.html
#[cfg(not(target_os = "fuchsia"))]
#[inline]
pub fn fchdir<Fd: AsFd>(fd: Fd) -> io::Result<()> {
    backend::process::syscalls::fchdir(fd.as_fd())
}

/// `getcwd`—Return the current working directory.
///
/// If `reuse` already has available capacity, reuse it if possible.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getcwd.html
/// [Linux]: https://man7.org/linux/man-pages/man3/getcwd.3.html
#[cfg(all(feature = "alloc", feature = "fs"))]
#[cfg(not(target_os = "wasi"))]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[inline]
pub fn getcwd<B: Into<Vec<u8>>>(reuse: B) -> io::Result<CString> {
    _getcwd(reuse.into())
}

#[cfg(all(feature = "alloc", feature = "fs"))]
#[allow(unsafe_code)]
fn _getcwd(mut buffer: Vec<u8>) -> io::Result<CString> {
    buffer.clear();
    buffer.reserve(SMALL_PATH_BUFFER_SIZE);

    loop {
        match backend::process::syscalls::getcwd(buffer.spare_capacity_mut()) {
            Err(io::Errno::RANGE) => {
                // Use `Vec` reallocation strategy to grow capacity
                // exponentially.
                buffer.reserve(buffer.capacity() + 1);
            }
            Ok(_) => {
                // SAFETY:
                // - “These functions return a null-terminated string”
                // - [POSIX definition 3.375: String]: “A contiguous sequence
                //   of bytes terminated by and including the first null byte.”
                //
                // Thus, there will be a single NUL byte at the end of the
                // string.
                //
                // [POSIX definition 3.375: String]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_375
                unsafe {
                    buffer.set_len(
                        CStr::from_ptr(buffer.as_ptr().cast())
                            .to_bytes_with_nul()
                            .len(),
                    );

                    return Ok(CString::from_vec_with_nul_unchecked(buffer));
                }
            }
            Err(errno) => return Err(errno),
        }
    }
}

/// Whether `getcwd_raw` uses the existing allocation-free Linux-raw syscall.
///
/// This is build availability only, not a cwd, resource, or authority witness.
#[cfg(all(feature = "alloc", feature = "fs"))]
pub const GETCWD_RAW_SUPPORTED: bool = cfg!(linux_raw);

/// Read the current directory into caller-provided storage without allocating.
///
/// On Linux-raw, returns the sealed buffer output for bytes including the NUL.
/// Initialized byte slices return their initialized byte count.
/// Empty or insufficient storage is an error. This never grows the buffer or
/// falls back through libc or procfs. Other backends return `Errno::NOSYS`.
#[cfg(all(feature = "alloc", feature = "fs"))]
#[allow(unsafe_code)]
#[inline]
pub fn getcwd_raw<Buf: crate::buffer::Buffer<u8>>(buf: Buf) -> io::Result<Buf::Output> {
    #[cfg(linux_raw)]
    {
        let mut buf = buf;
        let (ptr, capacity) = buf.parts_mut();
        if capacity == 0 {
            return Err(io::Errno::RANGE);
        }
        // SAFETY: the sealed Buffer implementation supplies a valid uniquely
        // borrowed byte region for capacity bytes. MaybeUninit<u8> has the same
        // layout/alignment as u8; this borrow neither reads nor owns its bytes.
        let uninit = unsafe {
            core::slice::from_raw_parts_mut(ptr.cast::<core::mem::MaybeUninit<u8>>(), capacity)
        };
        let initialized = backend::process::syscalls::getcwd(uninit)?;
        if initialized == 0 || initialized > capacity {
            return Err(io::Errno::IO);
        }
        // SAFETY: successful Linux getcwd initialized exactly this bounded
        // prefix, including NUL. The syscall borrow ended before this sealed
        // Buffer operation publishes its initialized prefix; no growth occurs.
        unsafe { Ok(buf.assume_init(initialized)) }
    }
    #[cfg(not(linux_raw))]
    {
        let _ = buf;
        Err(io::Errno::NOSYS)
    }
}
