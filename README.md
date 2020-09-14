<div align="center">
  <h1><code>posish</code></h1>

  <p>
    <strong>Safe Rust bindings to POSIX-ish libc APIs</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/posish/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/posish/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/posish"><img src="https://img.shields.io/crates/v/posish.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/posish"><img src="https://docs.rs/posish/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

`posish` provides safe wrappers to POSIX-ish `libc` APIs.

`posish` is focused on functionality that isn't already provided by [`std`]
or [`getrandom`]. It has a philosophy of considering raw file descriptors to be
equivalent to pointers in terms of making APIs unsafe, in the spirit of
[`std::os::unix::io::FromRawFd::from_raw_fd`] being unsafe.

`posish` is relatively low-level and does not support Windows; for higher-level
and portable APIs to this functionality, see the [`system-interface`] and
[`cap-std`] crates.

[`std`]: https://doc.rust-lang.org/std/
[`getrandom`]: https://crates.io/crates/getrandom
[`std::os::unix::io::FromRawFd::from_raw_fd`]: https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html#tymethod.from_raw_fd
[`system-interface`]: https://crates.io/crates/system-interface
[`cap-std`]: https://crates.io/crates/cap-std
