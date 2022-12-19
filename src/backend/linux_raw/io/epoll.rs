//! epoll support.
//!
//! This is an experiment, and it isn't yet clear whether epoll is the right
//! level of abstraction at which to introduce safety. But it works fairly well
//! in simple examples 🙂.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
//! # #[cfg(feature = "net")]
//! # fn main() -> std::io::Result<()> {
//! use io_lifetimes::AsFd;
//! use rustix::io::epoll::{self, Epoll};
//! use rustix::io::{ioctl_fionbio, read, write};
//! use rustix::net::{
//!     accept, bind_v4, listen, socket, AddressFamily, Ipv4Addr, Protocol, SocketAddrV4,
//!     SocketType,
//! };
//! use std::collections::HashMap;
//! use std::os::unix::io::AsRawFd;
//!
//! // Create a socket and listen on it.
//! let listen_sock = socket(AddressFamily::INET, SocketType::STREAM, Protocol::default())?;
//! bind_v4(&listen_sock, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?;
//! listen(&listen_sock, 1)?;
//!
//! // Create an epoll object. Using `Owning` here means the epoll object will
//! // take ownership of the file descriptors registered with it.
//! let epoll = Epoll::new(epoll::CreateFlags::CLOEXEC)?;
//!
//! // Register the socket with the epoll object.
//! epoll.add(&listen_sock, 1, epoll::EventFlags::IN)?;
//!
//! // Keep track of the sockets we've opened.
//! let mut next_id = 2;
//! let mut sockets = HashMap::new();
//!
//! // Process events.
//! let mut event_list = epoll::EventVec::with_capacity(4);
//! loop {
//!     epoll.wait(&mut event_list, -1)?;
//!     for (_event_flags, target) in &event_list {
//!         if target == 1 {
//!             // Accept a new connection, set it to non-blocking, and
//!             // register to be notified when it's ready to write to.
//!             let conn_sock = accept(&listen_sock)?;
//!             ioctl_fionbio(&conn_sock, true)?;
//!             epoll.add(&conn_sock, next_id, epoll::EventFlags::OUT | epoll::EventFlags::ET)?;
//!             
//!             // Keep track of the socket.
//!             sockets.insert(next_id, conn_sock);
//!             next_id += 1;
//!         } else {
//!             // Write a message to the stream and then unregister it.
//!             let target = sockets.remove(&target).unwrap();
//!             write(&target, b"hello\n")?;
//!             let _ = epoll.del(&target)?;
//!         }
//!     }
//! }
//! # }
//! # #[cfg(not(feature = "net"))]
//! # fn main() {}
//! ```

#![allow(unsafe_code)]

use super::super::c;
use crate::backend::io::syscalls::{epoll_add, epoll_create, epoll_del, epoll_mod, epoll_wait};
use crate::fd::{AsFd, AsRawFd, OwnedFd};
#[cfg(feature = "std")]
use crate::fd::{BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::io;
use alloc::vec::Vec;
use bitflags::bitflags;

bitflags! {
    /// `EPOLL_*` for use with [`Epoll::new`].
    pub struct CreateFlags: c::c_uint {
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

/// An "epoll", an interface to an OS object allowing one to repeatedly wait
/// for events from a set of file descriptors efficiently.
pub struct Epoll {
    epoll_fd: OwnedFd,
}

impl Epoll {
    /// `epoll_create1(flags)`—Creates a new `Epoll`.
    ///
    /// Use the [`CreateFlags::CLOEXEC`] flag to prevent the resulting file
    /// descriptor from being implicitly passed across `exec` boundaries.
    #[inline]
    #[doc(alias = "epoll_create1")]
    pub fn new(flags: CreateFlags) -> io::Result<Self> {
        // Safety: We're calling `epoll_create1` via FFI and we know how it
        // behaves.
        Ok(Self {
            epoll_fd: epoll_create(flags)?,
        })
    }

    /// `epoll_ctl(self, EPOLL_CTL_ADD, data, event)`—Adds an element to an
    /// `Epoll`.
    ///
    /// This registers interest in any of the events set in `events` occurring
    /// on the file descriptor associated with `data`.
    #[doc(alias = "epoll_ctl")]
    pub fn add(&self, source: &impl AsFd, data: u64, event_flags: EventFlags) -> io::Result<()> {
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            epoll_add(
                self.epoll_fd.as_fd(),
                source.as_fd().as_raw_fd(),
                &linux_raw_sys::general::epoll_event {
                    events: event_flags.bits(),
                    data,
                },
            )
        }
    }

    /// `epoll_ctl(self, EPOLL_CTL_MOD, target, event)`—Modifies an element in
    /// this `Epoll`.
    ///
    /// This sets the events of interest with `target` to `events`.
    #[doc(alias = "epoll_ctl")]
    pub fn mod_(&self, source: &impl AsFd, data: u64, event_flags: EventFlags) -> io::Result<()> {
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            let raw_fd = source.as_fd().as_raw_fd();
            epoll_mod(
                self.epoll_fd.as_fd(),
                raw_fd,
                &linux_raw_sys::general::epoll_event {
                    events: event_flags.bits(),
                    data,
                },
            )
        }
    }

    /// `epoll_ctl(self, EPOLL_CTL_DEL, target, NULL)`—Removes an element in
    /// this `Epoll`.
    ///
    /// This also returns the owning `Data`.
    #[doc(alias = "epoll_ctl")]
    pub fn del(&self, source: &impl AsFd) -> io::Result<()> {
        // Safety: We're calling `epoll_ctl` via FFI and we know how it
        // behaves.
        unsafe {
            let raw_fd = source.as_fd().as_raw_fd();
            epoll_del(self.epoll_fd.as_fd(), raw_fd)
        }
    }

    /// `epoll_wait(self, events, timeout)`—Waits for registered events of
    /// interest.
    ///
    /// For each event of interest, an element is written to `events`. On
    /// success, this returns the number of written elements.
    #[doc(alias = "epoll_wait")]
    pub fn wait(&self, event_list: &mut EventVec, timeout: c::c_int) -> io::Result<()> {
        // Safety: We're calling `epoll_wait` via FFI and we know how it
        // behaves.
        unsafe {
            event_list.events.set_len(0);
            let nfds = epoll_wait(
                self.epoll_fd.as_fd(),
                event_list.events[..].as_mut_ptr().cast(),
                event_list.events.capacity(),
                timeout,
            )?;
            event_list.events.set_len(nfds);
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl AsRawFd for Epoll {
    fn as_raw_fd(&self) -> RawFd {
        self.epoll_fd.as_raw_fd()
    }
}

#[cfg(feature = "std")]
impl IntoRawFd for Epoll {
    fn into_raw_fd(self) -> RawFd {
        self.epoll_fd.into_raw_fd()
    }
}

#[cfg(feature = "std")]
impl FromRawFd for Epoll {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self {
            epoll_fd: OwnedFd::from_raw_fd(fd),
        }
    }
}

#[cfg(feature = "std")]
impl AsFd for Epoll {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.epoll_fd.as_fd()
    }
}

#[cfg(feature = "std")]
impl From<Epoll> for OwnedFd {
    fn from(epoll: Epoll) -> Self {
        epoll.epoll_fd
    }
}

#[cfg(feature = "std")]
impl From<OwnedFd> for Epoll {
    fn from(fd: OwnedFd) -> Self {
        Self { epoll_fd: fd }
    }
}

/// An iterator over the `Event`s in an `EventVec`.
pub struct Iter<'a> {
    iter: core::slice::Iter<'a, Event>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (EventFlags, u64);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|event| (event.event_flags, event.encoded))
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

/// A vector of `Event`s, plus context for interpreting them.
pub struct EventVec {
    events: Vec<Event>,
}

impl EventVec {
    /// Constructs an `EventVec` with memory for `capacity` `Event`s.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
        }
    }

    /// Returns the current `Event` capacity of this `EventVec`.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.events.capacity()
    }

    /// Reserves enough memory for at least `additional` more `Event`s.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.events.reserve(additional);
    }

    /// Reserves enough memory for exactly `additional` more `Event`s.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.events.reserve_exact(additional);
    }

    /// Clears all the `Events` out of this `EventVec`.
    #[inline]
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Shrinks the capacity of this `EventVec` as much as possible.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.events.shrink_to_fit();
    }

    /// Returns an iterator over the `Event`s in this `EventVec`.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.events.iter(),
        }
    }

    /// Returns the number of `Event`s logically contained in this `EventVec`.
    #[inline]
    pub fn len(&mut self) -> usize {
        self.events.len()
    }

    /// Tests whether this `EventVec` is logically empty.
    #[inline]
    pub fn is_empty(&mut self) -> bool {
        self.events.is_empty()
    }
}

impl<'a> IntoIterator for &'a EventVec {
    type IntoIter = Iter<'a>;
    type Item = (EventFlags, u64);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
