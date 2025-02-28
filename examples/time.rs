//! A command which prints the current values of the realtime and monotonic
//! clocks it's given.

#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(feature = "time")]
fn main() {
    use rustix::time::{clock_gettime, ClockId};

    println!("Real time: {:?}", clock_gettime(ClockId::Realtime));
    println!("Monotonic time: {:?}", clock_gettime(ClockId::Monotonic));

    #[cfg(any(freebsdlike, target_os = "openbsd"))]
    println!("Uptime: {:?}", clock_gettime(ClockId::Uptime));

    #[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
    println!(
        "Process CPU time: {:?}",
        clock_gettime(ClockId::ProcessCPUTime)
    );

    #[cfg(not(any(solarish, target_os = "netbsd", target_os = "redox")))]
    println!(
        "Thread CPU time: {:?}",
        clock_gettime(ClockId::ThreadCPUTime)
    );

    #[cfg(any(linux_kernel, target_os = "freebsd"))]
    println!(
        "Realtime (coarse): {:?}",
        clock_gettime(ClockId::RealtimeCoarse)
    );

    #[cfg(any(linux_kernel, target_os = "freebsd"))]
    println!(
        "Monotonic (coarse): {:?}",
        clock_gettime(ClockId::MonotonicCoarse)
    );

    #[cfg(linux_kernel)]
    println!(
        "Monotonic (raw): {:?}",
        clock_gettime(ClockId::MonotonicRaw)
    );
}

#[cfg(any(windows, target_os = "espidf", not(feature = "time")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=time and is not supported on Windows or ESP-IDF.")
}
