//! Linux `mount`.
//!
//! These have been moved to a new `rustix::mount` module.

#[deprecated(note = "These functions have moved to `rustix::mount`.")]
pub use crate::mount::{mount, unmount};

#[deprecated(
    note = "`rustix::fs::remount` is renamed and moved to `rustix::mount::mount_remount`."
)]
pub use crate::mount::mount_remount as remount;

#[deprecated(
    note = "`rustix::fs::bind_mount` is renamed and moved to `rustix::mount::mount_bind`."
)]
pub use crate::mount::mount_bind as bind_mount;

#[deprecated(
    note = "`rustix::fs::recursive_bind_mount` is renamed and moved to `rustix::mount::mount_recursive_bind`."
)]
pub use crate::mount::mount_recursive_bind as recursive_bind_mount;

#[deprecated(
    note = "`rustix::fs::change_mount` is renamed and moved to `rustix::mount::mount_change`."
)]
pub use crate::mount::mount_change as change_mount;

#[deprecated(
    note = "`rustix::fs::move_mount` is renamed and moved to `rustix::mount::mount_move`."
)]
pub use crate::mount::mount_move as move_mount;
