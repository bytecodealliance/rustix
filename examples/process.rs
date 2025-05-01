//! A command which prints out information about the process it runs in.

#[cfg(all(feature = "process", feature = "param", feature = "system"))]
#[cfg(not(windows))]
fn main() -> rustix::io::Result<()> {
    #[cfg(not(target_os = "espidf"))]
    use rustix::param::*;
    use rustix::process::*;
    use rustix::system::*;

    println!("Pid: {}", getpid().as_raw_nonzero());
    println!("Parent Pid: {}", Pid::as_raw(getppid()));
    println!("Group Pid: {}", getpgrp().as_raw_nonzero());
    if let Some(ppid) = getppid() {
        println!(
            "Parent Group Pid: {}",
            getpgid(Some(ppid))?.as_raw_nonzero()
        );
    }
    println!("Uid: {}", getuid().as_raw());
    println!("Gid: {}", getgid().as_raw());
    #[cfg(any(
        all(target_os = "android", target_pointer_width = "64"),
        target_os = "linux",
    ))]
    {
        let (a, b) = linux_hwcap();
        println!("Linux hwcap: {:#x}, {:#x}", a, b);
    }
    #[cfg(not(target_os = "espidf"))]
    println!("Page size: {}", page_size());
    #[cfg(not(target_os = "espidf"))]
    println!("Clock ticks/sec: {}", clock_ticks_per_second());
    println!("Uname: {:?}", uname());
    #[cfg(not(any(target_os = "espidf", target_os = "fuchsia")))]
    {
        println!("Process group priority: {}", getpriority_pgrp(None)?);
        println!("Process priority: {}", getpriority_process(None)?);
        println!("User priority: {}", getpriority_user(Uid::ROOT)?);
    }
    println!(
        "Current working directory: {}",
        getcwd(Vec::new())?.to_string_lossy()
    );
    #[cfg(not(any(target_os = "espidf", target_os = "fuchsia", target_os = "redox")))]
    {
        println!("Cpu Limit: {:?}", getrlimit(Resource::Cpu));
        println!("Fsize Limit: {:?}", getrlimit(Resource::Fsize));
        println!("Data Limit: {:?}", getrlimit(Resource::Data));
        println!("Stack Limit: {:?}", getrlimit(Resource::Stack));
        #[cfg(not(target_os = "haiku"))]
        println!("Core Limit: {:?}", getrlimit(Resource::Core));
        #[cfg(not(any(solarish, target_os = "cygwin", target_os = "haiku")))]
        println!("Rss Limit: {:?}", getrlimit(Resource::Rss));
        #[cfg(not(any(solarish, target_os = "cygwin", target_os = "haiku")))]
        println!("Nproc Limit: {:?}", getrlimit(Resource::Nproc));
        #[cfg(not(target_os = "solaris"))]
        println!("Nofile Limit: {:?}", getrlimit(Resource::Nofile));
        #[cfg(not(any(solarish, target_os = "aix", target_os = "cygwin", target_os = "haiku")))]
        println!("Memlock Limit: {:?}", getrlimit(Resource::Memlock));
        #[cfg(not(target_os = "openbsd"))]
        println!("As Limit: {:?}", getrlimit(Resource::As));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "cygwin",
            target_os = "haiku",
        )))]
        println!("Locks Limit: {:?}", getrlimit(Resource::Locks));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "cygwin",
            target_os = "haiku",
        )))]
        println!("Sigpending Limit: {:?}", getrlimit(Resource::Sigpending));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "cygwin",
            target_os = "haiku",
        )))]
        println!("Msgqueue Limit: {:?}", getrlimit(Resource::Msgqueue));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "cygwin",
            target_os = "haiku",
        )))]
        println!("Nice Limit: {:?}", getrlimit(Resource::Nice));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "cygwin",
            target_os = "haiku",
        )))]
        println!("Rtprio Limit: {:?}", getrlimit(Resource::Rtprio));
        #[cfg(not(any(
            bsd,
            solarish,
            target_os = "aix",
            target_os = "android",
            target_os = "cygwin",
            target_os = "emscripten",
            target_os = "haiku",
        )))]
        println!("Rttime Limit: {:?}", getrlimit(Resource::Rttime));
    }
    #[cfg(any(
        all(target_os = "android", target_pointer_width = "64"),
        target_os = "linux"
    ))]
    println!("Execfn: {:?}", linux_execfn());
    Ok(())
}

#[cfg(any(
    windows,
    not(all(feature = "process", feature = "param", feature = "system"))
))]
fn main() -> Result<(), &'static str> {
    Err("This example requires --features=process,param,system and is not supported on Windows.")
}
