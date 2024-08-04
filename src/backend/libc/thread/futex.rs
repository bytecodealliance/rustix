use crate::backend::c;

bitflags::bitflags! {
    /// `FUTEX_*` flags for use with [`futex`].
    ///
    /// [`futex`]: mod@crate::thread::futex
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct FutexFlags: u32 {
        /// `FUTEX_PRIVATE_FLAG`
        const PRIVATE = bitcast!(c::FUTEX_PRIVATE_FLAG);
        /// `FUTEX_CLOCK_REALTIME`
        const CLOCK_REALTIME = bitcast!(c::FUTEX_CLOCK_REALTIME);
    }
}

/// `FUTEX_*` operations for use with [`futex`].
///
/// [`futex`]: mod@crate::thread::futex
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FutexOperation {
    /// `FUTEX_WAIT`
    Wait = bitcast!(c::FUTEX_WAIT),
    /// `FUTEX_WAKE`
    Wake = bitcast!(c::FUTEX_WAKE),
    /// `FUTEX_FD`
    Fd = bitcast!(c::FUTEX_FD),
    /// `FUTEX_REQUEUE`
    Requeue = bitcast!(c::FUTEX_REQUEUE),
    /// `FUTEX_CMP_REQUEUE`
    CmpRequeue = bitcast!(c::FUTEX_CMP_REQUEUE),
    /// `FUTEX_WAKE_OP`
    WakeOp = bitcast!(c::FUTEX_WAKE_OP),
    /// `FUTEX_LOCK_PI`
    LockPi = bitcast!(c::FUTEX_LOCK_PI),
    /// `FUTEX_UNLOCK_PI`
    UnlockPi = bitcast!(c::FUTEX_UNLOCK_PI),
    /// `FUTEX_TRYLOCK_PI`
    TrylockPi = bitcast!(c::FUTEX_TRYLOCK_PI),
    /// `FUTEX_WAIT_BITSET`
    WaitBitset = bitcast!(c::FUTEX_WAIT_BITSET),
    /// `FUTEX_WAKE_BITSET`
    WakeBitset = bitcast!(c::FUTEX_WAKE_BITSET),
    /// `FUTEX_WAIT_REQUEUE_PI`
    WaitRequeuePi = bitcast!(c::FUTEX_WAIT_REQUEUE_PI),
    /// `FUTEX_CMP_REQUEUE_PI`
    CmpRequeuePi = bitcast!(c::FUTEX_CMP_REQUEUE_PI),
    /// `FUTEX_LOCK_PI2`
    LockPi2 = bitcast!(c::FUTEX_LOCK_PI2),
}

/// `FUTEX_WAITERS`
pub const FUTEX_WAITERS: u32 = 0x80000000;

/// `FUTEX_OWNER_DIED`
pub const FUTEX_OWNER_DIED: u32 = 0x40000000;
