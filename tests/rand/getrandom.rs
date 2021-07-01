use posish::rand::{getrandom, GetRandomFlags};

#[test]
fn test_getrandom() {
    let mut buf = [0u8; 256];
    let _ = getrandom(&mut buf, GetRandomFlags::empty());
}
