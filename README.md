<div align="center">
  <h1><code>rsix</code></h1>

  <p>
    <strong>Safe Rust ("rs") bindings to POSIX-like/Unix-like/Linux ("ix") syscalls</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/rsix/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/rsix/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/rsix"><img src="https://img.shields.io/crates/v/rsix.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/rsix"><img src="https://docs.rs/rsix/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

`rsix` (formerly known as `posish`) provides efficient memory-safe and
[I/O-safe] wrappers to POSIX-like, Unix-like, and Linux syscall APIs, with
configurable backends. It uses Rust references, slices, and return values
instead of raw pointers, and [`io-lifetimes`] instead of raw file descriptors,
providing memory safety and [I/O safety]. It uses `Result`s for reporting
errors, [`bitflags`] instead of bare integer flags, an [`Arg`] trait with
optimizations to efficiently accept any Rust string type, and several other
efficient conveniences.

`rsix` is low-level and does not support Windows; for higher-level and more
portable APIs built on this functionality, see the [`system-interface`],
[`cap-std`], and [`fs-set-times`] crates, for example.

`rsix` currently has two backends available: `linux_raw` and `libc`.

The `linux_raw` backend is enabled by default on Linux on x86-64, x86, aarch64,
and riscv64gc, and uses raw Linux system calls and vDSO calls. It supports
stable as well as nightly Rust.
 - By being implemented entirely in Rust, avoiding `libc`, `errno`, and pthread
   cancellation, and employing some specialized optimizations, most functions
   compile down to very efficient code. On nightly Rust, they can often be
   fully inlined into user code.
 - Most functions in `linux_raw` preserve memory and I/O safety all the way
   down to the syscalls.
 - `linux_raw` uses a 64-bit `time_t` type on all platforms, avoiding the
   [y2038 bug].

The `libc` backend is enabled by default on all other platforms, and can be
explicitly for any target by setting `RUSTFLAGS` to `--cfg rsix_use_libc`.
It uses the [`libc`] crate which provides bindings to native `libc` libraries
and is portable to many OS's.

[`system-interface`]: https://crates.io/crates/system-interface
[`fs-set-times`]: https://crates.io/crates/fs-set-times
[`io-lifetimes`]: https://crates.io/crates/io-lifetimes
[`libc`]: https://crates.io/crates/libc
[`cap-std`]: https://crates.io/crates/cap-std
[`bitflags`]: https://crates.io/crates/bitflags
[`Arg`]: https://docs.rs/rsix/0.14.1/rsix/path/trait.Arg.html
[I/O-safe]: https://github.com/rust-lang/rfcs/pull/3128
[I/O safety]: https://github.com/rust-lang/rfcs/pull/3128
[y2038 bug]: https://en.wikipedia.org/wiki/Year_2038_problem
