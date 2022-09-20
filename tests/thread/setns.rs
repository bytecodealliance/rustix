use std::fs::File;

use rustix::fd::{AsFd, FromRawFd, OwnedFd, RawFd};
use rustix::thread::*;

#[test]
#[ignore = "Requires CAP_SYS_ADMIN capability"]
fn test_move_into_link_name_space() {
    let f = File::open("/proc/self/ns/uts").unwrap();

    rustix::thread::move_into_link_name_space(
        f.as_fd(),
        Some(rustix::thread::LinkNameSpaceType::HostNameAndNISDomainName),
    )
    .unwrap();
}

#[test]
#[ignore = "Requires CAP_SYS_ADMIN capability"]
fn test_move_into_thread_name_spaces() {
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, std::process::id() as usize, 0_usize) };
    if fd == -1 {
        panic!("{}", std::io::Error::last_os_error());
    }

    let fd = unsafe { OwnedFd::from_raw_fd(fd as RawFd) };

    rustix::thread::move_into_thread_name_spaces(
        fd.as_fd(),
        ThreadNameSpaceType::HOST_NAME_AND_NIS_DOMAIN_NAME,
    )
    .unwrap();
}
