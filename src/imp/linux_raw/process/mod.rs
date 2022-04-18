#[cfg(any(
    feature = "process",
    feature = "runtime",
    feature = "time",
    target_arch = "x86"
))]
pub(crate) mod auxv;
pub(crate) mod cpu_set;
pub(crate) mod syscalls;
pub(crate) mod types;
pub(crate) mod wait;
