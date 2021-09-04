//! The rsix `Error` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.

use errno::errno;
use libc::c_int;

/// The error type for rsix APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Error(pub(crate) c_int);

impl Error {
    pub const ACCES: Self = Self(libc::EACCES);
    pub const ADDRINUSE: Self = Self(libc::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self(libc::EADDRNOTAVAIL);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ADV: Self = Self(libc::EADV);
    pub const AFNOSUPPORT: Self = Self(libc::EAFNOSUPPORT);
    pub const AGAIN: Self = Self(libc::EAGAIN);
    pub const ALREADY: Self = Self(libc::EALREADY);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const AUTH: Self = Self(libc::EAUTH);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADE: Self = Self(libc::EBADE);
    pub const BADF: Self = Self(libc::EBADF);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADFD: Self = Self(libc::EBADFD);
    pub const BADMSG: Self = Self(libc::EBADMSG);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADR: Self = Self(libc::EBADR);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const BADRPC: Self = Self(libc::EBADRPC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADRQC: Self = Self(libc::EBADRQC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADSLT: Self = Self(libc::EBADSLT);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BFONT: Self = Self(libc::EBFONT);
    pub const BUSY: Self = Self(libc::EBUSY);
    pub const CANCELED: Self = Self(libc::ECANCELED);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const CAPMODE: Self = Self(libc::ECAPMODE);
    pub const CHILD: Self = Self(libc::ECHILD);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const CHRNG: Self = Self(libc::ECHRNG);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const COMM: Self = Self(libc::ECOMM);
    pub const CONNABORTED: Self = Self(libc::ECONNABORTED);
    pub const CONNREFUSED: Self = Self(libc::ECONNREFUSED);
    pub const CONNRESET: Self = Self(libc::ECONNRESET);
    pub const DEADLK: Self = Self(libc::EDEADLK);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "android",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DEADLOCK: Self = Self(libc::EDEADLOCK);
    pub const DESTADDRREQ: Self = Self(libc::EDESTADDRREQ);
    pub const DOM: Self = Self(libc::EDOM);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const DOOFUS: Self = Self(libc::EDOOFUS);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DOTDOT: Self = Self(libc::EDOTDOT);
    pub const DQUOT: Self = Self(libc::EDQUOT);
    pub const EXIST: Self = Self(libc::EEXIST);
    pub const FAULT: Self = Self(libc::EFAULT);
    pub const FBIG: Self = Self(libc::EFBIG);
    #[cfg(any(
        target_env = "newlib",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const FTYPE: Self = Self(libc::EFTYPE);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Self = Self(libc::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self(libc::EHOSTUNREACH);
    #[cfg(not(any(
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
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ISNAM: Self = Self(libc::EISNAM);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYEXPIRED: Self = Self(libc::EKEYEXPIRED);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREJECTED: Self = Self(libc::EKEYREJECTED);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREVOKED: Self = Self(libc::EKEYREVOKED);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2HLT: Self = Self(libc::EL2HLT);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2NSYNC: Self = Self(libc::EL2NSYNC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3HLT: Self = Self(libc::EL3HLT);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3RST: Self = Self(libc::EL3RST);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBACC: Self = Self(libc::ELIBACC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBBAD: Self = Self(libc::ELIBBAD);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBEXEC: Self = Self(libc::ELIBEXEC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBMAX: Self = Self(libc::ELIBMAX);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBSCN: Self = Self(libc::ELIBSCN);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LNRNG: Self = Self(libc::ELNRNG);
    pub const LOOP: Self = Self(libc::ELOOP);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const MEDIUMTYPE: Self = Self(libc::EMEDIUMTYPE);
    pub const MFILE: Self = Self(libc::EMFILE);
    pub const MLINK: Self = Self(libc::EMLINK);
    pub const MSGSIZE: Self = Self(libc::EMSGSIZE);
    #[cfg(not(target_os = "openbsd"))]
    pub const MULTIHOP: Self = Self(libc::EMULTIHOP);
    pub const NAMETOOLONG: Self = Self(libc::ENAMETOOLONG);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NAVAIL: Self = Self(libc::ENAVAIL);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NEEDAUTH: Self = Self(libc::ENEEDAUTH);
    pub const NETDOWN: Self = Self(libc::ENETDOWN);
    pub const NETRESET: Self = Self(libc::ENETRESET);
    pub const NETUNREACH: Self = Self(libc::ENETUNREACH);
    pub const NFILE: Self = Self(libc::ENFILE);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOANO: Self = Self(libc::ENOANO);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NOATTR: Self = Self(libc::ENOATTR);
    pub const NOBUFS: Self = Self(libc::ENOBUFS);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOCSI: Self = Self(libc::ENOCSI);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NODATA: Self = Self(libc::ENODATA);
    pub const NODEV: Self = Self(libc::ENODEV);
    pub const NOENT: Self = Self(libc::ENOENT);
    pub const NOEXEC: Self = Self(libc::ENOEXEC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOKEY: Self = Self(libc::ENOKEY);
    pub const NOLCK: Self = Self(libc::ENOLCK);
    #[cfg(not(target_os = "openbsd"))]
    pub const NOLINK: Self = Self(libc::ENOLINK);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOMEDIUM: Self = Self(libc::ENOMEDIUM);
    pub const NOMEM: Self = Self(libc::ENOMEM);
    pub const NOMSG: Self = Self(libc::ENOMSG);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NONET: Self = Self(libc::ENONET);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOPKG: Self = Self(libc::ENOPKG);
    pub const NOPROTOOPT: Self = Self(libc::ENOPROTOOPT);
    pub const NOSPC: Self = Self(libc::ENOSPC);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSR: Self = Self(libc::ENOSR);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSTR: Self = Self(libc::ENOSTR);
    pub const NOSYS: Self = Self(libc::ENOSYS);
    #[cfg(not(target_os = "wasi"))]
    pub const NOTBLK: Self = Self(libc::ENOTBLK);
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const NOTCAPABLE: Self = Self(libc::ENOTCAPABLE);
    pub const NOTCONN: Self = Self(libc::ENOTCONN);
    pub const NOTDIR: Self = Self(libc::ENOTDIR);
    pub const NOTEMPTY: Self = Self(libc::ENOTEMPTY);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOTNAM: Self = Self(libc::ENOTNAM);
    #[cfg(not(target_os = "netbsd"))]
    pub const NOTRECOVERABLE: Self = Self(libc::ENOTRECOVERABLE);
    pub const NOTSOCK: Self = Self(libc::ENOTSOCK);
    #[cfg(not(target_os = "redox"))]
    pub const NOTSUP: Self = Self(libc::ENOTSUP);
    pub const NOTTY: Self = Self(libc::ENOTTY);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
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
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCLIM: Self = Self(libc::EPROCLIM);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCUNAVAIL: Self = Self(libc::EPROCUNAVAIL);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGMISMATCH: Self = Self(libc::EPROGMISMATCH);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGUNAVAIL: Self = Self(libc::EPROGUNAVAIL);
    pub const PROTO: Self = Self(libc::EPROTO);
    pub const PROTONOSUPPORT: Self = Self(libc::EPROTONOSUPPORT);
    pub const PROTOTYPE: Self = Self(libc::EPROTOTYPE);
    pub const RANGE: Self = Self(libc::ERANGE);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMCHG: Self = Self(libc::EREMCHG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Self = Self(libc::EREMOTE);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMOTEIO: Self = Self(libc::EREMOTEIO);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const RESTART: Self = Self(libc::ERESTART);
    #[cfg(not(any(
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
    pub const RFKILL: Self = Self(libc::ERFKILL);
    pub const ROFS: Self = Self(libc::EROFS);
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
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
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const SRMNT: Self = Self(libc::ESRMNT);
    pub const STALE: Self = Self(libc::ESTALE);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const STRPIPE: Self = Self(libc::ESTRPIPE);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const TIME: Self = Self(libc::ETIME);
    pub const TIMEDOUT: Self = Self(libc::ETIMEDOUT);
    pub const TOOBIG: Self = Self(libc::E2BIG);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Self = Self(libc::ETOOMANYREFS);
    pub const TXTBSY: Self = Self(libc::ETXTBSY);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UCLEAN: Self = Self(libc::EUCLEAN);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UNATCH: Self = Self(libc::EUNATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Self = Self(libc::EUSERS);
    pub const WOULDBLOCK: Self = Self(libc::EWOULDBLOCK);
    pub const XDEV: Self = Self(libc::EXDEV);
    #[cfg(not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const XFULL: Self = Self(libc::EXFULL);
}

impl Error {
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
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

    pub(crate) fn last_os_error() -> Self {
        Self(errno().0)
    }
}
