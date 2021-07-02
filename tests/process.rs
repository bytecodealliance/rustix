#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod process {
    #[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
    mod id;
    mod sched_yield;
}
