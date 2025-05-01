use rustix::fs::{major, makedev, minor};

#[test]
fn makedev_roundtrip() {
    // Apple's, FreeBSD 11's, DragonFly's, and NetBSD's `makedev` doesn't
    // handle extra bits set.
    #[cfg(freebsdlike)]
    let (maj, min) = (0x0000_0026, 0x6564_0061);
    #[cfg(apple)]
    let (maj, min) = (0x0000_0026, 0x0064_6361);
    #[cfg(target_os = "netbsd")]
    let (maj, min) = (0x0000_0026, 0x0000_0061);
    #[cfg(target_os = "cygwin")]
    let (maj, min) = (0x0000_2526, 0x0000_6361);
    #[cfg(not(any(apple, freebsdlike, target_os = "cygwin", target_os = "netbsd")))]
    let (maj, min) = (0x2324_2526, 0x6564_6361);

    let dev = makedev(maj, min);
    assert_eq!(maj, major(dev));
    assert_eq!(min, minor(dev));
}
