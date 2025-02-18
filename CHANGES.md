# Changes from 0.38.x to 1.0

## Behavior changes

[`rustix::pipe::fcntl_setpipe_size`] now returns the new size, which may be
greater than the requested size.

[`rustix::pipe::fcntl_setpipe_size`]: https://docs.rs/rustix/1.0.0/rustix/pipe/fn.fcntl_setpipe_size.html

## API changes

`rustix::thread::FutexOperation` and `rustix::thread::futex` are removed. Use
the functions in the [`rustix::thread::futex`] module instead.

[`rustix::thread::futex`]: https://docs.rs/rustix/1.0.0/rustix/thread/futex/index.html

[`rustix::process::waitpid`]'s return type changed from `WaitStatus` to
`(Pid, WaitStatus)`, to additionally return the pid of the child.

[`rustix::process::waitpid`]: https://docs.rs/rustix/1.0.0/rustix/process/fn.waitpid.html

The `SLAVE` flag in [`rustix::mount::MountPropagationFlags`] is renamed to
[`DOWNSTREAM`].

[`rustix::mount::MountPropagationFlags`]: https://docs.rs/rustix/1.0.0/rustix/mount/struct.MountPropagationFlags.html
[`DOWNSTREAM`]: https://docs.rs/rustix/1.0.0/rustix/mount/struct.MountPropagationFlags.html#associatedconstant.DOWNSTREAM

The "cc" and "libc-extra-traits" features are removed. The "cc" feature hasn't
had any effect for several major releases. If you need the traits provided by
"libc-extra-traits", you should instead depend on libc directly and enable its
"extra_traits" feature.

`rustix::net::Shutdown::ReadWrite` is renamed to
[`rustix::net::Shutdown::Both`] to [align with std].

[`rustix::net::Shutdown::Both`]: https://docs.rs/rustix/1.0.0/rustix/net/enum.Shutdown.html#variant.Both
[align with std]: https://doc.rust-lang.org/stable/std/net/enum.Shutdown.html#variant.Both

The `rustix::io_uring::io_uring_register_files_skip` function is replaced with
a [`IORING_REGISTER_FILES_SKIP`] constant, similar to the [`rustix::fs::CWD`]
constant.

[`IORING_REGISTER_FILES_SKIP`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/constant.IORING_REGISTER_FILES_SKIP.html
[`rustix::fs::CWD`]: https://docs.rs/rustix/1.0.0/rustix/fs/constant.CWD.html

`rustix::process::WaitidOptions` and `rustix::process::WaitidStatus` are
renamed to
[`rustix::process::WaitIdOptions`] and [`rustix::process::WaitIdStatus`] (note
the capitalization), for consistency with [`crate::process::WaitId`].

[`rustix::process::WaitIdOptions`]: https://docs.rs/rustix/1.0.0/rustix/process/struct.WaitIdOptions.html
[`rustix::process::WaitIdStatus`]: https://docs.rs/rustix/1.0.0/rustix/process/struct.WaitIdStatus.html
[`rustix::process::WaitId`]: https://docs.rs/rustix/1.0.0/rustix/process/enum.WaitId.html

The offsets in [`rustix::fs::SeekFrom::Hole`] and
[`rustix::fs::SeekFrom::Data`] are changed from `i64` to `u64` since they
represent absolute offsets.

[`rustix::fs::SeekFrom::Hole`]: https://docs.rs/rustix/1.0.0/rustix/fs/enum.SeekFrom.html#variant.Hole
[`rustix::fs::SeekFrom::Data`]: https://docs.rs/rustix/1.0.0/rustix/fs/enum.SeekFrom.html#variant.Data

Functions in [`rustix::net::sockopt`] are renamed to remove the `get_` prefix,
to [align with Rust conventions].

[`rustix::net::sockopt`]: https://docs.rs/rustix/1.0.0/rustix/net/sockopt/index.html
[align with Rust conventions]: https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter

`rustix::process::sched_*` and `rustix::process::membarrier_*` are moved from
[`rustix::process`] to [`rustix::thread`], as they operate on the current
thread rather than the current process.

[`rustix::process`]: https://docs.rs/rustix/1.0.0/rustix/process/index.html
[`rustix::thread`]: https://docs.rs/rustix/1.0.0/rustix/thread/index.html

The `udata` in [`rustix::event::kqueue::Event`] is changed from `isize` to
`*mut c_void` to better propagate pointer provenance. To use arbitrary integer
values, convert using the [`without_provenance_mut`] and the [`.addr()`]
functions.

[`rustix::event::kqueue::Event`]: https://docs.rs/rustix/1.0.0/x86_64-unknown-freebsd/rustix/event/kqueue/struct.Event.html
[`without_provenance_mut`]: https://doc.rust-lang.org/stable/std/ptr/fn.without_provenance_mut.html
[`.addr()`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.addr

`rustix::mount::mount_recursive_bind` is renamed to
[`rustix::mount::mount_bind_recursive`]. See [this comment] for details.

[`rustix::mount::mount_bind_recursive`]: https://docs.rs/rustix/1.0.0/rustix/mount/fn.mount_bind_recursive.html
[this comment]: https://github.com/bytecodealliance/rustix/pull/763#issuecomment-1662756184

The `rustix::procfs` is removed. This functionality is now available in the
[rustix-linux-procfs crate].

[rustix-linux-procfs crate]: https://crates.io/crates/rustix-linux-procfs

The `flags` field of [`rustix::net::RecvMsgReturn`] changed type from
[`RecvFlags`] to a new [`ReturnFlags`], since it supports a different set of
flags.

[`rustix::net::RecvMsgReturn`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.RecvMsgReturn.html
[`RecvFlags`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.RecvFlags.html
[`ReturnFlags`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.ReturnFlags.html

[`rustix::event::poll`]'s and [`rustix::event::epoll`]'s `timeout` argument
changed from a `c_int` where `-1` means no timeout and non-negative numbers
mean a timeout in milliseconds to an `Option<&Timespec>`. The [`Timespec`]'s
fields are `tv_sec` which holds seconds and `tv_nsec` which holds nanoseconds.

[`rustix::event::poll`]: https://docs.rs/rustix/1.0.0/rustix/event/fn.poll.html
[`rustix::event::epoll`]: https://docs.rs/rustix/1.0.0/rustix/event/fn.epoll.html
[`Timespec`]: https://docs.rs/rustix/1.0.0/rustix/time/type.Timespec.html

Functions in [`rustix::event::port`] are renamed to remove the redundant
`port_*` prefix.

[`rustix::event::port`]: https://docs.rs/rustix/1.0.0/x86_64-unknown-illumos/rustix/event/port/index.html

`rustix::fs::inotify::InotifyEvent` is renamed to
[`rustix::fs::inotify::Event`] to remove the redundant prefix.

[`rustix::fs::inotify::Event`]: https://docs.rs/rustix/1.0.0/rustix/fs/inotify/struct.Event.html

`rustix::fs::StatExt` is removed, and the timestamp fields `st_atime`,
`st_mtime`, and `st_ctime` of [`rustix::fs::Stat`] may now be accessed
directly. They are now signed instead of unsigned, so that they can represent
times before the epoch.

[`rustix::fs::Stat`]: https://docs.rs/rustix/1.0.0/rustix/fs/type.Stat.html

`rustix::io::is_read_write` is removed, as it's higher-level functionality that
can be implemented in terms of lower-level rustix calls.

[`rustix::net::recv_uninit`] and [`rustix::net::recvfrom_uninit`] now include
the number of received bytes in their return types, as this number may differ
from the number of bytes written to the buffer when
[`rustix::net::RecvFlags::TRUNC`] is used.

[`rustix::net::recv_uninit`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.recv_uninit.html
[`rustix::net::recvfrom_uninit`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.recvfrom_uninit.html
[`rustix::net::RecvFlags::TRUNC`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.RecvFlags.html#associatedconstant.TRUNC

[`rustix::io_uring::io_uring_register`] now has a [`IoringRegisterFlags`]
argument, and `rustix::io_uring::io_uring_register` is removed.

[`rustix::io_uring::io_uring_register`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/fn.io_uring_register.html
[`IoringRegisterFlags`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/struct.IoringRegisterFlags.html

[`rustix::process::Signal`] constants are now upper-cased; for example,
`Signal::Int` is now named [`Signal::INT`]. Also, `Signal` is no longer
directly convertible to `i32`; use [`Signal::as_raw`] instead.

[`rustix::process::Signal`]: https://docs.rs/rustix/1.0.0/rustix/process/enum.Signal.html
[`Signal::INT`]: https://docs.rs/rustix/1.0.0/rustix/process/enum.Signal.html#variant.Int
[`Signal::as_raw`]: https://docs.rs/rustix/1.0.0/rustix/process/enum.Signal.html#method.as_raw

The associated constant `rustix::ioctl::Ioctl::OPCODE` is now replaced with an
associated method [`rustix::ioctl::Ioctl::opcode`], to support ioctls where the
opcode is computed rather than a constant.

[`rustix::ioctl::Ioctl::opcode`]: https://docs.rs/rustix/1.0.0/rustix/ioctl/trait.Ioctl.html#tymethod.opcode

`rustix::net::RecvMsgReturn` is renamed to [`rustix::net::RecvMsg`].

[`rustix::net::RecvMsg`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.RecvMsgReturn.html

The `ifindex` argument in
[`rustix::net::sockopt::set_ip_add_membership_with_ifindex`] and
[`rustix::net::sockopt::set_ip_drop_membership_with_ifindex`]
changed from `i32` to `u32`.

[`rustix::net::sockopt::set_ip_add_membership_with_ifindex`]: https://docs.rs/rustix/1.0.0/rustix/net/sockopt/fn.set_ip_add_membership_with_ifindex.html
[`rustix::net::sockopt::set_ip_drop_membership_with_ifindex`]: https://docs.rs/rustix/1.0.0/rustix/net/sockopt/fn.set_ip_drop_membership_with_ifindex.html

The `list` argument in [`rustix::fs::listxattr`], [`rustix::fs::flistxattr`],
and [`rustix::fs::llistxattr`] changed from `[c_char]`, which is `[i8]` on some
architectures, to [`u8`].

[`rustix::fs::listxattr`]: https://docs.rs/rustix/1.0.0/rustix/fs/fn.listxattr.html
[`rustix::fs::flistxattr`]: https://docs.rs/rustix/1.0.0/rustix/fs/fn.flistxattr.html
[`rustix::fs::llistxattr`]: https://docs.rs/rustix/1.0.0/rustix/fs/fn.llistxattr.html

On NetBSD, the nanoseconds fields of [`Stat`] have been renamed, for consistency
with other platforms:

| Old name       | New Name        |
| -------------- | --------------- |
| `st_atimensec` | `st_atime_nsec` |
| `st_mtimensec` | `st_mtime_nsec` |
| `st_ctimensec` | `st_ctime_nsec` |
| `st_birthtimensec` | `st_birthtime_nsec` |

[`Stat`]: https://docs.rs/rustix/1.0.0/x86_64-unknown-netbsd/rustix/fs/type.Stat.html

[`rustix::mount::mount`]'s `data` argument is now an `Option`, so it can now
be used in place of `mount2`, and `mount2` is now removed.

[`rustix::mount::mount`]: https://docs.rs/rustix/1.0.0/rustix/mount/fn.mount.html

The [`rustix::net`] functions ending with `_v4`, `_v6`, `_unix` and `_xdp` have
been merged into a single function that accepts any address type.

Specically, the following functions are removed:

  * `bind_any`, `bind_unix`, `bind_v4`, `bind_v6`, `bind_xdp` in favor of
    [`bind`],
  * `connect_any`, `connect_unix`, `connect_v4`, `connect_v6` in favor of
    [`connect`] (leaving address-less [`connect_unspec`]),
  * `sendmsg_v4`, `sendmsg_v6`, `sendmsg_unix`, `sendmsg_xdp`, `sendmsg_any` in
    favor of [`sendmsg_addr`] (leaving address-less [`sendmsg`]),
  * `sendto_any`, `sendto_v4`, `sendto_v6`, `sendto_unix`, `sendto_xdp` in
    favor of [`sendto`].

[`rustix::net`]: https://docs.rs/rustix/1.0.0/rustix/net/index.html
[`bind`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.bind.html
[`connect`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.connect.html
[`connect_unspec`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.connect_unspec.html
[`sendmsg_addr`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.sendmsg_addr.html
[`sendmsg`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.sendmsg.html
[`sendto`]: https://docs.rs/rustix/1.0.0/rustix/net/fn.sendto.html

The `SocketAddrAny` enum has changed to a [`SocketAddrAny`] struct which can
contain any kind of socket address. It can be converted to and from the more
specific socket types using `From`/`Into`/`TryFrom`/`TryInto` conversions.

[`SocketAddrAny`]: https://docs.rs/rustix/1.0.0/rustix/net/struct.SocketAddrAny.html

The `len` parameter to [`rustix::fs::fadvise`] has changed from `u64` to
`Option<NonZeroU64>`, to reflect that zero is a special case meaning the
advice applies to the end of the file. To convert an arbitrary `u64` value to
`Option<NonZeroU64>`, use `NonZeroU64::new`.

[`rustix::fs::fadvise`]: https://docs.rs/rustix/1.0.0/rustix/fs/fn.fadvise.html

The [`sigmask`] and [`ts`] fields of [`rustix::io_uring::getevents_arg`]
changed from `u64` to [`rustix::io_uring::io_uring_ptr`], to better preserve
pointer provenance.

[`sigmask`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/struct.io_uring_getevents_arg.html#structfield.sigmask
[`ts`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/struct.io_uring_getevents_arg.html#structfield.ts
[`rustix::io_uring::getevents_arg`]: https://docs.rs/rustix/1.0.0/rustix/io_uring/struct.io_uring_getevents_arg.html
[`rustix::io_uring::io_uring_ptr`]: https://docs.rs/rustix/1.0.0-prerelease.0/rustix/io_uring/struct.io_uring_ptr.html

All explicitly deprecated functions and types have been removed. Their
deprecation messages will have identified alternatives.
