//! Linux `mount` API.

// The `mount` module includes the `mount` function and related
// functions which were originally defined in `rustix::fs` but are
// now replaced by deprecated aliases.
//
// The `fsopen` module includes `fsopen` and related functions.

#[cfg(feature = "mount")]
mod fsopen;
mod mount;

#[cfg(feature = "mount")]
pub use fsopen::*;
pub use mount::*;
