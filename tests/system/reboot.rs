#[test]
#[cfg(feature = "thread")]
fn test_reboot() {
    use rustix::io::Errno;
    use rustix::system::{self, RebootCommand};
    use rustix::thread::{self, CapabilitySet};

    let mut capabilities = thread::capabilities(None).expect("Failed to get capabilities");

    capabilities.effective.set(CapabilitySet::SYS_BOOT, false);

    thread::set_capabilities(None, capabilities).expect("Failed to set capabilities");

    // The reboot syscall requires the `CapabilitySet::SYS_BOOT` permission
    // to be called, otherwise [`Errno::PERM`] is returned
    assert_eq!(system::reboot(RebootCommand::Restart), Err(Errno::PERM));
}
