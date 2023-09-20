#![allow(missing_docs)]

use crate::{
    backend::{self, c},
    io,
};

/// Reboot command to be used with [`reboot`]
///
/// [`reboot`]: crate::process::reboot
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RebootCommand {
    CadOff = c::LINUX_REBOOT_CMD_CAD_OFF,
    CadOn = c::LINUX_REBOOT_CMD_CAD_ON,
    Halt = c::LINUX_REBOOT_CMD_HALT,
    Kexec = c::LINUX_REBOOT_CMD_KEXEC,
    PowerOff = c::LINUX_REBOOT_CMD_POWER_OFF,
    Restart = c::LINUX_REBOOT_CMD_RESTART,
    Restart2 = c::LINUX_REBOOT_CMD_RESTART2,
    SwSuspend = c::LINUX_REBOOT_CMD_SW_SUSPEND,
}

pub fn reboot(cmd: RebootCommand) -> io::Result<()> {
    backend::process::syscalls::reboot(cmd)
}
