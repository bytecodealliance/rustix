#[cfg(any(all(linux_raw, feature = "procfs"), libc))]
use rsix::io::ttyname;
use rsix::io::{self, isatty, stderr, stdin, stdout};
use rsix::io_lifetimes::AsFd;

fn main() -> io::Result<()> {
    let (stdin, stdout, stderr) = unsafe { (stdin(), stdout(), stderr()) };

    println!("Stdin:");
    show(&stdin)?;

    println!("Stdout:");
    show(&stdout)?;

    println!("Stderr:");
    show(&stderr)?;

    Ok(())
}

fn show<Fd: AsFd>(fd: &Fd) -> io::Result<()> {
    if isatty(fd) {
        #[cfg(any(all(linux_raw, feature = "procfs"), libc))]
        println!(" - ttyname: {}", ttyname(fd, Vec::new())?.to_string_lossy());
        println!(" - attrs: {:?}", rsix::io::ioctl_tcgets(fd)?);
        println!(" - winsize: {:?}", rsix::io::ioctl_tiocgwinsz(fd)?);
        println!(" - ready: {:?}", rsix::io::ioctl_fionread(fd)?);
    } else {
        println!("Stderr is not a tty");
    }
    Ok(())
}
