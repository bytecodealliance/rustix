use crate::fd::{AsFd, BorrowedFd};
use crate::io::OwnedFd;

#[derive(Debug, Clone, Copy)]
pub(crate) enum SpawnAction<'a> {
    Dup2 {
        fd: BorrowedFd<'a>,
        new: &'a OwnedFd,
    },
}

/// Specifies changes to the initial state of the process created by `posix_spawn`.
///
/// Acts as a combination of `posix_spawn_file_actions_t` and `posix_spawnattr_t`
#[derive(Default, Debug, Clone)]
pub struct SpawnConfig<'a> {
    actions: Vec<SpawnAction<'a>>,
}

impl<'a> SpawnConfig<'a> {
    /// Cause the file descriptor `fd` to be duplicated as `new`
    /// (as if `dup2(fd, new)` had been called) when the new process is spawned.
    ///
    /// # References
    /// - [POSIX]
    /// - [Linux]
    ///
    /// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/posix_spawn_file_actions_adddup2.html
    /// [Linux]: https://man7.org/linux/man-pages/man3/posix_spawn_file_actions_adddup2.3p.html
    #[inline]
    pub fn add_dup2_action<Fd: AsFd>(&mut self, fd: &'a Fd, new: &'a OwnedFd) {
        let action = SpawnAction::Dup2 {
            fd: fd.as_fd(),
            new,
        };
        self.actions.push(action);
    }

    #[inline]
    pub(crate) fn get_actions(&self) -> impl Iterator<Item = &SpawnAction<'a>> {
        self.actions.iter()
    }
}
