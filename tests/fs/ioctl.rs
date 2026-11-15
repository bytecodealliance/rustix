// SPARC lacks `FICLONE`.
#[cfg(all(linux_kernel, not(any(target_arch = "sparc", target_arch = "sparc64"))))]
#[test]
fn test_ioctl_ficlone() {
    use rustix::io;

    let src = std::fs::File::open("Cargo.toml").unwrap();
    let dest = tempfile::tempfile().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let dir = std::fs::File::open(dir.path()).unwrap();

    // `src` isn't opened for writing, so passing it as the output fails.
    assert_eq!(rustix::fs::ioctl_ficlone(&src, &src), Err(io::Errno::BADF));

    // `FICLONE` operates on regular files, not directories.
    assert_eq!(rustix::fs::ioctl_ficlone(&dir, &dir), Err(io::Errno::ISDIR));

    // Now try something that might succeed, though be prepared for filesystems
    // that don't support this.
    match rustix::fs::ioctl_ficlone(&dest, &src) {
        Ok(()) | Err(io::Errno::OPNOTSUPP) => (),
        Err(e) if e == io::Errno::from_raw_os_error(0x12) => (),
        Err(err) => panic!("{:?}", err),
    }
}

#[cfg(linux_raw_dep)]
#[repr(C)]
#[derive(Debug, Default)]
struct RawFsxattr {
    fsx_xflags: u32,
    fsx_extsize: u32,
    fsx_nextents: u32,
    fsx_projid: u32,
    fsx_cowextsize: u32,
    fsx_pad: [u8; 8],
}

#[cfg(linux_raw_dep)]
fn raw_fsgetxattr(file: &std::fs::File) -> Result<RawFsxattr, rustix::io::Errno> {
    use std::os::unix::io::AsRawFd;

    let mut raw = RawFsxattr::default();
    let request = rustix::ioctl::opcode::read::<RawFsxattr>(b'X', 31);
    let result = unsafe { libc::ioctl(file.as_raw_fd(), request as _, &mut raw) };
    if result == -1 {
        Err(rustix::io::Errno::from_raw_os_error(libc_errno::errno().0))
    } else {
        assert_eq!(result, 0);
        Ok(raw)
    }
}

#[cfg(linux_raw_dep)]
#[test]
fn test_fsgetxattr_flag_names_and_unknown_bits() {
    use rustix::fs::{FsxattrFlags, IFlags};

    const UNKNOWN_XFLAG: u32 = 0x0000_0004;

    let xflags = FsxattrFlags::from_bits_retain(FsxattrFlags::APPEND.bits() | UNKNOWN_XFLAG);
    assert!(xflags.contains(FsxattrFlags::APPEND));
    assert_eq!((xflags & !FsxattrFlags::APPEND).bits(), UNKNOWN_XFLAG);

    assert_eq!(
        IFlags::ENCRYPTED.bits(),
        linux_raw_sys::general::FS_ENCRYPT_FL
    );
    assert_eq!(IFlags::VERITY.bits(), linux_raw_sys::general::FS_VERITY_FL);
    assert_eq!(
        IFlags::CASEFOLD.bits(),
        linux_raw_sys::general::FS_CASEFOLD_FL
    );
}

#[cfg(linux_raw_dep)]
#[test]
fn test_fsgetxattr_raw_layout() {
    use core::mem::{align_of, size_of};
    use linux_raw_sys::general::fsxattr as LinuxFsxattr;
    use memoffset::offset_of;

    assert_eq!(size_of::<RawFsxattr>(), 28);
    assert_eq!(align_of::<RawFsxattr>(), 4);
    assert_eq!(size_of::<LinuxFsxattr>(), size_of::<RawFsxattr>());
    assert_eq!(align_of::<LinuxFsxattr>(), align_of::<RawFsxattr>());

    assert_eq!(offset_of!(RawFsxattr, fsx_xflags), 0);
    assert_eq!(offset_of!(RawFsxattr, fsx_extsize), 4);
    assert_eq!(offset_of!(RawFsxattr, fsx_nextents), 8);
    assert_eq!(offset_of!(RawFsxattr, fsx_projid), 12);
    assert_eq!(offset_of!(RawFsxattr, fsx_cowextsize), 16);
    assert_eq!(offset_of!(RawFsxattr, fsx_pad), 20);

    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_xflags),
        offset_of!(RawFsxattr, fsx_xflags)
    );
    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_extsize),
        offset_of!(RawFsxattr, fsx_extsize)
    );
    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_nextents),
        offset_of!(RawFsxattr, fsx_nextents)
    );
    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_projid),
        offset_of!(RawFsxattr, fsx_projid)
    );
    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_cowextsize),
        offset_of!(RawFsxattr, fsx_cowextsize)
    );
    assert_eq!(
        offset_of!(LinuxFsxattr, fsx_pad),
        offset_of!(RawFsxattr, fsx_pad)
    );
}

#[cfg(linux_raw_dep)]
#[test]
fn test_ioctl_fsgetxattr_matches_raw_ioctl() {
    use rustix::io;

    let file = tempfile::tempfile().unwrap();
    let attrs = match rustix::fs::ioctl_fsgetxattr(&file) {
        Ok(attrs) => attrs,
        Err(err) if matches!(err, io::Errno::NOTTY | io::Errno::OPNOTSUPP) => {
            assert_eq!(raw_fsgetxattr(&file).unwrap_err(), err);
            eprintln!("skipped: the temporary filesystem does not support FS_IOC_FSGETXATTR");
            return;
        }
        Err(err) => panic!("FS_IOC_FSGETXATTR failed on the temporary file: {err:?}"),
    };
    let raw = raw_fsgetxattr(&file).unwrap();

    assert_eq!(attrs.fsx_xflags.bits(), raw.fsx_xflags);
    assert_eq!(attrs.fsx_extsize, raw.fsx_extsize);
    assert_eq!(attrs.fsx_nextents, raw.fsx_nextents);
    assert_eq!(attrs.fsx_projid, raw.fsx_projid);
    assert_eq!(attrs.fsx_cowextsize, raw.fsx_cowextsize);
    assert_eq!(raw.fsx_pad, [0; 8]);
}

#[cfg(linux_raw_dep)]
#[test]
fn test_ioctl_fsgetxattr_unsupported_fd_errno() {
    use rustix::io;

    let null = std::fs::File::open("/dev/null").unwrap();
    let safe_errno = rustix::fs::ioctl_fsgetxattr(&null).unwrap_err();
    let raw_errno = raw_fsgetxattr(&null).unwrap_err();

    assert_eq!(safe_errno, raw_errno);
    assert_eq!(safe_errno, io::Errno::NOTTY);
}
