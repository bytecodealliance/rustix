//! Packet MMAP.

#[cfg(all(
    feature = "mm",
    feature = "net",
    feature = "event",
    feature = "std",
    target_os = "linux"
))]
mod inner;

#[cfg(all(
    feature = "mm",
    feature = "net",
    feature = "event",
    feature = "std",
    target_os = "linux"
))]
fn main() -> std::io::Result<()> {
    inner::main()
}

#[cfg(any(
    not(feature = "mm"),
    not(feature = "net"),
    not(feature = "event"),
    not(feature = "std"),
    not(target_os = "linux")
))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=mm,net,event,std and is only supported on Linux.")
}
