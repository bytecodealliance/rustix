//! A simple example of the Solarish `port` API.
//!
//! This creates a fifo and then monitors it for data. To use it,
//! run the example, note the path it prints out, and then in
//! another window, run `cat <path>` and type some text.

#[cfg(all(solarish, feature = "event", feature = "fs"))]
fn main() -> std::io::Result<()> {
    use rustix::buffer::spare_capacity;
    use rustix::event::port;
    use rustix::fd::AsRawFd;
    use rustix::fs;
    use std::ptr::null_mut;

    let port = port::create()?;
    let mut out = Vec::with_capacity(10);

    let tmpdir = tempfile::tempdir()?;
    let fifo_path = tmpdir.path().join("fifo");
    fs::mknodat(
        fs::CWD,
        &fifo_path,
        fs::FileType::Fifo,
        fs::Mode::RUSR | fs::Mode::WUSR,
        0,
    )?;

    eprintln!("Listening for data on fifo {}", fifo_path.display());

    let fifo = fs::openat(fs::CWD, &fifo_path, fs::OFlags::RDONLY, fs::Mode::empty())?;

    loop {
        // Associate `some_fd` with the port.
        unsafe {
            port::associate_fd(&port, fifo.as_raw_fd(), port::PollFlags::IN, null_mut())?;
        }

        port::getn(&port, spare_capacity(&mut out), 1, None)?;

        for event in out.drain(..) {
            dbg!(event.events(), event.object(), event.userdata());

            let mut buf = [0_u8; 32];
            loop {
                match rustix::io::read(&fifo, &mut buf) {
                    Ok(0) => return Ok(()),
                    Ok(n) => {
                        dbg!(&buf[..n]);
                        break;
                    }
                    Err(rustix::io::Errno::INTR) => continue,
                    Err(err) => Err(err).unwrap(),
                }
            }
        }
    }
}

#[cfg(not(all(solarish, feature = "event", feature = "fs")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=event,fs and is only supported on Solarish.")
}
