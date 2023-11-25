use core::mem::MaybeUninit;
use rustix::rand::{getrandom, getrandom_uninit, GetRandomFlags};

#[test]
fn test_getrandom() {
    let mut buf = [0_u8; 256];
    let _ = getrandom(&mut buf, GetRandomFlags::empty());
}

#[test]
fn test_getrandom_uninit() {
    let mut buf = unsafe { MaybeUninit::<[MaybeUninit<u8>; 256]>::uninit().assume_init() };
    let (init, uninit) = getrandom_uninit(&mut buf, GetRandomFlags::empty()).unwrap();
    let combined_len = init.len() + uninit.len();
    assert_eq!(buf.len(), combined_len);
}
