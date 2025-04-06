#[cfg(feature = "system")]
use {
    std::io::{BufRead as _, Read as _},
    std::{fs, io},
};

use rustix::thread::*;

#[test]
fn test_get_keep_capabilities() {
    dbg!(get_keep_capabilities().unwrap());
}

#[test]
fn test_name() {
    dbg!(name().unwrap());
}

#[test]
fn test_capability_is_in_bounding_set() {
    dbg!(capability_is_in_bounding_set(CapabilitySet::CHOWN).unwrap());
    dbg!(capability_is_in_bounding_set(
        #[allow(deprecated)]
        Capability::ChangeOwnership
    )
    .unwrap());
}

#[test]
fn test_capabilities_secure_bits() {
    dbg!(capabilities_secure_bits().unwrap());
}

#[test]
fn test_current_timer_slack() {
    dbg!(current_timer_slack().unwrap());
}

#[test]
fn test_no_new_privs() {
    dbg!(no_new_privs().unwrap());
}

#[test]
fn test_capability_is_in_ambient_set() {
    dbg!(capability_is_in_ambient_set(CapabilitySet::CHOWN).unwrap());
    dbg!(capability_is_in_ambient_set(
        #[allow(deprecated)]
        Capability::ChangeOwnership
    )
    .unwrap());
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_sve_vector_length_configuration() {
    dbg!(sve_vector_length_configuration().unwrap());
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_current_tagged_address_mode() {
    dbg!(current_tagged_address_mode().unwrap());
}

#[test]
#[cfg_attr(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    ignore = "`transparent_huge_pages_are_disabled` doesn't work on qemu"
)]
fn test_transparent_huge_pages_are_disabled() {
    dbg!(transparent_huge_pages_are_disabled().unwrap());
}

#[cfg(feature = "system")]
#[cfg_attr(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    ignore = "`secure_computing_mode` doesn't work on qemu"
)]
#[test]
fn test_secure_computing_mode() {
    if !linux_kernel_config_item_is_enabled("CONFIG_SECCOMP").unwrap_or(false) {
        eprintln!(
            "test_secure_computing_mode: Test skipped due to missing kernel
            feature: CONFIG_SECCOMP."
        );
        return;
    }

    // If seccomp is enabled, calling `secure_computing_mode` could evoke a
    // `Signal::KILL`, so only call it if seccomp is disabled.
    for line in std::io::BufReader::new(std::fs::File::open("/proc/self/status").unwrap()).lines() {
        if let Some(seccomp) = line.unwrap().strip_prefix("Seccomp:") {
            let seccomp = seccomp.trim();
            let seccomp: i32 = seccomp.parse().unwrap();
            let seccomp: rustix::thread::SecureComputingMode = seccomp.try_into().unwrap();
            if seccomp == rustix::thread::SecureComputingMode::Disabled {
                assert_eq!(seccomp, secure_computing_mode().unwrap());
            }
        }
    }
}

#[cfg(feature = "system")]
#[test]
fn test_get_clear_child_tid_address() {
    if !linux_kernel_config_item_is_enabled("CONFIG_CHECKPOINT_RESTORE").unwrap_or(false) {
        eprintln!(
            "test_get_clear_child_tid_address: Test skipped due to missing kernel feature: \
             CONFIG_CHECKPOINT_RESTORE."
        );
        return;
    }

    match get_clear_child_tid_address() {
        Ok(address) => println!("get_clear_child_tid_address = {:?}", address),

        Err(r) => {
            let errno = r.raw_os_error();
            assert!(errno == libc::ENODEV || errno == libc::EINVAL);
            eprintln!("test_get_clear_child_tid_address: Test unsupported: {}", r);
        }
    }
}

#[cfg(feature = "system")]
#[test]
fn test_core_scheduling_cookie() {
    if !linux_kernel_config_item_is_enabled("CONFIG_SCHED_CORE").unwrap_or(false) {
        eprintln!(
            "test_core_scheduling_cookie: Test skipped due to missing kernel feature: \
             CONFIG_SCHED_CORE."
        );
        return;
    }

    match core_scheduling_cookie(rustix::thread::gettid(), CoreSchedulingScope::Thread) {
        Ok(cookie) => println!("core_scheduling_cookie = {:?}", cookie),

        Err(r) => {
            let errno = r.raw_os_error();
            assert!(errno == libc::ENODEV || errno == libc::EINVAL);
            eprintln!("test_core_scheduling_cookie: Test unsupported: {}", r);
        }
    }
}

//
// Helper functions.
//

#[cfg(feature = "system")]
fn load_linux_kernel_config() -> io::Result<Vec<u8>> {
    if let Ok(compressed_bytes) = fs::read("/proc/config.gz") {
        let mut decoder = flate2::bufread::GzDecoder::new(compressed_bytes.as_slice());
        let mut bytes = Vec::default();
        decoder.read_to_end(&mut bytes)?;
        return Ok(bytes);
    }

    let info = rustix::system::uname();
    let release = info
        .release()
        .to_str()
        .map_err(|_r| io::Error::from(io::ErrorKind::InvalidData))?;

    fs::read(format!("/boot/config-{}", release))
}

#[cfg(feature = "system")]
fn is_linux_kernel_config_item_enabled(config: &[u8], name: &str) -> io::Result<bool> {
    for line in io::Cursor::new(config).lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut iter = line.splitn(2, '=');
        if let Some(current_name) = iter.next().map(str::trim) {
            if current_name == name {
                if let Some(mut value) = iter.next().map(str::trim) {
                    if value.starts_with('"') && value.ends_with('"') {
                        // Just remove the quotes, but don't bother unescaping the inner string
                        // because we are only trying to find out if the option is a true boolean.
                        value = &value[1..(value.len() - 2)];
                    }

                    return Ok(value == "y" || value == "m");
                }
            }
        }
    }
    Ok(false)
}

#[cfg(feature = "system")]
fn linux_kernel_config_item_is_enabled(name: &str) -> io::Result<bool> {
    let config = load_linux_kernel_config()?;
    is_linux_kernel_config_item_enabled(&config, name)
}
