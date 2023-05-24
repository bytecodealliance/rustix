/// Make sure we can use `cwd` in const contexts.
#[allow(dead_code)]
#[cfg(not(target_os = "redox"))]
const CWD: rustix::fd::BorrowedFd<'static> = rustix::fs::CWD;
