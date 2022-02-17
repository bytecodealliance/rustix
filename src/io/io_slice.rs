#[cfg(not(feature = "std"))]
use crate::imp;

// Declare `IoSlice` and `IoSliceMut`.
#[cfg(not(feature = "std"))]
pub use imp::io::{IoSlice, IoSliceMut};
#[cfg(feature = "std")]
pub use std::io::{IoSlice, IoSliceMut};
