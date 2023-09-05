use core::sync::atomic;
use core::{mem, ptr};

use crate::fd::{OwnedFd, RawFd};
use crate::io;

use crate::alluring::register::{execute, Probe};
use crate::alluring::sys;
use crate::alluring::types::{CancelBuilder, Timespec};
use crate::alluring::util::cast_ptr;
use crate::alluring::Parameters;

use crate::alluring::register::Restriction;

use crate::alluring::types;

/// Interface for submitting submission queue events in an io_uring instance to the kernel for
/// executing and registering files or buffers with the instance.
///
/// io_uring supports both directly performing I/O on buffers and file descriptors and registering
/// them beforehand. Registering is slow, but it makes performing the actual I/O much faster.
pub struct Submitter<'a> {
    fd: &'a OwnedFd,
    params: &'a Parameters,

    sq_head: *const atomic::AtomicU32,
    sq_tail: *const atomic::AtomicU32,
    sq_flags: *const atomic::AtomicU32,
}

impl<'a> Submitter<'a> {
    #[inline]
    pub(crate) const fn new(
        fd: &'a OwnedFd,
        params: &'a Parameters,
        sq_head: *const atomic::AtomicU32,
        sq_tail: *const atomic::AtomicU32,
        sq_flags: *const atomic::AtomicU32,
    ) -> Submitter<'a> {
        Submitter {
            fd,
            params,
            sq_head,
            sq_tail,
            sq_flags,
        }
    }

    #[inline]
    fn sq_len(&self) -> usize {
        unsafe {
            let head = (*self.sq_head).load(atomic::Ordering::Acquire);
            let tail = (*self.sq_tail).load(atomic::Ordering::Acquire);

            tail.wrapping_sub(head) as usize
        }
    }

    /// Whether the kernel thread has gone to sleep because it waited for too long without
    /// submission queue entries.
    #[inline]
    fn sq_need_wakeup(&self) -> bool {
        unsafe {
            (*self.sq_flags).load(atomic::Ordering::Acquire)
                & sys::IoringSqFlags::NEED_WAKEUP.bits()
                != 0
        }
    }

    /// CQ ring is overflown
    fn sq_cq_overflow(&self) -> bool {
        unsafe {
            (*self.sq_flags).load(atomic::Ordering::Acquire)
                & sys::IoringSqFlags::CQ_OVERFLOW.bits()
                != 0
        }
    }

    /// Initiate and/or complete asynchronous I/O. This is a low-level wrapper around
    /// `io_uring_enter` - see `man io_uring_enter` (or [its online
    /// version](https://manpages.debian.org/unstable/liburing-dev/io_uring_enter.2.en.html) for
    /// more details.
    ///
    /// You will probably want to use a more high-level API such as
    /// [`submit`](Self::submit) or [`submit_and_wait`](Self::submit_and_wait).
    ///
    /// # Safety
    ///
    /// This provides a raw interface so developer must ensure that parameters are correct.
    pub unsafe fn enter<T: Sized>(
        &self,
        to_submit: u32,
        min_complete: u32,
        flag: sys::IoringEnterFlags,
        arg: Option<&T>,
    ) -> io::Result<usize> {
        let arg = arg
            .map(|arg| cast_ptr(arg).cast())
            .unwrap_or_else(ptr::null);
        let size = core::mem::size_of::<T>();
        let result = sys::io_uring_enter(self.fd, to_submit, min_complete, flag, arg, size)?;
        Ok(result as _)
    }

    /// Submit all queued submission queue events to the kernel.
    #[inline]
    pub fn submit(&self) -> io::Result<usize> {
        self.submit_and_wait(0)
    }

    /// Submit all queued submission queue events to the kernel and wait for at least `want`
    /// completion events to complete.
    pub fn submit_and_wait(&self, want: usize) -> io::Result<usize> {
        let len = self.sq_len();
        let mut flags = sys::IoringEnterFlags::empty();

        // This logic suffers from the fact the sq_cq_overflow and sq_need_wakeup
        // each cause an atomic load of the same variable, self.sq_flags.
        // In the hottest paths, when a server is running with sqpoll,
        // this is going to be hit twice, when once would be sufficient.

        if want > 0 || self.params.is_setup_iopoll() || self.sq_cq_overflow() {
            flags |= sys::IoringEnterFlags::GETEVENTS;
        }

        if self.params.is_setup_sqpoll() {
            if self.sq_need_wakeup() {
                flags |= sys::IoringEnterFlags::SQ_WAKEUP;
            } else if want == 0 {
                // The kernel thread is polling and hasn't fallen asleep, so we don't need to tell
                // it to process events or wake it up
                return Ok(len);
            }
        }

        unsafe { self.enter::<sys::sigset_t>(len as _, want as _, flags, None) }
    }

    pub fn submit_with_args(
        &self,
        want: usize,
        args: &types::SubmitArgs<'_, '_>,
    ) -> io::Result<usize> {
        let len = self.sq_len();
        let mut flags = sys::IoringEnterFlags::EXT_ARG;

        if want > 0 || self.params.is_setup_iopoll() || self.sq_cq_overflow() {
            flags |= sys::IoringEnterFlags::GETEVENTS;
        }

        if self.params.is_setup_sqpoll() {
            if self.sq_need_wakeup() {
                flags |= sys::IoringEnterFlags::SQ_WAKEUP;
            } else if want == 0 {
                // The kernel thread is polling and hasn't fallen asleep, so we don't need to tell
                // it to process events or wake it up
                return Ok(len);
            }
        }

        unsafe { self.enter(len as _, want as _, flags, Some(&args.args)) }
    }

    /// Wait for the submission queue to have free entries.
    pub fn squeue_wait(&self) -> io::Result<usize> {
        unsafe { self.enter::<sys::sigset_t>(0, 0, sys::IoringEnterFlags::SQ_WAIT, None) }
    }

    /// Register in-memory fixed buffers for I/O with the kernel. You can use these buffers with the
    /// [`ReadFixed`](crate::opcode::ReadFixed) and [`WriteFixed`](crate::opcode::WriteFixed)
    /// operations.
    ///
    /// # Safety
    ///
    /// Developers must ensure that the `iov_base` and `iov_len` values are valid and will
    /// be valid until buffers are unregistered or the ring destroyed, otherwise undefined
    /// behaviour may occur.
    pub unsafe fn register_buffers(&self, bufs: &[sys::iovec]) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterBuffers,
            bufs.as_ptr().cast(),
            bufs.len() as _,
        )
    }

    /// Registers an empty file table of nr_files number of file descriptors. The sparse variant is
    /// available in kernels 5.19 and later.
    ///
    /// Registering a file table is a prerequisite for using any request that
    /// uses direct descriptors.
    pub fn register_files_sparse(&self, nr: u32) -> io::Result<()> {
        let rr = sys::io_uring_rsrc_register {
            nr,
            flags: sys::IoringRsrcFlags::REGISTER_SPARSE,
            resv2: 0,
            data: 0,
            tags: 0,
        };
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterFiles2,
            cast_ptr::<sys::io_uring_rsrc_register>(&rr).cast(),
            mem::size_of::<sys::io_uring_rsrc_register>() as _,
        )
    }

    /// Register files for I/O. You can use the registered files with
    /// [`Fixed`](crate::types::Fixed).
    ///
    /// Each fd may be -1, in which case it is considered "sparse", and can be filled in later with
    /// [`register_files_update`](Self::register_files_update).
    ///
    /// Note that this will wait for the ring to idle; it will only return once all active requests
    /// are complete. Use [`register_files_update`](Self::register_files_update) to avoid this.
    pub fn register_files(&self, fds: &[RawFd]) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterFiles,
            fds.as_ptr().cast(),
            fds.len() as _,
        )
    }

    /// This operation replaces existing files in the registered file set with new ones,
    /// either turning a sparse entry (one where fd is equal to -1) into a real one, removing an existing entry (new one is set to -1),
    /// or replacing an existing entry with a new existing entry. The `offset` parameter specifies
    /// the offset into the list of registered files at which to start updating files.
    ///
    /// You can also perform this asynchronously with the
    /// [`FilesUpdate`](crate::opcode::FilesUpdate) opcode.
    pub fn register_files_update(&self, offset: u32, fds: &[RawFd]) -> io::Result<()> {
        let fu = sys::io_uring_files_update {
            offset,
            resv: 0,
            fds: fds.as_ptr() as _,
        };
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterFilesUpdate,
            cast_ptr::<sys::io_uring_files_update>(&fu).cast(),
            fds.len() as _,
        )
    }

    /// Register an eventfd created by [`eventfd`](libc::eventfd) with the io_uring instance.
    pub fn register_eventfd(&self, eventfd: RawFd) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterEventfd,
            cast_ptr::<RawFd>(&eventfd).cast(),
            1,
        )
    }

    /// This works just like [`register_eventfd`](Self::register_eventfd), except notifications are
    /// only posted for events that complete in an async manner, so requests that complete
    /// immediately will not cause a notification.
    pub fn register_eventfd_async(&self, eventfd: RawFd) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterEventfdAsync,
            cast_ptr::<RawFd>(&eventfd).cast(),
            1,
        )
    }

    /// Fill in the given [`Probe`] with information about the opcodes supported by io_uring on the
    /// running kernel.
    ///
    /// # Examples
    ///
    // This is marked no_run as it is only available from Linux 5.6+, however the latest Ubuntu (on
    // which CI runs) only has Linux 5.4.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let io_uring = io_uring::IoUring::new(1)?;
    /// let mut probe = io_uring::Probe::new();
    /// io_uring.submitter().register_probe(&mut probe)?;
    ///
    /// if probe.is_supported(io_uring::opcode::Read::CODE) {
    ///     println!("Reading is supported!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn register_probe(&self, probe: &mut Probe) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterProbe,
            probe.as_mut_ptr() as *const _,
            Probe::COUNT as _,
        )
    }

    /// Register credentials of the running application with io_uring, and get an id associated with
    /// these credentials. This ID can then be [passed](crate::squeue::Entry::personality) into
    /// submission queue entries to issue the request with this process' credentials.
    ///
    /// By default, if [`Parameters::is_feature_cur_personality`] is set then requests will use the
    /// credentials of the task that called [`Submitter::enter`], otherwise they will use the
    /// credentials of the task that originally registered the io_uring.
    ///
    /// [`Parameters::is_feature_cur_personality`]: crate::Parameters::is_feature_cur_personality
    pub fn register_personality(&self) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterPersonality,
            ptr::null(),
            0,
        )?;
        Ok(())
    }

    /// Unregister all previously registered buffers.
    ///
    /// You do not need to explicitly call this before dropping the [`IoUring`](crate::IoUring), as
    /// it will be cleaned up by the kernel automatically.
    pub fn unregister_buffers(&self) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::UnregisterBuffers,
            ptr::null(),
            0,
        )
    }

    /// Unregister all previously registered files.
    ///
    /// You do not need to explicitly call this before dropping the [`IoUring`](crate::IoUring), as
    /// it will be cleaned up by the kernel automatically.
    pub fn unregister_files(&self) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::UnregisterFiles,
            ptr::null(),
            0,
        )
    }

    /// Unregister an eventfd file descriptor to stop notifications.
    pub fn unregister_eventfd(&self) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::UnregisterEventfd,
            ptr::null(),
            0,
        )
    }

    /// Unregister a previously registered personality.
    pub fn unregister_personality(&self, personality: u16) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::UnregisterPersonality,
            ptr::null(),
            personality as _,
        )
    }

    /// Permanently install a feature allowlist. Once this has been called, attempting to perform
    /// an operation not on the allowlist will fail with `-EACCES`.
    ///
    /// This can only be called once, to prevent untrusted code from removing restrictions.
    pub fn register_restrictions(&self, res: &mut [Restriction]) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterRestrictions,
            res.as_mut_ptr().cast(),
            res.len() as _,
        )
    }

    /// Enable the rings of the io_uring instance if they have been disabled with
    /// [`setup_r_disabled`](crate::Builder::setup_r_disabled).
    pub fn register_enable_rings(&self) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterEnableRings,
            ptr::null(),
            0,
        )
    }

    /// Get and/or set the limit for number of io_uring worker threads per NUMA
    /// node. `max[0]` holds the limit for bounded workers, which process I/O
    /// operations expected to be bound in time, that is I/O on regular files or
    /// block devices. While `max[1]` holds the limit for unbounded workers,
    /// which carry out I/O operations that can never complete, for instance I/O
    /// on sockets. Passing `0` does not change the current limit. Returns
    /// previous limits on success.
    pub fn register_iowq_max_workers(&self, max: &mut [u32; 2]) -> io::Result<()> {
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterIowqMaxWorkers,
            max.as_mut_ptr().cast(),
            max.len() as _,
        )
    }

    /// Register buffer ring for provided buffers.
    ///
    /// Details can be found in the io_uring_register_buf_ring.3 man page.
    ///
    /// If the register command is not supported, or the ring_entries value exceeds
    /// 32768, the InvalidInput error is returned.
    ///
    /// Available since 5.19.
    ///
    /// # Safety
    ///
    /// Developers must ensure that the `ring_addr` and its length represented by `ring_entries`
    /// are valid and will be valid until the bgid is unregistered or the ring destroyed,
    /// otherwise undefined behaviour may occur.
    pub unsafe fn register_buf_ring(
        &self,
        ring_addr: u64,
        ring_entries: u16,
        bgid: u16,
    ) -> io::Result<()> {
        // The interface type for ring_entries is u32 but the same interface only allows a u16 for
        // the tail to be specified, so to try and avoid further confusion, we limit the
        // ring_entries to u16 here too. The value is actually limited to 2^15 (32768) but we can
        // let the kernel enforce that.
        let arg = sys::io_uring_buf_reg {
            ring_addr,
            ring_entries: ring_entries as _,
            bgid,
            ..Default::default()
        };
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterPbufRing,
            cast_ptr::<sys::io_uring_buf_reg>(&arg).cast(),
            1,
        )
    }

    /// Unregister a previously registered buffer ring.
    ///
    /// Available since 5.19.
    pub fn unregister_buf_ring(&self, bgid: u16) -> io::Result<()> {
        let arg = sys::io_uring_buf_reg {
            ring_addr: 0,
            ring_entries: 0,
            bgid,
            ..Default::default()
        };
        execute(
            self.fd,
            sys::IoringRegisterOp::UnregisterPbufRing,
            cast_ptr::<sys::io_uring_buf_reg>(&arg).cast(),
            1,
        )
    }

    /// Performs a synchronous cancellation request, similar to [AsyncCancel](crate::opcode::AsyncCancel),
    /// except that it completes synchronously.
    ///
    /// Cancellation can target a specific request, or all requests matching some criteria. The
    /// [CancelBuilder](types::CancelBuilder) builder supports describing the match criteria for cancellation.
    ///
    /// An optional `timeout` can be provided to specify how long to wait for matched requests to be
    /// canceled. If no timeout is provided, the default is to wait indefinitely.
    ///
    /// ### Errors
    ///
    /// If no requests are matched, returns:
    ///
    /// [io::ErrorKind::NotFound]: `No such file or directory (os error 2)`
    ///
    /// If a timeout is supplied, and the timeout elapses prior to all requests being canceled, returns:
    ///
    /// [io::ErrorKind::Uncategorized]: `Timer expired (os error 62)`
    ///
    /// ### Notes
    ///
    /// Only requests which have been submitted to the ring will be considered for cancellation. Requests
    /// which have been written to the SQ, but not submitted, will not be canceled.
    ///
    /// Available since 6.0.
    pub fn register_sync_cancel(
        &self,
        timeout: Option<Timespec>,
        builder: CancelBuilder,
    ) -> io::Result<()> {
        let timespec = timeout.map(|ts| ts.0).unwrap_or(sys::Timespec {
            tv_sec: -1,
            tv_nsec: -1,
        });
        let user_data = builder.user_data.unwrap_or(0);
        let flags = sys::IoringAsyncCancelFlags::from_bits_retain(builder.flags.bits());
        let fd = builder.to_fd();

        let arg = sys::io_uring_sync_cancel_reg {
            addr: user_data,
            fd,
            flags,
            timeout: timespec,
            pad: [0u64; 4],
        };
        execute(
            self.fd,
            sys::IoringRegisterOp::RegisterSyncCancel,
            cast_ptr::<sys::io_uring_sync_cancel_reg>(&arg).cast(),
            1,
        )
    }
}
