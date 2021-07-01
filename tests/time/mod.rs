mod clocks;
#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
mod monotonic;
mod timespec;
mod y2038;
