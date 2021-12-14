use super::{ClockId, Timespec};

#[inline]
#[must_use]
pub(crate) fn clock_getres(id: ClockId) -> Timespec {
    todo!("clock_getres")
}

#[inline]
#[must_use]
pub(crate) fn clock_gettime(id: ClockId) -> Timespec {
    todo!("clock_gettime")
}
