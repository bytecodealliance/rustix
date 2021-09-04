use libc::c_int;

pub const EXIT_SUCCESS: c_int = libc::EXIT_SUCCESS;
pub const EXIT_FAILURE: c_int = libc::EXIT_FAILURE;
#[cfg(not(target_os = "wasi"))]
pub const EXIT_SIGNALED_SIGABRT: c_int = 128 + libc::SIGABRT;

#[cfg(not(target_os = "wasi"))]
pub type RawPid = libc::pid_t;
#[cfg(not(target_os = "wasi"))]
pub type RawGid = libc::gid_t;
#[cfg(not(target_os = "wasi"))]
pub type RawUid = libc::uid_t;

#[cfg(not(target_os = "wasi"))]
pub type RawUname = libc::utsname;
