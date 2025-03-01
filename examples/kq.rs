//! A simple example of the BSD `kqueue` API.

#[cfg(all(bsd, feature = "event"))]
fn main() -> std::io::Result<()> {
    use rustix::buffer::spare_capacity;
    use rustix::event::kqueue::*;
    #[cfg(feature = "fs")]
    use rustix::{fd::AsRawFd, fs};
    use std::ptr::null_mut;

    let kq = kqueue()?;
    let mut out = Vec::with_capacity(10);

    #[cfg(feature = "fs")]
    let dir = fs::openat(
        fs::CWD,
        ".",
        fs::OFlags::RDONLY | fs::OFlags::DIRECTORY | fs::OFlags::CLOEXEC,
        fs::Mode::empty(),
    )?;

    let subs = [
        #[cfg(feature = "process")]
        Event::new(
            EventFilter::Signal {
                signal: rustix::process::Signal::INFO,
                times: 0,
            },
            EventFlags::ADD,
            null_mut(),
        ),
        #[cfg(feature = "fs")]
        Event::new(
            EventFilter::Vnode {
                vnode: dir.as_raw_fd(),
                flags: VnodeEvents::WRITE | VnodeEvents::LINK | VnodeEvents::EXTEND,
            },
            EventFlags::ADD | EventFlags::CLEAR,
            null_mut(),
        ),
        Event::new(
            EventFilter::Timer {
                ident: 0,
                timer: Some(std::time::Duration::from_secs(1)),
            },
            EventFlags::ADD,
            null_mut(),
        ),
        Event::new(
            EventFilter::Timer {
                ident: 1,
                timer: Some(std::time::Duration::from_secs(2)),
            },
            EventFlags::ADD | EventFlags::ONESHOT,
            null_mut(),
        ),
    ];

    eprintln!("Listening for various events");
    #[cfg(not(feature = "process"))]
    eprintln!("Run with --features process to enable more!");
    #[cfg(not(feature = "fs"))]
    eprintln!("Run with --features fs to enable more!");
    unsafe { kevent(&kq, &subs, spare_capacity(&mut out), None) }?;

    loop {
        while let Some(e) = out.pop() {
            match e.filter() {
                #[cfg(feature = "process")]
                EventFilter::Signal { signal, times } => {
                    eprintln!("Got signal {:?} {} times", signal, times)
                }
                #[cfg(feature = "fs")]
                EventFilter::Vnode { vnode: _, flags } => {
                    eprintln!("Current directory was touched ({:?})", flags)
                }
                EventFilter::Timer { ident: 0, timer: _ } => eprintln!("Second timer ticked"),
                EventFilter::Timer { ident: 1, timer: _ } => {
                    eprintln!("One-shot two second timer ticked")
                }
                _ => eprintln!("Unknown event"),
            }
        }
        unsafe { kevent(&kq, &[], spare_capacity(&mut out), None) }?;
    }
}

#[cfg(not(all(bsd, feature = "event")))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=event and is only supported on BSD.")
}
