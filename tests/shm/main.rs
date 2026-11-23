//! Tests for [`rustix::shm`].

#![cfg(all(
    feature = "fs",
    feature = "shm",
    not(any(
        windows,
        target_os = "android",
        target_os = "espidf",
        target_os = "horizon",
        target_os = "vita",
        target_os = "wasi"
    ))
))]

use core::{ptr::null_mut, slice};

use rustix::{
    fs::ftruncate,
    mm::{mmap, munmap, MapFlags, ProtFlags},
    shm::{self, Mode},
};

#[cfg(target_os = "linux")]
use {
    rustix::{
        fs::{symlink, unlink},
        io::Errno,
        shm::OFlags,
    },
    std::ffi::OsStr,
};

struct ShmDropGuard(pub String);

impl Drop for ShmDropGuard {
    fn drop(&mut self) {
        let _ = shm::unlink(&self.0);
    }
}

#[test]
fn shm_roundtrip() {
    const MSG: &[u8] = b"test";

    let rand_name = tempfile::NamedTempFile::new()
        .unwrap()
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let name = format!("/rustix-{rand_name}");

    let fd = shm::open(
        &name,
        shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    )
    .unwrap();
    let _guard = ShmDropGuard(name);
    ftruncate(&fd, MSG.len() as u64).unwrap();

    unsafe {
        let addr = mmap(
            null_mut(),
            MSG.len(),
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        )
        .unwrap();

        let bytes = slice::from_raw_parts_mut(addr.cast::<u8>(), MSG.len());
        bytes.copy_from_slice(MSG);
        assert_eq!(bytes, MSG);

        munmap(addr, MSG.len()).unwrap();
    }
}

#[test]
#[cfg(target_os = "linux")]
fn shm_open_rejects_symlink() {
    let link_path = tempfile::Builder::new()
        .prefix("rustix-shm-test-")
        .tempfile_in("/dev/shm")
        .expect("/dev/shm should exist and be writable")
        .into_temp_path();

    unlink(&*link_path).unwrap();
    symlink("/dev/null", &*link_path).unwrap();

    let name = format!(
        "/{}",
        link_path.file_name().and_then(OsStr::to_str).unwrap()
    );
    match shm::open(&name, OFlags::RDONLY, Mode::empty()) {
        Err(Errno::LOOP) => {}
        Err(e) => panic!("shm::open returned an unexpected error: {e}"),
        Ok(_) => panic!("shm::open followed the symlink in /dev/shm"),
    }
}
