#[test]
#[cfg(feature = "thread")]
fn test_reboot() {
    use rustix::{
        io::Errno,
        system::{self, RebootCommand},
        thread::{self, CapabilityFlags},
    };

    let mut capabilities = thread::capabilities(None).expect("Failed to get capabilities");

    capabilities.effective.set(CapabilityFlags::SYS_BOOT, false);

    thread::set_capabilities(None, capabilities).expect("Failed to set capabilities");

    // The reboot syscall requires the CAP_SYS_BOOT permission to be called, otherwise EPERM is returned
    assert_eq!(system::reboot(RebootCommand::Restart), Err(Errno::PERM));
}
