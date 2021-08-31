#![cfg(not(target_os = "wasi"))]

/// Use `dup2` to replace the stdin and stdout file descriptors.
#[test]
fn dup2_to_replace_stdio() {
    use io_lifetimes::AsFilelike;
    use rsix::io::{dup2, pipe};
    use std::io::Write;
    use std::mem::forget;

    // This test is flaky under qemu.
    if std::env::vars().any(|var| var.0.starts_with("CARGO_TARGET_") && var.0.ends_with("_RUNNER"))
    {
        return;
    }

    let (reader, writer) = pipe().unwrap();
    let (stdin, stdout) = unsafe { (rsix::io::take_stdin(), rsix::io::take_stdout()) };
    dup2(&reader, &stdin).unwrap();
    dup2(&writer, &stdout).unwrap();
    forget(stdin);
    forget(stdout);

    drop(reader);
    drop(writer);

    // Don't use std::io::stdout() because in tests it's captured.
    writeln!(
        unsafe { rsix::io::stdout() }.as_filelike_view::<std::fs::File>(),
        "hello, world!"
    )
    .unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    assert_eq!(s, "hello, world!\n");
}
