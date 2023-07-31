//! Tests for [`rustix::mount`].

#![cfg(feature = "mount")]
#![cfg(linux_kernel)]

// At this time, we have no tests for the `mount` functions, because they
// all require elevated privileges.
