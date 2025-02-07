//! Unimplemented functions.
//!
//! This module contains documentation for several functions that rustix does
//! not implement, either because they are out of scope, or because they are
//! could probably be implemented but are not yet.

macro_rules! not_implemented {
    ($func:ident) => {
        /// See the [module comment](self).
        pub fn $func() {
            unimplemented!()
        }
    };
}

/// Memory-allocation functions are out of scope for rustix.
///
/// It is possible to implement `malloc`, `free`, and similar functions in
/// Rust, however rustix itself is focused on syscall-like functions.
///
/// There are several allocator implementations for Rust; one of them is
/// [dlmalloc]. For a rustix-based implementation, see [rustix-dlmalloc].
///
/// [dlmalloc]: https://crates.io/crates/dlmalloc
/// [rustix-dlmalloc]: https://github.com/sunfishcode/rustix-dlmalloc
pub mod memory_allocation {
    not_implemented!(malloc);
    not_implemented!(realloc);
    not_implemented!(calloc);
    not_implemented!(free);
    not_implemented!(posix_memalign);
    not_implemented!(aligned_alloc);
    not_implemented!(malloc_usable_size);
}

/// Functions which need access to libc internals are out of scope for rustix.
///
/// Most Rust programs have a libc present, and when a libc is present, it
/// expects to be the only thing in the process that can do certain operations.
/// For example, there can be only one `atexit` list in a process, only one
/// `pthread_atfork` list in a process, only one implementation of pthreads in
/// a process, and so on, and libc expects to own the one of each of those
/// things. And libc implementations may expect to be involved in signal
/// handling. So, these functions are believed to be out of scope for rustix.
///
/// It would be possible to make a rust library which provides safe or
/// ergonomic wrappers around these libc functions, however that is out of
/// scope for rustix itself.
///
/// If you would like to write a Rust program which does not use a libc, and
/// which does provide APIs for some of these functions, [Eyra] and [origin]
/// are two libraries which may be useful, and which provide public interfaces
/// for some of this functionality.
///
/// If you are otherwise writing Rust code which you know will not share a
/// process with a libc, perhaps because you are writing a libc or similar
/// yourself, rustix's codebase does include experimental implementations of
/// the primitives needed to implement these functions.
///
/// [Eyra]: https://github.com/sunfishcode/eyra?tab=readme-ov-file#eyra
/// [origin]: https://github.com/sunfishcode/origin?tab=readme-ov-file#origin
pub mod libc_internals {
    not_implemented!(exit);
    not_implemented!(fork);
    not_implemented!(brk);
    not_implemented!(pthread_create);
    not_implemented!(pthread_mutex_init);
    not_implemented!(pthread_setschedparam);
    not_implemented!(pthread_setschedprio);
    not_implemented!(sigaction);
    not_implemented!(sigaltstack);
    not_implemented!(sigprocmask);
    not_implemented!(sigwait);
    not_implemented!(sigwaitinfo);
    not_implemented!(sigtimedwait);
    not_implemented!(set_thread_area);
    not_implemented!(set_tid_address);
    not_implemented!(tkill);
    not_implemented!(sched_setscheduler);
}

/// Functions which provide higher-level functionality are out of scope for
/// rustix.
///
/// These functions are provided by typical libc implementations, but are
/// higher-level than the simple syscall-like functions that rustix focuses
/// on. They could be implemented as a separate library built on top of rustix,
/// rather than being part of rustix itself.
pub mod higher_level {
    not_implemented!(getpwent);
    not_implemented!(getpwuid);
    not_implemented!(getpwnam);
    not_implemented!(getpwuid_r);
    not_implemented!(getpwnam_r);
    not_implemented!(gethostbyname);
    not_implemented!(execv);
    not_implemented!(execvp);
    not_implemented!(execvpe);
    not_implemented!(execvpe);
    not_implemented!(wordexp);

    /// See [rustix-openpty](https://github.com/sunfishcode/rustix-openpty).
    pub fn closefrom() {
        unimplemented!()
    }
    /// See [rustix-openpty](https://github.com/sunfishcode/rustix-openpty).
    pub fn login_tty() {
        unimplemented!()
    }
    /// See [rustix-openpty](https://github.com/sunfishcode/rustix-openpty).
    pub fn openpty() {
        unimplemented!()
    }

    /// See [`std::io::IsTerminal`].
    ///
    /// For Rust < 1.70, see [is-terminal]. For a rustix-based implementation,
    /// see [rustix-is-terminal].
    ///
    /// [`std::io::IsTerminal`]: https://doc.rust-lang.org/stable/std/io/trait.IsTerminal.html
    /// [is-terminal]: https://crates.io/crates/is-terminal
    /// [rustix-is-terminal]: https://github.com/sunfishcode/rustix-is-terminal
    pub fn isatty() {
        unimplemented!()
    }
}

/// These functions are not yet implemented in rustix, but probably could be.
///
/// These are functions that users have asked about, and which probably are
/// in scope for rustix, but are not yet implemented.
pub mod yet {
    not_implemented!(tgkill);
    not_implemented!(raise);
    not_implemented!(sysctl);
    not_implemented!(mq_open);
    not_implemented!(mq_send);
    not_implemented!(mq_unlink);
    not_implemented!(recvmmsg);
    not_implemented!(cachestat);
    not_implemented!(fanotify_init);
    not_implemented!(fanotify_mark);
    not_implemented!(getifaddrs);
    not_implemented!(signalfd);
    not_implemented!(pidfd_send_signal);
    not_implemented!(mount_setattr);
    not_implemented!(extattr_delete_fd);
    not_implemented!(extattr_delete_link);
    not_implemented!(extattr_get_fd);
    not_implemented!(extattr_get_link);
    not_implemented!(extattr_list_fd);
    not_implemented!(extattr_list_link);
    not_implemented!(extattr_set_fd);
    not_implemented!(extattr_set_link);
    not_implemented!(get_mempolicy);
    not_implemented!(mbind);
    not_implemented!(set_mempolicy);
    not_implemented!(migrate_pages);
    not_implemented!(move_pages);
    not_implemented!(fchmodat2);
    not_implemented!(shmat);
    not_implemented!(shmdt);
    not_implemented!(shmget);
    not_implemented!(shmctl);
}

/// These functions are not yet finished in rustix.
///
/// Rustix's codebase includes experimental implementations of these functions,
/// however they are not yet publicly exposed because their API might need more
/// work and/or they don't yet have a libc backend implementation.
///
/// See [#1314] for more information, and please leave comments if there are
/// specific functions you're interested in.
///
/// [#1314]: https://github.com/bytecodealliance/rustix/issues/1314
pub mod quite_yet {
    not_implemented!(_exit);
    not_implemented!(_Exit);
    not_implemented!(exit_group);
    not_implemented!(sigpending);
    not_implemented!(sigsuspend);
    not_implemented!(execveat);
    not_implemented!(execve);

    /// For now, use `rustix::process::uname().nodename()` instead.
    ///
    /// See also the [module comment](self).
    pub fn gethostname() {
        unimplemented!()
    }
}
