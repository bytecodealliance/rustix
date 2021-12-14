//! The `rustix` `Error` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.
#![allow(unsafe_code)]
#![allow(missing_docs)]

use super::super::wasi_filesystem;
use crate::imp::fd::RawFd;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Inner {
    Filesystem(wasi_filesystem::Errno),
    BadF,
}

/// The error type for `rustix` APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Error(Inner);

impl Error {
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        let bits = io_err.raw_os_error()?;
        Some(Self::from_raw_os_error(bits))
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        match self.0 {
            Inner::Filesystem(errno) => errno as i32,
            Inner::BadF => -1,
        }
    }

    /// Construct an `Error` from a raw OS error number.
    #[inline]
    pub fn from_raw_os_error(raw: i32) -> Self {
        todo!("from_raw_os_error")
    }
}

impl Error {
    pub const ACCESS: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Access));
    pub const ADDRINUSE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Addrinuse));
    pub const ADDRNOTAVAIL: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Addrnotavail));
    pub const AFNOSUPPORT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Afnosupport));
    pub const AGAIN: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Again));
    pub const ALREADY: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Already));
    pub const BADMSG: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Badmsg));
    pub const BUSY: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Busy));
    pub const CANCELED: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Canceled));
    pub const CHILD: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Child));
    pub const CONNABORTED: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Connaborted));
    pub const CONNREFUSED: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Connrefused));
    pub const CONNRESET: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Connreset));
    pub const DEADLK: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Deadlk));
    pub const DESTADDRREQ: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Destaddrreq));
    pub const DOM: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Dom));
    pub const DQUOT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Dquot));
    pub const EXIST: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Exist));
    pub const FAULT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Fault));
    pub const FBIG: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Fbig));
    pub const HOSTUNREACH: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Hostunreach));
    pub const IDRM: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Idrm));
    pub const ILSEQ: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Ilseq));
    pub const INTR: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Intr));
    pub const INVAL: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Inval));
    pub const INPROGRESS: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Inprogress));
    pub const IO: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Io));
    pub const ISCONN: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Isconn));
    pub const ISDIR: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Isdir));
    pub const LOOP: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Loop));
    pub const MFILE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Mfile));
    pub const MLINK: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Mlink));
    pub const MSGSIZE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Msgsize));
    pub const MULTIHOP: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Multihop));
    pub const NAMETOOLONG: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nametoolong));
    pub const NETDOWN: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Netdown));
    pub const NETUNREACH: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Netunreach));
    pub const NETRESET: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Netreset));
    pub const NFILE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nfile));
    pub const NOBUFS: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nobufs));
    pub const NODEV: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nodev));
    pub const NOENT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Noent));
    pub const NOEXEC: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Noexec));
    pub const NOLCK: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nolck));
    pub const NOLINK: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nolink));
    pub const NOMEM: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nomem));
    pub const NOMSG: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nomsg));
    pub const NOPROTOOPT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Noprotoopt));
    pub const NOSPC: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nospc));
    pub const NOSYS: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nosys));
    pub const NOTCONN: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notconn));
    pub const NOTDIR: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notdir));
    pub const NOTEMPTY: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notempty));
    pub const NOTRECOVERABLE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notrecoverable));
    pub const NOTSOCK: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notsock));
    pub const NOTSUP: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notsup));
    pub const NOTTY: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notty));
    pub const NXIO: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Nxio));
    // On WASI, `EOPNOTSUPP` has the same value as `ENOTSUP`.
    pub const OPNOTSUPP: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Notsup));
    pub const OVERFLOW: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Overflow));
    pub const OWNERDEAD: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Ownerdead));
    pub const PERM: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Perm));
    pub const PIPE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Pipe));
    pub const PROTO: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Proto));
    pub const PROTONOSUPPORT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Protonosupport));
    pub const PROTOTYPE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Prototype));
    pub const RANGE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Range));
    pub const ROFS: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Rofs));
    pub const SPIPE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Spipe));
    pub const SRCH: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Srch));
    pub const STALE: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Stale));
    pub const TIMEDOUT: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Timedout));
    pub const TOOBIG: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Toobig));
    pub const TXTBSY: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Txtbsy));
    pub const XDEV: Self = Self(Inner::Filesystem(wasi_filesystem::Errno::Xdev));

    pub const BADF: Self = Self(Inner::BadF);
}

impl From<wasi_filesystem::Errno> for Error {
    #[inline]
    fn from(os: wasi_filesystem::Errno) -> Self {
        Self(Inner::Filesystem(os))
    }
}
