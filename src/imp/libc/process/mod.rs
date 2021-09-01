mod types;

#[cfg(not(target_os = "wasi"))]
pub use types::EXIT_SIGNALED_SIGABRT;
pub use types::{EXIT_FAILURE, EXIT_SUCCESS};
