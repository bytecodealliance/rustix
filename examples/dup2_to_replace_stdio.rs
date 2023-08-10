//! This is an example of how to use `dup2` to replace the stdin and stdout
//! file descriptors.

#[cfg(all(not(windows), feature = "pipe", feature = "stdio"))]
fn main() -> std::io::Result<()> {
    use rustix::pipe::pipe;
    use rustix::stdio::{dup2_stdin, dup2_stdout};

    // Create some new file descriptors that we'll use to replace stdio's file
    // descriptors with.
    let (reader, writer) = pipe()?;

    // Use `dup2` to copy our new file descriptors over the stdio file
    // descriptors.
    //
    // Rustix has a plain `dup2` function too, but it requires a
    // `&mut OwnedFd`, so these helper functions make it easier to use when
    // replacing stdio fds.
    dup2_stdin(&reader)?;
    dup2_stdout(&writer)?;

    // We can also drop the original file descriptors now, since `dup2` creates
    // new file descriptors with independent lifetimes.
    drop(reader);
    drop(writer);

    // Now we can print to “stdout” in the usual way, and it'll go to our pipe.
    println!("hello, world!");

    // And we can read from stdin, and it'll read from our pipe. It's a little
    // silly that we connected our stdout to our own stdin, but it's just an
    // example 😀.
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    assert_eq!(s, "hello, world!\n");

    Ok(())
}

#[cfg(not(all(not(windows), feature = "pipe", feature = "stdio")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=pipe,stdio and is not supported on Windows.")
}
