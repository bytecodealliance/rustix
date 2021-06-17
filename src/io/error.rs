//! The `Error` type, which is a minimal wrapper around an errno value.
//!
//! We define the errno constants as invididual `const`s instead of an
//! enum because we may not know about all of the host's errno values
//! and we don't want unrecognized values to create UB.

#![allow(missing_docs)]

use std::{error, fmt, result};
#[cfg(libc)]
use {errno::errno, std::os::raw::c_int};

/// A specialized `Result` type for posish APIs.
pub type Result<T> = result::Result<T, Error>;

/// The error type for posish APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[cfg(libc)]
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Error(pub(crate) c_int);

/// The error type for posish APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[cfg(linux_raw)]
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
// Linux errno codes are in 1..4096, which is 12 bits.
pub struct Error(pub(crate) u16);

#[cfg(libc)]
impl Error {
    pub const ACCES: Self = Self(libc::EACCES);
    pub const ADDRINUSE: Self = Self(libc::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self(libc::EADDRNOTAVAIL);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const ADV: Self = Self(libc::EADV);
    pub const AFNOSUPPORT: Self = Self(libc::EAFNOSUPPORT);
    pub const AGAIN: Self = Self(libc::EAGAIN);
    pub const ALREADY: Self = Self(libc::EALREADY);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const AUTH: Self = Self(libc::EAUTH);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BADE: Self = Self(libc::EBADE);
    pub const BADF: Self = Self(libc::EBADF);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BADFD: Self = Self(libc::EBADFD);
    pub const BADMSG: Self = Self(libc::EBADMSG);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BADR: Self = Self(libc::EBADR);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const BADRPC: Self = Self(libc::EBADRPC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BADRQC: Self = Self(libc::EBADRQC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BADSLT: Self = Self(libc::EBADSLT);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const BFONT: Self = Self(libc::EBFONT);
    pub const BUSY: Self = Self(libc::EBUSY);
    pub const CANCELED: Self = Self(libc::ECANCELED);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const CAPMODE: Self = Self(libc::ECAPMODE);
    pub const CHILD: Self = Self(libc::ECHILD);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const CHRNG: Self = Self(libc::ECHRNG);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const COMM: Self = Self(libc::ECOMM);
    pub const CONNABORTED: Self = Self(libc::ECONNABORTED);
    pub const CONNREFUSED: Self = Self(libc::ECONNREFUSED);
    pub const CONNRESET: Self = Self(libc::ECONNRESET);
    pub const DEADLK: Self = Self(libc::EDEADLK);
    #[cfg(not(any(
        target_os = "android",
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const DEADLOCK: Self = Self(libc::EDEADLOCK);
    pub const DESTADDRREQ: Self = Self(libc::EDESTADDRREQ);
    pub const DOM: Self = Self(libc::EDOM);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const DOOFUS: Self = Self(libc::EDOOFUS);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const DOTDOT: Self = Self(libc::EDOTDOT);
    pub const DQUOT: Self = Self(libc::EDQUOT);
    pub const EXIST: Self = Self(libc::EEXIST);
    pub const FAULT: Self = Self(libc::EFAULT);
    pub const FBIG: Self = Self(libc::EFBIG);
    #[cfg(any(
        target_env = "newlib",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const FTYPE: Self = Self(libc::EFTYPE);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Self = Self(libc::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self(libc::EHOSTUNREACH);
    #[cfg(not(any(
        target_os = "android",
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi",
        target_os = "redox"
    )))]
    pub const HWPOISON: Self = Self(libc::EHWPOISON);
    pub const IDRM: Self = Self(libc::EIDRM);
    pub const ILSEQ: Self = Self(libc::EILSEQ);
    pub const INPROGRESS: Self = Self(libc::EINPROGRESS);
    pub const INTR: Self = Self(libc::EINTR);
    pub const INVAL: Self = Self(libc::EINVAL);
    pub const IO: Self = Self(libc::EIO);
    pub const ISCONN: Self = Self(libc::EISCONN);
    pub const ISDIR: Self = Self(libc::EISDIR);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const ISNAM: Self = Self(libc::EISNAM);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const KEYEXPIRED: Self = Self(libc::EKEYEXPIRED);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const KEYREJECTED: Self = Self(libc::EKEYREJECTED);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const KEYREVOKED: Self = Self(libc::EKEYREVOKED);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const L2HLT: Self = Self(libc::EL2HLT);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const L2NSYNC: Self = Self(libc::EL2NSYNC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const L3HLT: Self = Self(libc::EL3HLT);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const L3RST: Self = Self(libc::EL3RST);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LIBACC: Self = Self(libc::ELIBACC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LIBBAD: Self = Self(libc::ELIBBAD);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LIBEXEC: Self = Self(libc::ELIBEXEC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LIBMAX: Self = Self(libc::ELIBMAX);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LIBSCN: Self = Self(libc::ELIBSCN);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const LNRNG: Self = Self(libc::ELNRNG);
    pub const LOOP: Self = Self(libc::ELOOP);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const MEDIUMTYPE: Self = Self(libc::EMEDIUMTYPE);
    pub const MFILE: Self = Self(libc::EMFILE);
    pub const MLINK: Self = Self(libc::EMLINK);
    pub const MSGSIZE: Self = Self(libc::EMSGSIZE);
    pub const MULTIHOP: Self = Self(libc::EMULTIHOP);
    pub const NAMETOOLONG: Self = Self(libc::ENAMETOOLONG);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NAVAIL: Self = Self(libc::ENAVAIL);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const NEEDAUTH: Self = Self(libc::ENEEDAUTH);
    pub const NETDOWN: Self = Self(libc::ENETDOWN);
    pub const NETRESET: Self = Self(libc::ENETRESET);
    pub const NETUNREACH: Self = Self(libc::ENETUNREACH);
    pub const NFILE: Self = Self(libc::ENFILE);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOANO: Self = Self(libc::ENOANO);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const NOATTR: Self = Self(libc::ENOATTR);
    pub const NOBUFS: Self = Self(libc::ENOBUFS);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOCSI: Self = Self(libc::ENOCSI);
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "wasi"
    )))]
    pub const NODATA: Self = Self(libc::ENODATA);
    pub const NODEV: Self = Self(libc::ENODEV);
    pub const NOENT: Self = Self(libc::ENOENT);
    pub const NOEXEC: Self = Self(libc::ENOEXEC);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOKEY: Self = Self(libc::ENOKEY);
    pub const NOLCK: Self = Self(libc::ENOLCK);
    pub const NOLINK: Self = Self(libc::ENOLINK);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOMEDIUM: Self = Self(libc::ENOMEDIUM);
    pub const NOMEM: Self = Self(libc::ENOMEM);
    pub const NOMSG: Self = Self(libc::ENOMSG);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NONET: Self = Self(libc::ENONET);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOPKG: Self = Self(libc::ENOPKG);
    pub const NOPROTOOPT: Self = Self(libc::ENOPROTOOPT);
    pub const NOSPC: Self = Self(libc::ENOSPC);
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "wasi"
    )))]
    pub const NOSR: Self = Self(libc::ENOSR);
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "wasi"
    )))]
    pub const NOSTR: Self = Self(libc::ENOSTR);
    pub const NOSYS: Self = Self(libc::ENOSYS);
    #[cfg(not(target_os = "wasi"))]
    pub const NOTBLK: Self = Self(libc::ENOTBLK);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const NOTCAPABLE: Self = Self(libc::ENOTCAPABLE);
    pub const NOTCONN: Self = Self(libc::ENOTCONN);
    pub const NOTDIR: Self = Self(libc::ENOTDIR);
    pub const NOTEMPTY: Self = Self(libc::ENOTEMPTY);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOTNAM: Self = Self(libc::ENOTNAM);
    #[cfg(not(target_os = "netbsd"))]
    pub const NOTRECOVERABLE: Self = Self(libc::ENOTRECOVERABLE);
    pub const NOTSOCK: Self = Self(libc::ENOTSOCK);
    #[cfg(not(target_os = "redox"))]
    pub const NOTSUP: Self = Self(libc::ENOTSUP);
    pub const NOTTY: Self = Self(libc::ENOTTY);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const NOTUNIQ: Self = Self(libc::ENOTUNIQ);
    pub const NXIO: Self = Self(libc::ENXIO);
    pub const OPNOTSUPP: Self = Self(libc::EOPNOTSUPP);
    pub const OVERFLOW: Self = Self(libc::EOVERFLOW);
    #[cfg(not(target_os = "netbsd"))]
    pub const OWNERDEAD: Self = Self(libc::EOWNERDEAD);
    pub const PERM: Self = Self(libc::EPERM);
    #[cfg(not(target_os = "wasi"))]
    pub const PFNOSUPPORT: Self = Self(libc::EPFNOSUPPORT);
    pub const PIPE: Self = Self(libc::EPIPE);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const PROCLIM: Self = Self(libc::EPROCLIM);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const PROCUNAVAIL: Self = Self(libc::EPROCUNAVAIL);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const PROGMISMATCH: Self = Self(libc::EPROGMISMATCH);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const PROGUNAVAIL: Self = Self(libc::EPROGUNAVAIL);
    pub const PROTO: Self = Self(libc::EPROTO);
    pub const PROTONOSUPPORT: Self = Self(libc::EPROTONOSUPPORT);
    pub const PROTOTYPE: Self = Self(libc::EPROTOTYPE);
    pub const RANGE: Self = Self(libc::ERANGE);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const REMCHG: Self = Self(libc::EREMCHG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Self = Self(libc::EREMOTE);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const REMOTEIO: Self = Self(libc::EREMOTEIO);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const RESTART: Self = Self(libc::ERESTART);
    #[cfg(not(any(
        target_os = "android",
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi",
        target_os = "redox"
    )))]
    pub const RFKILL: Self = Self(libc::ERFKILL);
    pub const ROFS: Self = Self(libc::EROFS);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "macos",
        target_os = "ios",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "openbsd"
    ))]
    pub const RPCMISMATCH: Self = Self(libc::ERPCMISMATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const SHUTDOWN: Self = Self(libc::ESHUTDOWN);
    #[cfg(not(target_os = "wasi"))]
    pub const SOCKTNOSUPPORT: Self = Self(libc::ESOCKTNOSUPPORT);
    pub const SPIPE: Self = Self(libc::ESPIPE);
    pub const SRCH: Self = Self(libc::ESRCH);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const SRMNT: Self = Self(libc::ESRMNT);
    pub const STALE: Self = Self(libc::ESTALE);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const STRPIPE: Self = Self(libc::ESTRPIPE);
    #[cfg(not(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "wasi"
    )))]
    pub const TIME: Self = Self(libc::ETIME);
    pub const TIMEDOUT: Self = Self(libc::ETIMEDOUT);
    pub const TOOBIG: Self = Self(libc::E2BIG);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Self = Self(libc::ETOOMANYREFS);
    pub const TXTBSY: Self = Self(libc::ETXTBSY);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const UCLEAN: Self = Self(libc::EUCLEAN);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const UNATCH: Self = Self(libc::EUNATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Self = Self(libc::EUSERS);
    pub const WOULDBLOCK: Self = Self(libc::EWOULDBLOCK);
    pub const XDEV: Self = Self(libc::EXDEV);
    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "macos",
        target_os = "wasi"
    )))]
    pub const XFULL: Self = Self(libc::EXFULL);
}

// These have type `u32` in the bindgen bindings; cast them to `u16` as
// knowledge that the platform errno type is signed is widespread.
#[cfg(linux_raw)]
impl Error {
    pub const ACCES: Self = Self(linux_raw_sys::errno::EACCES as u16);
    pub const ADDRINUSE: Self = Self(linux_raw_sys::errno::EADDRINUSE as u16);
    pub const ADDRNOTAVAIL: Self = Self(linux_raw_sys::errno::EADDRNOTAVAIL as u16);
    pub const ADV: Self = Self(linux_raw_sys::errno::EADV as u16);
    pub const AFNOSUPPORT: Self = Self(linux_raw_sys::errno::EAFNOSUPPORT as u16);
    pub const AGAIN: Self = Self(linux_raw_sys::errno::EAGAIN as u16);
    pub const ALREADY: Self = Self(linux_raw_sys::errno::EALREADY as u16);
    pub const BADE: Self = Self(linux_raw_sys::errno::EBADE as u16);
    pub const BADF: Self = Self(linux_raw_sys::errno::EBADF as u16);
    pub const BADFD: Self = Self(linux_raw_sys::errno::EBADFD as u16);
    pub const BADMSG: Self = Self(linux_raw_sys::errno::EBADMSG as u16);
    pub const BADR: Self = Self(linux_raw_sys::errno::EBADR as u16);
    pub const BADRQC: Self = Self(linux_raw_sys::errno::EBADRQC as u16);
    pub const BADSLT: Self = Self(linux_raw_sys::errno::EBADSLT as u16);
    pub const BFONT: Self = Self(linux_raw_sys::errno::EBFONT as u16);
    pub const BUSY: Self = Self(linux_raw_sys::errno::EBUSY as u16);
    pub const CANCELED: Self = Self(linux_raw_sys::errno::ECANCELED as u16);
    pub const CHILD: Self = Self(linux_raw_sys::errno::ECHILD as u16);
    pub const CHRNG: Self = Self(linux_raw_sys::errno::ECHRNG as u16);
    pub const COMM: Self = Self(linux_raw_sys::errno::ECOMM as u16);
    pub const CONNABORTED: Self = Self(linux_raw_sys::errno::ECONNABORTED as u16);
    pub const CONNREFUSED: Self = Self(linux_raw_sys::errno::ECONNREFUSED as u16);
    pub const CONNRESET: Self = Self(linux_raw_sys::errno::ECONNRESET as u16);
    pub const DEADLK: Self = Self(linux_raw_sys::errno::EDEADLK as u16);
    pub const DEADLOCK: Self = Self(linux_raw_sys::errno::EDEADLOCK as u16);
    pub const DESTADDRREQ: Self = Self(linux_raw_sys::errno::EDESTADDRREQ as u16);
    pub const DOM: Self = Self(linux_raw_sys::errno::EDOM as u16);
    pub const DOTDOT: Self = Self(linux_raw_sys::errno::EDOTDOT as u16);
    pub const DQUOT: Self = Self(linux_raw_sys::errno::EDQUOT as u16);
    pub const EXIST: Self = Self(linux_raw_sys::errno::EEXIST as u16);
    pub const FAULT: Self = Self(linux_raw_sys::errno::EFAULT as u16);
    pub const FBIG: Self = Self(linux_raw_sys::errno::EFBIG as u16);
    pub const HOSTDOWN: Self = Self(linux_raw_sys::errno::EHOSTDOWN as u16);
    pub const HOSTUNREACH: Self = Self(linux_raw_sys::errno::EHOSTUNREACH as u16);
    pub const HWPOISON: Self = Self(linux_raw_sys::v5_4::errno::EHWPOISON as u16);
    pub const IDRM: Self = Self(linux_raw_sys::errno::EIDRM as u16);
    pub const ILSEQ: Self = Self(linux_raw_sys::errno::EILSEQ as u16);
    pub const INPROGRESS: Self = Self(linux_raw_sys::errno::EINPROGRESS as u16);
    pub const INTR: Self = Self(linux_raw_sys::errno::EINTR as u16);
    pub const INVAL: Self = Self(linux_raw_sys::errno::EINVAL as u16);
    pub const IO: Self = Self(linux_raw_sys::errno::EIO as u16);
    pub const ISCONN: Self = Self(linux_raw_sys::errno::EISCONN as u16);
    pub const ISDIR: Self = Self(linux_raw_sys::errno::EISDIR as u16);
    pub const ISNAM: Self = Self(linux_raw_sys::errno::EISNAM as u16);
    pub const KEYEXPIRED: Self = Self(linux_raw_sys::errno::EKEYEXPIRED as u16);
    pub const KEYREJECTED: Self = Self(linux_raw_sys::errno::EKEYREJECTED as u16);
    pub const KEYREVOKED: Self = Self(linux_raw_sys::errno::EKEYREVOKED as u16);
    pub const L2HLT: Self = Self(linux_raw_sys::errno::EL2HLT as u16);
    pub const L2NSYNC: Self = Self(linux_raw_sys::errno::EL2NSYNC as u16);
    pub const L3HLT: Self = Self(linux_raw_sys::errno::EL3HLT as u16);
    pub const L3RST: Self = Self(linux_raw_sys::errno::EL3RST as u16);
    pub const LIBACC: Self = Self(linux_raw_sys::errno::ELIBACC as u16);
    pub const LIBBAD: Self = Self(linux_raw_sys::errno::ELIBBAD as u16);
    pub const LIBEXEC: Self = Self(linux_raw_sys::errno::ELIBEXEC as u16);
    pub const LIBMAX: Self = Self(linux_raw_sys::errno::ELIBMAX as u16);
    pub const LIBSCN: Self = Self(linux_raw_sys::errno::ELIBSCN as u16);
    pub const LNRNG: Self = Self(linux_raw_sys::errno::ELNRNG as u16);
    pub const LOOP: Self = Self(linux_raw_sys::errno::ELOOP as u16);
    pub const MEDIUMTYPE: Self = Self(linux_raw_sys::errno::EMEDIUMTYPE as u16);
    pub const MFILE: Self = Self(linux_raw_sys::errno::EMFILE as u16);
    pub const MLINK: Self = Self(linux_raw_sys::errno::EMLINK as u16);
    pub const MSGSIZE: Self = Self(linux_raw_sys::errno::EMSGSIZE as u16);
    pub const MULTIHOP: Self = Self(linux_raw_sys::errno::EMULTIHOP as u16);
    pub const NAMETOOLONG: Self = Self(linux_raw_sys::errno::ENAMETOOLONG as u16);
    pub const NAVAIL: Self = Self(linux_raw_sys::errno::ENAVAIL as u16);
    pub const NETDOWN: Self = Self(linux_raw_sys::errno::ENETDOWN as u16);
    pub const NETRESET: Self = Self(linux_raw_sys::errno::ENETRESET as u16);
    pub const NETUNREACH: Self = Self(linux_raw_sys::errno::ENETUNREACH as u16);
    pub const NFILE: Self = Self(linux_raw_sys::errno::ENFILE as u16);
    pub const NOANO: Self = Self(linux_raw_sys::errno::ENOANO as u16);
    pub const NOBUFS: Self = Self(linux_raw_sys::errno::ENOBUFS as u16);
    pub const NOCSI: Self = Self(linux_raw_sys::errno::ENOCSI as u16);
    pub const NODATA: Self = Self(linux_raw_sys::errno::ENODATA as u16);
    pub const NODEV: Self = Self(linux_raw_sys::errno::ENODEV as u16);
    pub const NOENT: Self = Self(linux_raw_sys::errno::ENOENT as u16);
    pub const NOEXEC: Self = Self(linux_raw_sys::errno::ENOEXEC as u16);
    pub const NOKEY: Self = Self(linux_raw_sys::errno::ENOKEY as u16);
    pub const NOLCK: Self = Self(linux_raw_sys::errno::ENOLCK as u16);
    pub const NOLINK: Self = Self(linux_raw_sys::errno::ENOLINK as u16);
    pub const NOMEDIUM: Self = Self(linux_raw_sys::errno::ENOMEDIUM as u16);
    pub const NOMEM: Self = Self(linux_raw_sys::errno::ENOMEM as u16);
    pub const NOMSG: Self = Self(linux_raw_sys::errno::ENOMSG as u16);
    pub const NONET: Self = Self(linux_raw_sys::errno::ENONET as u16);
    pub const NOPKG: Self = Self(linux_raw_sys::errno::ENOPKG as u16);
    pub const NOPROTOOPT: Self = Self(linux_raw_sys::errno::ENOPROTOOPT as u16);
    pub const NOSPC: Self = Self(linux_raw_sys::errno::ENOSPC as u16);
    pub const NOSR: Self = Self(linux_raw_sys::errno::ENOSR as u16);
    pub const NOSTR: Self = Self(linux_raw_sys::errno::ENOSTR as u16);
    pub const NOSYS: Self = Self(linux_raw_sys::errno::ENOSYS as u16);
    pub const NOTBLK: Self = Self(linux_raw_sys::errno::ENOTBLK as u16);
    pub const NOTCONN: Self = Self(linux_raw_sys::errno::ENOTCONN as u16);
    pub const NOTDIR: Self = Self(linux_raw_sys::errno::ENOTDIR as u16);
    pub const NOTEMPTY: Self = Self(linux_raw_sys::errno::ENOTEMPTY as u16);
    pub const NOTNAM: Self = Self(linux_raw_sys::errno::ENOTNAM as u16);
    pub const NOTRECOVERABLE: Self = Self(linux_raw_sys::errno::ENOTRECOVERABLE as u16);
    pub const NOTSOCK: Self = Self(linux_raw_sys::errno::ENOTSOCK as u16);
    // On Linux, `ENOTSUP` has the same value as `EOPNOTSUPP`.
    pub const NOTSUP: Self = Self(linux_raw_sys::errno::EOPNOTSUPP as u16);
    pub const NOTTY: Self = Self(linux_raw_sys::errno::ENOTTY as u16);
    pub const NOTUNIQ: Self = Self(linux_raw_sys::errno::ENOTUNIQ as u16);
    pub const NXIO: Self = Self(linux_raw_sys::errno::ENXIO as u16);
    pub const OPNOTSUPP: Self = Self(linux_raw_sys::errno::EOPNOTSUPP as u16);
    pub const OVERFLOW: Self = Self(linux_raw_sys::errno::EOVERFLOW as u16);
    pub const OWNERDEAD: Self = Self(linux_raw_sys::errno::EOWNERDEAD as u16);
    pub const PERM: Self = Self(linux_raw_sys::errno::EPERM as u16);
    pub const PFNOSUPPORT: Self = Self(linux_raw_sys::errno::EPFNOSUPPORT as u16);
    pub const PIPE: Self = Self(linux_raw_sys::errno::EPIPE as u16);
    pub const PROTO: Self = Self(linux_raw_sys::errno::EPROTO as u16);
    pub const PROTONOSUPPORT: Self = Self(linux_raw_sys::errno::EPROTONOSUPPORT as u16);
    pub const PROTOTYPE: Self = Self(linux_raw_sys::errno::EPROTOTYPE as u16);
    pub const RANGE: Self = Self(linux_raw_sys::errno::ERANGE as u16);
    pub const REMCHG: Self = Self(linux_raw_sys::errno::EREMCHG as u16);
    pub const REMOTE: Self = Self(linux_raw_sys::errno::EREMOTE as u16);
    pub const REMOTEIO: Self = Self(linux_raw_sys::errno::EREMOTEIO as u16);
    pub const RESTART: Self = Self(linux_raw_sys::errno::ERESTART as u16);
    pub const RFKILL: Self = Self(linux_raw_sys::errno::ERFKILL as u16);
    pub const ROFS: Self = Self(linux_raw_sys::errno::EROFS as u16);
    pub const SHUTDOWN: Self = Self(linux_raw_sys::errno::ESHUTDOWN as u16);
    pub const SOCKTNOSUPPORT: Self = Self(linux_raw_sys::errno::ESOCKTNOSUPPORT as u16);
    pub const SPIPE: Self = Self(linux_raw_sys::errno::ESPIPE as u16);
    pub const SRCH: Self = Self(linux_raw_sys::errno::ESRCH as u16);
    pub const SRMNT: Self = Self(linux_raw_sys::errno::ESRMNT as u16);
    pub const STALE: Self = Self(linux_raw_sys::errno::ESTALE as u16);
    pub const STRPIPE: Self = Self(linux_raw_sys::errno::ESTRPIPE as u16);
    pub const TIME: Self = Self(linux_raw_sys::errno::ETIME as u16);
    pub const TIMEDOUT: Self = Self(linux_raw_sys::errno::ETIMEDOUT as u16);
    pub const TOOBIG: Self = Self(linux_raw_sys::errno::E2BIG as u16);
    pub const TOOMANYREFS: Self = Self(linux_raw_sys::errno::ETOOMANYREFS as u16);
    pub const TXTBSY: Self = Self(linux_raw_sys::errno::ETXTBSY as u16);
    pub const UCLEAN: Self = Self(linux_raw_sys::errno::EUCLEAN as u16);
    pub const UNATCH: Self = Self(linux_raw_sys::errno::EUNATCH as u16);
    pub const USERS: Self = Self(linux_raw_sys::errno::EUSERS as u16);
    pub const WOULDBLOCK: Self = Self(linux_raw_sys::errno::EWOULDBLOCK as u16);
    pub const XDEV: Self = Self(linux_raw_sys::errno::EXDEV as u16);
    pub const XFULL: Self = Self(linux_raw_sys::errno::EXFULL as u16);
}

impl Error {
    /// Shorthand for `std::io::Error::from(self).kind()`.
    #[inline]
    pub fn kind(self) -> std::io::ErrorKind {
        std::io::Error::from(self).kind()
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        // This should be `i32::from` but that isn't a `const fn`.
        // Fortunately, we know `as i32` won't overflow.
        self.0 as i32
    }

    #[cfg(libc)]
    pub(crate) fn last_os_error() -> Self {
        Self(errno().0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl error::Error for Error {}

impl From<Error> for std::io::Error {
    #[inline]
    fn from(err: Error) -> Self {
        Self::from_raw_os_error(err.0 as _)
    }
}
