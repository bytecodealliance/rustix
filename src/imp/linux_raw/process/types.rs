use std::os::raw::c_int;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SIGNALED_SIGABRT: c_int = 128 + linux_raw_sys::general::SIGABRT as i32;
