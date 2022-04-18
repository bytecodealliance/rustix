use crate::termios::{Speed, Termios};
use crate::{imp, io};

/// `cfgetospeed(termios)`
#[inline]
#[must_use]
pub fn cfgetospeed(termios: &Termios) -> Speed {
    imp::termios::syscalls::cfgetospeed(termios)
}

/// `cfgetispeed(termios)`
#[inline]
#[must_use]
pub fn cfgetispeed(termios: &Termios) -> Speed {
    imp::termios::syscalls::cfgetispeed(termios)
}

/// `cfmakeraw(termios)`
#[inline]
pub fn cfmakeraw(termios: &mut Termios) {
    imp::termios::syscalls::cfmakeraw(termios)
}

/// `cfsetospeed(termios, speed)`
#[inline]
pub fn cfsetospeed(termios: &mut Termios, speed: Speed) -> io::Result<()> {
    imp::termios::syscalls::cfsetospeed(termios, speed)
}

/// `cfsetispeed(termios, speed)`
#[inline]
pub fn cfsetispeed(termios: &mut Termios, speed: Speed) -> io::Result<()> {
    imp::termios::syscalls::cfsetispeed(termios, speed)
}

/// `cfsetspeed(termios, speed)`
#[inline]
pub fn cfsetspeed(termios: &mut Termios, speed: Speed) -> io::Result<()> {
    imp::termios::syscalls::cfsetspeed(termios, speed)
}
