/// The error type for posish APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
// Linux errno codes are in 1..4096, which is 12 bits.
pub struct Error(pub(crate) u16);

// These have type `u32` in the bindgen bindings; cast them to `u16` as
// knowledge that the platform errno type is signed is widespread.
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
    /// Extract the raw OS error number from this error.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        io_err.raw_os_error().and_then(|raw| {
            // Maintain the invariant that we only hold error codes in Linux's
            // range.
            if (1..4096).contains(&raw) {
                Some(Self(raw as u16))
            } else {
                None
            }
        })
    }
}
