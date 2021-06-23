#![cfg(not(any(target_os = "redox", target_os = "wasi")))]

#[test]
fn test_mmap() {
    use posish::{
        fs::{cwd, openat, Mode, OFlags},
        io::{mmap, munmap, write, MapFlags, ProtFlags},
    };
    use std::{ptr::null_mut, slice};

    let tmp = tempfile::tempdir().unwrap();
    let dir = openat(cwd(), tmp.path(), OFlags::RDONLY, Mode::empty()).unwrap();

    let file = openat(
        &dir,
        "foo",
        OFlags::CREATE | OFlags::WRONLY | OFlags::TRUNC,
        Mode::IRUSR,
    )
    .unwrap();
    write(&file, &[b'a'; 8192]).unwrap();
    drop(file);

    let file = openat(&dir, "foo", OFlags::RDONLY, Mode::empty()).unwrap();
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
}
