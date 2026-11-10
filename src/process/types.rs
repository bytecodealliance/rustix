//! Types for use with [`rustix::process`] functions.
//!
//! [`rustix::process`]: crate::process

#![allow(unsafe_code)]

#[cfg(not(target_os = "horizon"))]
use crate::backend::c;
#[cfg(not(target_os = "horizon"))]
use crate::pid::Pid;
#[cfg(not(target_os = "horizon"))]
use core::mem::transmute;

/// File lock data structure used in [`fcntl_getlk`].
///
/// On Linux, this is also used in `fcntl_ofd_getlk`.
///
/// [`fcntl_getlk`]: crate::process::fcntl_getlk()
#[cfg(not(target_os = "horizon"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flock {
    /// Starting offset for lock
    pub start: u64,
    /// Number of bytes to lock
    pub length: u64,
    /// PID of the process blocking our lock. On input, `None` refers to the
    /// current process. On output, `None` indicates an open file description
    /// lock, which has no owning process.
    pub pid: Option<Pid>,
    /// Type of lock
    pub typ: FlockType,
    /// Offset type of lock
    pub offset_type: FlockOffsetType,
}

#[cfg(not(target_os = "horizon"))]
impl Flock {
    pub(crate) const unsafe fn from_raw_unchecked(raw_fl: c::flock) -> Self {
        // Open file description locks have no owning process; the kernel
        // reports these with `l_pid` set to -1.
        let pid = if raw_fl.l_pid == -1 {
            None
        } else {
            Pid::from_raw(raw_fl.l_pid)
        };
        #[cfg(not(all(target_os = "hurd", target_arch = "x86")))]
        {
            Self {
                start: raw_fl.l_start as _,
                length: raw_fl.l_len as _,
                pid,
                typ: transmute::<i16, FlockType>(raw_fl.l_type),
                offset_type: transmute::<i16, FlockOffsetType>(raw_fl.l_whence),
            }
        }
        #[cfg(all(target_os = "hurd", target_arch = "x86"))]
        {
            Self {
                start: raw_fl.l_start as _,
                length: raw_fl.l_len as _,
                pid,
                typ: transmute::<i32, FlockType>(raw_fl.l_type),
                offset_type: transmute::<i32, FlockOffsetType>(raw_fl.l_whence),
            }
        }
    }

    pub(crate) fn as_raw(&self) -> c::flock {
        let mut f: c::flock = unsafe { core::mem::zeroed() };
        f.l_start = self.start as _;
        f.l_len = self.length as _;
        f.l_pid = Pid::as_raw(self.pid);
        f.l_type = self.typ as _;
        f.l_whence = self.offset_type as _;
        f
    }
}

#[cfg(not(target_os = "horizon"))]
impl From<FlockType> for Flock {
    fn from(value: FlockType) -> Self {
        Self {
            start: 0,
            length: 0,
            pid: None,
            typ: value,
            offset_type: FlockOffsetType::Set,
        }
    }
}

/// `F_*LCK` constants for use with [`fcntl_getlk`].
///
/// [`fcntl_getlk`]: crate::process::fcntl_getlk()
#[cfg(not(target_os = "horizon"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(all(target_os = "hurd", target_arch = "x86")), repr(i16))]
#[cfg_attr(all(target_os = "hurd", target_arch = "x86"), repr(i32))]
pub enum FlockType {
    /// `F_RDLCK`
    ReadLock = c::F_RDLCK as _,
    /// `F_WRLCK`
    WriteLock = c::F_WRLCK as _,
    /// `F_UNLCK`
    Unlocked = c::F_UNLCK as _,
}

/// `F_SEEK*` constants for use with [`fcntl_getlk`].
///
/// [`fcntl_getlk`]: crate::process::fcntl_getlk()
#[cfg(not(target_os = "horizon"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(all(target_os = "hurd", target_arch = "x86")), repr(i16))]
#[cfg_attr(all(target_os = "hurd", target_arch = "x86"), repr(i32))]
pub enum FlockOffsetType {
    /// `F_SEEK_SET`
    Set = c::SEEK_SET as _,
    /// `F_SEEK_CUR`
    Current = c::SEEK_CUR as _,
    /// `F_SEEK_END`
    End = c::SEEK_END as _,
}
