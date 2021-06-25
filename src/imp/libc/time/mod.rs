mod types;

#[cfg(not(target_os = "wasi"))]
pub use types::ClockId;
pub use types::{Nsecs, Secs, Timespec};
