#[inline]
pub(crate) fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

#[inline]
#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    unsafe {
        let hwcap = libc::getauxval(libc::AT_HWCAP) as usize;
        let hwcap2 = libc::getauxval(libc::AT_HWCAP2) as usize;
        (hwcap, hwcap2)
    }
}
