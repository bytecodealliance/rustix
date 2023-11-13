#[cfg(any(linux_kernel, target_os = "freebsd", target_os = "illumos"))]
#[test]
fn test_eventfd() {
    use rustix::event::{eventfd, EventfdFlags};
    use rustix::io::{read, write};
    use std::mem::size_of;
    use std::thread;

    let efd = match eventfd(0, EventfdFlags::CLOEXEC) {
        Ok(efd) => efd,
        #[cfg(target_os = "freebsd")]
        Err(rustix::io::Errno::NOSYS) => return, // FreeBSD 12 lacks `eventfd`
        Err(err) => panic!("{:?}", err),
    };

    let child = thread::spawn(move || {
        for u in [1_u64, 3, 6, 11, 5000] {
            assert_eq!(write(&efd, &u.to_ne_bytes()).unwrap(), size_of::<u64>());
        }
        efd
    });

    let efd = child.join().unwrap();

    let mut bytes = [0_u8; size_of::<u64>()];
    let s = read(&efd, &mut bytes).unwrap();
    assert_eq!(s, bytes.len());
    let u = u64::from_ne_bytes(bytes);
    assert_eq!(u, 5021);
}
