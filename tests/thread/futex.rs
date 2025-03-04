#[cfg(feature = "std")]
use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};
use rustix::io::Errno;
use rustix::thread::futex;

#[test]
fn test_lock_unlock_pi() {
    let lock = AtomicU32::new(0);
    futex::lock_pi(&lock, futex::Flags::empty(), None).unwrap();
    assert_ne!(lock.load(Ordering::SeqCst), 0);

    let err = futex::lock_pi(&lock, futex::Flags::empty(), None).unwrap_err();
    assert_eq!(err, Errno::DEADLK);

    futex::unlock_pi(&lock, futex::Flags::empty()).unwrap();
    assert_eq!(lock.load(Ordering::SeqCst), 0);

    let err = futex::unlock_pi(&lock, futex::Flags::empty()).unwrap_err();
    assert_eq!(err, Errno::PERM);
}

#[cfg(feature = "std")]
#[test]
fn test_wait_wake() {
    let lock = std::sync::Arc::new(AtomicU32::new(0));

    match futex::wait(&lock, futex::Flags::empty(), 1, None) {
        Ok(()) => panic!("Nobody should be waking us!"),
        Err(Errno::AGAIN) => {
            assert_eq!(lock.load(Ordering::SeqCst), 0, "the lock should still be 0")
        }
        Err(err) => panic!("{err}"),
    }

    let other = std::thread::spawn({
        let lock = std::sync::Arc::clone(&lock);
        move || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            lock.store(1, Ordering::SeqCst);
            futex::wake(&lock, futex::Flags::empty(), 1).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(50));
            match futex::wait(&lock, futex::Flags::empty(), 1, None) {
                Ok(()) => panic!("Nobody should be waking us now!"),
                Err(Errno::AGAIN) => {
                    assert_eq!(lock.load(Ordering::SeqCst), 2, "the lock should now be 2")
                }
                Err(err) => panic!("{err}"),
            }
        }
    });

    match futex::wait(&lock, futex::Flags::empty(), 0, None) {
        Ok(()) => (),
        Err(Errno::AGAIN) => assert_eq!(lock.load(Ordering::SeqCst), 1, "the lock should now be 1"),
        Err(err) => panic!("{err}"),
    }

    lock.store(2, Ordering::SeqCst);
    futex::wake(&lock, futex::Flags::empty(), 1).unwrap();

    other.join().unwrap();
}

// Same as `test_wait_wake` but using `waitv`.
#[cfg(feature = "std")]
#[test]
fn test_waitv_wake() {
    let lock = std::sync::Arc::new(AtomicU32::new(0));

    let mut wait = futex::Wait::new();
    // In Rust 1.70, use `AtomicU32::as_ptr`.
    wait.uaddr = (&*lock as *const AtomicU32 as *mut u32)
        .cast::<c_void>()
        .into();
    wait.flags = futex::WaitFlags::SIZE_U32;
    wait.val = 1;
    match futex::waitv(
        &[wait],
        futex::WaitvFlags::empty(),
        None,
        futex::ClockId::Monotonic,
    ) {
        Ok(_) => panic!("Nobody should be waking us!"),
        Err(Errno::AGAIN) => {
            assert_eq!(lock.load(Ordering::SeqCst), 0, "the lock should still be 0")
        }
        // Skip this test if the kernel doesn't support futex_waitv.
        Err(Errno::NOSYS) => return,
        Err(err) => panic!("{err}"),
    }

    let other = std::thread::spawn({
        let lock = std::sync::Arc::clone(&lock);
        move || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            lock.store(1, Ordering::SeqCst);
            futex::wake(&lock, futex::Flags::empty(), 1).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut wait = futex::Wait::new();
            // In Rust 1.70, use `AtomicU32::as_ptr`.
            wait.uaddr = (&*lock as *const AtomicU32 as *mut u32)
                .cast::<c_void>()
                .into();
            wait.flags = futex::WaitFlags::SIZE_U32;
            wait.val = 1;
            match futex::waitv(
                &[wait],
                futex::WaitvFlags::empty(),
                None,
                futex::ClockId::Monotonic,
            ) {
                Ok(_) => panic!("Nobody should be waking us now!"),
                Err(Errno::AGAIN) => {
                    assert_eq!(lock.load(Ordering::SeqCst), 2, "the lock should now be 2")
                }
                Err(err) => panic!("{err}"),
            }
        }
    });

    let mut wait = futex::Wait::new();
    // In Rust 1.70, use `AtomicU32::as_ptr`.
    wait.uaddr = (&*lock as *const AtomicU32 as *mut u32)
        .cast::<c_void>()
        .into();
    wait.flags = futex::WaitFlags::SIZE_U32;
    wait.val = 0;
    match futex::waitv(
        &[wait],
        futex::WaitvFlags::empty(),
        None,
        futex::ClockId::Monotonic,
    ) {
        Ok(0) => {}
        Ok(n) => panic!("Somehow we woke up waiter {}!", n),
        Err(Errno::AGAIN) => assert_eq!(lock.load(Ordering::SeqCst), 1, "the lock should now be 1"),
        Err(err) => panic!("{err}"),
    }

    lock.store(2, Ordering::SeqCst);
    futex::wake(&lock, futex::Flags::empty(), 1).unwrap();

    other.join().unwrap();
}

#[cfg(feature = "std")]
#[test]
fn test_wait_timeout() {
    use rustix::thread::futex::Timespec;

    let lock = AtomicU32::new(0);

    let err = futex::wait(
        &lock,
        futex::Flags::empty(),
        0,
        Some(&Timespec {
            tv_sec: 1,
            tv_nsec: 13,
        }),
    )
    .unwrap_err();
    assert_eq!(err, Errno::TIMEDOUT);

    let err = futex::wait(
        &lock,
        futex::Flags::empty(),
        0,
        Some(&Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000_000,
        }),
    )
    .unwrap_err();
    assert_eq!(err, Errno::INVAL);

    let err = futex::wait(
        &lock,
        futex::Flags::empty(),
        0,
        Some(&Timespec {
            tv_sec: -1,
            tv_nsec: 0,
        }),
    )
    .unwrap_err();
    assert_eq!(err, Errno::INVAL);
}

// Same as `test_wait_timeout` but using `waitv`.
#[cfg(feature = "std")]
#[test]
fn test_waitv_timeout() {
    use rustix::thread::futex::Timespec;

    let lock = AtomicU32::new(0);

    let mut wait = futex::Wait::new();
    // In Rust 1.70, use `AtomicU32::as_ptr`.
    wait.uaddr = (&lock as *const AtomicU32 as *mut u32)
        .cast::<c_void>()
        .into();
    wait.flags = futex::WaitFlags::SIZE_U32;
    wait.val = 0;

    let err = futex::waitv(
        &[wait],
        futex::WaitvFlags::empty(),
        Some(&Timespec {
            tv_sec: 1,
            tv_nsec: 13,
        }),
        futex::ClockId::Monotonic,
    )
    .unwrap_err();
    if err == Errno::NOSYS {
        // Skip this test if the kernel doesn't support futex_waitv.
        return;
    }
    assert_eq!(err, Errno::TIMEDOUT);

    let err = futex::waitv(
        &[wait],
        futex::WaitvFlags::empty(),
        Some(&Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000_000,
        }),
        futex::ClockId::Monotonic,
    )
    .unwrap_err();
    assert_eq!(err, Errno::INVAL);

    let err = futex::waitv(
        &[wait],
        futex::WaitvFlags::empty(),
        Some(&Timespec {
            tv_sec: -1,
            tv_nsec: 0,
        }),
        futex::ClockId::Monotonic,
    )
    .unwrap_err();
    assert_eq!(err, Errno::INVAL);
}
