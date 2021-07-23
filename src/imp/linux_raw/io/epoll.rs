//! epoll support.
//!
//! This is an experiment, and it isn't yet clear whether epoll is the right
//! level of abstraction at which to introduce safety. But it works fairly well
//! in simple examples 🙂.
//!
//! # Examples
//!
//! ```rust,no_run
//! # fn main() -> std::io::Result<()> {
//! use posish::io::{
//!     epoll::{self, Epoll},
//!     ioctl_fionbio, read, write,
//! };
//! use posish::net::{
//!     accept, bind_v4, listen, socket, AddressFamily, Ipv4Addr, Protocol, SocketAddr,
//!     SocketAddrV4, SocketType,
//! };
//! use std::os::unix::io::AsRawFd;
//!
//! // Create a socket and listen on it.
//! let listen_sock = socket(AddressFamily::INET, SocketType::STREAM, Protocol::default())?;
//! bind_v4(&listen_sock, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?;
//! listen(&listen_sock, 1)?;
//!
//! // Create an epoll object. Using `Owning` here means the epoll object will
//! // take ownership of the file descriptors registered with it.
//! let epoll = Epoll::new(epoll::CreateFlags::CLOEXEC, epoll::Owning::new())?;
//!
//! // Remember the socket raw fd, which we use for comparisons only.
//! let raw_listen_sock = listen_sock.as_raw_fd();
//!
//! // Register the socket with the epoll object.
//! epoll.add(listen_sock, epoll::EventFlags::IN)?;
//!
//! // Process events.
//! let mut event_list = epoll::EventVec::with_capacity(4);
//! loop {
//!     epoll.wait(&mut event_list, -1)?;
//!     for (_event_flags, target) in &event_list {
//!         if target.as_raw_fd() == raw_listen_sock {
//!             // Accept a new connection, set it to non-blocking, and
//!             // register to be notified when it's ready to write to.
//!             let conn_sock = accept(&*target)?;
//!             ioctl_fionbio(&conn_sock, true)?;
//!             epoll
//!                 .add(conn_sock, epoll::EventFlags::OUT | epoll::EventFlags::ET)
//!                 ?;
//!         } else {
//!             // Write a message to the stream and then unregister it.
//!             write(&*target, b"hello\n")?;
//!             let _ = epoll.del(target)?;
//!         }
//!     }
//! }
//! # }
//! ```

#![allow(unsafe_code)]

use super::super::syscalls::{epoll_add, epoll_create, epoll_del, epoll_mod, epoll_wait};
use crate::{
    io,
    io::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};
use bitflags::bitflags;
use io_lifetimes::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
use std::{
    fmt,
    marker::PhantomData,
    ops::Deref,
    os::raw::{c_int, c_uint},
    ptr::null,
};

bitflags! {
    /// `EPOLL_*` for use with [`Epoll::new`].
    pub struct CreateFlags: c_uint {
        /// `EPOLL_CLOEXEC`
        const CLOEXEC = linux_raw_sys::general::EPOLL_CLOEXEC;
    }
}

bitflags! {
    /// `EPOLL*` for use with [`Epoll::add`].
    #[derive(Default)]
    pub struct EventFlags: u32 {
        /// `EPOLLIN`
        const IN = linux_raw_sys::general::EPOLLIN as u32;

        /// `EPOLLOUT`
        const OUT = linux_raw_sys::general::EPOLLOUT as u32;

        /// `EPOLLPRI`
        const PRI = linux_raw_sys::general::EPOLLPRI as u32;

        /// `EPOLLERR`
        const ERR = linux_raw_sys::general::EPOLLERR as u32;

        /// `EPOLLHUP`
        const HUP = linux_raw_sys::general::EPOLLHUP as u32;

        /// `EPOLLET`
        const ET = linux_raw_sys::general::EPOLLET as u32;

        /// `EPOLLONESHOT`
        const ONESHOT = linux_raw_sys::general::EPOLLONESHOT as u32;

        /// `EPOLLWAKEUP`
        const WAKEUP = linux_raw_sys::general::EPOLLWAKEUP as u32;

        /// `EPOLLEXCLUSIVE`
        const EXCLUSIVE = linux_raw_sys::general::EPOLLEXCLUSIVE as u32;
    }
}

/// A reference to a `T`.
pub struct Ref<'a, T> {
    t: T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Ref<'a, T> {
    #[inline]
    fn new(t: T) -> Self {
        Self {
            t,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn consume(self) -> T {
        self.t
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.t
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Ref<'a, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.t.fmt(fmt)
    }
}

/// A trait for data stored within an [`Epoll`] instance.
pub trait Context {
    /// The type of an element owned by this context.
    type Data;

    /// The type of a value used to refer to an element owned by this context.
    type Target: AsFd;

    /// Assume ownership of `data`, and returning a `Target`.
    fn acquire<'call>(&self, data: Self::Data) -> Ref<'call, Self::Target>;

    /// Encode `target` as a `u64`. The only requirement on this value is that
    /// it be decodable by `decode`.
    fn encode(&self, target: Ref<'_, Self::Target>) -> u64;

    /// Decode `raw`, which is a value encoded by `encode`, into a `Target`.
    ///
    /// # Safety
    ///
    /// `raw` must be a `u64` value returned from `encode`, from the same
    /// context, and within the context's lifetime.
    unsafe fn decode<'call>(&self, raw: u64) -> Ref<'call, Self::Target>;

    /// Release ownership of the value refered to by `target` and return it.
    fn release(&self, target: Ref<'_, Self::Target>) -> Self::Data;
}

/// A type implementing [`Context`] where the `Data` type is `BorrowedFd<'a>`.
pub struct Borrowing<'a> {
    _phantom: PhantomData<BorrowedFd<'a>>,
}

impl<'a> Context for Borrowing<'a> {
    type Data = BorrowedFd<'a>;
    type Target = BorrowedFd<'a>;

    #[inline]
    fn acquire<'call>(&self, data: Self::Data) -> Ref<'call, Self::Target> {
        Ref::new(data)
    }

    #[inline]
    fn encode(&self, target: Ref<'_, Self::Target>) -> u64 {
        target.as_raw_fd() as u64
    }

    #[inline]
    unsafe fn decode<'call>(&self, raw: u64) -> Ref<'call, Self::Target> {
        Ref::new(BorrowedFd::<'a>::borrow_raw_fd(raw as RawFd))
    }

    #[inline]
    fn release(&self, target: Ref<'_, Self::Target>) -> Self::Data {
        target.consume()
    }
}

/// A type implementing [`Context`] where the `Data` type is `T`, a type
/// implementing `IntoFd` and `FromFd`.
///
/// This may be used with [`OwnedFd`], or higher-level types like
/// [`std::fs::File`] or [`std::net::TcpStream`].
pub struct Owning<'context, T: IntoFd + FromFd> {
    _phantom: PhantomData<&'context T>,
}

impl<'context, T: IntoFd + FromFd> Owning<'context, T> {
    /// Creates a new empty `Owning`.
    #[allow(clippy::new_without_default)] // This is a specialized type that doesn't need to be generically constructible.
    #[inline]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'context, T: AsFd + IntoFd + FromFd> Context for Owning<'context, T> {
    type Data = T;
    type Target = BorrowedFd<'context>;

    #[inline]
    fn acquire<'call>(&self, data: Self::Data) -> Ref<'call, Self::Target> {
        let raw_fd = data.into_fd().into_raw_fd();
        // Safety: `epoll` will assign ownership of the file descriptor to the
        // kernel epoll object. We use `IntoFd`+`IntoRawFd` to consume the
        // `Data` and extract the raw file descriptor and then "borrow" it
        // with `borrow_raw_fd` knowing that the borrow won't outlive the
        // kernel epoll object.
        unsafe { Ref::new(BorrowedFd::<'context>::borrow_raw_fd(raw_fd)) }
    }

    #[inline]
    fn encode(&self, target: Ref<'_, Self::Target>) -> u64 {
        target.as_fd().as_raw_fd() as u64
    }

    #[inline]
    unsafe fn decode<'call>(&self, raw: u64) -> Ref<'call, Self::Target> {
        Ref::new(BorrowedFd::<'context>::borrow_raw_fd(raw as RawFd))
    }

    #[inline]
    fn release(&self, target: Ref<'_, Self::Target>) -> Self::Data {
        // Safety: The file descriptor was held by the kernel epoll object and
        // is now being released, so we can create a new `OwnedFd` that assumes
        // ownership.
        let raw_fd = target.consume().as_raw_fd();
        unsafe { T::from_fd(OwnedFd::from_raw_fd(raw_fd)) }
    }
}

/// An "epoll", an interface to an OS object allowing one to repeatedly wait
/// for events from a set of file descriptors efficiently.
pub struct Epoll<Context: self::Context> {
    epoll_fd: OwnedFd,
    context: Context,
}

impl<Context: self::Context> Epoll<Context> {
    /// `epoll_create1(flags)`—Creates a new `Epoll`.
    ///
    /// Use the [`CreateFlags::CLOEXEC`] flag to prevent the resulting file
    /// descriptor from being implicity passed across `exec` boundaries.
    #[inline]
    #[doc(alias = "epoll_create1")]
    pub fn new(flags: CreateFlags, context: Context) -> io::Result<Self> {
        // Safety: We're calling `epoll_create1` via FFI and we know how it
        // behaves.
        Ok(Self {
            epoll_fd: epoll_create(flags)?,
            context,
        })
    }

    /// `epoll_ctl(self, EPOLL_CTL_ADD, data, event)`—Adds an element to an
    /// `Epoll`.
    ///
    /// This registers interest in any of the events set in `events` occuring
    /// on the file descriptor associated with `data`.
    #[doc(alias = "epoll_ctl")]
    pub fn add(
        &self,
        data: Context::Data,
        event_flags: EventFlags,
    ) -> io::Result<Ref<'_, Context::Target>> {
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            let target = self.context.acquire(data);
            let raw_fd = target.as_fd().as_raw_fd();
            let encoded = self.context.encode(target);
            epoll_add(
                self.epoll_fd.as_fd(),
                raw_fd,
                &linux_raw_sys::general::epoll_event {
                    events: event_flags.bits(),
                    data: encoded,
                },
            )?;
            Ok(self.context.decode(encoded))
        }
    }

    /// `epoll_ctl(self, EPOLL_CTL_MOD, target, event)`—Modifies an element in
    /// this `Epoll`.
    ///
    /// This sets the events of interest with `target` to `events`.
    #[doc(alias = "epoll_ctl")]
    pub fn mod_(
        &self,
        target: Ref<'_, Context::Target>,
        event_flags: EventFlags,
    ) -> io::Result<()> {
        let raw_fd = target.as_fd().as_raw_fd();
        let encoded = self.context.encode(target);
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            epoll_mod(
                self.epoll_fd.as_fd(),
                raw_fd,
                &linux_raw_sys::general::epoll_event {
                    events: event_flags.bits(),
                    data: encoded,
                },
            )
        }
    }

    /// `epoll_ctl(self, EPOLL_CTL_DEL, target, NULL)`—Removes an element in
    /// this `Epoll`.
    ///
    /// This also returns the owning `Data`.
    #[doc(alias = "epoll_ctl")]
    pub fn del(&self, target: Ref<'_, Context::Target>) -> io::Result<Context::Data> {
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            let raw_fd = target.as_fd().as_raw_fd();
            epoll_del(self.epoll_fd.as_fd(), raw_fd)?;
        }
        Ok(self.context.release(target))
    }

    /// `epoll_wait(self, events, timeout)`—Waits for registered events of
    /// interest.
    ///
    /// For each event of interest, an element is written to `events`. On
    /// success, this returns the number of written elements.
    #[doc(alias = "epoll_wait")]
    pub fn wait<'context>(
        &'context self,
        event_list: &mut EventVec<'context, Context>,
        timeout: c_int,
    ) -> io::Result<()> {
        // Safety: We're calling `epoll_wait` via FFI and we know how it
        // behaves.
        unsafe {
            event_list.events.set_len(0);
            let nfds = epoll_wait(
                self.epoll_fd.as_fd(),
                event_list.events[..].as_mut_ptr().cast::<_>(),
                event_list.events.capacity(),
                timeout,
            )?;
            event_list.events.set_len(nfds);
            event_list.context = &self.context;
        }

        Ok(())
    }
}

pub struct Iter<'context, Context: self::Context> {
    iter: std::slice::Iter<'context, Event>,
    context: *const Context,
    _phantom: PhantomData<&'context Context>,
}

impl<'context, Context: self::Context> Iterator for Iter<'context, Context> {
    type Item = (EventFlags, Ref<'context, Context::Target>);

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: `self.context` is guaranteed to be valid because we hold
        // `'context` for it. And we know this event is associated with this
        // context because `wait` sets both.
        self.iter.next().map(|event| {
            (event.event_flags, unsafe {
                (*self.context).decode(event.encoded)
            })
        })
    }
}

/// A record of an event that occurred.
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))]
struct Event {
    // Match the layout of `linux_raw_sys::general::epoll_event`. We just use a
    // `u64` instead of the full union; `Context` implementations will simply
    // need to deal with casting the value into and out of the `u64`
    // themselves.
    event_flags: EventFlags,
    encoded: u64,
}

pub struct EventVec<'context, Context: self::Context> {
    events: Vec<Event>,
    context: *const Context,
    _phantom: PhantomData<&'context Context>,
}

impl<'context, Context: self::Context> EventVec<'context, Context> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            context: null(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.events.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.events.reserve(additional);
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.events.reserve_exact(additional);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.events.clear();
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.events.shrink_to_fit();
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Context> {
        Iter {
            iter: self.events.iter(),
            context: self.context,
            _phantom: PhantomData,
        }
    }
}

impl<'context, Context: self::Context> IntoIterator for &'context EventVec<'context, Context> {
    type IntoIter = Iter<'context, Context>;
    type Item = (EventFlags, Ref<'context, Context::Target>);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
