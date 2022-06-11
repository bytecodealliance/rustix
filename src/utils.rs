use crate::process::{Gid, Uid};

/// Convert a `&T` into a `*const T` without using an `as`.
#[inline]
#[allow(dead_code)]
pub(crate) const fn as_ptr<T>(t: &T) -> *const T {
    t
}

/// Convert a `&mut T` into a `*mut T` without using an `as`.
#[inline]
#[allow(dead_code)]
pub(crate) fn as_mut_ptr<T>(t: &mut T) -> *mut T {
    t
}

pub fn unwrap_fchown_args(owner: Option<Uid>, group: Option<Gid>) -> (u32, u32) {
    let ow = match owner {
        Some(o) => o.as_raw(),
        None => u32::MAX,
    };

    let gr = match group {
        Some(g) => g.as_raw(),
        None => u32::MAX,
    };

    (ow, gr)
}