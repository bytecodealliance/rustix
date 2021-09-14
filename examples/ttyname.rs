use rsix::io::{self, isatty, stderr, stdin, stdout, ttyname};
use std::ffi::OsString;

fn main() -> io::Result<()> {
    let (stdin, stdout, stderr) = unsafe { (stdin(), stdout(), stderr()) };
    if isatty(&stdin) {
        println!(
            "Stdin ttyname: {}",
            ttyname(&stdin, OsString::new())?.to_string_lossy()
        );
    } else {
        println!("Stdin is not a tty");
    }

    if isatty(&stdout) {
        println!(
            "Stdout ttyname: {}",
            ttyname(&stdout, OsString::new())?.to_string_lossy()
        );
    } else {
        println!("Stdout is not a tty");
    }

    if isatty(&stderr) {
        println!(
            "Stderr ttyname: {}",
            ttyname(&stderr, OsString::new())?.to_string_lossy()
        );
    } else {
        println!("Stderr is not a tty");
    }
    Ok(())
}
