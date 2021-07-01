mod isatty;
mod mmap;
#[cfg(not(target_os = "redox"))] // redox doesn't have cwd/openat
mod readwrite;
