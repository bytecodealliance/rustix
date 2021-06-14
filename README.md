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

`posish` provides safe wrappers to POSIX-ish `libc` APIs and syscalls.

`posish` is focused on functionality that isn't already provided by [`std`]
or other low-level crates such as [`getrandom`] and [`errno`]. It prioritizes
memory safety and [I/O safety].

`posish` is relatively low-level and does not support Windows; for higher-level
and portable APIs to this functionality, see the [`system-interface`],
[`cap-std`], and [`fs-set-times`] crates.

## Linux raw syscall support

On Linux, `posish` can optionally be configured to target the raw
Linux syscall ABI directly instead of calling through `libc`. To enable this,
add `--cfg linux_raw` to the `RUSTFLAGS` environment variable, or otherwise
pass `--cfg linux_raw` to rustc.

[`std`]: https://doc.rust-lang.org/std/
[`getrandom`]: https://crates.io/crates/getrandom
[`errno`]: https://crates.io/crates/errno
[`system-interface`]: https://crates.io/crates/system-interface
[`fs-set-times`]: https://crates.io/crates/fs-set-times
[`cap-std`]: https://crates.io/crates/cap-std
[I/O safety]: https://github.com/sunfishcode/io-lifetimes
