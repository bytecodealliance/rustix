//! Solaris/illumos event ports.

use crate::backend::c;
use crate::backend::event::syscalls;
use crate::fd::{AsFd, AsRawFd, OwnedFd};
use crate::ffi;
use crate::io;

use super::PollFlags;

use core::time::Duration;

/// The structure representing a port event.
#[repr(transparent)]
pub struct Event(pub(crate) c::port_event);

impl Event {
    /// Get the events associated with this event.
    pub fn events(&self) -> i32 {
        self.0.portev_events
    }

    /// Get the event source associated with this event.
    pub fn object(&self) -> usize {
        self.0.portev_object
    }

    /// Get the userdata associated with this event.
    pub fn userdata(&self) -> *mut ffi::c_void {
        self.0.portev_user
    }
}

/// `port_create()`—Creates a new port.
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_create/
/// [illumos]: https://illumos.org/man/3C/port_create
pub fn create() -> io::Result<OwnedFd> {
    syscalls::port_create()
}

/// `port_associate(_, PORT_SOURCE_FD, _, _, _)`—Associates a file descriptor
/// with a port.
///
/// # Safety
///
/// Any `object`s passed into the `port` must be valid for the lifetime of the
/// `port`. Logically, `port` keeps a borrowed reference to the `object` until
/// it is removed via [`dissociate_fd`].
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_associate/
/// [illumos]: https://illumos.org/man/3C/port_associate
pub unsafe fn associate_fd<Fd: AsFd, RawFd: AsRawFd>(
    port: Fd,
    object: RawFd,
    events: PollFlags,
    userdata: *mut ffi::c_void,
) -> io::Result<()> {
    syscalls::port_associate(
        port.as_fd(),
        c::PORT_SOURCE_FD,
        object.as_raw_fd() as _,
        events.bits() as _,
        userdata.cast(),
    )
}

/// `port_dissociate(_, PORT_SOURCE_FD, _)`—Dissociates a file descriptor
/// from a port.
///
/// # Safety
///
/// The file descriptor passed into this function must have been previously
/// associated with the port via [`associate_fd`].
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_dissociate
/// [illumos]: https://illumos.org/man/3C/port_dissociate
pub unsafe fn dissociate_fd<Fd: AsFd, RawFd: AsRawFd>(port: Fd, object: RawFd) -> io::Result<()> {
    syscalls::port_dissociate(port.as_fd(), c::PORT_SOURCE_FD, object.as_raw_fd() as _)
}

/// `port_get(port, timeout)`—Gets an event from a port.
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_get/
/// [illumos]: https://illumos.org/man/3C/port_get
pub fn get<Fd: AsFd>(port: Fd, timeout: Option<Duration>) -> io::Result<Event> {
    let mut timeout = timeout.map(|timeout| c::timespec {
        tv_sec: timeout.as_secs().try_into().unwrap(),
        tv_nsec: timeout.subsec_nanos() as _,
    });

    syscalls::port_get(port.as_fd(), timeout.as_mut())
}

/// `port_getn(port, events, min_events, timeout)`—Gets multiple events from a
/// port.
///
/// This requests up to a max of `events.capacity()` events, and then resizes
/// `events` to the number of events retrieved. If `events.capacity()` is 0,
/// this does nothing and returns immediately.
///
/// To query the number of events without retrieving any, use
/// [`getn_query`].
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_getn/
/// [illumos]: https://illumos.org/man/3C/port_getn
#[cfg(feature = "alloc")]
pub fn getn<Fd: AsFd>(
    port: Fd,
    events: &mut Vec<Event>,
    min_events: usize,
    timeout: Option<Duration>,
) -> io::Result<()> {
    events.clear();

    let mut timeout = timeout.map(|timeout| c::timespec {
        tv_sec: timeout.as_secs().try_into().unwrap(),
        tv_nsec: timeout.subsec_nanos() as _,
    });

    syscalls::port_getn(
        port.as_fd(),
        timeout.as_mut(),
        events,
        min_events.try_into().unwrap(),
    )
}

/// `port_getn(port, NULL, 0, NULL)`—Queries the number of events
/// available from a port.
///
/// To retrieve the events, use [`getn`].
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_getn/
/// [illumos]: https://illumos.org/man/3C/port_getn
pub fn getn_query<Fd: AsFd>(port: Fd) -> io::Result<u32> {
    syscalls::port_getn_query(port.as_fd())
}

/// `port_send(port, events, userdata)`—Sends an event to a port.
///
/// # References
///  - [OpenSolaris]
///  - [illumos]
///
/// [OpenSolaris]: https://www.unix.com/man-page/opensolaris/3C/port_send/
/// [illumos]: https://illumos.org/man/3C/port_send
pub fn send<Fd: AsFd>(port: Fd, events: i32, userdata: *mut ffi::c_void) -> io::Result<()> {
    syscalls::port_send(port.as_fd(), events, userdata.cast())
}
