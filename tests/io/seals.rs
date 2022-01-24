use rustix::fd::FromFd;
use rustix::fs::{fcntl_add_seals, ftruncate, memfd_create, MemfdFlags, SealFlags};
use std::fs::File;
use std::io::Write;

#[test]
fn test_seals() {
    let fd = memfd_create("test", MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING).unwrap();
    let mut file = File::from_fd(fd.into());

    writeln!(&mut file, "Hello!").unwrap();

    fcntl_add_seals(&file, SealFlags::GROW).unwrap();

    // We sealed growing, so this should fail.
    writeln!(&mut file, "World?").unwrap_err();

    // We can still shrink for now.
    ftruncate(&mut file, 1).unwrap();

    fcntl_add_seals(&file, SealFlags::SHRINK).unwrap();

    // We sealed shrinking, so this should fail.
    ftruncate(&mut file, 0).unwrap_err();
}
