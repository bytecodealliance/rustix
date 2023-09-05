//! The io_uring backend.
//!
//! This uses the io_uring API directly, without going through libc.

#![allow(unsafe_code)]

// Re-export all of linux-raw.
pub(crate) use crate::linux_raw::*;

use crate::alluring::IoUring;
use core::cell::RefCell;

#[cfg(feature = "std")]
thread_local! {
    static IO_URING: RefCell<Option<IoUring>> = RefCell::new(None);
}

#[cfg(feature = "net")]
pub(crate) mod net {
    pub(crate) use crate::linux_raw::net::*;

    pub(crate) mod syscalls {
        pub(crate) use crate::linux_raw::net::syscalls::*;

        use crate::alluring::{opcode, squeue, types, IoUring};
        use crate::backend::IO_URING;
        use crate::fd::{AsRawFd, BorrowedFd, OwnedFd};
        use crate::io;
        use crate::linux_raw::conv::{ret, ret_owned_fd};
        use crate::linux_raw::reg::{FromAsm, RetReg, R0};
        use crate::net::{AddressFamily, Protocol, Shutdown, SocketType};
        use core::ptr::null_mut;

        pub(crate) fn socket(
            family: AddressFamily,
            type_: SocketType,
            protocol: Option<Protocol>,
        ) -> io::Result<OwnedFd> {
            with_ring(|ring| {
                let op = opcode::Socket::new(
                    family.as_raw() as i32,
                    type_.as_raw() as i32,
                    protocol.map(|p| p.as_raw().get() as i32).unwrap_or(0),
                )
                .build();
                unsafe { ret_owned_fd(run(ring, &op)?) }
            })
        }

        pub(crate) fn accept(fd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
            with_ring(|ring| {
                let op =
                    opcode::Accept::new(types::Fd(fd.as_raw_fd()), null_mut(), null_mut()).build();
                unsafe { ret_owned_fd(run(ring, &op)?) }
            })
        }

        pub(crate) fn shutdown(fd: BorrowedFd, how: Shutdown) -> io::Result<()> {
            with_ring(|ring| {
                let op = opcode::Shutdown::new(types::Fd(fd.as_raw_fd()), how as i32).build();
                unsafe { ret(run(ring, &op)?) }
            })
        }

        fn with_ring<T, F: FnOnce(&mut IoUring) -> io::Result<T>>(f: F) -> io::Result<T> {
            IO_URING.with_borrow_mut(|ring| {
                let ring = match ring {
                    Some(ring) => ring,
                    None => {
                        let new_ring = IoUring::new(8)?; // TODO: what value?
                        *ring = Some(new_ring);
                        ring.as_mut().unwrap()
                    }
                };

                f(ring)
            })
        }

        unsafe fn run(ring: &mut IoUring, op: &squeue::Entry) -> io::Result<RetReg<R0>> {
            ring.submission()
                .push(op)
                .expect("submission queue is full");

            ring.submit_and_wait(1)?;

            let cqe = ring.completion().next().expect("completion queue is empty");

            let result = cqe.result();
            Ok(RetReg::from_asm(result as usize as _))
        }
    }
}

// TODO:

//#[cfg(not(feature = "std"))]
//#[thread_local]
//static IO_URING: IoUring;

// #[thread_local] submit/complete buffers with cargo feature configurable sizes?

// public module exposing the queues and fd for pushing, completion, and configuration?

// eventually, an async runtime on top of this thing too?

// eventually, a Windows backend for this thing too?
