//! A command which prints the current values of the realtime and monotonic
//! clocks it's given.

#[cfg(not(any(windows, target_os = "espidf")))]
#[cfg(feature = "time")]
fn main() {
    println!(
        "Real time: {:?}",
        rustix::time::clock_gettime(rustix::time::ClockId::Realtime)
    );
    println!(
        "Monotonic time: {:?}",
        rustix::time::clock_gettime(rustix::time::ClockId::Monotonic)
    );
}

#[cfg(any(windows, target_os = "espidf", not(feature = "time")))]
fn main() {
    unimplemented!()
}
