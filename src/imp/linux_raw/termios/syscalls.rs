//! linux_raw syscalls supporting `rustix::termios`.
//!
//! # Safety
//!
//! See the `rustix::imp::syscalls` module documentation for details.

#![allow(unsafe_code)]

use super::super::arch::choose::{syscall3, syscall3_readonly};
use super::super::conv::{borrowed_fd, by_ref, c_uint, out, regular_pid, ret};
use super::super::reg::nr;
use crate::fd::BorrowedFd;
use crate::io;
use crate::process::{Pid, RawNonZeroPid};
use crate::termios::{
    Action, OptionalActions, QueueSelector, Termios, Winsize, BRKINT, CBAUD, CS8, CSIZE, ECHO,
    ECHONL, ICANON, ICRNL, IEXTEN, IGNBRK, IGNCR, INLCR, ISIG, ISTRIP, IXON, OPOST, PARENB, PARMRK,
    VMIN, VTIME,
};
#[cfg(feature = "procfs")]
use crate::{ffi::ZStr, fs::FileType, path::DecInt};
use core::mem::MaybeUninit;
use linux_raw_sys::general::{__NR_ioctl, __kernel_pid_t};
use linux_raw_sys::ioctl::{
    TCFLSH, TCGETS, TCSBRK, TCSETS, TCXONC, TIOCGPGRP, TIOCGSID, TIOCGWINSZ, TIOCSPGRP, TIOCSWINSZ,
};

#[inline]
pub(crate) fn tcgetwinsize(fd: BorrowedFd<'_>) -> io::Result<Winsize> {
    unsafe {
        let mut result = MaybeUninit::<Winsize>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCGWINSZ),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn tcgetattr(fd: BorrowedFd<'_>) -> io::Result<Termios> {
    unsafe {
        let mut result = MaybeUninit::<Termios>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCGETS),
            out(&mut result),
        ))
        .map(|()| result.assume_init())
    }
}

#[inline]
pub(crate) fn tcgetpgrp(fd: BorrowedFd<'_>) -> io::Result<Pid> {
    unsafe {
        let mut result = MaybeUninit::<__kernel_pid_t>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCGPGRP),
            out(&mut result),
        ))
        .map(|()| {
            let pid = result.assume_init();
            debug_assert!(pid > 0);
            Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(pid as u32))
        })
    }
}

#[inline]
pub(crate) fn tcsetattr(
    fd: BorrowedFd,
    optional_actions: OptionalActions,
    termios: &Termios,
) -> io::Result<()> {
    // Translate from `optional_actions` into an ioctl request code. On MIPS,
    // `optional_actions` already has `TCGETS` added to it.
    let request = if cfg!(any(target_arch = "mips", target_arch = "mips64")) {
        optional_actions as u32
    } else {
        TCSETS + optional_actions as u32
    };
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(request as u32),
            by_ref(termios),
        ))
    }
}

#[inline]
pub(crate) fn tcsendbreak(fd: BorrowedFd) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCSBRK),
            c_uint(0),
        ))
    }
}

#[inline]
pub(crate) fn tcdrain(fd: BorrowedFd) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCSBRK),
            c_uint(1),
        ))
    }
}

#[inline]
pub(crate) fn tcflush(fd: BorrowedFd, queue_selector: QueueSelector) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCFLSH),
            c_uint(queue_selector as u32),
        ))
    }
}

#[inline]
pub(crate) fn tcflow(fd: BorrowedFd, action: Action) -> io::Result<()> {
    unsafe {
        ret(syscall3_readonly(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TCXONC),
            c_uint(action as u32),
        ))
    }
}

#[inline]
pub(crate) fn tcgetsid(fd: BorrowedFd) -> io::Result<Pid> {
    unsafe {
        let mut result = MaybeUninit::<__kernel_pid_t>::uninit();
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCGSID),
            out(&mut result),
        ))
        .map(|()| {
            let pid = result.assume_init();
            debug_assert!(pid > 0);
            Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(pid as u32))
        })
    }
}

#[inline]
pub(crate) fn tcsetwinsize(fd: BorrowedFd, winsize: Winsize) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCSWINSZ),
            by_ref(&winsize),
        ))
    }
}

#[inline]
pub(crate) fn tcsetpgrp(fd: BorrowedFd<'_>, pid: Pid) -> io::Result<()> {
    unsafe {
        ret(syscall3(
            nr(__NR_ioctl),
            borrowed_fd(fd),
            c_uint(TIOCSPGRP),
            regular_pid(pid),
        ))
    }
}

#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn cfgetospeed(termios: &Termios) -> u32 {
    termios.c_cflag & CBAUD
}

#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn cfgetispeed(termios: &Termios) -> u32 {
    termios.c_cflag & CBAUD
}

#[inline]
pub fn cfmakeraw(termios: &mut Termios) {
    // From the Linux [`cfmakeraw` man page]:
    //
    // [`cfmakeraw` man page]: https://man7.org/linux/man-pages/man3/cfmakeraw.3.html
    termios.c_iflag &= !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
    termios.c_oflag &= !OPOST;
    termios.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    termios.c_cflag &= !(CSIZE | PARENB);
    termios.c_cflag |= CS8;

    // Musl and glibc also do these:
    termios.c_cc[VMIN] = 1;
    termios.c_cc[VTIME] = 0;
}

#[inline]
pub fn cfsetospeed(termios: &mut Termios, speed: u32) -> io::Result<()> {
    if (speed & !CBAUD) != 0 {
        return Err(io::Error::INVAL);
    }
    termios.c_cflag &= !CBAUD;
    termios.c_cflag |= speed;
    Ok(())
}

#[inline]
pub fn cfsetispeed(termios: &mut Termios, speed: u32) -> io::Result<()> {
    if speed == 0 {
        return Ok(());
    }
    if (speed & !CBAUD) != 0 {
        return Err(io::Error::INVAL);
    }
    termios.c_cflag &= !CBAUD;
    termios.c_cflag |= speed;
    Ok(())
}

#[inline]
pub fn cfsetspeed(termios: &mut Termios, speed: u32) -> io::Result<()> {
    if (speed & !CBAUD) != 0 {
        return Err(io::Error::INVAL);
    }
    termios.c_cflag &= !CBAUD;
    termios.c_cflag |= speed;
    Ok(())
}

#[inline]
pub(crate) fn isatty(fd: BorrowedFd<'_>) -> bool {
    // On error, Linux will return either `EINVAL` (2.6.32) or `ENOTTY`
    // (otherwise), because we assume we're never passing an invalid
    // file descriptor (which would get `EBADF`). Either way, an error
    // means we don't have a tty.
    tcgetwinsize(fd).is_ok()
}

#[cfg(feature = "procfs")]
pub(crate) fn ttyname(fd: BorrowedFd<'_>, buf: &mut [u8]) -> io::Result<usize> {
    let fd_stat = super::super::fs::syscalls::fstat(fd)?;

    // Quick check: if `fd` isn't a character device, it's not a tty.
    if FileType::from_raw_mode(fd_stat.st_mode) != FileType::CharacterDevice {
        return Err(crate::io::Error::NOTTY);
    }

    // Check that `fd` is really a tty.
    tcgetwinsize(fd)?;

    // Get a fd to '/proc/self/fd'.
    let proc_self_fd = io::proc_self_fd()?;

    // Gather the ttyname by reading the 'fd' file inside 'proc_self_fd'.
    let r =
        super::super::fs::syscalls::readlinkat(proc_self_fd, DecInt::from_fd(&fd).as_z_str(), buf)?;

    // If the number of bytes is equal to the buffer length, truncation may
    // have occurred. This check also ensures that we have enough space for
    // adding a NUL terminator.
    if r == buf.len() {
        return Err(io::Error::RANGE);
    }
    buf[r] = b'\0';

    // Check that the path we read refers to the same file as `fd`.
    let path = ZStr::from_bytes_with_nul(&buf[..=r]).unwrap();

    let path_stat = super::super::fs::syscalls::stat(path)?;
    if path_stat.st_dev != fd_stat.st_dev || path_stat.st_ino != fd_stat.st_ino {
        return Err(crate::io::Error::NODEV);
    }

    Ok(r)
}
