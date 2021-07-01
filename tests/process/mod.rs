#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched_yield;
