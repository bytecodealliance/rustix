//! Tests for `EventFilter::MachPort`.

use std::mem::MaybeUninit;
use std::ptr::null_mut;

use rustix::event::kqueue::{kevent, kqueue, Event, EventFilter, EventFlags};

type KernReturn = libc::c_int;

extern "C" {
    static mach_task_self_: u32;
    fn mach_port_allocate(task: u32, right: libc::c_uint, name: *mut u32) -> KernReturn;
    fn mach_port_insert_right(
        task: u32,
        name: u32,
        poly: u32,
        poly_poly: libc::c_uint,
    ) -> KernReturn;
    fn mach_msg(
        msg: *mut MachMsgHeader,
        option: libc::c_int,
        send_size: u32,
        rcv_size: u32,
        rcv_name: u32,
        timeout: u32,
        notify: u32,
    ) -> KernReturn;
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MachMsgHeader {
    bits: u32,
    size: u32,
    remote_port: u32,
    local_port: u32,
    voucher_port: u32,
    id: i32,
}

/// A port with a message queued is reported ready, and the message is left in
/// place for the caller to receive itself.
#[test]
fn test_kqueue_machport() {
    const MACH_PORT_RIGHT_RECEIVE: libc::c_uint = 1;
    const MACH_MSG_TYPE_MAKE_SEND: libc::c_uint = 20;
    const MACH_MSG_TYPE_COPY_SEND: u32 = 19;
    const MACH_SEND_MSG: libc::c_int = 1;

    let mut port = 0u32;
    unsafe {
        assert_eq!(
            mach_port_allocate(mach_task_self_, MACH_PORT_RIGHT_RECEIVE, &mut port),
            0
        );
        assert_eq!(
            mach_port_insert_right(mach_task_self_, port, port, MACH_MSG_TYPE_MAKE_SEND),
            0
        );
    }

    let queue = kqueue().unwrap();
    let change = Event::new(
        EventFilter::MachPort { port },
        EventFlags::ADD | EventFlags::ENABLE,
        null_mut(),
    );
    // Registering only: an events buffer with no timeout would block here.
    let mut none: [MaybeUninit<Event>; 0] = [];
    unsafe { kevent(&queue, &[change], &mut none[..], None).unwrap() };

    // Nothing has been sent, so nothing is ready.
    let mut events = [MaybeUninit::<Event>::uninit(); 1];
    let (ready, _) =
        unsafe { kevent(&queue, &[], &mut events[..], Some(Default::default())).unwrap() };
    assert!(ready.is_empty());

    let size = core::mem::size_of::<MachMsgHeader>() as u32;
    let mut message = MachMsgHeader {
        bits: MACH_MSG_TYPE_COPY_SEND,
        size,
        remote_port: port,
        id: 1234,
        ..Default::default()
    };
    assert_eq!(
        unsafe { mach_msg(&mut message, MACH_SEND_MSG, size, 0, 0, 0, 0) },
        0
    );

    let mut events = [MaybeUninit::<Event>::uninit(); 1];
    let (ready, _) =
        unsafe { kevent(&queue, &[], &mut events[..], Some(Default::default())).unwrap() };
    assert_eq!(ready.len(), 1);
    assert!(matches!(
        ready[0].filter(),
        EventFilter::MachPort { port: p } if p == port
    ));
}
