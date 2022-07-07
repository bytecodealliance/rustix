#[test]
fn test_statx_unknown_flags() {
    use rustix::fs::{AtFlags, StatxFlags};

    let f = std::fs::File::open(".").unwrap();

    // It's ok (though still unwise) to construct flags values that have
    // unknown bits. Exclude `STATX__RESERVED` here as that evokes an explicit
    // failure; that's tested separately below.
    let too_many_flags =
        unsafe { StatxFlags::from_bits_unchecked(!0 & !linux_raw_sys::general::STATX__RESERVED) };

    // It's also ok to pass such flags to `statx`.
    let result = rustix::fs::statx(&f, "Cargo.toml", AtFlags::empty(), too_many_flags).unwrap();

    // But, rustix should mask off bits it doesn't recognize, because these
    // extra flags may tell future kernels to set extra fields beyond the
    // extend of rustix's statx buffer. So make sure we didn't get extra
    // fields.
    assert_eq!(result.stx_mask & !StatxFlags::all().bits(), 0);
}

#[test]
fn test_statx_reserved() {
    use rustix::fs::{AtFlags, StatxFlags};

    let f = std::fs::File::open(".").unwrap();

    // It's ok (though still unwise) to construct a `STATX__RESERVED` flag
    // value but `statx` should reliably fail with `INVAL`.
    let reserved =
        unsafe { StatxFlags::from_bits_unchecked(linux_raw_sys::general::STATX__RESERVED) };
    assert_eq!(
        rustix::fs::statx(&f, "Cargo.toml", AtFlags::empty(), reserved).unwrap_err(),
        rustix::io::Errno::INVAL
    );
}
