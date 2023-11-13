//! Tests for `mlock`.
//!
//! We can't easily test that it actually locks memory, but we can test that we
//! can call it and either get success or a reasonable error message.

use std::ffi::c_void;

#[test]
fn test_mlock() {
    let mut buf = vec![0_u8; 4096];
    let ptr = buf.as_mut_ptr();

    // On Linux, `mlock` automatically rounds the address down to the nearest
    // page size. On other platforms, we need to do it manually.
    #[cfg(not(linux_kernel))]
    let ptr = ((ptr as usize) & (-4096_isize) as usize) as *mut u8;

    unsafe {
        match rustix::mm::mlock(ptr.cast::<c_void>(), buf.len()) {
            Ok(()) => rustix::mm::munlock(ptr.cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Errno::PERM | rustix::io::Errno::NOMEM) => {}
            // But they shouldn't fail otherwise.
            Err(other) => panic!("{:?}", other),
        }
    }
}

#[cfg(linux_kernel)]
#[test]
fn test_mlock_with() {
    let mut buf = vec![0_u8; 4096];

    unsafe {
        match rustix::mm::mlock_with(
            buf.as_mut_ptr().cast::<c_void>(),
            buf.len(),
            rustix::mm::MlockFlags::empty(),
        ) {
            Ok(()) => rustix::mm::munlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Errno::PERM | rustix::io::Errno::NOMEM | rustix::io::Errno::NOSYS) => {}
            // But they shouldn't fail otherwise.
            Err(other) => panic!("{:?}", other),
        }
    }
}

#[cfg(linux_kernel)]
#[test]
fn test_mlock_with_onfault() {
    // With glibc, `mlock2` with `MLOCK_ONFAULT` returns `EINVAL` if the
    // `mlock2` system call returns `ENOSYS`. That's not what we want
    // here though, because `ENOSYS` just means the OS doesn't have
    // `mlock2`, while `EINVAL` may indicate a bug in rustix.
    //
    // To work around this, we use `libc::syscall` to make a `mlock2`
    // syscall directly to test for `ENOSYS`, before running the main
    // test below.
    unsafe {
        if libc::syscall(libc::SYS_mlock2, 0, 0, 0) == -1 && libc_errno::errno().0 == libc::ENOSYS {
            return;
        }
    }

    let mut buf = vec![0_u8; 4096];

    unsafe {
        match rustix::mm::mlock_with(
            buf.as_mut_ptr().cast::<c_void>(),
            buf.len(),
            rustix::mm::MlockFlags::ONFAULT,
        ) {
            Ok(()) => rustix::mm::munlock(buf.as_mut_ptr().cast::<c_void>(), buf.len()).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Errno::PERM | rustix::io::Errno::NOMEM | rustix::io::Errno::NOSYS) => {}
            // But they shouldn't fail otherwise.
            Err(other) => panic!("{:?}", other),
        }
    }
}
