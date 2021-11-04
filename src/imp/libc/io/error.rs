//! The `rustix` `Error` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.

use super::super::c;
use errno::errno;

/// The error type for `rustix` APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Error(pub(crate) c::c_int);

impl Error {
    pub const ACCES: Self = Self(c::EACCES);
    pub const ADDRINUSE: Self = Self(c::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self(c::EADDRNOTAVAIL);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ADV: Self = Self(c::EADV);
    pub const AFNOSUPPORT: Self = Self(c::EAFNOSUPPORT);
    pub const AGAIN: Self = Self(c::EAGAIN);
    pub const ALREADY: Self = Self(c::EALREADY);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const AUTH: Self = Self(c::EAUTH);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADE: Self = Self(c::EBADE);
    pub const BADF: Self = Self(c::EBADF);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADFD: Self = Self(c::EBADFD);
    #[cfg(not(windows))]
    pub const BADMSG: Self = Self(c::EBADMSG);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADR: Self = Self(c::EBADR);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const BADRPC: Self = Self(c::EBADRPC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADRQC: Self = Self(c::EBADRQC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADSLT: Self = Self(c::EBADSLT);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BFONT: Self = Self(c::EBFONT);
    #[cfg(not(windows))]
    pub const BUSY: Self = Self(c::EBUSY);
    pub const CANCELED: Self = Self(c::ECANCELED);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const CAPMODE: Self = Self(c::ECAPMODE);
    #[cfg(not(windows))]
    pub const CHILD: Self = Self(c::ECHILD);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const CHRNG: Self = Self(c::ECHRNG);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const COMM: Self = Self(c::ECOMM);
    pub const CONNABORTED: Self = Self(c::ECONNABORTED);
    pub const CONNREFUSED: Self = Self(c::ECONNREFUSED);
    pub const CONNRESET: Self = Self(c::ECONNRESET);
    #[cfg(not(windows))]
    pub const DEADLK: Self = Self(c::EDEADLK);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "android",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DEADLOCK: Self = Self(c::EDEADLOCK);
    pub const DESTADDRREQ: Self = Self(c::EDESTADDRREQ);
    #[cfg(windows)]
    pub const DISCON: Self = Self(c::EDISCON);
    #[cfg(not(windows))]
    pub const DOM: Self = Self(c::EDOM);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const DOOFUS: Self = Self(c::EDOOFUS);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DOTDOT: Self = Self(c::EDOTDOT);
    pub const DQUOT: Self = Self(c::EDQUOT);
    #[cfg(not(windows))]
    pub const EXIST: Self = Self(c::EEXIST);
    pub const FAULT: Self = Self(c::EFAULT);
    #[cfg(not(windows))]
    pub const FBIG: Self = Self(c::EFBIG);
    #[cfg(any(
        target_env = "newlib",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const FTYPE: Self = Self(c::EFTYPE);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Self = Self(c::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self(c::EHOSTUNREACH);
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    )))]
    pub const HWPOISON: Self = Self(c::EHWPOISON);
    #[cfg(not(windows))]
    pub const IDRM: Self = Self(c::EIDRM);
    #[cfg(not(windows))]
    pub const ILSEQ: Self = Self(c::EILSEQ);
    pub const INPROGRESS: Self = Self(c::EINPROGRESS);
    pub const INTR: Self = Self(c::EINTR);
    pub const INVAL: Self = Self(c::EINVAL);
    #[cfg(windows)]
    pub const INVALIDPROCTABLE: Self = Self(c::EINVALIDPROCTABLE);
    #[cfg(windows)]
    pub const INVALIDPROVIDER: Self = Self(c::EINVALIDPROVIDER);
    #[cfg(not(windows))]
    pub const IO: Self = Self(c::EIO);
    pub const ISCONN: Self = Self(c::EISCONN);
    #[cfg(not(windows))]
    pub const ISDIR: Self = Self(c::EISDIR);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ISNAM: Self = Self(c::EISNAM);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYEXPIRED: Self = Self(c::EKEYEXPIRED);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREJECTED: Self = Self(c::EKEYREJECTED);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREVOKED: Self = Self(c::EKEYREVOKED);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2HLT: Self = Self(c::EL2HLT);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2NSYNC: Self = Self(c::EL2NSYNC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3HLT: Self = Self(c::EL3HLT);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3RST: Self = Self(c::EL3RST);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBACC: Self = Self(c::ELIBACC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBBAD: Self = Self(c::ELIBBAD);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBEXEC: Self = Self(c::ELIBEXEC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBMAX: Self = Self(c::ELIBMAX);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBSCN: Self = Self(c::ELIBSCN);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LNRNG: Self = Self(c::ELNRNG);
    pub const LOOP: Self = Self(c::ELOOP);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const MEDIUMTYPE: Self = Self(c::EMEDIUMTYPE);
    pub const MFILE: Self = Self(c::EMFILE);
    #[cfg(not(windows))]
    pub const MLINK: Self = Self(c::EMLINK);
    pub const MSGSIZE: Self = Self(c::EMSGSIZE);
    #[cfg(not(any(windows, target_os = "openbsd")))]
    pub const MULTIHOP: Self = Self(c::EMULTIHOP);
    pub const NAMETOOLONG: Self = Self(c::ENAMETOOLONG);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NAVAIL: Self = Self(c::ENAVAIL);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NEEDAUTH: Self = Self(c::ENEEDAUTH);
    pub const NETDOWN: Self = Self(c::ENETDOWN);
    pub const NETRESET: Self = Self(c::ENETRESET);
    pub const NETUNREACH: Self = Self(c::ENETUNREACH);
    #[cfg(not(windows))]
    pub const NFILE: Self = Self(c::ENFILE);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOANO: Self = Self(c::ENOANO);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NOATTR: Self = Self(c::ENOATTR);
    pub const NOBUFS: Self = Self(c::ENOBUFS);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOCSI: Self = Self(c::ENOCSI);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NODATA: Self = Self(c::ENODATA);
    #[cfg(not(windows))]
    pub const NODEV: Self = Self(c::ENODEV);
    #[cfg(not(windows))]
    pub const NOENT: Self = Self(c::ENOENT);
    #[cfg(not(windows))]
    pub const NOEXEC: Self = Self(c::ENOEXEC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOKEY: Self = Self(c::ENOKEY);
    #[cfg(not(windows))]
    pub const NOLCK: Self = Self(c::ENOLCK);
    #[cfg(not(any(windows, target_os = "openbsd")))]
    pub const NOLINK: Self = Self(c::ENOLINK);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOMEDIUM: Self = Self(c::ENOMEDIUM);
    #[cfg(not(windows))]
    pub const NOMEM: Self = Self(c::ENOMEM);
    #[cfg(windows)]
    pub const NOMORE: Self = Self(c::ENOMORE);
    #[cfg(not(windows))]
    pub const NOMSG: Self = Self(c::ENOMSG);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NONET: Self = Self(c::ENONET);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOPKG: Self = Self(c::ENOPKG);
    pub const NOPROTOOPT: Self = Self(c::ENOPROTOOPT);
    #[cfg(not(windows))]
    pub const NOSPC: Self = Self(c::ENOSPC);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSR: Self = Self(c::ENOSR);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSTR: Self = Self(c::ENOSTR);
    #[cfg(not(windows))]
    pub const NOSYS: Self = Self(c::ENOSYS);
    #[cfg(not(any(windows, target_os = "wasi")))]
    pub const NOTBLK: Self = Self(c::ENOTBLK);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const NOTCAPABLE: Self = Self(c::ENOTCAPABLE);
    pub const NOTCONN: Self = Self(c::ENOTCONN);
    #[cfg(not(windows))]
    pub const NOTDIR: Self = Self(c::ENOTDIR);
    pub const NOTEMPTY: Self = Self(c::ENOTEMPTY);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOTNAM: Self = Self(c::ENOTNAM);
    #[cfg(not(any(windows, target_os = "netbsd")))]
    pub const NOTRECOVERABLE: Self = Self(c::ENOTRECOVERABLE);
    pub const NOTSOCK: Self = Self(c::ENOTSOCK);
    #[cfg(not(any(windows, target_os = "redox")))]
    pub const NOTSUP: Self = Self(c::ENOTSUP);
    #[cfg(not(windows))]
    pub const NOTTY: Self = Self(c::ENOTTY);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOTUNIQ: Self = Self(c::ENOTUNIQ);
    #[cfg(not(windows))]
    pub const NXIO: Self = Self(c::ENXIO);
    pub const OPNOTSUPP: Self = Self(c::EOPNOTSUPP);
    #[cfg(not(windows))]
    pub const OVERFLOW: Self = Self(c::EOVERFLOW);
    #[cfg(not(any(windows, target_os = "netbsd")))]
    pub const OWNERDEAD: Self = Self(c::EOWNERDEAD);
    #[cfg(not(windows))]
    pub const PERM: Self = Self(c::EPERM);
    #[cfg(not(target_os = "wasi"))]
    pub const PFNOSUPPORT: Self = Self(c::EPFNOSUPPORT);
    #[cfg(not(windows))]
    pub const PIPE: Self = Self(c::EPIPE);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCLIM: Self = Self(c::EPROCLIM);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCUNAVAIL: Self = Self(c::EPROCUNAVAIL);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGMISMATCH: Self = Self(c::EPROGMISMATCH);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGUNAVAIL: Self = Self(c::EPROGUNAVAIL);
    #[cfg(not(windows))]
    pub const PROTO: Self = Self(c::EPROTO);
    pub const PROTONOSUPPORT: Self = Self(c::EPROTONOSUPPORT);
    pub const PROTOTYPE: Self = Self(c::EPROTOTYPE);
    #[cfg(windows)]
    pub const PROVIDERFAILEDINIT: Self = Self(c::EPROVIDERFAILEDINIT);
    #[cfg(not(windows))]
    pub const RANGE: Self = Self(c::ERANGE);
    #[cfg(windows)]
    pub const REFUSED: Self = Self(c::EREFUSED);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMCHG: Self = Self(c::EREMCHG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Self = Self(c::EREMOTE);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMOTEIO: Self = Self(c::EREMOTEIO);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const RESTART: Self = Self(c::ERESTART);
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    )))]
    pub const RFKILL: Self = Self(c::ERFKILL);
    #[cfg(not(windows))]
    pub const ROFS: Self = Self(c::EROFS);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const RPCMISMATCH: Self = Self(c::ERPCMISMATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const SHUTDOWN: Self = Self(c::ESHUTDOWN);
    #[cfg(not(target_os = "wasi"))]
    pub const SOCKTNOSUPPORT: Self = Self(c::ESOCKTNOSUPPORT);
    #[cfg(not(windows))]
    pub const SPIPE: Self = Self(c::ESPIPE);
    #[cfg(not(windows))]
    pub const SRCH: Self = Self(c::ESRCH);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const SRMNT: Self = Self(c::ESRMNT);
    pub const STALE: Self = Self(c::ESTALE);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const STRPIPE: Self = Self(c::ESTRPIPE);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const TIME: Self = Self(c::ETIME);
    pub const TIMEDOUT: Self = Self(c::ETIMEDOUT);
    #[cfg(not(windows))]
    pub const TOOBIG: Self = Self(c::E2BIG);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Self = Self(c::ETOOMANYREFS);
    #[cfg(not(windows))]
    pub const TXTBSY: Self = Self(c::ETXTBSY);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UCLEAN: Self = Self(c::EUCLEAN);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UNATCH: Self = Self(c::EUNATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Self = Self(c::EUSERS);
    pub const WOULDBLOCK: Self = Self(c::EWOULDBLOCK);
    #[cfg(not(windows))]
    pub const XDEV: Self = Self(c::EXDEV);
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const XFULL: Self = Self(c::EXFULL);
}

impl Error {
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        io_err
            .raw_os_error()
            .and_then(|raw| if raw != 0 { Some(Self(raw)) } else { None })
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        self.0
    }

    /// Construct an `Error` from a raw OS error number.
    #[inline]
    pub const fn from_raw_os_error(raw: i32) -> Self {
        Self(raw)
    }

    pub(crate) fn last_os_error() -> Self {
        Self(errno().0)
    }
}
