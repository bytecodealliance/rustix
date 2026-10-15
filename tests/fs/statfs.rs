#[cfg(linux_kernel)]
#[test]
fn test_statfs_abi() {
    use rustix::fs::{FsWord, StatFs, NFS_SUPER_MAGIC, PROC_SUPER_MAGIC};

    // Ensure these all have consistent types.
    let t: StatFs = unsafe { std::mem::zeroed() };
    let _s: FsWord = t.f_type;
    let _u: FsWord = PROC_SUPER_MAGIC;
    let _v: FsWord = NFS_SUPER_MAGIC;

    // Ensure that after all the platform-specific dancing we have to do, this
    // constant comes out with the correct value.
    #[cfg(all(libc, not(target_env = "musl")))]
    {
        assert_eq!(
            i128::from(PROC_SUPER_MAGIC),
            i128::from(libc::PROC_SUPER_MAGIC)
        );
        assert_eq!(
            i128::from(NFS_SUPER_MAGIC),
            i128::from(libc::NFS_SUPER_MAGIC)
        );
    }

    #[cfg(linux_raw)]
    {
        assert_eq!(
            i128::from(PROC_SUPER_MAGIC),
            i128::from(linux_raw_sys::general::PROC_SUPER_MAGIC)
        );
        assert_eq!(
            i128::from(NFS_SUPER_MAGIC),
            i128::from(linux_raw_sys::general::NFS_SUPER_MAGIC)
        );
    }

    assert_eq!(PROC_SUPER_MAGIC, 0x0000_9fa0);
    assert_eq!(NFS_SUPER_MAGIC, 0x0000_6969);
}

#[cfg(not(any(solarish, target_os = "netbsd")))]
#[test]
fn test_statfs() {
    let statfs = rustix::fs::statfs("Cargo.toml").unwrap();
    let f_blocks = statfs.f_blocks;
    assert_ne!(f_blocks, 0);
    // Previously we checked f_files != 0 here, but at least btrfs doesn't set
    // that.
}

#[cfg(not(any(solarish, target_os = "netbsd")))]
#[test]
fn test_fstatfs() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    let statfs = rustix::fs::fstatfs(&file).unwrap();
    let f_blocks = statfs.f_blocks;
    assert_ne!(f_blocks, 0);
    // Previously we checked f_files != 0 here, but at least btrfs doesn't set
    // that.
}

/// Test that files in procfs are in a filesystem with `PROC_SUPER_MAGIC`.
#[cfg(linux_kernel)]
#[test]
fn test_statfs_procfs() {
    let statfs = rustix::fs::statfs("/proc/self/maps").unwrap();

    assert_eq!(statfs.f_type, rustix::fs::PROC_SUPER_MAGIC);
}

#[test]
fn test_statvfs() {
    let statvfs = rustix::fs::statvfs("Cargo.toml").unwrap();

    let f_frsize = statvfs.f_frsize;
    assert_ne!(f_frsize, 0);
}

/// `StatVfsMountFlags` has the `ST_*` values, not the `MS_*` ones.
#[cfg(all(linux_kernel, target_env = "gnu"))]
#[test]
fn test_statvfs_mount_flags_abi() {
    use rustix::fs::StatVfsMountFlags as Flags;

    assert_eq!(Flags::MANDLOCK.bits(), libc::ST_MANDLOCK as u64);
    assert_eq!(Flags::NOATIME.bits(), libc::ST_NOATIME as u64);
    assert_eq!(Flags::NODEV.bits(), libc::ST_NODEV as u64);
    assert_eq!(Flags::NODIRATIME.bits(), libc::ST_NODIRATIME as u64);
    assert_eq!(Flags::NOEXEC.bits(), libc::ST_NOEXEC as u64);
    assert_eq!(Flags::NOSUID.bits(), libc::ST_NOSUID as u64);
    assert_eq!(Flags::RDONLY.bits(), libc::ST_RDONLY as u64);
    assert_eq!(Flags::RELATIME.bits(), libc::ST_RELATIME as u64);
    assert_eq!(Flags::SYNCHRONOUS.bits(), libc::ST_SYNCHRONOUS as u64);
}

/// The flags `statvfs` reports for a mount are the per-mount options its
/// `/proc/self/mountinfo` line lists.
#[cfg(linux_kernel)]
#[test]
fn test_statvfs_mount_flags_match_mountinfo() {
    use rustix::fs::StatVfsMountFlags as Flags;

    let table = std::fs::read_to_string("/proc/self/mountinfo").unwrap();
    let mut checked = 0;
    for point in ["/", "/proc"] {
        // The last line at a path is the mount on top, which the path
        // reaches. Field 5 is the mount point, field 6 the per-mount options.
        let Some(options) = table
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .filter(|fields| fields.len() > 5 && fields[4] == point)
            .map(|fields| fields[5].to_owned())
            .next_back()
        else {
            continue;
        };
        let options: Vec<&str> = options.split(',').collect();
        let flags = rustix::fs::statvfs(point).unwrap().f_flag;
        #[allow(unused_mut)]
        let mut pairs = vec![
            ("ro", Flags::RDONLY),
            ("nosuid", Flags::NOSUID),
            ("nodev", Flags::NODEV),
            ("noexec", Flags::NOEXEC),
            ("noatime", Flags::NOATIME),
            ("nodiratime", Flags::NODIRATIME),
        ];
        // The libc backend has `RELATIME` only where libc defines
        // `ST_RELATIME`.
        #[cfg(any(
            linux_raw,
            target_os = "android",
            all(target_os = "linux", target_env = "gnu")
        ))]
        pairs.push(("relatime", Flags::RELATIME));
        for (option, flag) in pairs {
            assert_eq!(
                flags.contains(flag),
                options.contains(&option),
                "{point}: {option} in {options:?}, statvfs says {flags:?}"
            );
        }
        checked += 1;
    }
    assert_ne!(checked, 0, "neither / nor /proc is in the mount table");
}

#[test]
fn test_fstatvfs() {
    let file = std::fs::File::open("Cargo.toml").unwrap();
    let statvfs = rustix::fs::fstatvfs(&file).unwrap();

    let f_frsize = statvfs.f_frsize;
    assert_ne!(f_frsize, 0);
}
