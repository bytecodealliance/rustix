//! A command which demonstrates using `tcgetattr` and `tcsetattr` including
//! enabling raw (press any key) input.

#[cfg(all(not(windows), feature = "termios"))]
fn main() -> std::io::Result<()> {
    use rustix::termios::{tcgetattr, tcsetattr, OptionalActions};
    use std::io::{Read as _, Write as _};

    let tty = std::io::stdin();
    let termios = tcgetattr(&tty)?;
    println!("Original termios: {:?}", termios);

    print!("Original settings; enter some text: ");
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    let _input = std::io::stdin().read_line(&mut buffer)?;

    tcsetattr(&tty, OptionalActions::Flush, &termios)?;

    print!("Reset original settings; enter some text: ");
    std::io::stdout().flush()?;
    let _input = std::io::stdin().read_line(&mut buffer)?;

    let mut raw = termios.clone();
    raw.make_raw();

    println!("Raw termios: {:?}", raw);

    tcsetattr(&tty, OptionalActions::Flush, &raw)?;

    print!("Raw settings; press any key…");
    std::io::stdout().flush()?;
    let mut buf = [0_u8];
    let _input = std::io::stdin().read(&mut buf)?;

    tcsetattr(&tty, OptionalActions::Flush, &termios)?;
    println!();

    print!("Reset original settings again; enter some text: ");
    std::io::stdout().flush()?;
    let _input = std::io::stdin().read_line(&mut buffer)?;

    Ok(())
}

#[cfg(any(windows, not(feature = "termios")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=termios and is not supported on Windows.")
}
