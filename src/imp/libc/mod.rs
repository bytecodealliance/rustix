#[cfg(not(target_os = "wasi"))]
#[macro_use]
mod weak;

mod conv;
mod offset;

pub(crate) mod fs;
pub(crate) mod io;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))] // WASI doesn't support `net` yet.
pub(crate) mod net;
pub(crate) mod process;
pub(crate) mod rand;
pub(crate) mod syscalls;
pub(crate) mod time;
