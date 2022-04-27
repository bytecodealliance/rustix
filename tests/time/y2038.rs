/// Test that `Timespec` and `Secs` support a 64-bit number of seconds,
/// avoiding the y2038 bug.
///
/// The Rust Musl target and libc crate are currently using Musl 1.1. It is
/// expected to update to Musl 1.2 at some point, at which point it'll gain a
/// 64-bit `time_t`.
///
/// 32-bit Android is [not y2038 compatible]. In theory we could use
/// `libc::syscall` and call the new syscalls ourselves, however that doesn't
/// seem worth the effort on a platform that will likely never support add
/// such support itself.
///
/// [not y2038 compatible]: https://android.googlesource.com/platform/bionic/+/refs/heads/master/docs/32-bit-abi.md#is-32_bit-on-lp32-y2038
#[test]
#[cfg(not(all(target_env = "musl", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "android", target_pointer_width = "32")))]
#[cfg(not(all(target_os = "emscripten", target_pointer_width = "32")))]
fn test_y2038() {
    use rustix::time::{Secs, Timespec};

    let tv_sec: i64 = 0;
    let _ = Timespec { tv_sec, tv_nsec: 0 };
    let _: Secs = tv_sec;

    #[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]
    {
        use rustix::time::Itimerspec;

        let _ = Itimerspec {
            it_interval: Timespec { tv_sec, tv_nsec: 0 },
            it_value: Timespec { tv_sec, tv_nsec: 0 },
        };
    }
}
