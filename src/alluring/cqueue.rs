//! Completion Queue

use core::fmt::{self, Debug};
use core::mem;
use core::mem::MaybeUninit;
use core::sync::atomic;

use crate::alluring::sys;
use crate::alluring::util::{private, unsync_load, Mmap};

use bitflags::bitflags;

pub(crate) struct Inner<E: EntryMarker> {
    head: *const atomic::AtomicU32,
    tail: *const atomic::AtomicU32,
    ring_mask: u32,
    ring_entries: u32,

    overflow: *const atomic::AtomicU32,

    cqes: *const E,

    flags: *const atomic::AtomicU32,
}

/// An io_uring instance's completion queue. This stores all the I/O operations that have completed.
pub struct CompletionQueue<'a, E: EntryMarker = Entry> {
    head: u32,
    tail: u32,
    queue: &'a Inner<E>,
}

/// A completion queue entry (CQE), representing a complete I/O operation.
///
/// This is implemented for [`Entry`] and [`Entry32`].
pub trait EntryMarker: Clone + Debug + Into<Entry> + private::Sealed {
    const BUILD_FLAGS: sys::IoringSetupFlags;
}

/// A 16-byte completion queue entry (CQE), representing a complete I/O operation.
#[repr(C)]
pub struct Entry(pub(crate) sys::io_uring_cqe);

/// A 32-byte completion queue entry (CQE), representing a complete I/O operation.
#[repr(C)]
#[derive(Clone)]
pub struct Entry32(pub(crate) Entry, pub(crate) [u64; 2]);

#[test]
fn test_entry_sizes() {
    assert_eq!(mem::size_of::<Entry>(), 16);
    assert_eq!(mem::size_of::<Entry32>(), 32);
}

bitflags! {
    /// Completion flags
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Flags: u32 {
        const BUFFER = sys::IoringCqeFlags::BUFFER.bits();

        const MORE = sys::IoringCqeFlags::MORE.bits();

        const SOCK_NONEMPTY = sys::IoringCqeFlags::SOCK_NONEMPTY.bits();

        const NOTIF = sys::IoringCqeFlags::NOTIF.bits();
    }
}

impl<E: EntryMarker> Inner<E> {
    #[rustfmt::skip]
    pub(crate) unsafe fn new(cq_mmap: &Mmap, p: &sys::io_uring_params) -> Self {
        let head         = cq_mmap.offset(p.cq_off.head         ) as *const atomic::AtomicU32;
        let tail         = cq_mmap.offset(p.cq_off.tail         ) as *const atomic::AtomicU32;
        let ring_mask    = cq_mmap.offset(p.cq_off.ring_mask    ).cast::<u32>().read();
        let ring_entries = cq_mmap.offset(p.cq_off.ring_entries ).cast::<u32>().read();
        let overflow     = cq_mmap.offset(p.cq_off.overflow     ) as *const atomic::AtomicU32;
        let cqes         = cq_mmap.offset(p.cq_off.cqes         ) as *const E;
        let flags        = cq_mmap.offset(p.cq_off.flags        ) as *const atomic::AtomicU32;

        Self {
            head,
            tail,
            ring_mask,
            ring_entries,
            overflow,
            cqes,
            flags,
        }
    }

    #[inline]
    pub(crate) unsafe fn borrow_shared(&self) -> CompletionQueue<'_, E> {
        CompletionQueue {
            head: unsync_load(self.head),
            tail: (*self.tail).load(atomic::Ordering::Acquire),
            queue: self,
        }
    }

    #[inline]
    pub(crate) fn borrow(&mut self) -> CompletionQueue<'_, E> {
        unsafe { self.borrow_shared() }
    }
}

impl<E: EntryMarker> CompletionQueue<'_, E> {
    /// Synchronize this type with the real completion queue.
    ///
    /// This will flush any entries consumed in this iterator and will make available new entries
    /// in the queue if the kernel has produced some entries in the meantime.
    #[inline]
    pub fn sync(&mut self) {
        unsafe {
            (*self.queue.head).store(self.head, atomic::Ordering::Release);
            self.tail = (*self.queue.tail).load(atomic::Ordering::Acquire);
        }
    }

    /// If queue is full and [`is_feature_nodrop`](crate::Parameters::is_feature_nodrop) is not set,
    /// new events may be dropped. This records the number of dropped events.
    pub fn overflow(&self) -> u32 {
        unsafe { (*self.queue.overflow).load(atomic::Ordering::Acquire) }
    }

    /// Whether eventfd notifications are disabled when a request is completed and queued to the CQ
    /// ring. This library currently does not provide a way to set it, so this will always be
    /// `false`.
    pub fn eventfd_disabled(&self) -> bool {
        unsafe {
            sys::IoringCqFlags::from_bits_retain(
                (*self.queue.flags).load(atomic::Ordering::Acquire),
            )
            .contains(sys::IoringCqFlags::EVENTFD_DISABLED)
        }
    }

    /// Get the total number of entries in the completion queue ring buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.queue.ring_entries as usize
    }

    /// Returns `true` if there are no completion queue events to be processed.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the completion queue is at maximum capacity. If
    /// [`is_feature_nodrop`](crate::Parameters::is_feature_nodrop) is not set, this will cause any
    /// new completion queue events to be dropped by the kernel.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    #[inline]
    pub fn fill<'a>(&mut self, entries: &'a mut [MaybeUninit<E>]) -> &'a mut [E] {
        let len = core::cmp::min(self.len(), entries.len());

        for entry in &mut entries[..len] {
            *entry = MaybeUninit::new(unsafe { self.pop() });
        }

        unsafe { core::slice::from_raw_parts_mut(entries as *mut _ as *mut E, len) }
    }

    #[inline]
    unsafe fn pop(&mut self) -> E {
        let entry = &*self
            .queue
            .cqes
            .add((self.head & self.queue.ring_mask) as usize);
        self.head = self.head.wrapping_add(1);
        entry.clone()
    }
}

impl<E: EntryMarker> Drop for CompletionQueue<'_, E> {
    #[inline]
    fn drop(&mut self) {
        unsafe { &*self.queue.head }.store(self.head, atomic::Ordering::Release);
    }
}

impl<E: EntryMarker> Iterator for CompletionQueue<'_, E> {
    type Item = E;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.head != self.tail {
            Some(unsafe { self.pop() })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<E: EntryMarker> ExactSizeIterator for CompletionQueue<'_, E> {
    #[inline]
    fn len(&self) -> usize {
        self.tail.wrapping_sub(self.head) as usize
    }
}

impl Entry {
    /// The operation-specific result code. For example, for a [`Read`](crate::opcode::Read)
    /// operation this is equivalent to the return value of the `read(2)` system call.
    #[inline]
    pub fn result(&self) -> i32 {
        self.0.res
    }

    /// The user data of the request, as set by
    /// [`Entry::user_data`](crate::squeue::Entry::user_data) on the submission queue event.
    #[inline]
    pub fn user_data(&self) -> u64 {
        self.0.user_data.u64_()
    }

    /// The user data of the request, as set by
    /// [`Entry::user_data`](crate::squeue::Entry::user_data) on the submission queue event.
    #[inline]
    pub fn user_data_ptr(&self) -> *mut core::ffi::c_void {
        self.0.user_data.ptr()
    }

    /// Metadata related to the operation.
    ///
    /// This is currently used for:
    /// - Storing the selected buffer ID, if one was selected. See
    /// [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info.
    #[inline]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_retain(self.0.flags.bits())
    }
}

impl private::Sealed for Entry {}

impl EntryMarker for Entry {
    const BUILD_FLAGS: sys::IoringSetupFlags = sys::IoringSetupFlags::empty();
}

impl Clone for Entry {
    fn clone(&self) -> Entry {
        // io_uring_cqe doesn't implement Clone due to the 'big_cqe' incomplete array field.
        Entry(unsafe { mem::transmute_copy(&self.0) })
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("result", &self.result())
            .field("user_data", &self.user_data())
            .field("flags", &self.flags())
            .finish()
    }
}

impl Entry32 {
    /// The operation-specific result code. For example, for a [`Read`](crate::opcode::Read)
    /// operation this is equivalent to the return value of the `read(2)` system call.
    #[inline]
    pub fn result(&self) -> i32 {
        self.0 .0.res
    }

    /// The user data of the request, as set by
    /// [`Entry::user_data`](crate::squeue::Entry::user_data) on the submission queue event.
    #[inline]
    pub fn user_data(&self) -> u64 {
        self.0 .0.user_data.u64_()
    }

    /// The user data of the request, as set by
    /// [`Entry::user_data`](crate::squeue::Entry::user_data) on the submission queue event.
    #[inline]
    pub fn user_data_ptr(&self) -> *mut core::ffi::c_void {
        self.0 .0.user_data.ptr()
    }

    /// Metadata related to the operation.
    ///
    /// This is currently used for:
    /// - Storing the selected buffer ID, if one was selected. See
    /// [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info.
    #[inline]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_retain(self.0 .0.flags.bits())
    }

    /// Additional data available in 32-byte completion queue entries (CQEs).
    #[inline]
    pub fn big_cqe(&self) -> &[u64; 2] {
        &self.1
    }
}

impl private::Sealed for Entry32 {}

impl EntryMarker for Entry32 {
    const BUILD_FLAGS: sys::IoringSetupFlags = sys::IoringSetupFlags::CQE32;
}

impl From<Entry32> for Entry {
    fn from(entry32: Entry32) -> Self {
        entry32.0
    }
}

impl Debug for Entry32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry32")
            .field("result", &self.result())
            .field("user_data", &self.user_data())
            .field("flags", &self.flags())
            .field("big_cqe", &self.big_cqe())
            .finish()
    }
}

/// Return which dynamic buffer was used by this operation.
///
/// This corresponds to the `IORING_CQE_F_BUFFER` flag (and related bit-shifting),
/// and it signals to the consumer which provided contains the result of this
/// operation.
pub fn buffer_select(flags: Flags) -> Option<u16> {
    if flags.contains(Flags::BUFFER) {
        let id = flags.bits() >> sys::IORING_CQE_BUFFER_SHIFT;

        // FIXME
        //
        // Should we return u16? maybe kernel will change value of `IORING_CQE_BUFFER_SHIFT` in future.
        Some(id as u16)
    } else {
        None
    }
}

/// Return whether further completion events will be submitted for
/// this same operation.
///
/// This corresponds to the `IORING_CQE_F_MORE` flag, and it signals to
/// the consumer that it should expect further CQE entries after this one,
/// still from the same original SQE request (e.g. for multishot operations).
pub fn more(flags: Flags) -> bool {
    flags.contains(Flags::MORE)
}

/// Return whether socket has more data ready to read.
///
/// This corresponds to the `IORING_CQE_F_SOCK_NONEMPTY` flag, and it signals to
/// the consumer that the socket has more data that can be read immediately.
///
/// The io_uring documentation says recv, recv-multishot, recvmsg, and recvmsg-multishot
/// can provide this bit in their respective CQE.
pub fn sock_nonempty(flags: Flags) -> bool {
    flags.contains(Flags::SOCK_NONEMPTY)
}
