use bitflags::bitflags;
use linux_raw_sys::general::{
    CLONE_FILES, CLONE_FS, CLONE_NEWCGROUP, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID,
    CLONE_NEWTIME, CLONE_NEWUSER, CLONE_NEWUTS, CLONE_SYSVSEM,
};

use crate::backend::c::{
    c_int, NS_GET_NSTYPE, NS_GET_OWNER_UID, NS_GET_PARENT, NS_GET_PID_FROM_PIDNS,
    NS_GET_PID_IN_PIDNS, NS_GET_TGID_FROM_PIDNS, NS_GET_TGID_IN_PIDNS, NS_GET_USERNS,
};
use crate::backend::thread::syscalls;
use crate::fd::BorrowedFd;
use crate::fd::{AsFd, FromRawFd, OwnedFd};
use crate::io::{self, Errno};
use crate::ioctl;

use super::{Pid, RawUid, Uid};

bitflags! {
    /// Namespace type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct NamespaceType: u32 {
        /// Control group (CGroup) namespace.
        const CGROUP = CLONE_NEWCGROUP;
        /// System V IPC and POSIX message queue namespace.
        const IPC = CLONE_NEWIPC;
        /// Mount namespace.
        const MOUNT = CLONE_NEWNS;
        /// Network namespace.
        const NETWORK = CLONE_NEWNET;
        /// Process ID namespace.
        const PID = CLONE_NEWPID;
        /// Time namespace.
        const TIME = CLONE_NEWTIME;
        /// User and group ID namespace.
        const USER = CLONE_NEWUSER;
        /// `Host name` and `NIS domain name` (UTS) namespace.
        const UTS = CLONE_NEWUTS;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `CLONE_*` for use with [`unshare`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct UnshareFlags: u32 {
        /// `CLONE_FILES`
        const FILES = CLONE_FILES;
        /// `CLONE_FS`
        const FS = CLONE_FS;
        /// `CLONE_NEWCGROUP`
        const NEWCGROUP = CLONE_NEWCGROUP;
        /// `CLONE_NEWIPC`
        const NEWIPC = CLONE_NEWIPC;
        /// `CLONE_NEWNET`
        const NEWNET = CLONE_NEWNET;
        /// `CLONE_NEWNS`
        const NEWNS = CLONE_NEWNS;
        /// `CLONE_NEWPID`
        const NEWPID = CLONE_NEWPID;
        /// `CLONE_NEWTIME`
        const NEWTIME = CLONE_NEWTIME;
        /// `CLONE_NEWUSER`
        const NEWUSER = CLONE_NEWUSER;
        /// `CLONE_NEWUTS`
        const NEWUTS = CLONE_NEWUTS;
        /// `CLONE_SYSVSEM`
        const SYSVSEM = CLONE_SYSVSEM;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

pub use allow_deprecated_workaround::*;
mod allow_deprecated_workaround {
    #![allow(deprecated)]

    use linux_raw_sys::general::{
        CLONE_NEWCGROUP, CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWTIME,
        CLONE_NEWUSER, CLONE_NEWUTS,
    };

    bitflags::bitflags! {
        /// Thread name space type.
        #[deprecated(since = "1.1.0", note = "Use NamespaceType instead")]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct ThreadNameSpaceType: u32 {
            /// Time name space.
            const TIME = CLONE_NEWTIME;
            /// Mount name space.
            const MOUNT = CLONE_NEWNS;
            /// Control group (CGroup) name space.
            const CONTROL_GROUP = CLONE_NEWCGROUP;
            /// `Host name` and `NIS domain name` (UTS) name space.
            const HOST_NAME_AND_NIS_DOMAIN_NAME = CLONE_NEWUTS;
            /// Inter-process communication (IPC) name space.
            const INTER_PROCESS_COMMUNICATION = CLONE_NEWIPC;
            /// User name space.
            const USER = CLONE_NEWUSER;
            /// Process ID name space.
            const PROCESS_ID = CLONE_NEWPID;
            /// Network name space.
            const NETWORK = CLONE_NEWNET;

            /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
            const _ = !0;
        }
    }
}

/// Type of name space referred to by a link.
#[deprecated(since = "1.1.0", note = "Use NamespaceType instead")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum LinkNameSpaceType {
    /// Time name space.
    Time = CLONE_NEWTIME,
    /// Mount name space.
    Mount = CLONE_NEWNS,
    /// Control group (CGroup) name space.
    ControlGroup = CLONE_NEWCGROUP,
    /// `Host name` and `NIS domain name` (UTS) name space.
    HostNameAndNISDomainName = CLONE_NEWUTS,
    /// Inter-process communication (IPC) name space.
    InterProcessCommunication = CLONE_NEWIPC,
    /// User name space.
    User = CLONE_NEWUSER,
    /// Process ID name space.
    ProcessID = CLONE_NEWPID,
    /// Network name space.
    Network = CLONE_NEWNET,
}

/// Move the calling thread into different namespaces
///
/// This function has two different semantics depending on the `fd` argument.
///
/// - If `fd` refers to one of the magic links in a `/proc/[pid]/ns/` directory
///   or a bind mount to such a link, the calling thread is moved to the namespaces
///   referred to by `fd`. `namespace_type` must either be [`NamespaceType::empty()`]
///   in which case all namespace types can be joined or a single [`NamespaceType`]
///   bit in which case only namespaces of this type can be joined.
/// - If `fd` refers to a pidfd, the calling thread is moved to all namespaces of this
///   process that are specified in `namespace_type`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/setns.2.html
#[deprecated(since = "1.1.0", note = "Use setns instead")]
#[doc(alias = "setns")]
pub fn set_namespace(fd: BorrowedFd<'_>, namespace_type: NamespaceType) -> io::Result<()> {
    syscalls::setns(fd, namespace_type.bits() as c_int)?;

    Ok(())
}

/// Reassociate the calling thread with the namespace associated with link
/// referred to by `fd`.
///
/// `fd` must refer to one of the magic links in a `/proc/[pid]/ns/` directory,
/// or a bind mount to such a link.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/setns.2.html
#[deprecated(since = "1.1.0", note = "Use setns instead")]
#[doc(alias = "setns")]
#[allow(deprecated)]
pub fn move_into_link_name_space(
    fd: BorrowedFd<'_>,
    allowed_type: Option<LinkNameSpaceType>,
) -> io::Result<()> {
    let allowed_type = allowed_type.map_or(0, |t| t as c_int);
    syscalls::setns(fd, allowed_type).map(|_r| ())
}

/// Atomically move the calling thread into one or more of the same namespaces
/// as the thread referred to by `fd`.
///
/// `fd` must refer to a thread ID. See: `pidfd_open` and `clone`.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/setns.2.html
#[deprecated(since = "1.1.0", note = "Use setns instead")]
#[doc(alias = "setns")]
#[allow(deprecated)]
pub fn move_into_thread_name_spaces(
    fd: BorrowedFd<'_>,
    allowed_types: ThreadNameSpaceType,
) -> io::Result<()> {
    syscalls::setns(fd, allowed_types.bits() as c_int).map(|_r| ())
}

/// `unshare(flags)`—Disassociate parts of the current thread's execution
/// context with other threads.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/unshare.2.html
pub fn unshare(flags: UnshareFlags) -> io::Result<()> {
    syscalls::unshare(flags)
}

/// `ioctl(ns_fd, NS_GET_USERNS)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/*` file.
#[inline]
#[doc(alias = "NS_GET_USERNS")]
pub fn ioctl_ns_get_userns<FD: AsFd>(fd: FD) -> io::Result<OwnedFd> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::NoArgGetter::<{ NS_GET_USERNS }>::new();
        ioctl::ioctl(fd, ctl).map(|fd| OwnedFd::from_raw_fd(fd))
    }
}

/// `ioctl(ns_fd, NS_GET_PARENT)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/*` file.
#[inline]
#[doc(alias = "NS_GET_PARENT")]
pub fn ioctl_ns_get_parent<FD: AsFd>(fd: FD) -> io::Result<OwnedFd> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::NoArgGetter::<{ NS_GET_PARENT }>::new();
        ioctl::ioctl(fd, ctl).map(|fd| OwnedFd::from_raw_fd(fd))
    }
}

/// `ioctl(ns_fd, NS_GET_NSTYPE)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/*` file.
#[inline]
#[doc(alias = "NS_GET_NSTYPE")]
pub fn ioctl_ns_get_nstype<FD: AsFd>(fd: FD) -> io::Result<NamespaceType> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::NoArgGetter::<{ NS_GET_NSTYPE }>::new();
        ioctl::ioctl(fd, ctl).map(|ns| NamespaceType::from_bits_retain(ns as u32))
    }
}

/// `ioctl(ns_fd, NS_GET_OWNER_UID)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/*` file.
#[inline]
#[doc(alias = "NS_GET_OWNER_UID")]
pub fn ioctl_ns_get_owner_uid<FD: AsFd>(fd: FD) -> io::Result<Uid> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::Getter::<{ NS_GET_OWNER_UID }, RawUid>::new();
        ioctl::ioctl(fd, ctl).map(Uid::from_raw)
    }
}

/// `ioctl(ns_fd, NS_GET_PID_FROM_PIDNS, pid)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/pid` file.
#[inline]
#[doc(alias = "NS_GET_PID_FROM_PIDNS")]
pub fn ioctl_ns_get_pid_from_pidns<FD: AsFd>(fd: FD, pid: Pid) -> io::Result<Pid> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::ParameterizedReturnGetter::<{ NS_GET_PID_FROM_PIDNS }>::new(
            pid.as_raw_pid() as usize,
        );
        ioctl::ioctl(fd, ctl).and_then(|pid| Pid::from_raw(pid).ok_or(Errno::INVAL))
    }
}

/// `ioctl(ns_fd, NS_GET_TGID_FROM_PIDNS, tgid)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/pid` file.
#[inline]
#[doc(alias = "NS_GET_TGID_FROM_PIDNS")]
pub fn ioctl_ns_get_tgid_from_pidns<FD: AsFd>(fd: FD, tgid: Pid) -> io::Result<Pid> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::ParameterizedReturnGetter::<{ NS_GET_TGID_FROM_PIDNS }>::new(
            tgid.as_raw_pid() as usize,
        );
        ioctl::ioctl(fd, ctl).and_then(|tgid| Pid::from_raw(tgid).ok_or(Errno::INVAL))
    }
}

/// `ioctl(ns_fd, NS_GET_PID_IN_PIDNS, pid)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/pid` file.
#[inline]
#[doc(alias = "NS_GET_PID_IN_PIDNS")]
pub fn ioctl_ns_get_pid_in_pidns<FD: AsFd>(fd: FD, pid: Pid) -> io::Result<Pid> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::ParameterizedReturnGetter::<{ NS_GET_PID_IN_PIDNS }>::new(
            pid.as_raw_pid() as usize,
        );
        ioctl::ioctl(fd, ctl).and_then(|pid| Pid::from_raw(pid).ok_or(Errno::INVAL))
    }
}

/// `ioctl(ns_fd, NS_GET_TGID_IN_PIDNS, tgid)`
///
/// # Safety
///
/// `fd` must refer to a `/proc/{pid}/ns/pid` file.
#[inline]
#[doc(alias = "NS_GET_TGID_IN_PIDNS")]
pub fn ioctl_ns_get_tgid_in_pidns<FD: AsFd>(fd: FD, tgid: Pid) -> io::Result<Pid> {
    #[allow(unsafe_code)]
    unsafe {
        let ctl = ioctl::ParameterizedReturnGetter::<{ NS_GET_TGID_IN_PIDNS }>::new(
            tgid.as_raw_pid() as usize,
        );
        ioctl::ioctl(fd, ctl).and_then(|tgid| Pid::from_raw(tgid).ok_or(Errno::INVAL))
    }
}
