#[cfg(not(windows))]
fn main() {
    println!(
        "Raeal time: {:?}",
        rsix::time::clock_gettime(rsix::time::ClockId::Realtime)
    );
    println!(
        "Monotonic time: {:?}",
        rsix::time::clock_gettime(rsix::time::ClockId::Monotonic)
    );
}

#[cfg(windows)]
fn main() {
    unimplemented!()
}
