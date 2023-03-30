#[cfg(bsd)]
fn main() -> std::io::Result<()> {
    use rustix::io::kqueue::*;
    #[cfg(feature = "fs")]
    use rustix::{fd::AsRawFd, fs};

    let kq = kqueue()?;
    let mut out = Vec::with_capacity(10);

    #[cfg(feature = "fs")]
    let dir = fs::openat(fs::cwd(), ".", fs::OFlags::DIRECTORY, fs::Mode::empty())?;

    let subs = [
        #[cfg(feature = "process")]
        Event::new(
            EventFilter::Signal {
                signal: rustix::process::Signal::Info,
                times: 0,
            },
            EventFlags::ADD,
            0,
        ),
        #[cfg(feature = "fs")]
        Event::new(
            EventFilter::Vnode {
                vnode: dir.as_raw_fd(),
                flags: VnodeEvents::WRITE | VnodeEvents::LINK | VnodeEvents::EXTEND,
            },
            EventFlags::ADD | EventFlags::CLEAR,
            0,
        ),
        Event::new(
            EventFilter::Timer(Some(core::time::Duration::from_secs(1))),
            EventFlags::ADD,
            0,
        ),
    ];

    eprintln!("Listening for various events");
    #[cfg(not(feature = "process"))]
    eprintln!("Run with --features process to enable more!");
    #[cfg(not(feature = "fs"))]
    eprintln!("Run with --features fs to enable more!");
    unsafe { kevent(&kq, &subs, &mut out, None) }?;

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
                EventFilter::Timer(_) => eprintln!("Second timer ticked"),
                _ => eprintln!("Unknown event"),
            }
        }
        unsafe { kevent(&kq, &[], &mut out, None) }?;
    }
}

#[cfg(not(bsd))]
fn main() {
    unimplemented!()
}
