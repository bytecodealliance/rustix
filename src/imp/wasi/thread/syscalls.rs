use super::super::time::Timespec;
use crate::thread::NanosleepRelativeResult;

#[inline]
pub(crate) fn nanosleep(request: &Timespec) -> NanosleepRelativeResult {
    todo!("nanosleep")
}
