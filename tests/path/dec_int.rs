use rustix::path::DecInt;

macro_rules! check {
    ($i:expr) => {
        let i = $i;
        assert_eq!(DecInt::new(i).as_ref().to_str().unwrap(), i.to_string());
    };
}

#[test]
fn test_dec_int() {
    check!(0);
    check!(-1);
    check!(789);

    check!(u8::MAX);
    check!(i8::MIN);
    check!(u16::MAX);
    check!(i16::MIN);
    check!(u32::MAX);
    check!(i32::MIN);
    check!(u64::MAX);
    check!(i64::MIN);
    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ))]
    {
        check!(usize::MAX);
        check!(isize::MIN);
    }
}

#[test]
#[should_panic]
fn test_unsupported_max_u128_dec_int() {
    check!(u128::MAX);
}

#[test]
#[should_panic]
fn test_unsupported_min_u128_dec_int() {
    check!(i128::MIN);
}
