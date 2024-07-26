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

use crate::fd::{FromRawFd, OwnedFd};
use crate::thread::Timespec;
use crate::{backend, io};

pub use backend::thread::futex::FutexFlags;
use backend::thread::futex::FutexOperation;

/// `FUTEX_WAITERS`
pub const FUTEX_WAITERS: u32 = backend::thread::futex::FUTEX_WAITERS;
/// `FUTEX_OWNER_DIED`
pub const FUTEX_OWNER_DIED: u32 = backend::thread::futex::FUTEX_OWNER_DIED;

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
pub unsafe fn futex_wait(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::Wait,
        flags,
        val,
        timeout
            .map(|timeout| &timeout as *const Timespec)
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
pub unsafe fn futex_wake(uaddr: &AtomicU32, flags: FutexFlags, val: u32) -> io::Result<usize> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::Wake,
        flags,
        val,
        ptr::null(),
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
pub unsafe fn futex_fd(uaddr: &AtomicU32, flags: FutexFlags, val: u32) -> io::Result<OwnedFd> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::Fd,
        flags,
        val,
        ptr::null(),
        ptr::null(),
        0,
    )
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
pub unsafe fn futex_requeue(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
) -> io::Result<usize> {
    // the least significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::Requeue,
        flags,
        val,
        timeout,
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
pub unsafe fn futex_cmp_requeue(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    // the least significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::CmpRequeue,
        flags,
        val,
        timeout,
        uaddr2,
        val3,
    )
}

/// `FUTEX_OP_*` operations for use with [`futex_wake_op`].
///
/// [`futex`]: crate::thread::futex
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FutexOp {
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

/// `FUTEX_OP_CMP_*` operations for use with [`futex_wake_op`].
///
/// [`futex`]: crate::thread::futex
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FutexOpCmp {
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
pub unsafe fn futex_wake_op(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val2: u32,
    uaddr2: &AtomicU32,
    op: FutexOp,
    cmp: FutexOpCmp,
    oparg: u16,
    cmparg: u16,
) -> io::Result<usize> {
    if oparg >= 1 << 12 || cmparg >= 1 << 12 {
        return Err(io::Errno::INVAL);
    }

    // the least significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;

    let val3 =
        ((op as u32) << 28) | ((cmp as u32) << 24) | ((oparg as u32) << 12) | (cmparg as u32);

    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::WakeOp,
        flags,
        val,
        timeout,
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
pub unsafe fn futex_lock_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    timeout: Option<Timespec>,
) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::LockPi,
        flags,
        0,
        timeout
            .map(|timeout| &timeout as *const Timespec)
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
pub unsafe fn futex_unlock_pi(uaddr: &AtomicU32, flags: FutexFlags) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::UnlockPi,
        flags,
        0,
        ptr::null(),
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
pub unsafe fn futex_trylock_pi(uaddr: &AtomicU32, flags: FutexFlags) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::TrylockPi,
        flags,
        0,
        ptr::null(),
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
pub unsafe fn futex_wait_bitset(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
    val3: NonZeroU32,
) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::WaitBitset,
        flags,
        val,
        timeout
            .map(|timeout| &timeout as *const Timespec)
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
pub unsafe fn futex_wake_bitset(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    val3: NonZeroU32,
) -> io::Result<usize> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::WakeBitset,
        flags,
        val,
        ptr::null(),
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
pub unsafe fn futex_wait_requeue_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val: u32,
    timeout: Option<Timespec>,
    uaddr2: &AtomicU32,
) -> io::Result<()> {
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::WaitRequeuePi,
        flags,
        val,
        timeout
            .map(|timeout| &timeout as *const Timespec)
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
pub unsafe fn futex_cmp_requeue_pi(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    val2: u32,
    uaddr2: &AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    // the least significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;
    backend::thread::syscalls::futex(
        uaddr,
        FutexOperation::CmpRequeuePi,
        flags,
        1,
        timeout,
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
pub unsafe fn futex_lock_pi2(
    uaddr: &AtomicU32,
    flags: FutexFlags,
    timeout: &Timespec,
) -> io::Result<()> {
    backend::thread::syscalls::futex(
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
