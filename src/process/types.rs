#![allow(unsafe_code)]

use crate::backend::c;
use crate::pid::Pid;
use core::mem::transmute;

/// File lock data structure used in [`fcntl_getlk`].
///
/// [`fcntl_getlk`]: crate::fs::fcntl_getlk
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flock {
    /// Starting offset for lock
    pub start: isize,
    /// Number of bytes to lock
    pub length: isize,
    /// PID of process blocking our lock. If set to `None`, it refers to the current process
    pub pid: Option<Pid>,
    /// Type of lock
    pub typ: FlockType,
    /// Offset type of lock
    pub offset_type: FlockOffsetType,
}

impl Flock {
    pub(crate) const unsafe fn from_raw_unchecked(raw_fl: c::flock) -> Flock {
        Flock {
            start: raw_fl.l_start as _,
            length: raw_fl.l_len as _,
            pid: transmute(raw_fl.l_pid),
            typ: transmute(raw_fl.l_type),
            offset_type: transmute(raw_fl.l_whence),
        }
    }

    pub(crate) fn as_raw(&self) -> c::flock {
        let mut f: c::flock = unsafe { core::mem::zeroed() };
        f.l_start = self.start as _;
        f.l_len = self.length as _;
        f.l_pid = unsafe { transmute(self.pid) };
        f.l_type = self.typ as _;
        f.l_whence = self.offset_type as _;
        f
    }
}

impl From<FlockType> for Flock {
    fn from(value: FlockType) -> Self {
        Flock {
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
/// [`fcntl_getlk`]: crate::fs::fcntl_getlk
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
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
/// [`fcntl_getlk`]: crate::fs::fcntl_getlk
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum FlockOffsetType {
    /// `F_SEEK_SET`
    Set = c::SEEK_SET as _,
    /// `F_SEEK_CUR`
    Current = c::SEEK_CUR as _,
    /// `F_SEEK_END`
    End = c::SEEK_END as _,
}
