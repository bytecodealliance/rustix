//! Linux [io_uring] APIs.
//!
//! # Safety
//!
//! io_uring operates on raw pointers and raw file descriptors. Rustix does not
//! attempt to provide a safe API for these, because the abstraction level is
//! too low for this to be practical. Safety should be introduced in
//! higher-level abstraction layers.
//!
//! # References
//!  - [Linux]
//!
//! [Linux]: https://man.archlinux.org/man/io_uring.7.en
//! [io_uring]: https://en.wikipedia.org/wiki/Io_uring
#![allow(unsafe_code)]

use crate::fd::{AsFd, BorrowedFd, RawFd};
use crate::imp;
use crate::io::{self, OwnedFd};
use core::ffi::c_void;
use linux_raw_sys::general as sys;

/// `io_uring_setup(entries, params)`—Setup a context for performing
/// asynchronous I/O.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man.archlinux.org/man/io_uring_setup.2.en
#[inline]
pub fn io_uring_setup(entries: u32, params: &mut io_uring_params) -> io::Result<OwnedFd> {
    imp::io_uring::syscalls::io_uring_setup(entries, params)
}

/// `io_uring_register(fd, opcode, arg, nr_args)`—Register files or user
/// buffers for asynchronous I/O.
///
/// # Safety
///
/// io_uring operates on raw pointers and raw file descriptors. Users are
/// responsible for ensuring that memory and resources are only accessed in
/// valid ways.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man.archlinux.org/man/io_uring_register.2.en
#[inline]
pub unsafe fn io_uring_register<Fd: AsFd>(
    fd: Fd,
    opcode: IoringRegisterOp,
    arg: *const c_void,
    nr_args: u32,
) -> io::Result<()> {
    imp::io_uring::syscalls::io_uring_register(fd.as_fd(), opcode, arg, nr_args)
}

/// `io_uring_enter(fd, to_submit, min_complete, flags, arg, size)`—Initiate
/// and/or complete asynchronous I/O.
///
/// # Safety
///
/// io_uring operates on raw pointers and raw file descriptors. Users are
/// responsible for ensuring that memory and resources are only accessed in
/// valid ways.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man.archlinux.org/man/io_uring_enter.2.en
#[inline]
pub unsafe fn io_uring_enter<Fd: AsFd>(
    fd: Fd,
    to_submit: u32,
    min_complete: u32,
    flags: IoringEnterFlags,
    arg: *const c_void,
    size: usize,
) -> io::Result<u32> {
    imp::io_uring::syscalls::io_uring_enter(fd.as_fd(), to_submit, min_complete, flags, arg, size)
}

bitflags::bitflags! {
    /// `IORING_ENTER_*` flags for use with [`io_uring_enter`].
    #[derive(Default)]
    pub struct IoringEnterFlags: u32 {
        /// `IORING_ENTER_GETEVENTS`
        const GETEVENTS = sys::IORING_ENTER_GETEVENTS;

        /// `IORING_ENTER_SQ_WAKEUP`
        const SQ_WAKEUP = sys::IORING_ENTER_SQ_WAKEUP;

        /// `IORING_ENTER_SQ_WAIT`
        const SQ_WAIT = sys::IORING_ENTER_SQ_WAIT;

        /// `IORING_ENTER_EXT_ARG`
        const EXT_ARG = sys::IORING_ENTER_EXT_ARG;
    }
}

/// `IORING_REGISTER_*` and `IORING_UNREGISTER_*` constants for use with
/// [`io_uring_register`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum IoringRegisterOp {
    /// `IORING_REGISTER_BUFFERS`
    RegisterBuffers = sys::IORING_REGISTER_BUFFERS as _,

    /// `IORING_UNREGISTER_BUFFERS`
    UnregisterBuffers = sys::IORING_UNREGISTER_BUFFERS as _,

    /// `IORING_REGISTER_FILES`
    RegisterFiles = sys::IORING_REGISTER_FILES as _,

    /// `IORING_UNREGISTER_FILES`
    UnregisterFiles = sys::IORING_UNREGISTER_FILES as _,

    /// `IORING_REGISTER_EVENTFD`
    RegisterEventfd = sys::IORING_REGISTER_EVENTFD as _,

    /// `IORING_UNREGISTER_EVENTFD`
    UnregisterEventfd = sys::IORING_UNREGISTER_EVENTFD as _,

    /// `IORING_REGISTER_FILES_UPDATE`
    RegisterFilesUpdate = sys::IORING_REGISTER_FILES_UPDATE as _,

    /// `IORING_REGISTER_EVENTFD_ASYNC`
    RegisterEventfdAsync = sys::IORING_REGISTER_EVENTFD_ASYNC as _,

    /// `IORING_REGISTER_PROBE`
    RegisterProbe = sys::IORING_REGISTER_PROBE as _,

    /// `IORING_REGISTER_PERSONALITY`
    RegisterPersonality = sys::IORING_REGISTER_PERSONALITY as _,

    /// `IORING_UNREGISTER_PERSONALITY`
    UnregisterPersonality = sys::IORING_UNREGISTER_PERSONALITY as _,

    /// `IORING_REGISTER_RESTRICTIONS`
    RegisterRestrictions = sys::IORING_REGISTER_RESTRICTIONS as _,

    /// `IORING_REGISTER_ENABLE_RINGS`
    RegisterEnableRings = sys::IORING_REGISTER_ENABLE_RINGS as _,

    /// `IORING_REGISTER_BUFFERS2`
    RegisterBuffers2 = sys::IORING_REGISTER_BUFFERS2 as _,

    /// `IORING_REGISTER_BUFFERS_UPDATE`
    RegisterBuffersUpdate = sys::IORING_REGISTER_BUFFERS_UPDATE as _,

    /// `IORING_REGISTER_FILES2`
    RegisterFiles2 = sys::IORING_REGISTER_FILES2 as _,

    /// `IORING_REGISTER_FILES_SKIP`
    RegisterFilesSkip = sys::IORING_REGISTER_FILES_SKIP as _,

    /// `IORING_REGISTER_FILES_UPDATE2`
    RegisterFilesUpdate2 = sys::IORING_REGISTER_FILES_UPDATE2 as _,

    /// `IORING_REGISTER_IOWQ_AFF`
    RegisterIowqAff = sys::IORING_REGISTER_IOWQ_AFF as _,

    /// `IORING_UNREGISTER_IOWQ_AFF`
    UnregisterIowqAff = sys::IORING_UNREGISTER_IOWQ_AFF as _,

    /// `IORING_REGISTER_IOWQ_MAX_WORKERS`
    RegisterIowqMaxWorkers = sys::IORING_REGISTER_IOWQ_MAX_WORKERS as _,
}

/// `IORING_OP_*` constants for use with [`io_uring_sqe`].
#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum IoringOp {
    /// `IORING_OP_NOP`
    Nop = sys::IORING_OP_NOP as _,

    /// `IORING_OP_ACCEPT`
    Accept = sys::IORING_OP_ACCEPT as _,

    /// `IORING_OP_ASYNC_CANCEL`
    AsyncCancel = sys::IORING_OP_ASYNC_CANCEL as _,

    /// `IORING_OP_CLOSE`
    Close = sys::IORING_OP_CLOSE as _,

    /// `IORING_OP_CONNECT`
    Connect = sys::IORING_OP_CONNECT as _,

    /// `IORING_OP_EPOLL_CTL`
    EpollCtl = sys::IORING_OP_EPOLL_CTL as _,

    /// `IORING_OP_FADVISE`
    Fadvise = sys::IORING_OP_FADVISE as _,

    /// `IORING_OP_FALLOCATE`
    Fallocate = sys::IORING_OP_FALLOCATE as _,

    /// `IORING_OP_FILES_UPDATE`
    FilesUpdate = sys::IORING_OP_FILES_UPDATE as _,

    /// `IORING_OP_FSYNC`
    Fsync = sys::IORING_OP_FSYNC as _,

    /// `IORING_OP_LINKAT`
    Linkat = sys::IORING_OP_LINKAT as _,

    /// `IORING_OP_LINK_TIMEOUT`
    LinkTimeout = sys::IORING_OP_LINK_TIMEOUT as _,

    /// `IORING_OP_MADVISE`
    Madvise = sys::IORING_OP_MADVISE as _,

    /// `IORING_OP_MKDIRAT`
    Mkdirat = sys::IORING_OP_MKDIRAT as _,

    /// `IORING_OP_OPENAT`
    Openat = sys::IORING_OP_OPENAT as _,

    /// `IORING_OP_OPENAT2`
    Openat2 = sys::IORING_OP_OPENAT2 as _,

    /// `IORING_OP_POLL_ADD`
    PollAdd = sys::IORING_OP_POLL_ADD as _,

    /// `IORING_OP_POLL_REMOVE`
    PollRemove = sys::IORING_OP_POLL_REMOVE as _,

    /// `IORING_OP_PROVIDE_BUFFERS`
    ProvideBuffers = sys::IORING_OP_PROVIDE_BUFFERS as _,

    /// `IORING_OP_READ`
    Read = sys::IORING_OP_READ as _,

    /// `IORING_OP_READV`
    Readv = sys::IORING_OP_READV as _,

    /// `IORING_OP_READ_FIXED`
    ReadFixed = sys::IORING_OP_READ_FIXED as _,

    /// `IORING_OP_RECV`
    Recv = sys::IORING_OP_RECV as _,

    /// `IORING_OP_RECVMSG`
    Recvmsg = sys::IORING_OP_RECVMSG as _,

    /// `IORING_OP_REMOVE_BUFFERS`
    RemoveBuffers = sys::IORING_OP_REMOVE_BUFFERS as _,

    /// `IORING_OP_RENAMEAT`
    Renameat = sys::IORING_OP_RENAMEAT as _,

    /// `IORING_OP_SEND`
    Send = sys::IORING_OP_SEND as _,

    /// `IORING_OP_SENDMSG`
    Sendmsg = sys::IORING_OP_SENDMSG as _,

    /// `IORING_OP_SHUTDOWN`
    Shutdown = sys::IORING_OP_SHUTDOWN as _,

    /// `IORING_OP_SPLICE`
    Splice = sys::IORING_OP_SPLICE as _,

    /// `IORING_OP_STATX`
    Statx = sys::IORING_OP_STATX as _,

    /// `IORING_OP_SYMLINKAT`
    Symlinkat = sys::IORING_OP_SYMLINKAT as _,

    /// `IORING_OP_SYNC_FILE_RANGE`
    SyncFileRange = sys::IORING_OP_SYNC_FILE_RANGE as _,

    /// `IORING_OP_TEE`
    Tee = sys::IORING_OP_TEE as _,

    /// `IORING_OP_TIMEOUT`
    Timeout = sys::IORING_OP_TIMEOUT as _,

    /// `IORING_OP_TIMEOUT_REMOVE`
    TimeoutRemove = sys::IORING_OP_TIMEOUT_REMOVE as _,

    /// `IORING_OP_UNLINKAT`
    Unlinkat = sys::IORING_OP_UNLINKAT as _,

    /// `IORING_OP_WRITE`
    Write = sys::IORING_OP_WRITE as _,

    /// `IORING_OP_WRITEV`
    Writev = sys::IORING_OP_WRITEV as _,

    /// `IORING_OP_WRITE_FIXED`
    WriteFixed = sys::IORING_OP_WRITE_FIXED as _,
}

impl Default for IoringOp {
    #[inline]
    fn default() -> Self {
        Self::Nop
    }
}

bitflags::bitflags! {
    /// `IORING_SETUP_*` flags for use with [`io_uring_params`].
    #[derive(Default)]
    pub struct IoringSetupFlags: u32 {
        /// `IORING_SETUP_ATTACH_WQ`
        const ATTACH_WQ = sys::IORING_SETUP_ATTACH_WQ;

        /// `IORING_SETUP_CLAMP`
        const CLAMP = sys::IORING_SETUP_CLAMP;

        /// `IORING_SETUP_CQSIZE`
        const CQSIZE = sys::IORING_SETUP_CQSIZE;

        /// `IORING_SETUP_IOPOLL`
        const IOPOLL = sys::IORING_SETUP_IOPOLL;

        /// `IORING_SETUP_R_DISABLED`
        const R_DISABLED = sys::IORING_SETUP_R_DISABLED;

        /// `IORING_SETUP_SQPOLL`
        const SQPOLL = sys::IORING_SETUP_SQPOLL;

        /// `IORING_SETUP_SQ_AFF`
        const SQ_AFF = sys::IORING_SETUP_SQ_AFF;
    }
}

bitflags::bitflags! {
    /// `IOSQE_*` flags for use with [`io_uring_sqe`].
    #[derive(Default)]
    pub struct IoringSqeFlags: u8 {
        /// `1 << IOSQE_ASYNC_BIT`
        const ASYNC = 1 << sys::IOSQE_ASYNC_BIT as u8;

        /// `1 << IOSQE_BUFFER_SELECT_BIT`
        const BUFFER_SELECT = 1 << sys::IOSQE_BUFFER_SELECT_BIT as u8;

        /// `1 << IOSQE_FIXED_FILE_BIT`
        const FIXED_FILE = 1 << sys::IOSQE_FIXED_FILE_BIT as u8;

        /// 1 << `IOSQE_IO_DRAIN_BIT`
        const IO_DRAIN = 1 << sys::IOSQE_IO_DRAIN_BIT as u8;

        /// `1 << IOSQE_IO_HARDLINK_BIT`
        const IO_HARDLINK = 1 << sys::IOSQE_IO_HARDLINK_BIT as u8;

        /// `1 << IOSQE_IO_LINK_BIT`
        const IO_LINK = 1 << sys::IOSQE_IO_LINK_BIT as u8;
    }
}

bitflags::bitflags! {
    /// `IORING_CQE_F_*` flags for use with [`io_uring_cqe`].
    #[derive(Default)]
    pub struct IoringCqeFlags: u32 {
        /// `IORING_CQE_F_BUFFER`
        const BUFFER = sys::IORING_CQE_F_BUFFER as _;

        /// `IORING_CQE_F_MORE`
        const MORE = sys::IORING_CQE_F_MORE as _;
    }
}

bitflags::bitflags! {
    /// `IORING_FSYNC_*` flags for use with [`io_uring_sqe`].
    #[derive(Default)]
    pub struct IoringFsyncFlags: u32 {
        /// `IORING_FSYNC_DATASYNC`
        const DATASYNC = sys::IORING_FSYNC_DATASYNC;
    }
}

bitflags::bitflags! {
    /// `IORING_TIMEOUT_*` flags for use with [`io_uring_sqe`].
    #[derive(Default)]
    pub struct IoringTimeoutFlags: u32 {
        /// `IORING_TIMEOUT_ABS`
        const ABS = sys::IORING_TIMEOUT_ABS;
        /// `IORING_TIMEOUT_UPDATE`
        const UPDATE = sys::IORING_TIMEOUT_UPDATE;
    }
}

bitflags::bitflags! {
    /// `SPLICE_F_*` flags for use with [`io_uring_seq`].
    #[derive(Default)]
    pub struct SpliceFlags: u32 {
        /// `SPLICE_F_FD_IN_FIXED`
        const FD_IN_FIXED = sys::SPLICE_F_FD_IN_FIXED;
    }
}

bitflags::bitflags! {
    /// `IORING_FEAT_*` flags for use with [`io_uring_params`].
    #[derive(Default)]
    pub struct IoringFeatureFlags: u32 {
        /// `IORING_FEAT_CQE_SKIP`
        const CQE_SKIP = sys::IORING_FEAT_CQE_SKIP;

        /// `IORING_FEAT_CUR_PERSONALITY`
        const CUR_PERSONALITY = sys::IORING_FEAT_CUR_PERSONALITY;

        /// `IORING_FEAT_EXT_ARG`
        const EXT_ARG = sys::IORING_FEAT_EXT_ARG;

        /// `IORING_FEAT_FAST_POLL`
        const FAST_POLL = sys::IORING_FEAT_FAST_POLL;

        /// `IORING_FEAT_NATIVE_WORKERS`
        const NATIVE_WORKERS = sys::IORING_FEAT_NATIVE_WORKERS;

        /// `IORING_FEAT_NODROP`
        const NODROP = sys::IORING_FEAT_NODROP;

        /// `IORING_FEAT_POLL_32BITS`
        const POLL_32BITS = sys::IORING_FEAT_POLL_32BITS;

        /// `IORING_FEAT_RSRC_TAGS`
        const RSRC_TAGS = sys::IORING_FEAT_RSRC_TAGS;

        /// `IORING_FEAT_RW_CUR_POS`
        const RW_CUR_POS = sys::IORING_FEAT_RW_CUR_POS;

        /// `IORING_FEAT_SINGLE_MMAP`
        const SINGLE_MMAP = sys::IORING_FEAT_SINGLE_MMAP;

        /// `IORING_FEAT_SQPOLL_NONFIXED`
        const SQPOLL_NONFIXED = sys::IORING_FEAT_SQPOLL_NONFIXED;

        /// `IORING_FEAT_SUBMIT_STABLE`
        const SUBMIT_STABLE = sys::IORING_FEAT_SUBMIT_STABLE;
    }
}

// Structs and constants that have not yet been converted into manual
// declarations.
pub use sys::{
    io_uring_files_update, io_uring_getevents_arg, io_uring_probe, io_uring_probe_op,
    io_uring_rsrc_register, io_uring_rsrc_update, io_uring_rsrc_update2, iovec, open_how,
    IORING_CQE_BUFFER_SHIFT, IORING_CQ_EVENTFD_DISABLED, IORING_LINK_TIMEOUT_UPDATE,
    IORING_OFF_CQ_RING, IORING_OFF_SQES, IORING_OFF_SQ_RING, IORING_POLL_ADD_MULTI,
    IORING_POLL_UPDATE_EVENTS, IORING_POLL_UPDATE_USER_DATA, IORING_RESTRICTION_REGISTER_OP,
    IORING_RESTRICTION_SQE_FLAGS_ALLOWED, IORING_RESTRICTION_SQE_FLAGS_REQUIRED,
    IORING_RESTRICTION_SQE_OP, IORING_SQ_CQ_OVERFLOW, IORING_SQ_NEED_WAKEUP,
    IORING_TIMEOUT_BOOTTIME, IORING_TIMEOUT_CLOCK_MASK, IORING_TIMEOUT_ETIME_SUCCESS,
    IORING_TIMEOUT_REALTIME, IORING_TIMEOUT_UPDATE_MASK, IO_URING_OP_SUPPORTED,
};

/// `IORING_REGISTER_FILES_SKIP`
/// TODO: Make this a const fn. It needs borrow_raw to be a const fn.
#[inline]
#[doc(alias = "IORING_REGISTER_FILES_SKIP")]
pub fn io_uring_register_files_skip() -> BorrowedFd<'static> {
    let files_skip = sys::IORING_REGISTER_FILES_SKIP as RawFd;

    // # Safety
    //
    // `IORING_REGISTER_FILES_SKIP` is a reserved value that is never
    // dynamically allocated, so it'll remain valid for the duration of
    // `'static`.
    #[allow(unsafe_code)]
    unsafe {
        BorrowedFd::<'static>::borrow_raw(files_skip)
    }
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct io_uring_sqe {
    pub opcode: IoringOp,
    pub flags: IoringSqeFlags,
    pub ioprio: u16,
    pub fd: RawFd,
    pub off_or_addr2: off_or_addr2_union,
    pub addr_or_splice_off_in: addr_or_splice_off_in_union,
    pub len: u32,
    pub op_flags: op_flags_union,
    pub user_data: u64,
    pub buf: buf_union,
    pub personality: u16,
    pub splice_fd_in_or_file_index: splice_fd_in_or_file_index_union,
    pub __pad2: [u64; 2usize],
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union off_or_addr2_union {
    pub off: u64,
    pub addr2: u64,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union addr_or_splice_off_in_union {
    pub addr: u64,
    pub splice_off_in: u64,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union op_flags_union {
    pub rw_flags: crate::io::ReadWriteFlags,
    pub fsync_flags: IoringFsyncFlags,
    pub poll_events: u16,
    pub poll32_events: u32,
    pub sync_range_flags: u32,
    /// `msg_flags` is split into `send_flags` and `recv_flags`.
    #[doc(alias = "msg_flags")]
    pub send_flags: crate::net::SendFlags,
    /// `msg_flags` is split into `send_flags` and `recv_flags`.
    #[doc(alias = "msg_flags")]
    pub recv_flags: crate::net::RecvFlags,
    pub timeout_flags: IoringTimeoutFlags,
    pub accept_flags: crate::net::AcceptFlags,
    pub cancel_flags: u32,
    pub open_flags: crate::fs::AtFlags,
    pub statx_flags: crate::fs::AtFlags,
    pub fadvise_advice: crate::fs::Advice,
    pub splice_flags: SpliceFlags,
    pub rename_flags: crate::fs::RenameFlags,
    pub unlink_flags: crate::fs::AtFlags,
    pub hardlink_flags: crate::fs::AtFlags,
}

#[allow(missing_docs)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union buf_union {
    pub buf_index: u16,
    pub buf_group: u16,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union splice_fd_in_or_file_index_union {
    pub splice_fd_in: i32,
    pub file_index: u32,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: IoringCqeFlags,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct io_uring_restriction {
    pub opcode: u16,
    pub register_or_sqe_op_or_sqe_flags: register_or_sqe_op_or_sqe_flags_union,
    pub resv: u8,
    pub resv2: [u32; 3usize],
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union register_or_sqe_op_or_sqe_flags_union {
    pub register_op: IoringRegisterOp,
    pub sqe_op: IoringOp,
    pub sqe_flags: IoringSqeFlags,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct io_uring_params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: IoringSetupFlags,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub features: IoringFeatureFlags,
    pub wq_fd: u32,
    pub resv: [u32; 3usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct io_sqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct io_cqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub flags: u32,
    pub resv1: u32,
    pub resv2: u64,
}

impl Default for off_or_addr2_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for addr_or_splice_off_in_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for op_flags_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for buf_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for splice_fd_in_or_file_index_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Default for register_or_sqe_op_or_sqe_flags_union {
    #[inline]
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

/// Check that our custom structs and unions have the same layout as the
/// kernel's versions.
#[test]
fn io_uring_layouts() {
    use core::mem::{align_of, size_of};
    use memoffset::{offset_of, span_of};

    // Check that the size and alignment of a type match the `sys` bindings.
    macro_rules! check_type {
        ($struct:ident) => {
            assert_eq!(
                (size_of::<$struct>(), align_of::<$struct>()),
                (size_of::<sys::$struct>(), align_of::<sys::$struct>())
            );
        };
    }

    // The same as `check_type`, but for unions we've renamed to avoid having
    // types like "bindgen_ty_1" in the API.
    macro_rules! check_renamed_union {
        ($to:ident, $from:ident) => {
            assert_eq!(
                (size_of::<$to>(), align_of::<$to>()),
                (size_of::<sys::$from>(), align_of::<sys::$from>())
            );
        };
    }

    // Check that the field of a struct has the same offset as the
    // corresponding field in the `sys` bindings.
    macro_rules! check_struct_field {
        ($struct:ident, $field:ident) => {
            assert_eq!(
                offset_of!($struct, $field),
                offset_of!(sys::$struct, $field)
            );
            assert_eq!(span_of!($struct, $field), span_of!(sys::$struct, $field));
        };
    }

    // The same as `check_struct_field`, but for unions we've renamed to avoid
    // having types like "bindgen_ty_1" in the API.
    macro_rules! check_struct_renamed_union_field {
        ($struct:ident, $to:ident, $from:ident) => {
            assert_eq!(offset_of!($struct, $to), offset_of!(sys::$struct, $from));
            assert_eq!(span_of!($struct, $to), span_of!(sys::$struct, $from));
        };
    }

    check_renamed_union!(off_or_addr2_union, io_uring_sqe__bindgen_ty_1);
    check_renamed_union!(addr_or_splice_off_in_union, io_uring_sqe__bindgen_ty_2);
    check_renamed_union!(op_flags_union, io_uring_sqe__bindgen_ty_3);
    check_renamed_union!(buf_union, io_uring_sqe__bindgen_ty_4);
    check_renamed_union!(splice_fd_in_or_file_index_union, io_uring_sqe__bindgen_ty_5);
    check_renamed_union!(
        register_or_sqe_op_or_sqe_flags_union,
        io_uring_restriction__bindgen_ty_1
    );
    check_type!(io_uring_sqe);
    check_type!(io_uring_cqe);
    check_type!(io_uring_restriction);
    check_type!(io_uring_params);
    check_type!(io_sqring_offsets);
    check_type!(io_cqring_offsets);

    check_struct_field!(io_uring_sqe, opcode);
    check_struct_field!(io_uring_sqe, opcode);
    check_struct_field!(io_uring_sqe, flags);
    check_struct_field!(io_uring_sqe, ioprio);
    check_struct_field!(io_uring_sqe, fd);
    check_struct_renamed_union_field!(io_uring_sqe, off_or_addr2, __bindgen_anon_1);
    check_struct_renamed_union_field!(io_uring_sqe, addr_or_splice_off_in, __bindgen_anon_2);
    check_struct_field!(io_uring_sqe, len);
    check_struct_renamed_union_field!(io_uring_sqe, op_flags, __bindgen_anon_3);
    check_struct_field!(io_uring_sqe, user_data);
    check_struct_renamed_union_field!(io_uring_sqe, buf, __bindgen_anon_4);
    check_struct_field!(io_uring_sqe, personality);
    check_struct_renamed_union_field!(io_uring_sqe, splice_fd_in_or_file_index, __bindgen_anon_5);
    check_struct_field!(io_uring_sqe, __pad2);

    check_struct_field!(io_uring_cqe, user_data);
    check_struct_field!(io_uring_cqe, res);
    check_struct_field!(io_uring_cqe, flags);

    check_struct_field!(io_uring_restriction, opcode);
    check_struct_renamed_union_field!(
        io_uring_restriction,
        register_or_sqe_op_or_sqe_flags,
        __bindgen_anon_1
    );
    check_struct_field!(io_uring_restriction, resv);
    check_struct_field!(io_uring_restriction, resv2);

    check_struct_field!(io_uring_params, sq_entries);
    check_struct_field!(io_uring_params, cq_entries);
    check_struct_field!(io_uring_params, flags);
    check_struct_field!(io_uring_params, sq_thread_cpu);
    check_struct_field!(io_uring_params, sq_thread_idle);
    check_struct_field!(io_uring_params, features);
    check_struct_field!(io_uring_params, wq_fd);
    check_struct_field!(io_uring_params, resv);
    check_struct_field!(io_uring_params, sq_off);
    check_struct_field!(io_uring_params, cq_off);

    check_struct_field!(io_sqring_offsets, head);
    check_struct_field!(io_sqring_offsets, tail);
    check_struct_field!(io_sqring_offsets, ring_mask);
    check_struct_field!(io_sqring_offsets, ring_entries);
    check_struct_field!(io_sqring_offsets, flags);
    check_struct_field!(io_sqring_offsets, dropped);
    check_struct_field!(io_sqring_offsets, array);
    check_struct_field!(io_sqring_offsets, resv1);
    check_struct_field!(io_sqring_offsets, resv2);

    check_struct_field!(io_cqring_offsets, head);
    check_struct_field!(io_cqring_offsets, tail);
    check_struct_field!(io_cqring_offsets, ring_mask);
    check_struct_field!(io_cqring_offsets, ring_entries);
    check_struct_field!(io_cqring_offsets, overflow);
    check_struct_field!(io_cqring_offsets, cqes);
    check_struct_field!(io_cqring_offsets, flags);
    check_struct_field!(io_cqring_offsets, resv1);
    check_struct_field!(io_cqring_offsets, resv2);
}
