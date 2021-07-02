#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg(not(any(target_os = "wasi", target_os = "redox")))] // WASI doesn't support `net` yet.

mod net {
    mod unix;
    mod v4;
    mod v6;
}
