//! The following is derived from Rust's
//! library/std/src/io/mod.rs at revision
//! dca3f1b786efd27be3b325ed1e01e247aa589c3b.

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is used by the [`Seek`] trait.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    #[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))]
    Start(#[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))] u64),

    /// Sets the offset to the size of this object plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error
    /// to seek before byte 0.
    #[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))]
    End(#[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))] i64),

    /// Sets the offset to the current position plus the specified number of
    /// bytes.
    ///
    /// It is possible to seek beyond the end of an object, but it's an error
    /// to seek before byte 0.
    #[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))]
    Current(#[cfg_attr(staged_api, stable(feature = "rust1", since = "1.0.0"))] i64),

    /// Sets the offset to next location in the file greater than or equal to the
    /// specified number of bytes that contains data.
    ///
    /// If the offset points to data, the file offset is set to it.
    #[cfg(any(
        target_os = "linux",
        target_os = "solaris",
        target_os = "freebsd",
        target_os = "dragonfly",
    ))]
    Data(i64),

    /// Sets the offset to the next hole in the file greater than or equal to the
    /// specified number of bytes.
    ///
    /// If offset points into the middle of a hole, then the file offset is set to
    /// offset.
    ///
    /// If there is no hole past offset, then the file offset is adjusted to the end
    /// of the file (i.e., there is an implicit hole at the end of any file).
    #[cfg(any(
        target_os = "linux",
        target_os = "solaris",
        target_os = "freebsd",
        target_os = "dragonfly",
    ))]
    Hole(i64),
}
