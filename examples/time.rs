#[cfg(not(windows))]
fn main() {
    println!(
        "Raeal time: {:?}",
        rustix::time::clock_gettime(rustix::time::ClockId::Realtime)
    );
    println!(
        "Monotonic time: {:?}",
        rustix::time::clock_gettime(rustix::time::ClockId::Monotonic)
    );
}

#[cfg(windows)]
fn main() {
    unimplemented!()
}
