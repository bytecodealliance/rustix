//! inotify support for working with inotifies

use crate::backend::c;
use bitflags::bitflags;

bitflags! {
    /// `IN_*` for use with [`inotify_init`].
    ///
    /// [`inotify_init`]: crate::fs::inotify::inotify_init
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct CreateFlags: u32 {
        /// `IN_CLOEXEC`
        const CLOEXEC = bitcast!(c::IN_CLOEXEC);
        /// `IN_NONBLOCK`
        const NONBLOCK = bitcast!(c::IN_NONBLOCK);

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `IN*` for use with [`inotify_add_watch`].
    ///
    /// [`inotify_add_watch`]: crate::fs::inotify::inotify_add_watch
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct WatchFlags: u32 {
        /// `IN_ACCESS`
        const ACCESS = c::IN_ACCESS;
        /// `IN_ATTRIB`
        const ATTRIB = c::IN_ATTRIB;
        /// `IN_CLOSE_NOWRITE`
        const CLOSE_NOWRITE = c::IN_CLOSE_NOWRITE;
        /// `IN_CLOSE_WRITE`
        const CLOSE_WRITE = c::IN_CLOSE_WRITE;
        /// `IN_CREATE`
        const CREATE = c::IN_CREATE;
        /// `IN_DELETE`
        const DELETE = c::IN_DELETE;
        /// `IN_DELETE_SELF`
        const DELETE_SELF = c::IN_DELETE_SELF;
        /// `IN_MODIFY`
        const MODIFY = c::IN_MODIFY;
        /// `IN_MOVE_SELF`
        const MOVE_SELF = c::IN_MOVE_SELF;
        /// `IN_MOVED_FROM`
        const MOVED_FROM = c::IN_MOVED_FROM;
        /// `IN_MOVED_TO`
        const MOVED_TO = c::IN_MOVED_TO;
        /// `IN_OPEN`
        const OPEN = c::IN_OPEN;

        /// `IN_CLOSE`
        const CLOSE = c::IN_CLOSE;
        /// `IN_MOVE`
        const MOVE = c::IN_MOVE;
        /// `IN_ALL_EVENTS`
        const ALL_EVENTS = c::IN_ALL_EVENTS;

        /// `IN_DONT_FOLLOW`
        const DONT_FOLLOW = c::IN_DONT_FOLLOW;
        /// `IN_EXCL_UNLINK`
        const EXCL_UNLINK = 1;
        /// `IN_MASK_ADD`
        const MASK_ADD = 1;
        /// `IN_MASK_CREATE`
        const MASK_CREATE = 1;
        /// `IN_ONESHOT`
        const ONESHOT = c::IN_ONESHOT;
        /// `IN_ONLYDIR`
        const ONLYDIR = c::IN_ONLYDIR;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
