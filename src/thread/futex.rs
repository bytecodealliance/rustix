//! Linux `futex`.
//!
//! # Safety
//!
//! Futex is a very low-level mechanism for implementing concurrency
//! primitives.
#![allow(unsafe_code)]

use core::num::NonZeroU32;
use core::ptr;
use core::sync::atomic::AtomicU32;

use crate::backend::thread::syscalls::{futex_timespec, futex_val2};
use crate::fd::{FromRawFd, OwnedFd};
use crate::thread::Timespec;
use crate::{backend, io};

pub use backend::thread::futex::FutexFlags;
pub use backend::thread::futex::FutexOperation;

/// `FUTEX_WAITERS`
pub const FUTEX_WAITERS: u32 = backend::thread::futex::FUTEX_WAITERS;
/// `FUTEX_OWNER_DIED`
pub const FUTEX_OWNER_DIED: u32 = backend::thread::futex::FUTEX_OWNER_DIED;

/// DEPRECATED: There are now individual functions available to perform futex operations with improved type safety. See the [futex module](`self`).
///
/// `futex(uaddr, op, val, utime, uaddr2, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[deprecated(
    since = "0.38.35",
    note = "There are now individual functions available to perform futex operations with improved type safety. See the futex module."
)]
#[inline]
pub unsafe fn futex(
    uaddr: *mut u32,
    op: FutexOperation,
    flags: FutexFlags,
    val: u32,
    utime: *const Timespec,
    uaddr2: *mut u32,
    val3: u32,
) -> io::Result<usize> {
    use FutexOperation::*;

    match op {
        Wait | LockPi | WaitBitset | WaitRequeuePi | LockPi2 => futex_timespec(
            uaddr as *const AtomicU32,
            op,
            flags,
            val,
            utime,
            uaddr2 as *const AtomicU32,
            val3,
        ),
        Wake | Fd | Requeue | CmpRequeue | WakeOp | UnlockPi | TrylockPi | WakeBitset
        | CmpRequeuePi => futex_val2(
            uaddr as *const AtomicU32,
            op,
            flags,
            val,
            utime as usize as u32,
            uaddr2 as *const AtomicU32,
            val3,
        ),
    }
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAIT, val, timeout, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wait(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
) -> io::Result<()> {
    backend::thread::syscalls::futex_timespec(
        uaddr,
        FutexOperation::Wait,
        flags,
        val,
        timeout
            .as_ref()
            .map(|timeout| timeout as *const Timespec)
            .unwrap_or(ptr::null()),
        ptr::null(),
        0,
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAKE, val, NULL, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wake(uaddr: &AtomicU32, flags: FutexFlags, val: u32) -> io::Result<usize> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::Wake,
        flags,
        val,
        0,
        ptr::null(),
        0,
    )
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_FD, val, NULL, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn fd(uaddr: &AtomicU32, flags: FutexFlags, val: u32) -> io::Result<OwnedFd> {
    backend::thread::syscalls::futex_val2(uaddr, FutexOperation::Fd, flags, val, 0, ptr::null(), 0)
        .map(|fd| OwnedFd::from_raw_fd(fd.try_into().expect("return value should be a valid fd")))
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_REQUEUE, val, val2, uaddr2, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn requeue(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
) -> io::Result<usize> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::Requeue,
        flags,
        val,
        val2,
        uaddr2,
        0,
    )
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_CMP_REQUEUE, val, val2, uaddr2, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn cmp_requeue(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::CmpRequeue,
        flags,
        val,
        val2,
        uaddr2,
        val3,
    )
}

/// `FUTEX_OP_*` operations for use with [`wake_op`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum WakeOp {
    /// `FUTEX_OP_SET`: `uaddr2 = oparg;`
    Set = 0,
    /// `FUTEX_OP_ADD`: `uaddr2 += oparg;`
    Add = 1,
    /// `FUTEX_OP_OR`: `uaddr2 |= oparg;`
    Or = 2,
    /// `FUTEX_OP_ANDN`: `uaddr2 &= ~oparg;`
    AndN = 3,
    /// `FUTEX_OP_XOR`: `uaddr2 ^= oparg;`
    XOr = 4,
    /// `FUTEX_OP_SET | FUTEX_OP_ARG_SHIFT`: `uaddr2 = (oparg << 1);`
    SetShift = 0 | 8,
    /// `FUTEX_OP_ADD | FUTEX_OP_ARG_SHIFT`: `uaddr2 += (oparg << 1);`
    AddShift = 1 | 8,
    /// `FUTEX_OP_OR | FUTEX_OP_ARG_SHIFT`: `uaddr2 |= (oparg << 1);`
    OrShift = 2 | 8,
    /// `FUTEX_OP_ANDN | FUTEX_OP_ARG_SHIFT`: `uaddr2 &= !(oparg << 1);`
    AndNShift = 3 | 8,
    /// `FUTEX_OP_XOR | FUTEX_OP_ARG_SHIFT`: `uaddr2 ^= (oparg << 1);`
    XOrShift = 4 | 8,
}

/// `FUTEX_OP_CMP_*` operations for use with [`wake_op`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum WakeOpCmp {
    /// `FUTEX_OP_CMP_EQ`: `if oldval == cmparg { wake(); }`
    Eq = 0,
    /// `FUTEX_OP_CMP_EQ`: `if oldval != cmparg { wake(); }`
    Ne = 1,
    /// `FUTEX_OP_CMP_EQ`: `if oldval < cmparg { wake(); }`
    Lt = 2,
    /// `FUTEX_OP_CMP_EQ`: `if oldval <= cmparg { wake(); }`
    Le = 3,
    /// `FUTEX_OP_CMP_EQ`: `if oldval > cmparg { wake(); }`
    Gt = 4,
    /// `FUTEX_OP_CMP_EQ`: `if oldval >= cmparg { wake(); }`
    Ge = 5,
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAKE_OP, val, val2, uaddr2, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wake_op(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
    op: WakeOp,
    cmp: WakeOpCmp,
    oparg: u16,
    cmparg: u16,
) -> io::Result<usize> {
    if oparg >= 1 << 12 || cmparg >= 1 << 12 {
        return Err(io::Errno::INVAL);
    }

    let val3 =
        ((op as u32) << 28) | ((cmp as u32) << 24) | ((oparg as u32) << 12) | (cmparg as u32);

    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::WakeOp,
        flags,
        val,
        val2,
        uaddr2,
        val3,
    )
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_LOCK_PI, 0, timeout, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn lock_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    timeout: Option<Timespec>,
) -> io::Result<()> {
    backend::thread::syscalls::futex_timespec(
        uaddr,
        FutexOperation::LockPi,
        flags,
        0,
        timeout
            .as_ref()
            .map(|timeout| timeout as *const Timespec)
            .unwrap_or(ptr::null()),
        ptr::null(),
        0,
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_UNLOCK_PI, 0, NULL, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn unlock_pi(uaddr: &AtomicU32, flags: FutexFlags) -> io::Result<()> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::UnlockPi,
        flags,
        0,
        0,
        ptr::null(),
        0,
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_TRYLOCK_PI, 0, NULL, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn trylock_pi(uaddr: &AtomicU32, flags: FutexFlags) -> io::Result<()> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::TrylockPi,
        flags,
        0,
        0,
        ptr::null(),
        0,
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAIT_BITSET, val, timeout/val2, NULL, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wait_bitset(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
    val3: NonZeroU32,
) -> io::Result<()> {
    backend::thread::syscalls::futex_timespec(
        uaddr,
        FutexOperation::WaitBitset,
        flags,
        val,
        timeout
            .as_ref()
            .map(|timeout| timeout as *const Timespec)
            .unwrap_or(ptr::null()),
        ptr::null(),
        val3.get(),
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAKE_BITSET, val, NULL, NULL, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wake_bitset(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val3: NonZeroU32,
) -> io::Result<usize> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::WakeBitset,
        flags,
        val,
        0,
        ptr::null(),
        val3.get(),
    )
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_WAIT_REQUEUE_PI, val, timeout, uaddr2, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn wait_requeue_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
    uaddr2: &AtomicU32,
) -> io::Result<()> {
    backend::thread::syscalls::futex_timespec(
        uaddr,
        FutexOperation::WaitRequeuePi,
        flags,
        val,
        timeout
            .as_ref()
            .map(|timeout| timeout as *const Timespec)
            .unwrap_or(ptr::null()),
        uaddr2,
        0,
    )?;
    Ok(())
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_CMP_REQUEUE_PI, 1, val2, uaddr2, val3)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn cmp_requeue_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val2: u32,
    uaddr2: &AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    backend::thread::syscalls::futex_val2(
        uaddr,
        FutexOperation::CmpRequeuePi,
        flags,
        1,
        val2,
        uaddr2,
        val3,
    )
}

/// Equivalent to `syscall(SYS_futex, uaddr, FUTEX_LOCK_PI2, 0, timeout, NULL, 0)`
///
/// # References
///  - [Linux `futex` system call]
///  - [Linux `futex` feature]
///
/// # Safety
///
/// This is a very low-level feature for implementing synchronization
/// primitives. See the references links above.
///
/// [Linux `futex` system call]: https://man7.org/linux/man-pages/man2/futex.2.html
/// [Linux `futex` feature]: https://man7.org/linux/man-pages/man7/futex.7.html
#[inline]
pub unsafe fn lock_pi2(uaddr: &AtomicU32, flags: FutexFlags, timeout: &Timespec) -> io::Result<()> {
    backend::thread::syscalls::futex_timespec(
        uaddr,
        FutexOperation::LockPi2,
        flags,
        0,
        timeout,
        ptr::null(),
        0,
    )?;
    Ok(())
}
