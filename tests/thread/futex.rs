use core::sync::atomic::{AtomicU32, Ordering};
use rustix::{
    io::Errno,
    thread::{futex, FutexFlags},
};

#[test]
fn test_lock_unlock_pi() {
    let lock = AtomicU32::new(0);
    unsafe {
        futex::lock_pi(&lock, FutexFlags::empty(), None).unwrap();
    }
    assert_ne!(lock.load(Ordering::SeqCst), 0);

    let err = unsafe { futex::lock_pi(&lock, FutexFlags::empty(), None).unwrap_err() };
    assert_eq!(err, Errno::DEADLK);

    unsafe {
        futex::unlock_pi(&lock, FutexFlags::empty()).unwrap();
    }
    assert_eq!(lock.load(Ordering::SeqCst), 0);

    let err = unsafe { futex::unlock_pi(&lock, FutexFlags::empty()).unwrap_err() };
    assert_eq!(err, Errno::PERM);
}

#[cfg(feature = "std")]
#[test]
fn test_wait_wake() {
    let lock = std::sync::Arc::new(AtomicU32::new(0));

    match unsafe { futex::wait(&lock, FutexFlags::empty(), 1, None) } {
			Ok(()) => panic!("Nobody should be waking us!"),
			Err(Errno::AGAIN) => assert_eq!(lock.load(Ordering::SeqCst), 0, "the lock should still be 0"),
			Err(err) => panic!("{err}"),
	}

    let other = std::thread::spawn({
        let lock = lock.clone();
        move || {
            std::thread::sleep(std::time::Duration::from_millis(1));
						lock.store(1, Ordering::SeqCst);
            unsafe {
                futex::wake(&lock, FutexFlags::empty(), 1);
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
						match unsafe { futex::wait(&lock, FutexFlags::empty(), 1, None) } {
								Ok(()) => (),
								Err(Errno::AGAIN) => assert_eq!(lock.load(Ordering::SeqCst), 2, "the lock should now be 2"),
								Err(err) => panic!("{err}"),
						}
        }
    });

    match unsafe { futex::wait(&lock, FutexFlags::empty(), 0, None) } {
        Ok(()) => (),
        Err(Errno::AGAIN) => assert_eq!(lock.load(Ordering::SeqCst), 1, "the lock should now be 1"),
        Err(err) => panic!("{err}"),
    }

		lock.store(2, Ordering::SeqCst);
		unsafe {
				futex::wake(&lock, FutexFlags::empty(), 1);
		}

		other.join().unwrap();
}

#[cfg(feature = "std")]
#[test]
fn test_timeout() {
    use rustix::fs::Timespec;

    let lock = AtomicU32::new(0);
		
    let err = unsafe {
			futex::wait(&lock, FutexFlags::empty(), 0, Some(Timespec{tv_sec: 1, tv_nsec: 13})).unwrap_err()
		};
		assert_eq!(err, Errno::TIMEDOUT);
		
    let err = unsafe {
			futex::wait(&lock, FutexFlags::empty(), 0, Some(Timespec{tv_sec: 0, tv_nsec: 1_000_000_000})).unwrap_err()
		};
		assert_eq!(err, Errno::INVAL);
		
    let err = unsafe {
			futex::wait(&lock, FutexFlags::empty(), 0, Some(Timespec{tv_sec: -1, tv_nsec: 0})).unwrap_err()
		};
		assert_eq!(err, Errno::INVAL);
}
