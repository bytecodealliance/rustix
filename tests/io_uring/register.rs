use std::ptr;

use libc::c_void;
use rustix::fd::{AsFd, AsRawFd, BorrowedFd};
use rustix::io::{Errno, Result};
use rustix::io_uring::{
    io_uring_buf, io_uring_buf_reg, io_uring_buf_ring, io_uring_params, io_uring_ptr,
    io_uring_register_with, io_uring_rsrc_update, io_uring_setup, IoringFeatureFlags,
    IoringRegisterFlags, IoringRegisterOp,
};
#[cfg(feature = "mm")]
use rustix::mm::{MapFlags, ProtFlags};

fn do_register<FD>(
    fd: FD,
    registered_fd: bool,
    opcode: IoringRegisterOp,
    arg: *mut c_void,
    arg_nr: u32,
) -> Result<()>
where
    FD: AsFd,
{
    let flags = if registered_fd {
        IoringRegisterFlags::USE_REGISTERED_RING
    } else {
        IoringRegisterFlags::default()
    };

    // Cast `arg` to `*const c_void` to match the current API. See
    // <https://github.com/bytecodealliance/rustix/issues/1545>.
    unsafe {
        io_uring_register_with(fd, opcode, flags, arg as *const c_void, arg_nr)?;
    }

    Ok(())
}

fn register_ring(fd: BorrowedFd<'_>) -> Result<BorrowedFd<'_>> {
    let mut update = io_uring_rsrc_update::default();
    update.data = io_uring_ptr::new(fd.as_raw_fd() as usize as *mut c_void);
    update.offset = u32::MAX;

    do_register(
        fd,
        false,
        IoringRegisterOp::RegisterRingFds,
        (&mut update as *mut io_uring_rsrc_update).cast::<c_void>(),
        1,
    )?;

    let registered_fd = unsafe { BorrowedFd::borrow_raw(update.offset as i32) };
    Ok(registered_fd)
}

fn unregister_ring<FD>(fd: FD) -> Result<()>
where
    FD: AsRawFd + AsFd,
{
    let mut update = io_uring_rsrc_update::default();
    update.offset = fd.as_raw_fd() as u32;
    update.data = io_uring_ptr::null();

    do_register(
        fd,
        true,
        IoringRegisterOp::UnregisterRingFds,
        (&update as *const io_uring_rsrc_update as *mut io_uring_rsrc_update).cast::<c_void>(),
        1,
    )?;

    Ok(())
}

/// Set bounded and unbounded async kernel worker counts to 0, to test
/// registering with registered ring fd.
fn register_iowq_max_workers<FD>(fd: FD) -> Result<()>
where
    FD: AsFd,
{
    let iowq_max_workers = [0_u32; 2];
    do_register(
        fd,
        true,
        IoringRegisterOp::RegisterIowqMaxWorkers,
        (&iowq_max_workers as *const [u32; 2] as *mut [u32; 2]).cast::<c_void>(),
        2,
    )?;

    Ok(())
}

fn register_buf_ring<FD>(fd: FD, reg: &io_uring_buf_reg) -> Result<()>
where
    FD: AsFd,
{
    do_register(
        fd,
        false,
        IoringRegisterOp::RegisterPbufRing,
        (reg as *const io_uring_buf_reg as *mut io_uring_buf_reg).cast::<c_void>(),
        1,
    )
}

#[test]
fn test_io_uring_register_with() {
    let mut params = io_uring_params::default();
    let ring_fd = unsafe { io_uring_setup(4, &mut params).unwrap() };
    assert_eq!(params.sq_entries, 4);
    assert_eq!(params.cq_entries, 8);

    if !params.features.contains(IoringFeatureFlags::REG_REG_RING) {
        // Kernel does not support `io_uring_register` with a registered ring fd
        return;
    }

    let ring_fd = register_ring(ring_fd.as_fd()).unwrap();
    let register_result = register_iowq_max_workers(ring_fd);
    unregister_ring(ring_fd).unwrap();
    register_result.unwrap();
}

#[cfg(feature = "mm")]
#[test]
fn io_uring_buf_ring_can_be_registered() {
    const ENTRIES: usize = 8;
    const BGID: u16 = 42;

    let mut params = io_uring_params::default();
    let ring_fd = unsafe { io_uring_setup(4, &mut params).unwrap() };

    // Test that the kernel version supports IORING_REGISTER_PBUF_RING. If it
    // doesn't, the kernel will return EINVAL. Not setting a `ring_addr` on
    // `io_uring_buf_reg` will return `EFAULT`.
    if let Err(e) = register_buf_ring(ring_fd.as_fd(), &io_uring_buf_reg::default()) {
        if e == Errno::INVAL {
            // Skip the test, as the current kernel version doesn't support what we need to
            // test.
            return;
        }
    }

    let buf_ring_size = ENTRIES * std::mem::size_of::<io_uring_buf>();

    let br_ptr = unsafe {
        rustix::mm::mmap_anonymous(
            ptr::null_mut(),
            buf_ring_size,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::PRIVATE,
        )
    }
    .unwrap()
    .cast::<io_uring_buf_ring>();

    let br = unsafe { br_ptr.as_mut() }.expect("A valid io_uring_buf_ring struct");

    let mut reg = io_uring_buf_reg::default();
    reg.ring_addr = br_ptr.cast::<c_void>().into();
    reg.ring_entries = ENTRIES as u32;
    reg.bgid = BGID;
    reg.flags = 0;

    assert_eq!(register_buf_ring(ring_fd, &reg), Ok(()));

    let tail = unsafe { br.tail_or_bufs.tail.as_mut() };
    tail.tail = 0;
    let bufs = unsafe { br.tail_or_bufs.bufs.as_mut().bufs.as_mut_slice(ENTRIES) };

    assert_eq!(bufs[0].bid, 0);
    bufs[7].bid = 7;
    assert_eq!(bufs[7].bid, 7);
}
