#![allow(missing_docs)]

use std::{io, os::raw::c_int};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(c_int);

#[cfg(libc)]
impl Errno {
    pub const ACCES: Self = Self(libc::EACCES);
    pub const ADDRINUSE: Self = Self(libc::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self(libc::EADDRNOTAVAIL);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADE: Self = Self(libc::EBADE);
    pub const BADF: Self = Self(libc::EBADF);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADFD: Self = Self(libc::EBADFD);
    pub const BADMSG: Self = Self(libc::EBADMSG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADRQC: Self = Self(libc::EBADRQC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BADSLT: Self = Self(libc::EBADSLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const BFONT: Self = Self(libc::EBFONT);
    pub const BUSY: Self = Self(libc::EBUSY);
    pub const CANCELED: Self = Self(libc::ECANCELED);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const CAPMODE: Self = Self(libc::ECAPMODE);
    pub const CHILD: Self = Self(libc::ECHILD);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const CHRNG: Self = Self(libc::ECHRNG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const COMM: Self = Self(libc::ECOMM);
    pub const CONNABORTED: Self = Self(libc::ECONNABORTED);
    pub const CONNREFUSED: Self = Self(libc::ECONNREFUSED);
    pub const CONNRESET: Self = Self(libc::ECONNRESET);
    pub const DEADLK: Self = Self(libc::EDEADLK);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DEADLOCK: Self = Self(libc::EDEADLOCK);
    pub const DESTADDRREQ: Self = Self(libc::EDESTADDRREQ);
    pub const DOM: Self = Self(libc::EDOM);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const DOOFUS: Self = Self(libc::EDOOFUS);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    #[cfg(not(target_os = "redox"))]
    pub const HWPOISON: Self = Self(libc::EHWPOISON);
    pub const IDRM: Self = Self(libc::EIDRM);
    pub const ILSEQ: Self = Self(libc::EILSEQ);
    pub const INPROGRESS: Self = Self(libc::EINPROGRESS);
    pub const INTR: Self = Self(libc::EINTR);
    pub const INVAL: Self = Self(libc::EINVAL);
    pub const IO: Self = Self(libc::EIO);
    pub const ISCONN: Self = Self(libc::EISCONN);
    pub const ISDIR: Self = Self(libc::EISDIR);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const ISNAM: Self = Self(libc::EISNAM);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYEXPIRED: Self = Self(libc::EKEYEXPIRED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYREJECTED: Self = Self(libc::EKEYREJECTED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    #[cfg(not(target_os = "wasi"))]
    pub const KEYREVOKED: Self = Self(libc::EKEYREVOKED);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L2HLT: Self = Self(libc::EL2HLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L2NSYNC: Self = Self(libc::EL2NSYNC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L3HLT: Self = Self(libc::EL3HLT);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const L3RST: Self = Self(libc::EL3RST);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBACC: Self = Self(libc::ELIBACC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBBAD: Self = Self(libc::ELIBBAD);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBEXEC: Self = Self(libc::ELIBEXEC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBMAX: Self = Self(libc::ELIBMAX);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LIBSCN: Self = Self(libc::ELIBSCN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const LNRNG: Self = Self(libc::ELNRNG);
    pub const LOOP: Self = Self(libc::ELOOP);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const MEDIUMTYPE: Self = Self(libc::EMEDIUMTYPE);
    pub const MFILE: Self = Self(libc::EMFILE);
    pub const MLINK: Self = Self(libc::EMLINK);
    pub const MSGSIZE: Self = Self(libc::EMSGSIZE);
    pub const MULTIHOP: Self = Self(libc::EMULTIHOP);
    pub const NAMETOOLONG: Self = Self(libc::ENAMETOOLONG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOCSI: Self = Self(libc::ENOCSI);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NODATA: Self = Self(libc::ENODATA);
    pub const NODEV: Self = Self(libc::ENODEV);
    pub const NOENT: Self = Self(libc::ENOENT);
    pub const NOEXEC: Self = Self(libc::ENOEXEC);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOKEY: Self = Self(libc::ENOKEY);
    pub const NOLCK: Self = Self(libc::ENOLCK);
    pub const NOLINK: Self = Self(libc::ENOLINK);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOMEDIUM: Self = Self(libc::ENOMEDIUM);
    pub const NOMEM: Self = Self(libc::ENOMEM);
    pub const NOMSG: Self = Self(libc::ENOMSG);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NONET: Self = Self(libc::ENONET);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOPKG: Self = Self(libc::ENOPKG);
    pub const NOPROTOOPT: Self = Self(libc::ENOPROTOOPT);
    pub const NOSPC: Self = Self(libc::ENOSPC);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSR: Self = Self(libc::ENOSR);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOSTR: Self = Self(libc::ENOSTR);
    pub const NOSYS: Self = Self(libc::ENOSYS);
    #[cfg(not(target_os = "wasi"))]
    pub const NOTBLK: Self = Self(libc::ENOTBLK);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    pub const NOTCAPABLE: Self = Self(libc::ENOTCAPABLE);
    pub const NOTCONN: Self = Self(libc::ENOTCONN);
    pub const NOTDIR: Self = Self(libc::ENOTDIR);
    pub const NOTEMPTY: Self = Self(libc::ENOTEMPTY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOTNAM: Self = Self(libc::ENOTNAM);
    #[cfg(not(target_os = "netbsd"))]
    pub const NOTRECOVERABLE: Self = Self(libc::ENOTRECOVERABLE);
    pub const NOTSOCK: Self = Self(libc::ENOTSOCK);
    #[cfg(not(target_os = "redox"))]
    pub const NOTSUP: Self = Self(libc::ENOTSUP);
    pub const NOTTY: Self = Self(libc::ENOTTY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMCHG: Self = Self(libc::EREMCHG);
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Self = Self(libc::EREMOTE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTEIO: Self = Self(libc::EREMOTEIO);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const RESTART: Self = Self(libc::ERESTART);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    #[cfg(not(target_os = "redox"))]
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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const SRMNT: Self = Self(libc::ESRMNT);
    pub const STALE: Self = Self(libc::ESTALE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const STRPIPE: Self = Self(libc::ESTRPIPE);
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(target_os = "wasi"))]
    pub const TIME: Self = Self(libc::ETIME);
    pub const TIMEDOUT: Self = Self(libc::ETIMEDOUT);
    pub const TOOBIG: Self = Self(libc::E2BIG);
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Self = Self(libc::ETOOMANYREFS);
    pub const TXTBSY: Self = Self(libc::ETXTBSY);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UCLEAN: Self = Self(libc::EUCLEAN);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const UNATCH: Self = Self(libc::EUNATCH);
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Self = Self(libc::EUSERS);
    pub const WOULDBLOCK: Self = Self(libc::EWOULDBLOCK);
    pub const XDEV: Self = Self(libc::EXDEV);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const XFULL: Self = Self(libc::EXFULL);
}

#[cfg(linux_raw)]
impl Errno {
    pub const ACCES: Self = Self(linux_raw_sys::errno::EACCES as c_int);
    pub const ADDRINUSE: Self = Self(linux_raw_sys::errno::EADDRINUSE as c_int);
    pub const ADDRNOTAVAIL: Self = Self(linux_raw_sys::errno::EADDRNOTAVAIL as c_int);
    pub const ADV: Self = Self(linux_raw_sys::errno::EADV as c_int);
    pub const AFNOSUPPORT: Self = Self(linux_raw_sys::errno::EAFNOSUPPORT as c_int);
    pub const AGAIN: Self = Self(linux_raw_sys::errno::EAGAIN as c_int);
    pub const ALREADY: Self = Self(linux_raw_sys::errno::EALREADY as c_int);
    pub const BADE: Self = Self(linux_raw_sys::errno::EBADE as c_int);
    pub const BADF: Self = Self(linux_raw_sys::errno::EBADF as c_int);
    pub const BADFD: Self = Self(linux_raw_sys::errno::EBADFD as c_int);
    pub const BADMSG: Self = Self(linux_raw_sys::errno::EBADMSG as c_int);
    pub const BADR: Self = Self(linux_raw_sys::errno::EBADR as c_int);
    pub const BADRQC: Self = Self(linux_raw_sys::errno::EBADRQC as c_int);
    pub const BADSLT: Self = Self(linux_raw_sys::errno::EBADSLT as c_int);
    pub const BFONT: Self = Self(linux_raw_sys::errno::EBFONT as c_int);
    pub const BUSY: Self = Self(linux_raw_sys::errno::EBUSY as c_int);
    pub const CANCELED: Self = Self(linux_raw_sys::errno::ECANCELED as c_int);
    pub const CHILD: Self = Self(linux_raw_sys::errno::ECHILD as c_int);
    pub const CHRNG: Self = Self(linux_raw_sys::errno::ECHRNG as c_int);
    pub const COMM: Self = Self(linux_raw_sys::errno::ECOMM as c_int);
    pub const CONNABORTED: Self = Self(linux_raw_sys::errno::ECONNABORTED as c_int);
    pub const CONNREFUSED: Self = Self(linux_raw_sys::errno::ECONNREFUSED as c_int);
    pub const CONNRESET: Self = Self(linux_raw_sys::errno::ECONNRESET as c_int);
    pub const DEADLK: Self = Self(linux_raw_sys::errno::EDEADLK as c_int);
    pub const DEADLOCK: Self = Self(linux_raw_sys::errno::EDEADLOCK as c_int);
    pub const DESTADDRREQ: Self = Self(linux_raw_sys::errno::EDESTADDRREQ as c_int);
    pub const DOM: Self = Self(linux_raw_sys::errno::EDOM as c_int);
    pub const DOTDOT: Self = Self(linux_raw_sys::errno::EDOTDOT as c_int);
    pub const DQUOT: Self = Self(linux_raw_sys::errno::EDQUOT as c_int);
    pub const EXIST: Self = Self(linux_raw_sys::errno::EEXIST as c_int);
    pub const FAULT: Self = Self(linux_raw_sys::errno::EFAULT as c_int);
    pub const FBIG: Self = Self(linux_raw_sys::errno::EFBIG as c_int);
    pub const HOSTDOWN: Self = Self(linux_raw_sys::errno::EHOSTDOWN as c_int);
    pub const HOSTUNREACH: Self = Self(linux_raw_sys::errno::EHOSTUNREACH as c_int);
    pub const HWPOISON: Self = Self(linux_raw_sys::v5_4::errno::EHWPOISON as c_int);
    pub const IDRM: Self = Self(linux_raw_sys::errno::EIDRM as c_int);
    pub const ILSEQ: Self = Self(linux_raw_sys::errno::EILSEQ as c_int);
    pub const INPROGRESS: Self = Self(linux_raw_sys::errno::EINPROGRESS as c_int);
    pub const INTR: Self = Self(linux_raw_sys::errno::EINTR as c_int);
    pub const INVAL: Self = Self(linux_raw_sys::errno::EINVAL as c_int);
    pub const IO: Self = Self(linux_raw_sys::errno::EIO as c_int);
    pub const ISCONN: Self = Self(linux_raw_sys::errno::EISCONN as c_int);
    pub const ISDIR: Self = Self(linux_raw_sys::errno::EISDIR as c_int);
    pub const ISNAM: Self = Self(linux_raw_sys::errno::EISNAM as c_int);
    pub const KEYEXPIRED: Self = Self(linux_raw_sys::errno::EKEYEXPIRED as c_int);
    pub const KEYREJECTED: Self = Self(linux_raw_sys::errno::EKEYREJECTED as c_int);
    pub const KEYREVOKED: Self = Self(linux_raw_sys::errno::EKEYREVOKED as c_int);
    pub const L2HLT: Self = Self(linux_raw_sys::errno::EL2HLT as c_int);
    pub const L2NSYNC: Self = Self(linux_raw_sys::errno::EL2NSYNC as c_int);
    pub const L3HLT: Self = Self(linux_raw_sys::errno::EL3HLT as c_int);
    pub const L3RST: Self = Self(linux_raw_sys::errno::EL3RST as c_int);
    pub const LIBACC: Self = Self(linux_raw_sys::errno::ELIBACC as c_int);
    pub const LIBBAD: Self = Self(linux_raw_sys::errno::ELIBBAD as c_int);
    pub const LIBEXEC: Self = Self(linux_raw_sys::errno::ELIBEXEC as c_int);
    pub const LIBMAX: Self = Self(linux_raw_sys::errno::ELIBMAX as c_int);
    pub const LIBSCN: Self = Self(linux_raw_sys::errno::ELIBSCN as c_int);
    pub const LNRNG: Self = Self(linux_raw_sys::errno::ELNRNG as c_int);
    pub const LOOP: Self = Self(linux_raw_sys::errno::ELOOP as c_int);
    pub const MEDIUMTYPE: Self = Self(linux_raw_sys::errno::EMEDIUMTYPE as c_int);
    pub const MFILE: Self = Self(linux_raw_sys::errno::EMFILE as c_int);
    pub const MLINK: Self = Self(linux_raw_sys::errno::EMLINK as c_int);
    pub const MSGSIZE: Self = Self(linux_raw_sys::errno::EMSGSIZE as c_int);
    pub const MULTIHOP: Self = Self(linux_raw_sys::errno::EMULTIHOP as c_int);
    pub const NAMETOOLONG: Self = Self(linux_raw_sys::errno::ENAMETOOLONG as c_int);
    pub const NAVAIL: Self = Self(linux_raw_sys::errno::ENAVAIL as c_int);
    pub const NETDOWN: Self = Self(linux_raw_sys::errno::ENETDOWN as c_int);
    pub const NETRESET: Self = Self(linux_raw_sys::errno::ENETRESET as c_int);
    pub const NETUNREACH: Self = Self(linux_raw_sys::errno::ENETUNREACH as c_int);
    pub const NFILE: Self = Self(linux_raw_sys::errno::ENFILE as c_int);
    pub const NOANO: Self = Self(linux_raw_sys::errno::ENOANO as c_int);
    pub const NOBUFS: Self = Self(linux_raw_sys::errno::ENOBUFS as c_int);
    pub const NOCSI: Self = Self(linux_raw_sys::errno::ENOCSI as c_int);
    pub const NODATA: Self = Self(linux_raw_sys::errno::ENODATA as c_int);
    pub const NODEV: Self = Self(linux_raw_sys::errno::ENODEV as c_int);
    pub const NOENT: Self = Self(linux_raw_sys::errno::ENOENT as c_int);
    pub const NOEXEC: Self = Self(linux_raw_sys::errno::ENOEXEC as c_int);
    pub const NOKEY: Self = Self(linux_raw_sys::errno::ENOKEY as c_int);
    pub const NOLCK: Self = Self(linux_raw_sys::errno::ENOLCK as c_int);
    pub const NOLINK: Self = Self(linux_raw_sys::errno::ENOLINK as c_int);
    pub const NOMEDIUM: Self = Self(linux_raw_sys::errno::ENOMEDIUM as c_int);
    pub const NOMEM: Self = Self(linux_raw_sys::errno::ENOMEM as c_int);
    pub const NOMSG: Self = Self(linux_raw_sys::errno::ENOMSG as c_int);
    pub const NONET: Self = Self(linux_raw_sys::errno::ENONET as c_int);
    pub const NOPKG: Self = Self(linux_raw_sys::errno::ENOPKG as c_int);
    pub const NOPROTOOPT: Self = Self(linux_raw_sys::errno::ENOPROTOOPT as c_int);
    pub const NOSPC: Self = Self(linux_raw_sys::errno::ENOSPC as c_int);
    pub const NOSR: Self = Self(linux_raw_sys::errno::ENOSR as c_int);
    pub const NOSTR: Self = Self(linux_raw_sys::errno::ENOSTR as c_int);
    pub const NOSYS: Self = Self(linux_raw_sys::errno::ENOSYS as c_int);
    pub const NOTBLK: Self = Self(linux_raw_sys::errno::ENOTBLK as c_int);
    pub const NOTCONN: Self = Self(linux_raw_sys::errno::ENOTCONN as c_int);
    pub const NOTDIR: Self = Self(linux_raw_sys::errno::ENOTDIR as c_int);
    pub const NOTEMPTY: Self = Self(linux_raw_sys::errno::ENOTEMPTY as c_int);
    pub const NOTNAM: Self = Self(linux_raw_sys::errno::ENOTNAM as c_int);
    pub const NOTRECOVERABLE: Self = Self(linux_raw_sys::errno::ENOTRECOVERABLE as c_int);
    pub const NOTSOCK: Self = Self(linux_raw_sys::errno::ENOTSOCK as c_int);
    // On Linux, `ENOTSUP` has the same value as `EOPNOTSUPP`.
    pub const NOTSUP: Self = Self(linux_raw_sys::errno::EOPNOTSUPP as c_int);
    pub const NOTTY: Self = Self(linux_raw_sys::errno::ENOTTY as c_int);
    pub const NOTUNIQ: Self = Self(linux_raw_sys::errno::ENOTUNIQ as c_int);
    pub const NXIO: Self = Self(linux_raw_sys::errno::ENXIO as c_int);
    pub const OPNOTSUPP: Self = Self(linux_raw_sys::errno::EOPNOTSUPP as c_int);
    pub const OVERFLOW: Self = Self(linux_raw_sys::errno::EOVERFLOW as c_int);
    pub const OWNERDEAD: Self = Self(linux_raw_sys::errno::EOWNERDEAD as c_int);
    // These have type `u32` in the bindgen bindings; cast them to `c_int` as
    // knowledge that the platform errno type is signed is widespread.
    pub const PERM: Self = Self(linux_raw_sys::errno::EPERM as c_int);
    pub const PFNOSUPPORT: Self = Self(linux_raw_sys::errno::EPFNOSUPPORT as c_int);
    pub const PIPE: Self = Self(linux_raw_sys::errno::EPIPE as c_int);
    pub const PROTO: Self = Self(linux_raw_sys::errno::EPROTO as c_int);
    pub const PROTONOSUPPORT: Self = Self(linux_raw_sys::errno::EPROTONOSUPPORT as c_int);
    pub const PROTOTYPE: Self = Self(linux_raw_sys::errno::EPROTOTYPE as c_int);
    pub const RANGE: Self = Self(linux_raw_sys::errno::ERANGE as c_int);
    pub const REMCHG: Self = Self(linux_raw_sys::errno::EREMCHG as c_int);
    pub const REMOTE: Self = Self(linux_raw_sys::errno::EREMOTE as c_int);
    pub const REMOTEIO: Self = Self(linux_raw_sys::errno::EREMOTEIO as c_int);
    pub const RESTART: Self = Self(linux_raw_sys::errno::ERESTART as c_int);
    pub const RFKILL: Self = Self(linux_raw_sys::errno::ERFKILL as c_int);
    pub const ROFS: Self = Self(linux_raw_sys::errno::EROFS as c_int);
    pub const SHUTDOWN: Self = Self(linux_raw_sys::errno::ESHUTDOWN as c_int);
    pub const SOCKTNOSUPPORT: Self = Self(linux_raw_sys::errno::ESOCKTNOSUPPORT as c_int);
    pub const SPIPE: Self = Self(linux_raw_sys::errno::ESPIPE as c_int);
    pub const SRCH: Self = Self(linux_raw_sys::errno::ESRCH as c_int);
    pub const SRMNT: Self = Self(linux_raw_sys::errno::ESRMNT as c_int);
    pub const STALE: Self = Self(linux_raw_sys::errno::ESTALE as c_int);
    pub const STRPIPE: Self = Self(linux_raw_sys::errno::ESTRPIPE as c_int);
    pub const TIME: Self = Self(linux_raw_sys::errno::ETIME as c_int);
    pub const TIMEDOUT: Self = Self(linux_raw_sys::errno::ETIMEDOUT as c_int);
    pub const TOOBIG: Self = Self(linux_raw_sys::errno::E2BIG as c_int);
    pub const TOOMANYREFS: Self = Self(linux_raw_sys::errno::ETOOMANYREFS as c_int);
    pub const TXTBSY: Self = Self(linux_raw_sys::errno::ETXTBSY as c_int);
    pub const UCLEAN: Self = Self(linux_raw_sys::errno::EUCLEAN as c_int);
    pub const UNATCH: Self = Self(linux_raw_sys::errno::EUNATCH as c_int);
    pub const USERS: Self = Self(linux_raw_sys::errno::EUSERS as c_int);
    pub const WOULDBLOCK: Self = Self(linux_raw_sys::errno::EWOULDBLOCK as c_int);
    pub const XDEV: Self = Self(linux_raw_sys::errno::EXDEV as c_int);
    pub const XFULL: Self = Self(linux_raw_sys::errno::EXFULL as c_int);
}

impl Errno {
    #[inline]
    pub fn io_error(&self) -> io::Error {
        io::Error::from_raw_os_error(self.0)
    }

    #[inline]
    pub fn from_io_error(err: &io::Error) -> Option<Self> {
        err.raw_os_error().map(Self)
    }
}
