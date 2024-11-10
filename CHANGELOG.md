# Changes from 0.38.x to 1.0

`rustix::pipe::fcntl_getpipe_size` now returns the new size, which may be
greater than the requested size.

`rustix::thread::FutexOperation` and `rustix::thread::futex` are removed. Use
the functions in the `rustix::thread::futex` module instead.

`rustix::process::waitpid`'s return type changed from `WaitStatus` to
`(Pid, WaitStatus)`, to additionally return the pid of the child.

The `SLAVE` flag in `rustix::mount::MountPropagationFlags` is renamed
to `DOWNSTREAM`.

The "cc" feature is removed. It hasn't had any effect for several
major releases.

All explicitly deprecated functions and types have been removed. Their
deprecation messages will have identified alternatives.
