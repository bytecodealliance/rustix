/// Use `dup2` to replace the stdin and stdout file descriptors.
#[test]
fn dup2_to_replace_stdio() {
    use io_lifetimes::AsFilelike;
    use posish::io::{dup2, pipe};
    use std::io::Write;
    use std::mem::forget;

    let (reader, writer) = pipe().unwrap();
    let (stdin, stdout) = unsafe { (posish::io::take_stdin(), posish::io::take_stdout()) };
    dup2(&reader, &stdin).unwrap();
    dup2(&writer, &stdout).unwrap();
    forget(stdin);
    forget(stdout);

    drop(reader);
    drop(writer);

    // Don't use std::io::stdout() because in tests it's captured.
    writeln!(
        unsafe { posish::io::stdout() }.as_filelike_view::<std::fs::File>(),
        "hello, world!"
    )
    .unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    assert_eq!(s, "hello, world!\n");
}
