#[cfg(any(target_os = "android", target_os = "linux"))]
use rsix::process::linux_hwcap;
use rsix::process::page_size;

#[test]
fn test_page_size() {
    let size = page_size();
    assert_ne!(size, 0);
    assert!(size.is_power_of_two());
    assert_eq!(size, page_size());
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn test_linux_hwcap() {
    let (_hwcap, _hwcap2) = linux_hwcap();

    // These values are architecture-defined.
}
