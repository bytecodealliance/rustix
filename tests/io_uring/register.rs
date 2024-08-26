use libc::c_void;
use rustix::fd::{AsFd, AsRawFd, BorrowedFd};
use rustix::io::Result;
use rustix::io_uring::{
    io_uring_params, io_uring_register_with, io_uring_rsrc_update, io_uring_setup,
    IoringFeatureFlags, IoringRegisterFlags, IoringRegisterOp,
};

fn do_register<FD>(
    fd: FD,
    registered_fd: bool,
    opcode: IoringRegisterOp,
    arg: *const c_void,
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

    unsafe {
        io_uring_register_with(fd, opcode, flags, arg, arg_nr)?;
    }

    Ok(())
}

fn register_ring(fd: BorrowedFd<'_>) -> Result<BorrowedFd<'_>> {
    let update = io_uring_rsrc_update {
        data: fd.as_raw_fd() as u64,
        offset: u32::MAX,
        resv: 0,
    };

    do_register(
        fd,
        false,
        IoringRegisterOp::RegisterRingFds,
        (&update) as *const io_uring_rsrc_update as *const c_void,
        1,
    )?;

    let registered_fd = unsafe { BorrowedFd::borrow_raw(update.offset as i32) };
    Ok(registered_fd)
}

fn unregister_ring<FD>(fd: FD) -> Result<()>
where
    FD: AsRawFd + AsFd,
{
    let update = io_uring_rsrc_update {
        offset: fd.as_raw_fd() as u32,
        data: 0,
        resv: 0,
    };

    do_register(
        fd,
        true,
        IoringRegisterOp::UnregisterRingFds,
        (&update) as *const io_uring_rsrc_update as *const c_void,
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
    let iowq_max_workers = [0u32; 2];
    do_register(
        fd,
        true,
        IoringRegisterOp::RegisterIowqMaxWorkers,
        (&iowq_max_workers) as *const [u32; 2] as *const c_void,
        2,
    )?;

    Ok(())
}

#[test]
fn test_io_uring_register_with() {
    let mut params = io_uring_params::default();
    let ring_fd = io_uring_setup(4, &mut params).unwrap();
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
