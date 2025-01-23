//! Tests for [`rustix::termios`].

#![cfg(feature = "termios")]

#[cfg(not(windows))]
mod isatty;
#[cfg(not(any(windows, target_os = "wasi")))]
mod pgrp;
#[cfg(not(any(windows, target_os = "wasi")))]
mod sid;
#[cfg(all(not(windows), feature = "pty"))]
mod termios;
#[cfg(feature = "fs")]
#[cfg(not(any(windows, target_os = "fuchsia")))]
mod ttyname;
