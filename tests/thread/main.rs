//! Tests for [`rustix::thread`].

#![cfg(feature = "thread")]
#![cfg(not(windows))]

#[cfg(not(target_os = "redox"))]
mod clocks;
mod cpu_set;
#[cfg(linux_kernel)]
mod futex;
#[cfg(linux_kernel)]
mod id;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod libcap;
#[cfg(linux_kernel)]
mod membarrier;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod prctl;
mod sched;
#[cfg(all(linux_kernel, linux_raw_dep))]
mod setns;
