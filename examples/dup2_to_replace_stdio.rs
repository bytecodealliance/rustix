//! This is an example of how to use `dup2` to replace the stdin and stdout file
//! descriptors.

#[cfg(not(windows))]
fn main() {
    use io_lifetimes::AsFilelike;
    use rustix::io::{dup2, pipe};
    use std::io::{BufRead, BufReader, Write};
    use std::mem::forget;

    let (reader, writer) = pipe().unwrap();
    let (stdin, stdout) = unsafe { (rustix::io::take_stdin(), rustix::io::take_stdout()) };
    dup2(&reader, &stdin).unwrap();
    dup2(&writer, &stdout).unwrap();
    forget(stdin);
    forget(stdout);

    drop(reader);
    drop(writer);

    // Don't use `std::io::stdout()` because in tests it's captured.
    unsafe {
        writeln!(
            rustix::io::stdout().as_filelike_view::<std::fs::File>(),
            "hello, world!"
        )
        .unwrap();

        let mut s = String::new();
        BufReader::new(&*rustix::io::stdin().as_filelike_view::<std::fs::File>())
            .read_line(&mut s)
            .unwrap();
        assert_eq!(s, "hello, world!\n");
    }
}

#[cfg(windows)]
fn main() {
    unimplemented!()
}
