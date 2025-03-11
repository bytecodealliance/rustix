#[cfg(not(any(
    target_os = "espidf",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
use crate::backend::c;

/// A resource value for use with [`getrlimit`], [`setrlimit`], and
/// [`prlimit`].
///
/// [`getrlimit`]: crate::process::getrlimit
/// [`setrlimit`]: crate::process::setrlimit
/// [`prlimit`]: crate::process::prlimit
#[cfg(not(any(
    target_os = "espidf",
    target_os = "fuchsia",
    target_os = "horizon",
    target_os = "redox",
    target_os = "vita",
    target_os = "wasi"
)))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(not(target_os = "l4re"), repr(u32))]
#[cfg_attr(target_os = "l4re", repr(u64))]
#[non_exhaustive]
pub enum Resource {
    /// `RLIMIT_CPU`
    Cpu = bitcast!(c::RLIMIT_CPU),
    /// `RLIMIT_FSIZE`
    Fsize = bitcast!(c::RLIMIT_FSIZE),
    /// `RLIMIT_DATA`
    Data = bitcast!(c::RLIMIT_DATA),
    /// `RLIMIT_STACK`
    Stack = bitcast!(c::RLIMIT_STACK),
    /// `RLIMIT_CORE`
    #[cfg(not(target_os = "haiku"))]
    Core = bitcast!(c::RLIMIT_CORE),
    /// `RLIMIT_RSS`
    // "nto" has `RLIMIT_RSS`, but it has the same value as `RLIMIT_AS`.
    #[cfg(not(any(
        apple,
        solarish,
        target_os = "haiku",
        target_os = "nto",
        target_os = "cygwin",
    )))]
    Rss = bitcast!(c::RLIMIT_RSS),
    /// `RLIMIT_NPROC`
    #[cfg(not(any(solarish, target_os = "haiku", target_os = "cygwin")))]
    Nproc = bitcast!(c::RLIMIT_NPROC),
    /// `RLIMIT_NOFILE`
    Nofile = bitcast!(c::RLIMIT_NOFILE),
    /// `RLIMIT_MEMLOCK`
    #[cfg(not(any(solarish, target_os = "aix", target_os = "haiku", target_os = "cygwin")))]
    Memlock = bitcast!(c::RLIMIT_MEMLOCK),
    /// `RLIMIT_AS`
    #[cfg(not(target_os = "openbsd"))]
    As = bitcast!(c::RLIMIT_AS),
    /// `RLIMIT_LOCKS`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin"
    )))]
    Locks = bitcast!(c::RLIMIT_LOCKS),
    /// `RLIMIT_SIGPENDING`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin"
    )))]
    Sigpending = bitcast!(c::RLIMIT_SIGPENDING),
    /// `RLIMIT_MSGQUEUE`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin"
    )))]
    Msgqueue = bitcast!(c::RLIMIT_MSGQUEUE),
    /// `RLIMIT_NICE`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin"
    )))]
    Nice = bitcast!(c::RLIMIT_NICE),
    /// `RLIMIT_RTPRIO`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin"
    )))]
    Rtprio = bitcast!(c::RLIMIT_RTPRIO),
    /// `RLIMIT_RTTIME`
    #[cfg(not(any(
        bsd,
        solarish,
        target_os = "aix",
        target_os = "android",
        target_os = "emscripten",
        target_os = "haiku",
        target_os = "hurd",
        target_os = "nto",
        target_os = "cygwin",
    )))]
    Rttime = bitcast!(c::RLIMIT_RTTIME),
}

#[cfg(apple)]
#[allow(non_upper_case_globals)]
impl Resource {
    /// `RLIMIT_RSS`
    pub const Rss: Self = Self::As;
}

#[cfg(freebsdlike)]
pub type RawId = c::id_t;
