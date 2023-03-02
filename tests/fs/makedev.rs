use rustix::fs::makedev;
#[cfg(not(bsd))]
use rustix::fs::{major, minor};

#[test]
fn makedev_roundtrip() {
    let maj = 0x2324_2526;
    let min = 0x6564_6361;
    let dev = makedev(maj, min);
    #[cfg(not(bsd))]
    {
        assert_eq!(maj, major(dev));
        assert_eq!(min, minor(dev));
    }
    #[cfg(bsd)]
    let _ = dev;
}
