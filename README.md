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
`libc` APIs and syscalls, with configurable backends. It uses Rust references,
slices, and return values instead of raw pointers, and [`io-lifetimes`] instead
of raw file descriptors, providing memory safety and [I/O safety]. It uses
`Result`s for reporting errors, [`bitflags`] instead of bare integer flags,
an [`Arg`] trait with optimizations to efficiently accept any Rust string type,
and several other efficient conveniences.

`posish` is low-level and does not support Windows; for higher-level and more
portable APIs built on this functionality, see the [`system-interface`],
[`cap-std`], and [`fs-set-times`] crates, for example.

Posish currently has two backends available: `libc` and `linux_raw`.

The `libc` backend is enabled by default and uses the widely-used [`libc`]
crate which provides bindings to native `libc` libraries and is portable to
many OS's.

The `linux_raw` backend can be enabled by setting the `RUSTFLAGS` environment
variable to `--cfg linux_raw`, and uses raw Linux system calls and vDSO calls.
This only supports Linux, currently on x86-64, x86, and aarch64. It supports
stable as well as nightly Rust.
 - By being implemented entirely in Rust, avoiding `libc`, `errno`, and pthread
   cancellation, and employing some specialized optimizations, most functions
   in `linux_raw` compile down to very efficient code. On nightly Rust, they
   can often be fully inlined into user code.
 - Most functions in `linux_raw` preserve memory and I/O safety all the way
   down to the syscalls.
 - `linux_raw` uses a 64-bit `time_t` type on all platforms, avoiding the
   [y2038 bug].

[`system-interface`]: https://crates.io/crates/system-interface
[`fs-set-times`]: https://crates.io/crates/fs-set-times
[`io-lifetimes`]: https://crates.io/crates/io-lifetimes
[`libc`]: https://crates.io/crates/libc
[`cap-std`]: https://crates.io/crates/cap-std
[`bitflags`]: https://crates.io/crates/bitflags
[`Arg`]: https://docs.rs/posish/0.14.1/posish/path/trait.Arg.html
[I/O-safe]: https://github.com/rust-lang/rfcs/pull/3128
[I/O safety]: https://github.com/rust-lang/rfcs/pull/3128
[y2038 bug]: https://en.wikipedia.org/wiki/Year_2038_problem
