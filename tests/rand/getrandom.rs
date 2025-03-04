use core::mem::MaybeUninit;
use rustix::rand::{getrandom, GetRandomFlags};

#[test]
fn test_getrandom() {
    let mut buf = [0_u8; 256];
    let len = getrandom(&mut buf, GetRandomFlags::empty()).unwrap();
    assert!(len <= buf.len());
}

#[test]
fn test_getrandom_uninit() {
    let mut buf = unsafe { MaybeUninit::<[MaybeUninit<u8>; 256]>::uninit().assume_init() };
    let (init, uninit) = getrandom(&mut buf, GetRandomFlags::empty()).unwrap();
    let combined_len = init.len() + uninit.len();
    assert_eq!(buf.len(), combined_len);
}
