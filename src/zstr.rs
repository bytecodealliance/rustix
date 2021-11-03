/// A simple macro for `ZStr` literals that doesn't depend on `proc_macro2` or
/// `syn` or anything else. Embedded NULs are not diagnosed at compile time,
/// but are diagnosed at runtime in `debug_assertions` builds.
#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! zstr {
    ($str:literal) => {
        crate::ffi::ZStr::from_bytes_with_nul(concat!($str, "\0").as_bytes()).unwrap()
    };
}
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! zstr {
    ($str:literal) => {{
        #[allow(unsafe_code, unused_unsafe)]
        unsafe {
            crate::ffi::ZStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes())
        }
    }};
}

#[cfg(not(feature = "rustc-dep-of-std"))]
#[allow(unused_macros)]
macro_rules! cstr {
    ($str:literal) => {{
        zstr!($str)
    }};
}

#[test]
fn test_zstr() {
    use crate::ffi::ZString;
    assert_eq!(zstr!(""), &*ZString::new("").unwrap());
    assert_eq!(zstr!("").to_owned(), ZString::new("").unwrap());
    assert_eq!(zstr!("hello"), &*ZString::new("hello").unwrap());
    assert_eq!(zstr!("hello").to_owned(), ZString::new("hello").unwrap());
}
