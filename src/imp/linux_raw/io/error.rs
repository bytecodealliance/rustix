//! The `rustix` `Error` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.
//!
//! # Safety
//!
//! Linux uses error codes in `-4095..0`; we use rustc attributes to describe
//! this restricted range of values.
#![allow(unsafe_code)]
#![allow(missing_docs)]
#![cfg_attr(not(rustc_attrs), allow(unused_unsafe))]

use super::super::c;
use crate::imp::fd::RawFd;
use crate::imp::reg::{RetNumber, RetReg};
use crate::io;
use linux_raw_sys::{errno, v5_4};

/// The error type for `rustix` APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
// Linux returns negated error codes, and we leave them in negated form, so
// error codes are in `-4095..0`.
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0xf001))]
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xffff))]
pub struct Error(u16);

impl Error {
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        io_err.raw_os_error().and_then(|raw| {
            // `std::io::Error` could theoretically have arbitrary "OS error"
            // values, so check that they're in Linux's range.
            if (1..4096).contains(&raw) {
                Some(Self::from_errno(raw as u32))
            } else {
                None
            }
        })
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        (self.0 as i16 as i32).wrapping_neg()
    }

    /// Construct an `Error` from a raw OS error number.
    #[inline]
    pub const fn from_raw_os_error(raw: i32) -> Self {
        Self::from_errno(raw as u32)
    }

    /// Convert from a C errno value (which is positive) to an `Error`.
    const fn from_errno(raw: u32) -> Self {
        // We store error values in negated form, so that we don't have to negate
        // them after every syscall.
        let encoded = raw.wrapping_neg() as u16;

        // TODO: Use Range::contains, once that's `const`.
        // TODO: Use `assert!`, once that's stable for use in const fn.
        const_assert!(encoded >= 0xf001);

        // Safety: Linux syscalls return negated error values in the range
        // `-4095..0`, which we just asserted.
        unsafe { Self(encoded) }
    }
}

/// Check for an error from the result of a syscall which encodes a
/// `c::c_int` on success.
#[inline]
pub(in crate::imp) fn try_decode_c_int<Num: RetNumber>(raw: RetReg<Num>) -> io::Result<c::c_int> {
    if raw.is_in_range(-4095..0) {
        // Safety: `raw` must be in `-4095..0`, and we just checked that raw is
        // in that range.
        return Err(unsafe { Error(raw.decode_error_code()) });
    }

    Ok(raw.decode_c_int())
}

/// Check for an error from the result of a syscall which encodes a
/// `c::c_uint` on success.
#[inline]
pub(in crate::imp) fn try_decode_c_uint<Num: RetNumber>(raw: RetReg<Num>) -> io::Result<c::c_uint> {
    if raw.is_in_range(-4095..0) {
        // Safety: `raw` must be in `-4095..0`, and we just checked that raw is
        // in that range.
        return Err(unsafe { Error(raw.decode_error_code()) });
    }

    Ok(raw.decode_c_uint())
}

/// Check for an error from the result of a syscall which encodes a `usize` on
/// success.
#[inline]
pub(in crate::imp) fn try_decode_usize<Num: RetNumber>(raw: RetReg<Num>) -> io::Result<usize> {
    if raw.is_in_range(-4095..0) {
        // Safety: `raw` must be in `-4095..0`, and we just checked that raw is
        // in that range.
        return Err(unsafe { Error(raw.decode_error_code()) });
    }

    Ok(raw.decode_usize())
}

/// Check for an error from the result of a syscall which encodes a
/// `*mut c_void` on success.
#[inline]
pub(in crate::imp) fn try_decode_void_star<Num: RetNumber>(
    raw: RetReg<Num>,
) -> io::Result<*mut c::c_void> {
    if raw.is_in_range(-4095..0) {
        // Safety: `raw` must be in `-4095..0`, and we just checked that raw is
        // in that range.
        return Err(unsafe { Error(raw.decode_error_code()) });
    }

    Ok(raw.decode_void_star())
}

/// Check for an error from the result of a syscall which encodes a
/// `u64` on success.
#[cfg(target_pointer_width = "64")]
#[inline]
pub(in crate::imp) fn try_decode_u64<Num: RetNumber>(raw: RetReg<Num>) -> io::Result<u64> {
    if raw.is_in_range(-4095..0) {
        // Safety: `raw` must be in `-4095..0`, and we just checked that raw is
        // in that range.
        return Err(unsafe { Error(raw.decode_error_code()) });
    }

    Ok(raw.decode_u64())
}

/// Check for an error from the result of a syscall which encodes a file
/// descriptor on success.
///
/// # Safety
///
/// This must only be used with syscalls which return file descriptors on
/// success.
#[inline]
pub(in crate::imp) unsafe fn try_decode_raw_fd<Num: RetNumber>(
    raw: RetReg<Num>,
) -> io::Result<RawFd> {
    // Instead of using `check_result` here, we just check for negative, since
    // this function is only used for system calls which return file
    // descriptors, and this produces smaller code.
    if raw.is_negative() {
        debug_assert!(raw.is_in_range(-4095..0));

        // Tell the optimizer that we know the value is in the error range.
        // This helps it avoid unnecessary integer conversions.
        #[cfg(core_intrinsics)]
        {
            core::intrinsics::assume(raw.is_in_range(-4095..0));
        }

        return Err(Error(raw.decode_error_code()));
    }

    Ok(raw.decode_raw_fd())
}

/// Check for an error from the result of a syscall which encodes no value on
/// success. On success, return the unconsumed `raw` value.
///
/// # Safety
///
/// This must only be used with syscalls which return no value on success.
#[inline]
pub(in crate::imp) unsafe fn try_decode_void<Num: RetNumber>(raw: RetReg<Num>) -> io::Result<()> {
    // Instead of using `check_result` here, we just check for zero, since this
    // function is only used for system calls which have no other return value,
    // and this produces smaller code.
    if raw.is_nonzero() {
        debug_assert!(raw.is_in_range(-4095..0));

        // Tell the optimizer that we know the value is in the error range.
        // This helps it avoid unnecessary integer conversions.
        #[cfg(core_intrinsics)]
        {
            core::intrinsics::assume(raw.is_in_range(-4095..0));
        }

        return Err(Error(raw.decode_error_code()));
    }

    raw.decode_void();

    Ok(())
}

/// Check for an error from the result of a syscall which does not return on
/// success. On success, return the unconsumed `raw` value.
///
/// # Safety
///
/// This must only be used with syscalls which do not return on success.
#[inline]
pub(in crate::imp) unsafe fn try_decode_error<Num: RetNumber>(raw: RetReg<Num>) -> io::Error {
    debug_assert!(raw.is_in_range(-4095..0));

    // Tell the optimizer that we know the value is in the error range.
    // This helps it avoid unnecessary integer conversions.
    #[cfg(core_intrinsics)]
    {
        core::intrinsics::assume(raw.is_in_range(-4095..0));
    }

    Error(raw.decode_error_code())
}

/// Return the contained `usize` value.
#[cfg(not(debug_assertions))]
#[inline]
pub(in crate::imp) fn decode_usize_infallible<Num: RetNumber>(raw: RetReg<Num>) -> usize {
    raw.decode_usize()
}

impl Error {
    #[doc(alias = "ACCES")]
    pub const ACCESS: Self = Self::from_errno(errno::EACCES);
    pub const ADDRINUSE: Self = Self::from_errno(errno::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self::from_errno(errno::EADDRNOTAVAIL);
    pub const ADV: Self = Self::from_errno(errno::EADV);
    pub const AFNOSUPPORT: Self = Self::from_errno(errno::EAFNOSUPPORT);
    pub const AGAIN: Self = Self::from_errno(errno::EAGAIN);
    pub const ALREADY: Self = Self::from_errno(errno::EALREADY);
    pub const BADE: Self = Self::from_errno(errno::EBADE);
    pub const BADF: Self = Self::from_errno(errno::EBADF);
    pub const BADFD: Self = Self::from_errno(errno::EBADFD);
    pub const BADMSG: Self = Self::from_errno(errno::EBADMSG);
    pub const BADR: Self = Self::from_errno(errno::EBADR);
    pub const BADRQC: Self = Self::from_errno(errno::EBADRQC);
    pub const BADSLT: Self = Self::from_errno(errno::EBADSLT);
    pub const BFONT: Self = Self::from_errno(errno::EBFONT);
    pub const BUSY: Self = Self::from_errno(errno::EBUSY);
    pub const CANCELED: Self = Self::from_errno(errno::ECANCELED);
    pub const CHILD: Self = Self::from_errno(errno::ECHILD);
    pub const CHRNG: Self = Self::from_errno(errno::ECHRNG);
    pub const COMM: Self = Self::from_errno(errno::ECOMM);
    pub const CONNABORTED: Self = Self::from_errno(errno::ECONNABORTED);
    pub const CONNREFUSED: Self = Self::from_errno(errno::ECONNREFUSED);
    pub const CONNRESET: Self = Self::from_errno(errno::ECONNRESET);
    pub const DEADLK: Self = Self::from_errno(errno::EDEADLK);
    pub const DEADLOCK: Self = Self::from_errno(errno::EDEADLOCK);
    pub const DESTADDRREQ: Self = Self::from_errno(errno::EDESTADDRREQ);
    pub const DOM: Self = Self::from_errno(errno::EDOM);
    pub const DOTDOT: Self = Self::from_errno(errno::EDOTDOT);
    pub const DQUOT: Self = Self::from_errno(errno::EDQUOT);
    pub const EXIST: Self = Self::from_errno(errno::EEXIST);
    pub const FAULT: Self = Self::from_errno(errno::EFAULT);
    pub const FBIG: Self = Self::from_errno(errno::EFBIG);
    pub const HOSTDOWN: Self = Self::from_errno(errno::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self::from_errno(errno::EHOSTUNREACH);
    pub const HWPOISON: Self = Self::from_errno(v5_4::errno::EHWPOISON);
    pub const IDRM: Self = Self::from_errno(errno::EIDRM);
    pub const ILSEQ: Self = Self::from_errno(errno::EILSEQ);
    pub const INPROGRESS: Self = Self::from_errno(errno::EINPROGRESS);
    pub const INTR: Self = Self::from_errno(errno::EINTR);
    pub const INVAL: Self = Self::from_errno(errno::EINVAL);
    pub const IO: Self = Self::from_errno(errno::EIO);
    pub const ISCONN: Self = Self::from_errno(errno::EISCONN);
    pub const ISDIR: Self = Self::from_errno(errno::EISDIR);
    pub const ISNAM: Self = Self::from_errno(errno::EISNAM);
    pub const KEYEXPIRED: Self = Self::from_errno(errno::EKEYEXPIRED);
    pub const KEYREJECTED: Self = Self::from_errno(errno::EKEYREJECTED);
    pub const KEYREVOKED: Self = Self::from_errno(errno::EKEYREVOKED);
    pub const L2HLT: Self = Self::from_errno(errno::EL2HLT);
    pub const L2NSYNC: Self = Self::from_errno(errno::EL2NSYNC);
    pub const L3HLT: Self = Self::from_errno(errno::EL3HLT);
    pub const L3RST: Self = Self::from_errno(errno::EL3RST);
    pub const LIBACC: Self = Self::from_errno(errno::ELIBACC);
    pub const LIBBAD: Self = Self::from_errno(errno::ELIBBAD);
    pub const LIBEXEC: Self = Self::from_errno(errno::ELIBEXEC);
    pub const LIBMAX: Self = Self::from_errno(errno::ELIBMAX);
    pub const LIBSCN: Self = Self::from_errno(errno::ELIBSCN);
    pub const LNRNG: Self = Self::from_errno(errno::ELNRNG);
    pub const LOOP: Self = Self::from_errno(errno::ELOOP);
    pub const MEDIUMTYPE: Self = Self::from_errno(errno::EMEDIUMTYPE);
    pub const MFILE: Self = Self::from_errno(errno::EMFILE);
    pub const MLINK: Self = Self::from_errno(errno::EMLINK);
    pub const MSGSIZE: Self = Self::from_errno(errno::EMSGSIZE);
    pub const MULTIHOP: Self = Self::from_errno(errno::EMULTIHOP);
    pub const NAMETOOLONG: Self = Self::from_errno(errno::ENAMETOOLONG);
    pub const NAVAIL: Self = Self::from_errno(errno::ENAVAIL);
    pub const NETDOWN: Self = Self::from_errno(errno::ENETDOWN);
    pub const NETRESET: Self = Self::from_errno(errno::ENETRESET);
    pub const NETUNREACH: Self = Self::from_errno(errno::ENETUNREACH);
    pub const NFILE: Self = Self::from_errno(errno::ENFILE);
    pub const NOANO: Self = Self::from_errno(errno::ENOANO);
    pub const NOBUFS: Self = Self::from_errno(errno::ENOBUFS);
    pub const NOCSI: Self = Self::from_errno(errno::ENOCSI);
    pub const NODATA: Self = Self::from_errno(errno::ENODATA);
    pub const NODEV: Self = Self::from_errno(errno::ENODEV);
    pub const NOENT: Self = Self::from_errno(errno::ENOENT);
    pub const NOEXEC: Self = Self::from_errno(errno::ENOEXEC);
    pub const NOKEY: Self = Self::from_errno(errno::ENOKEY);
    pub const NOLCK: Self = Self::from_errno(errno::ENOLCK);
    pub const NOLINK: Self = Self::from_errno(errno::ENOLINK);
    pub const NOMEDIUM: Self = Self::from_errno(errno::ENOMEDIUM);
    pub const NOMEM: Self = Self::from_errno(errno::ENOMEM);
    pub const NOMSG: Self = Self::from_errno(errno::ENOMSG);
    pub const NONET: Self = Self::from_errno(errno::ENONET);
    pub const NOPKG: Self = Self::from_errno(errno::ENOPKG);
    pub const NOPROTOOPT: Self = Self::from_errno(errno::ENOPROTOOPT);
    pub const NOSPC: Self = Self::from_errno(errno::ENOSPC);
    pub const NOSR: Self = Self::from_errno(errno::ENOSR);
    pub const NOSTR: Self = Self::from_errno(errno::ENOSTR);
    pub const NOSYS: Self = Self::from_errno(errno::ENOSYS);
    pub const NOTBLK: Self = Self::from_errno(errno::ENOTBLK);
    pub const NOTCONN: Self = Self::from_errno(errno::ENOTCONN);
    pub const NOTDIR: Self = Self::from_errno(errno::ENOTDIR);
    pub const NOTEMPTY: Self = Self::from_errno(errno::ENOTEMPTY);
    pub const NOTNAM: Self = Self::from_errno(errno::ENOTNAM);
    pub const NOTRECOVERABLE: Self = Self::from_errno(errno::ENOTRECOVERABLE);
    pub const NOTSOCK: Self = Self::from_errno(errno::ENOTSOCK);
    // On Linux, `ENOTSUP` has the same value as `EOPNOTSUPP`.
    pub const NOTSUP: Self = Self::from_errno(errno::EOPNOTSUPP);
    pub const NOTTY: Self = Self::from_errno(errno::ENOTTY);
    pub const NOTUNIQ: Self = Self::from_errno(errno::ENOTUNIQ);
    pub const NXIO: Self = Self::from_errno(errno::ENXIO);
    pub const OPNOTSUPP: Self = Self::from_errno(errno::EOPNOTSUPP);
    pub const OVERFLOW: Self = Self::from_errno(errno::EOVERFLOW);
    pub const OWNERDEAD: Self = Self::from_errno(errno::EOWNERDEAD);
    pub const PERM: Self = Self::from_errno(errno::EPERM);
    pub const PFNOSUPPORT: Self = Self::from_errno(errno::EPFNOSUPPORT);
    pub const PIPE: Self = Self::from_errno(errno::EPIPE);
    pub const PROTO: Self = Self::from_errno(errno::EPROTO);
    pub const PROTONOSUPPORT: Self = Self::from_errno(errno::EPROTONOSUPPORT);
    pub const PROTOTYPE: Self = Self::from_errno(errno::EPROTOTYPE);
    pub const RANGE: Self = Self::from_errno(errno::ERANGE);
    pub const REMCHG: Self = Self::from_errno(errno::EREMCHG);
    pub const REMOTE: Self = Self::from_errno(errno::EREMOTE);
    pub const REMOTEIO: Self = Self::from_errno(errno::EREMOTEIO);
    pub const RESTART: Self = Self::from_errno(errno::ERESTART);
    pub const RFKILL: Self = Self::from_errno(errno::ERFKILL);
    pub const ROFS: Self = Self::from_errno(errno::EROFS);
    pub const SHUTDOWN: Self = Self::from_errno(errno::ESHUTDOWN);
    pub const SOCKTNOSUPPORT: Self = Self::from_errno(errno::ESOCKTNOSUPPORT);
    pub const SPIPE: Self = Self::from_errno(errno::ESPIPE);
    pub const SRCH: Self = Self::from_errno(errno::ESRCH);
    pub const SRMNT: Self = Self::from_errno(errno::ESRMNT);
    pub const STALE: Self = Self::from_errno(errno::ESTALE);
    pub const STRPIPE: Self = Self::from_errno(errno::ESTRPIPE);
    pub const TIME: Self = Self::from_errno(errno::ETIME);
    pub const TIMEDOUT: Self = Self::from_errno(errno::ETIMEDOUT);
    pub const TOOBIG: Self = Self::from_errno(errno::E2BIG);
    pub const TOOMANYREFS: Self = Self::from_errno(errno::ETOOMANYREFS);
    pub const TXTBSY: Self = Self::from_errno(errno::ETXTBSY);
    pub const UCLEAN: Self = Self::from_errno(errno::EUCLEAN);
    pub const UNATCH: Self = Self::from_errno(errno::EUNATCH);
    pub const USERS: Self = Self::from_errno(errno::EUSERS);
    pub const WOULDBLOCK: Self = Self::from_errno(errno::EWOULDBLOCK);
    pub const XDEV: Self = Self::from_errno(errno::EXDEV);
    pub const XFULL: Self = Self::from_errno(errno::EXFULL);
}
