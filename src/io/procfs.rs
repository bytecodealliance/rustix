//! Utilities for working with `/proc`, where Linux's `procfs` is typically
//! mounted. `/proc` serves as an adjunct to Linux's main syscall surface area,
//! providing additional features with an awkward interface.
//!
//! This module does a considerable amount of work to determine whether `/proc`
//! is mounted, with actual `procfs`, and without any additional mount points
//! on top of the paths we open.

use crate::{
    fs::{cwd, fstat, fstatfs, major, openat, renameat, Mode, OFlags, Stat, PROC_SUPER_MAGIC},
    io::{self, OwnedFd},
    path::DecInt,
    process::{getgid, getpid, getuid},
};
use cstr::cstr;
use io_lifetimes::{AsFd, BorrowedFd};
use once_cell::sync::Lazy;

/// Linux's procfs always uses inode 1 for its root directory.
const PROC_ROOT_INO: u64 = 1;

// Identify a subdirectory of "/proc", to determine which anomalies to
// check for.
enum Subdir {
    Proc,
    Pid,
    Fd,
}

/// Check a subdirectory of "/proc" for anomalies.
fn check_proc_dir(
    kind: Subdir,
    dir: BorrowedFd<'_>,
    proc_stat: Option<&Stat>,
    uid: u32,
    gid: u32,
) -> io::Result<Stat> {
    // Check the filesystem magic.
    check_procfs(dir)?;

    let dir_stat = fstat(&dir)?;

    // We use `O_DIRECTORY`, so open should fail if we don't get a directory.
    assert_eq!(dir_stat.st_mode & Mode::IFMT.bits(), Mode::IFDIR.bits());

    // Check the root inode number.
    if let Subdir::Proc = kind {
        if dir_stat.st_ino != PROC_ROOT_INO {
            return Err(io::Error::NOTSUP);
        }

        // Proc is a non-device filesystem, so check for major number 0.
        // <https://www.kernel.org/doc/Documentation/admin-guide/devices.txt>
        if major(dir_stat.st_dev) != 0 {
            return Err(io::Error::NOTSUP);
        }

        // Check that "/proc" is a mountpoint.
        if !is_mountpoint(dir)? {
            return Err(io::Error::NOTSUP);
        }
    } else {
        // Check that we haven't been linked back to the root of "/proc".
        if dir_stat.st_ino == PROC_ROOT_INO {
            return Err(io::Error::NOTSUP);
        }

        // Check that we're still in procfs.
        if dir_stat.st_dev != proc_stat.unwrap().st_dev {
            return Err(io::Error::NOTSUP);
        }

        // Check that subdirectories of "/proc" are not mount points.
        if is_mountpoint(dir)? {
            return Err(io::Error::NOTSUP);
        }
    }

    // Check the ownership of the directory.
    if (dir_stat.st_uid, dir_stat.st_gid) != (uid, gid) {
        return Err(io::Error::NOTSUP);
    }

    // "/proc" directories are typically mounted r-xr-xr-x.
    // "/proc/self/fd" is r-x------. Allow them to have fewer permissions, but
    // not more.
    let expected_mode = if let Subdir::Fd = kind { 0o500 } else { 0o555 };
    if dir_stat.st_mode & 0o777 & !expected_mode != 0 {
        return Err(io::Error::NOTSUP);
    }

    if let Subdir::Fd = kind {
        // Check that the "/proc/self/fd" directory doesn't have any extraneous
        // links into it (which might include unexpected subdirectories).
        if dir_stat.st_nlink != 2 {
            return Err(io::Error::NOTSUP);
        }
    } else {
        // Check that the "/proc" and "/proc/self" directories aren't empty.
        if dir_stat.st_nlink <= 2 {
            return Err(io::Error::NOTSUP);
        }
    }

    Ok(dir_stat)
}

/// Check that `file` is opened on a `procfs` filesystem.
fn check_procfs(file: BorrowedFd<'_>) -> io::Result<()> {
    let statfs = fstatfs(&file)?;
    let f_type = statfs.f_type;
    if f_type != PROC_SUPER_MAGIC {
        return Err(io::Error::NOTSUP);
    }

    Ok(())
}

/// Check whether the given directory handle is a mount point. We use a
/// `rename` call that would otherwise fail, but which fails with `EXDEV`
/// first if it would cross a mount point.
fn is_mountpoint(file: BorrowedFd<'_>) -> io::Result<bool> {
    let err = renameat(&file, cstr!("../."), &file, cstr!(".")).unwrap_err();
    match err {
        io::Error::XDEV => Ok(true), // the rename failed due to crossing a mount point
        io::Error::BUSY => Ok(false), // the rename failed normally
        _ => panic!("Unexpected error from `renameat`: {:?}", err),
    }
}

/// Returns a handle to Linux's `/proc` directory.
///
/// This ensures that `procfs` is mounted on `/proc`, that nothing is
/// mounted on top of it, and that it looks normal. It also returns the
/// `Stat` of `/proc`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man5/proc.5.html
pub fn proc() -> io::Result<(BorrowedFd<'static>, &'static Stat)> {
    #[allow(clippy::useless_conversion)]
    static PROC: Lazy<io::Result<(OwnedFd, Stat)>> = Lazy::new(|| {
        let oflags =
            OFlags::NOFOLLOW | OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOCTTY;
        let proc: OwnedFd = openat(&cwd(), cstr!("/proc"), oflags, Mode::empty())?.into();
        let proc_stat = check_proc_dir(Subdir::Proc, proc.as_fd(), None, 0, 0)?;

        Ok((proc, proc_stat))
    });

    PROC.as_ref()
        .map(|(fd, stat)| (fd.as_fd(), stat))
        .map_err(|_err| io::Error::NOTSUP)
}

/// Returns a handle to Linux's `/proc/self` directory.
///
/// This ensures that `procfs` is mounted on `/proc/self`, that nothing is
/// mounted on top of it, and that it looks normal. It also returns the
/// `Stat` of `/proc/self`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man5/proc.5.html
pub fn proc_self() -> io::Result<(BorrowedFd<'static>, &'static Stat)> {
    #[allow(clippy::useless_conversion)]
    static PROC_SELF: Lazy<io::Result<(OwnedFd, Stat)>> = Lazy::new(|| {
        let (proc, proc_stat) = proc()?;

        let (uid, gid, pid) = (getuid(), getgid(), getpid());
        let oflags =
            OFlags::NOFOLLOW | OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOCTTY;

        // Open "/proc/self". Use our pid to compute the name rather than literally
        // using "self", as "self" is a symlink.
        let proc_self: OwnedFd = openat(&proc, DecInt::new(pid), oflags, Mode::empty())?.into();
        let proc_self_stat =
            check_proc_dir(Subdir::Pid, proc_self.as_fd(), Some(proc_stat), uid, gid)?;

        Ok((proc_self, proc_self_stat))
    });

    PROC_SELF
        .as_ref()
        .map(|(owned, stat)| (owned.as_fd(), stat))
        .map_err(|_err| io::Error::NOTSUP)
}

/// Returns a handle to Linux's `/proc/self/fd` directory.
///
/// This ensures that `procfs` is mounted on `/proc/self/fd`, that nothing is
/// mounted on top of it, and that it looks normal. It also returns the
/// `Stat` of `/proc/self/fd`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man5/proc.5.html
pub fn proc_self_fd() -> io::Result<(BorrowedFd<'static>, &'static Stat)> {
    #[allow(clippy::useless_conversion)]
    static PROC_SELF_FD: Lazy<io::Result<(OwnedFd, Stat)>> = Lazy::new(|| {
        let (_, proc_stat) = proc()?;

        let (proc_self, proc_self_stat) = proc_self()?;
        let oflags =
            OFlags::NOFOLLOW | OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOCTTY;

        // Open "/proc/self/fd".
        let proc_self_fd: OwnedFd = openat(&proc_self, cstr!("fd"), oflags, Mode::empty())?.into();
        let proc_self_fd_stat = check_proc_dir(
            Subdir::Fd,
            proc_self_fd.as_fd(),
            Some(proc_stat),
            proc_self_stat.st_uid,
            proc_self_stat.st_gid,
        )?;

        Ok((proc_self_fd, proc_self_fd_stat))
    });

    PROC_SELF_FD
        .as_ref()
        .map(|(owned, stat)| (owned.as_fd(), stat))
        .map_err(|_err| io::Error::NOTSUP)
}
