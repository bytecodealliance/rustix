<div align="center">
  <h1><code>posish</code></h1>

  <p>
    <strong>Safe Rust bindings to POSIX-ish libc APIs and syscalls</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/posish/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/posish/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/posish"><img src="https://img.shields.io/crates/v/posish.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/posish"><img src="https://docs.rs/posish/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

`posish` provides efficient memory-safe and [I/O-safe] wrappers to "POSIX-ish"
`libc` APIs and syscalls.

`posish` is relatively low-level and does not support Windows; for higher-level
and portable APIs to this functionality, see the [`system-interface`],
[`cap-std`], and [`fs-set-times`] crates.

### Linux raw syscall support

On Linux, `posish` can optionally be configured to use the raw Linux syscall
ABI directly instead of `libc`. To enable this, use Rust nightly, and set the
`RUSTFLAGS` environment variable to `--cfg linux_raw`, or otherwise pass
`--cfg linux_raw` to rustc. This mode is new, and so far only supports x86-64,
x86, and aarch64, but a fair amount of code has been successfully adapted to
use it, and ports to most other architectures should be straightforward.

This feature is fun in four ways:
 - By being implemented entirely in Rust, avoiding `libc`, `errno`, and pthread
   cancellation, and using type layout optimizations, most functions compile
   down to very simple code and can even be fully inlined into user code.
 - Memory buffers are kept in Rust slices, out parameters are returned as
   regular values, and file descriptors are kept in [`io-lifetimes`] types,
   so most functions preserve memory safety and I/O safety all the way down
   to the syscalls.
 - It uses a crate-level `deny(unsafe_code)`. While there are a few places that
   use `allow(unsafe_code)` locally to override this in places where `unsafe`
   is unavoidable, most of the code is safe.
 - It uses a 64-bit `time_t` type on 32-bit platforms, avoiding the
   [y2038 bug].

[`std`]: https://doc.rust-lang.org/std/
[`getrandom`]: https://crates.io/crates/getrandom
[`errno`]: https://crates.io/crates/errno
[`system-interface`]: https://crates.io/crates/system-interface
[`fs-set-times`]: https://crates.io/crates/fs-set-times
[`io-lifetimes`]: https://crates.io/crates/io-lifetimes
[`cap-std`]: https://crates.io/crates/cap-std
[I/O-safe]: https://github.com/rust-lang/rfcs/pull/3128
[y2038 bug]: https://en.wikipedia.org/wiki/Year_2038_problem
