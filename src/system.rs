//! Uname and other system-level functions.
//!
//! # Safety
//!
//! This function converts from `struct utsname` fields provided from the
//! kernel into `&str` references, which assumes that they're NUL-terminated.
#![allow(unsafe_code)]

use crate::backend::{self, c};
use crate::ffi::CStr;
#[cfg(not(any(target_os = "espidf", target_os = "emscripten")))]
use crate::io;
use core::fmt;

#[cfg(linux_kernel)]
pub use backend::system::types::Sysinfo;

/// `uname()`—Returns high-level information about the runtime OS and
/// hardware.
///
/// # References
///  - [POSIX]
///  - [Linux]
///  - [Apple]
///  - [NetBSD]
///  - [FreeBSD]
///  - [OpenBSD]
///  - [DragonFly BSD]
///  - [illumos]
///  - [glibc]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/uname.html
/// [Linux]: https://man7.org/linux/man-pages/man2/uname.2.html
/// [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/uname.3.html
/// [NetBSD]: https://man.netbsd.org/uname.3
/// [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=uname&sektion=3
/// [OpenBSD]: https://man.openbsd.org/uname.3
/// [DragonFly BSD]: https://man.dragonflybsd.org/?command=uname&section=3
/// [illumos]: https://illumos.org/man/2/uname
/// [glibc]: https://www.gnu.org/software/libc/manual/html_node/Platform-Type.html
#[inline]
pub fn uname() -> Uname {
    Uname(backend::system::syscalls::uname())
}

/// `struct utsname`—Return type for [`uname`].
#[doc(alias = "utsname")]
pub struct Uname(backend::system::types::RawUname);

impl Uname {
    /// `sysname`—Operating system release name
    #[inline]
    pub fn sysname(&self) -> &CStr {
        Self::to_cstr(self.0.sysname.as_ptr().cast())
    }

    /// `nodename`—Name with vague meaning
    ///
    /// This is intended to be a network name, however it's unable to convey
    /// information about hosts that have multiple names, or any information
    /// about where the names are visible.
    #[inline]
    pub fn nodename(&self) -> &CStr {
        Self::to_cstr(self.0.nodename.as_ptr().cast())
    }

    /// `release`—Operating system release version string
    #[inline]
    pub fn release(&self) -> &CStr {
        Self::to_cstr(self.0.release.as_ptr().cast())
    }

    /// `version`—Operating system build identifiers
    #[inline]
    pub fn version(&self) -> &CStr {
        Self::to_cstr(self.0.version.as_ptr().cast())
    }

    /// `machine`—Hardware architecture identifier
    #[inline]
    pub fn machine(&self) -> &CStr {
        Self::to_cstr(self.0.machine.as_ptr().cast())
    }

    /// `domainname`—NIS or YP domain identifier
    #[cfg(linux_kernel)]
    #[inline]
    pub fn domainname(&self) -> &CStr {
        Self::to_cstr(self.0.domainname.as_ptr().cast())
    }

    #[inline]
    fn to_cstr<'a>(ptr: *const u8) -> &'a CStr {
        // SAFETY: Strings returned from the kernel are always NUL-terminated.
        unsafe { CStr::from_ptr(ptr.cast()) }
    }
}

impl fmt::Debug for Uname {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(linux_kernel))]
        {
            write!(
                fmt,
                "{:?} {:?} {:?} {:?} {:?}",
                self.sysname(),
                self.nodename(),
                self.release(),
                self.version(),
                self.machine(),
            )
        }
        #[cfg(linux_kernel)]
        {
            write!(
                fmt,
                "{:?} {:?} {:?} {:?} {:?} {:?}",
                self.sysname(),
                self.nodename(),
                self.release(),
                self.version(),
                self.machine(),
                self.domainname(),
            )
        }
    }
}

/// `sysinfo()`—Returns status information about the runtime OS.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/uname.2.html
#[cfg(linux_kernel)]
#[inline]
pub fn sysinfo() -> Sysinfo {
    backend::system::syscalls::sysinfo()
}

/// `sethostname(name)`—Sets the system host name.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/sethostname.2.html
#[cfg(not(any(
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "redox",
    target_os = "wasi"
)))]
#[inline]
pub fn sethostname(name: &[u8]) -> io::Result<()> {
    backend::system::syscalls::sethostname(name)
}

/// Reboot command to be used with [`reboot`]
///
/// [`reboot`]: crate::system::reboot
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RebootCommand {
    /// CAD is disabled.
    /// This means that the CAD keystroke will cause a SIGINT signal to be sent to init (process 1),
    /// whereupon this process may decide upon a proper action (maybe: kill all processes, sync, reboot).
    CadOff = c::LINUX_REBOOT_CMD_CAD_OFF,
    /// CAD is enabled.
    /// This means that the CAD keystroke will immediately cause the action associated with LINUX_REBOOT_CMD_RESTART.
    CadOn = c::LINUX_REBOOT_CMD_CAD_ON,
    /// The message "System halted." is printed, and the system is halted.
    /// Control is given to the ROM monitor, if there is one.  
    /// If not preceded by a [`sync`], data will be lost.
    ///
    /// [`sync`]: crate::fs::sync
    Halt = c::LINUX_REBOOT_CMD_HALT,
    /// Execute a kernel that has been loaded earlier with [`kexec_load`].
    /// This option is available only if the kernel was configured with CONFIG_KEXEC.
    ///
    /// [`kexec_load`]: https://man7.org/linux/man-pages/man2/kexec_load.2.html
    Kexec = c::LINUX_REBOOT_CMD_KEXEC,
    /// The message "Power down." is printed, the system is stopped, and all power is removed from the system, if possible.
    /// If not preceded by a [`sync`], data will be lost.
    ///
    /// [`sync`]: crate::fs::sync
    PowerOff = c::LINUX_REBOOT_CMD_POWER_OFF,
    /// The message "Restarting system." is printed, and a default restart is performed immediately.
    /// If not preceded by a [`sync`], data will be lost.
    ///
    /// [`sync`]: crate::fs::sync
    Restart = c::LINUX_REBOOT_CMD_RESTART,
    /// The message "Restarting system with command '%s'" is printed, and a restart (using the command string given in arg) is performed immediately.
    /// If not preceded by a [`sync`], data will be lost.
    ///
    /// [`sync`]: crate::fs::sync
    Restart2 = c::LINUX_REBOOT_CMD_RESTART2,
    /// The system is suspended (hibernated) to disk.
    /// This option is available only if the kernel was configured with CONFIG_HIBERNATION.
    SwSuspend = c::LINUX_REBOOT_CMD_SW_SUSPEND,
}

/// `reboot`—Reboot or enable/disable Ctrl-Alt-Del
///
/// # References
/// - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/reboot.2.html
pub fn reboot(cmd: RebootCommand) -> io::Result<()> {
    backend::system::syscalls::reboot(cmd)
}
