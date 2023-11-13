#[cfg(feature = "fs")]
#[cfg(not(target_os = "redox"))]
#[test]
fn test_mmap() {
    use rustix::fs::{openat, Mode, OFlags, CWD};
    use rustix::io::write;
    use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
    use std::ptr::null_mut;
    use std::slice;

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(CWD, tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let file = openat(
        &dir,
        "file",
        OFlags::CREATE | OFlags::WRONLY | OFlags::TRUNC,
        Mode::RUSR,
    )
    .unwrap();
    write(&file, &[b'a'; 8192]).unwrap();
    drop(file);

    let file = openat(&dir, "file", OFlags::RDONLY, Mode::empty()).unwrap();
    unsafe {
        let addr = mmap(
            null_mut(),
            8192,
            ProtFlags::READ,
            MapFlags::PRIVATE,
            &file,
            0,
        )
        .unwrap();
        let slice = slice::from_raw_parts(addr.cast::<u8>(), 8192);
        assert_eq!(slice, &[b'a'; 8192]);

        munmap(addr, 8192).unwrap();
    }

    let file = openat(&dir, "file", OFlags::RDONLY, Mode::empty()).unwrap();
    unsafe {
        assert_eq!(
            mmap(
                null_mut(),
                8192,
                ProtFlags::READ,
                MapFlags::PRIVATE,
                &file,
                u64::MAX,
            )
            .unwrap_err()
            .raw_os_error(),
            libc::EINVAL
        );
    }
}

#[test]
fn test_mmap_anonymous() {
    use rustix::mm::{mmap_anonymous, munmap, MapFlags, ProtFlags};
    use std::ptr::null_mut;
    use std::slice;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();
        let slice = slice::from_raw_parts(addr.cast::<u8>(), 8192);
        assert_eq!(slice, &[b'\0'; 8192]);

        munmap(addr, 8192).unwrap();
    }
}

#[test]
fn test_mprotect() {
    use rustix::mm::{mmap_anonymous, mprotect, munmap, MapFlags, MprotectFlags, ProtFlags};
    use std::ptr::null_mut;
    use std::slice;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();

        mprotect(addr, 8192, MprotectFlags::empty()).unwrap();
        mprotect(addr, 8192, MprotectFlags::READ).unwrap();

        let slice = slice::from_raw_parts(addr.cast::<u8>(), 8192);
        assert_eq!(slice, &[b'\0'; 8192]);

        munmap(addr, 8192).unwrap();
    }
}

#[test]
fn test_mlock() {
    use rustix::mm::{mlock, mmap_anonymous, munlock, munmap, MapFlags, ProtFlags};
    #[cfg(linux_kernel)]
    use rustix::mm::{mlock_with, MlockFlags};
    use std::ptr::null_mut;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();

        match mlock(addr, 8192) {
            Ok(()) => munlock(addr, 8192).unwrap(),
            // Tests won't always have enough memory or permissions, and that's ok.
            Err(rustix::io::Errno::PERM | rustix::io::Errno::NOMEM) => (),
            // But they shouldn't fail otherwise.
            Err(other) => panic!("{:?}", other),
        }

        #[cfg(linux_kernel)]
        {
            match mlock_with(addr, 8192, MlockFlags::empty()) {
                Err(rustix::io::Errno::NOSYS) => (),
                Err(err) => panic!("{:?}", err),
                Ok(()) => munlock(addr, 8192).unwrap(),
            }

            match mlock_with(addr, 8192, MlockFlags::ONFAULT) {
                // Linux versions that lack `mlock` return this.
                Err(rustix::io::Errno::NOSYS) => (),
                // Linux versions that don't recognize `ONFAULT` return this.
                Err(rustix::io::Errno::INVAL) => (),
                Err(err) => panic!("{:?}", err),
                Ok(()) => munlock(addr, 8192).unwrap(),
            }

            munlock(addr, 8192).unwrap();
        }

        munmap(addr, 8192).unwrap();
    }
}

#[cfg(not(target_os = "redox"))]
#[test]
fn test_madvise() {
    use rustix::mm::{madvise, mmap_anonymous, munmap, Advice, MapFlags, ProtFlags};
    use std::ptr::null_mut;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();

        madvise(addr, 8192, Advice::Normal).unwrap();
        madvise(addr, 8192, Advice::DontNeed).unwrap();

        #[cfg(linux_kernel)]
        madvise(addr, 8192, Advice::LinuxDontNeed).unwrap();

        munmap(addr, 8192).unwrap();
    }
}

#[test]
fn test_msync() {
    use rustix::mm::{mmap_anonymous, msync, munmap, MapFlags, MsyncFlags, ProtFlags};
    use std::ptr::null_mut;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();

        msync(addr, 8192, MsyncFlags::SYNC).unwrap();
        msync(addr, 8192, MsyncFlags::ASYNC).unwrap();

        munmap(addr, 8192).unwrap();
    }
}

#[cfg(any(target_os = "emscripten", target_os = "linux"))]
#[test]
fn test_mremap() {
    use rustix::mm::{mmap_anonymous, mremap, munmap, MapFlags, MremapFlags, ProtFlags};
    use std::ptr::null_mut;

    unsafe {
        let addr = mmap_anonymous(null_mut(), 8192, ProtFlags::READ, MapFlags::PRIVATE).unwrap();

        assert_eq!(
            mremap(addr, 4096, 16384, MremapFlags::empty()),
            Err(rustix::io::Errno::NOMEM)
        );
        let new = mremap(addr, 4096, 16384, MremapFlags::MAYMOVE).unwrap();
        assert_ne!(new, addr);
        assert!(!new.is_null());

        munmap(new, 16384).unwrap();
        munmap(addr.offset(4096), 4096).unwrap();
    }
}
