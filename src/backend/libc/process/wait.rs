use crate::backend::c;

pub(crate) use c::{
    WEXITSTATUS, WIFCONTINUED, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WNOHANG, WSTOPSIG, WTERMSIG,
};

#[cfg(not(target_os = "horizon"))]
pub(crate) use c::{WCONTINUED, WUNTRACED};

#[cfg(not(any(
    target_os = "horizon",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi",
    target_os = "cygwin"
)))]
pub(crate) use c::{WEXITED, WNOWAIT, WSTOPPED};
