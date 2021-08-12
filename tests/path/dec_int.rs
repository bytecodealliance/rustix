use rsix::path::DecInt;

#[test]
fn test_dec_int() {
    assert_eq!((*DecInt::new(0)).to_str().unwrap(), "0");
    assert_eq!((*DecInt::new(-1)).to_str().unwrap(), "-1");
    assert_eq!((*DecInt::new(789)).to_str().unwrap(), "789");
    assert_eq!(
        (*DecInt::new(i64::MIN)).to_str().unwrap(),
        i64::MIN.to_string()
    );
    assert_eq!(
        (*DecInt::new(i64::MAX)).to_str().unwrap(),
        i64::MAX.to_string()
    );
    assert_eq!(
        (*DecInt::new(u64::MAX)).to_str().unwrap(),
        u64::MAX.to_string()
    );
}
