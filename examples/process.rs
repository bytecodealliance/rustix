use rsix::io;
use rsix::process::*;

fn main() -> io::Result<()> {
    println!("Pid: {}", getpid().as_raw());
    println!("Uid: {}", getuid().as_raw());
    println!("Gid: {}", getgid().as_raw());
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        let (a, b) = linux_hwcap();
        println!("Linux hwcap: {:#x}, {:#x}", a, b);
    }
    println!("Page size: {}", page_size());
    println!("Uname: {:?}", uname());
    println!("Process group priority: {}", getpriority_pgrp(Pid::NONE)?);
    println!("Process priority: {}", getpriority_process(Pid::NONE)?);
    println!("User priority: {}", getpriority_user(Uid::ROOT)?);
    println!(
        "Current working directory: {}",
        getcwd(Default::default())?.to_string_lossy()
    );
    Ok(())
}
