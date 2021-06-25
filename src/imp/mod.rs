#![allow(missing_docs)]

#[cfg(libc)]
mod libc;
#[cfg(linux_raw)]
mod linux_raw;

#[cfg(libc)]
pub(crate) use self::libc::*;
#[cfg(linux_raw)]
pub(crate) use self::linux_raw::*;
