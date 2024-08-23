#[cfg(linux_kernel)]
#[cfg(feature = "io_uring")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod register;
