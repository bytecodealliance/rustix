#[test]
fn test_statx_unknown_flags() {
    use rustix::fs::{AtFlags, StatxFlags};

    let f = std::fs::File::open(".").unwrap();

    // It's ok (though still unwise) to construct flags values that have
    // unknown bits. Exclude `STATX__RESERVED` here as that evokes an explicit
    // failure; that's tested separately below.
    #[cfg(not(linux_raw_dep))]
    const STATX__RESERVED: u32 = libc::STATX__RESERVED as u32;
    #[cfg(linux_raw_dep)]
    const STATX__RESERVED: u32 = linux_raw_sys::general::STATX__RESERVED;
    let too_many_flags = StatxFlags::from_bits_retain(!STATX__RESERVED);

    // It's also ok to pass such flags to `statx`.
    let result = match rustix::fs::statx(&f, "Cargo.toml", AtFlags::empty(), too_many_flags) {
        // If we don't have `statx` at all, skip the rest of this test.
        Err(rustix::io::Errno::NOSYS) => return,
        otherwise => otherwise.unwrap(),
    };

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
    #[cfg(not(linux_raw_dep))]
    const STATX__RESERVED: u32 = libc::STATX__RESERVED as u32;
    #[cfg(linux_raw_dep)]
    const STATX__RESERVED: u32 = linux_raw_sys::general::STATX__RESERVED;
    let reserved = StatxFlags::from_bits_retain(STATX__RESERVED);
    match rustix::fs::statx(&f, "Cargo.toml", AtFlags::empty(), reserved) {
        Ok(_) => panic!("statx succeeded with `STATX__RESERVED`"),
        Err(err) => assert_eq!(err, rustix::io::Errno::INVAL),
    }
}
