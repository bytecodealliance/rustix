//! A wrapper around `rustix::fs::pivot_root`.

#[cfg(all(target_os = "linux", feature = "fs", feature = "process"))]
fn main() -> rustix::io::Result<()> {
    let mut args = std::env::args();
    if args.len() != 3 {
        eprintln!("Usage: {} new_root put_old", args.next().unwrap());
        std::process::exit(1);
    }

    let _argv0 = args.next().unwrap();
    let new_root = args.next().unwrap();
    let put_old = args.next().unwrap();

    rustix::process::pivot_root(new_root, put_old)?;

    Ok(())
}

#[cfg(any(
    not(target_os = "linux"),
    not(feature = "fs"),
    not(feature = "process")
))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=fs,process and is only supported on Linux.")
}
