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
safe interfaces, including considering raw file descriptors to be equivalent
to pointers in terms of making APIs unsafe, in the spirit of
[`std::os::unix::io::FromRawFd::from_raw_fd`] being unsafe and
[`RawFd` not implementing `AsRawFd`/`IntoRawFd`].

`posish` is relatively low-level and does not support Windows; for higher-level
and portable APIs to this functionality, see the [`system-interface`],
[`cap-std`], and [`fs-set-times`] crates.

[`std`]: https://doc.rust-lang.org/std/
[`getrandom`]: https://crates.io/crates/getrandom
[`errno`]: https://crates.io/crates/errno
[`std::os::unix::io::FromRawFd::from_raw_fd`]: https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html#tymethod.from_raw_fd
[`system-interface`]: https://crates.io/crates/system-interface
[`fs-set-times`]: https://crates.io/crates/fs-set-times
[`cap-std`]: https://crates.io/crates/cap-std
[`RawFd` not implementing `AsRawFd`/`IntoRawFd`]: https://github.com/rust-lang/rust/pull/41035
