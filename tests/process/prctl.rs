#[cfg(feature = "system")]
use {
    std::io::{BufRead as _, Read as _},
    std::os::raw::c_int,
    std::{fs, io},
};

use rustix::process::*;
#[cfg(feature = "thread")]
use rustix::thread::CapabilitySet;

#[test]
fn test_parent_process_death_signal() {
    dbg!(parent_process_death_signal().unwrap());
}

#[test]
fn test_dumpable_behavior() {
    dbg!(dumpable_behavior().unwrap());
}

#[test]
fn test_timing_method() {
    dbg!(timing_method().unwrap());
}

#[test]
fn test_machine_check_memory_corruption_kill_policy() {
    dbg!(machine_check_memory_corruption_kill_policy().unwrap());
}

#[cfg(target_arch = "x86")]
#[test]
fn test_time_stamp_counter_readability() {
    dbg!(time_stamp_counter_readability().unwrap());
}

#[cfg(target_arch = "powerpc")]
#[test]
#[ignore = "Doesn't work on qemu-ppc"]
fn test_unaligned_access_control() {
    dbg!(unaligned_access_control().unwrap());
}

#[cfg(target_arch = "powerpc")]
#[test]
#[ignore = "Doesn't work on qemu-ppc"]
fn test_floating_point_exception_mode() {
    dbg!(floating_point_exception_mode().unwrap());
}

#[cfg(target_arch = "powerpc")]
#[test]
#[ignore = "Doesn't work on qemu-ppc"]
fn test_endian_mode() {
    dbg!(endian_mode().unwrap());
}

#[cfg(any(target_arch = "mips", target_arch = "mips32r6"))]
#[test]
fn test_floating_point_mode() {
    dbg!(floating_point_mode().unwrap());
}

#[cfg(target_arch = "aarch64")]
#[test]
#[ignore = "Only on ARMv8.3 and later"]
fn test_enabled_pointer_authentication_keys() {
    dbg!(enabled_pointer_authentication_keys().unwrap());
}

#[test]
fn test_child_subreaper() {
    dbg!(child_subreaper().unwrap());
}

#[test]
fn test_speculative_feature_state() {
    dbg!(speculative_feature_state(SpeculationFeature::SpeculativeStoreBypass).unwrap());
    dbg!(speculative_feature_state(SpeculationFeature::IndirectBranchSpeculation).unwrap());
    dbg!(
        speculative_feature_state(SpeculationFeature::FlushL1DCacheOnContextSwitchOutOfTask)
            .unwrap()
    );
}

#[cfg(feature = "thread")]
#[test]
fn test_is_io_flusher() {
    if !thread_has_capability(CapabilitySet::SYS_RESOURCE).unwrap() {
        eprintln!("test_is_io_flusher: Test skipped due to missing capability: CAP_SYS_RESOURCE.");
        return;
    }

    dbg!(is_io_flusher().unwrap());
}

#[cfg(feature = "thread")]
#[cfg(feature = "system")]
#[test]
fn test_virtual_memory_map_config_struct_size() {
    if !thread_has_capability(CapabilitySet::SYS_RESOURCE).unwrap() {
        eprintln!(
            "test_virtual_memory_map_config_struct_size: Test skipped due to missing capability: \
             CAP_SYS_RESOURCE."
        );
        return;
    }

    if !linux_kernel_config_item_is_enabled("CONFIG_CHECKPOINT_RESTORE").unwrap_or(false) {
        eprintln!(
            "test_virtual_memory_map_config_struct_size: Test skipped due to missing kernel \
             feature: CONFIG_CHECKPOINT_RESTORE."
        );
        return;
    }

    dbg!(virtual_memory_map_config_struct_size().unwrap());
}

#[test]
#[ignore = "Only on ia64"]
fn test_floating_point_emulation_control() {
    dbg!(floating_point_emulation_control().unwrap());
}

//
// Helper functions.
//

#[cfg(feature = "thread")]
pub(crate) fn thread_has_capability(capability: CapabilitySet) -> io::Result<bool> {
    const _LINUX_CAPABILITY_VERSION_3: u32 = 0x2008_0522;

    #[repr(C)]
    struct cap_user_header_t {
        version: u32,
        pid: c_int,
    }

    #[repr(C)]
    struct cap_user_data_t {
        effective: u32,
        permitted: u32,
        inheritable: u32,
    }

    let header = cap_user_header_t {
        version: _LINUX_CAPABILITY_VERSION_3,
        pid: 0,
    };

    let mut data: [cap_user_data_t; 2] = [
        cap_user_data_t {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        },
        cap_user_data_t {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        },
    ];

    let r = unsafe {
        libc::syscall(
            libc::SYS_capget,
            &header as *const cap_user_header_t,
            data.as_mut_ptr(),
        )
    };

    if r == -1 {
        return Err(io::Error::last_os_error());
    }

    let cap_bits = capability.bits();
    assert_eq!(cap_bits.count_ones(), 1);
    let cap_index = cap_bits.leading_zeros();
    let (data_index, cap_index) = if cap_index < 32 {
        (0, cap_index)
    } else {
        (1, cap_index - 32)
    };
    let flag = 1_u32 << cap_index;
    Ok((flag & data[data_index].effective) != 0)
}

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
