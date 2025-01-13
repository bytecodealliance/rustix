//! A command which prints the current values of the realtime and monotonic
//! clocks it's given.

struct DebugTimespec(rustix::time::Timespec);

impl core::fmt::Debug for DebugTimespec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut d = f.debug_struct("Timespec");
        d.field("tv_sec", &self.0.tv_sec);
        d.field("tv_nsec", &self.0.tv_nsec);
        d.finish()
    }
}

#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(feature = "time")]
fn main() {
    use rustix::time::{clock_gettime, ClockId};

    println!(
        "Real time: {:?}",
        DebugTimespec(clock_gettime(ClockId::Realtime))
    );
    println!(
        "Monotonic time: {:?}",
        DebugTimespec(clock_gettime(ClockId::Monotonic))
    );

    #[cfg(any(freebsdlike, target_os = "openbsd"))]
    println!("Uptime: {:?}", clock_gettime(ClockId::Uptime));

    #[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
    println!(
        "Process CPU time: {:?}",
        DebugTimespec(clock_gettime(ClockId::ProcessCPUTime))
    );

    #[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
    println!(
        "Thread CPU time: {:?}",
        DebugTimespec(clock_gettime(ClockId::ThreadCPUTime))
    );

    #[cfg(any(linux_kernel, target_os = "freebsd"))]
    println!(
        "Realtime (coarse): {:?}",
        DebugTimespec(clock_gettime(ClockId::RealtimeCoarse))
    );

    #[cfg(any(linux_kernel, target_os = "freebsd"))]
    println!(
        "Monotonic (coarse): {:?}",
        DebugTimespec(clock_gettime(ClockId::MonotonicCoarse))
    );

    #[cfg(linux_kernel)]
    println!(
        "Monotonic (raw): {:?}",
        DebugTimespec(clock_gettime(ClockId::MonotonicRaw))
    );
}

#[cfg(any(windows, target_os = "espidf", not(feature = "time")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=time and is not supported on Windows or ESP-IDF.")
}
