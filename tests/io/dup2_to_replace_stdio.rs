/// Use `dup2` to replace the stdin and stdout file descriptors.
#[test]
fn dup2_to_replace_stdio() {
    use std::io::Write;
    use posish::io::{pipe, dup2};
    use io_lifetimes::AsFilelike;

    let (reader, writer) = pipe().unwrap();

    dup2(&reader, &std::io::stdin()).unwrap();
    dup2(&writer, &std::io::stdout()).unwrap();

    drop(reader);
    drop(writer);

    // Don't use std::io::stdout() because in tests it's captured.
    writeln!(unsafe { posish::io::stdout() }.as_filelike_view::<std::fs::File>(), "hello, world!").unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    assert_eq!(s, "hello, world!\n");
}
