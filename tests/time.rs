#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod time {
    mod clocks;
    mod dynamic_clocks;
    #[cfg(not(any(target_os = "redox", target_os = "wasi")))]
    mod monotonic;
    mod timespec;
    mod y2038;
}
