#![allow(missing_docs)]

use std::io;
use std::os::raw::c_int;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(c_int);

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
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const DOTDOT: Self = Self(libc::EDOTDOT);
    pub const DQUOT: Self = Self(libc::EDQUOT);
    pub const EXIST: Self = Self(libc::EEXIST);
    pub const FAULT: Self = Self(libc::EFAULT);
    pub const FBIG: Self = Self(libc::EFBIG);
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Self = Self(libc::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self(libc::EHOSTUNREACH);
    #[cfg(not(target_os = "android"))]
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
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
    pub const NETDOWN: Self = Self(libc::ENETDOWN);
    pub const NETRESET: Self = Self(libc::ENETRESET);
    pub const NETUNREACH: Self = Self(libc::ENETUNREACH);
    pub const NFILE: Self = Self(libc::ENFILE);
    #[cfg(not(target_os = "netbsd"))]
    #[cfg(not(target_os = "freebsd"))]
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    #[cfg(not(target_os = "wasi"))]
    pub const NOANO: Self = Self(libc::ENOANO);
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
    pub const RFKILL: Self = Self(libc::ERFKILL);
    pub const ROFS: Self = Self(libc::EROFS);
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

    #[inline]
    pub fn io_error(&self) -> io::Error {
        io::Error::from_raw_os_error(self.0)
    }

    #[inline]
    pub fn from_io_error(err: &io::Error) -> Option<Self> {
        err.raw_os_error().map(Self)
    }
}
