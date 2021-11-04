#[cfg(any(target_os = "android", target_os = "linux"))]
use rustix::process::linux_hwcap;
use rustix::process::page_size;

#[test]
fn test_page_size() {
    let size = page_size();
    assert_ne!(size, 0);
    assert!(size.is_power_of_two());
    assert_eq!(size, page_size());
    assert_eq!(size, unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize });
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn test_linux_hwcap() {
    let (_hwcap, hwcap2) = linux_hwcap();

    // GLIBC seems to return a different value than `LD_SHOW_AUXV=1` reports.
    #[cfg(not(target_env = "gnu"))]
    assert_eq!(_hwcap, unsafe { libc::getauxval(libc::AT_HWCAP) } as usize);

    assert_eq!(hwcap2, unsafe { libc::getauxval(libc::AT_HWCAP2) } as usize);
}
