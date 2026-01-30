# Copyright © 2026 Google
# SPDX-License-Identifier: MIT

project(
  'rustix',
  'rust',
  version : '1.1.3',
  license : 'Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT',
)

dep_errno = dependency('errno-0.3-rs', fallback: ['errno-0.3-rs', 'dep_errno'])
dep_libc = dependency('libc-0.2-rs', fallback: ['libc-0.2-rs', 'dep_libc'])
dep_bitflags = dependency('bitflags-2-rs', fallback: ['bitflags-2-rs', 'dep_bitflags'])

rustix_args = [
  '--cfg', 'libc',
  '--cfg', 'feature="use-libc"',
]

os_deps = []
if host_machine.system() == 'linux' or host_machine.system() == 'android'
  rustix_args += [
  '--cfg', 'linux_like',
  '--cfg', 'linux_kernel',
  '--cfg', 'feature="std"',
  '--cfg', 'feature="alloc"',
  '--cfg', 'feature="event"',
  '--cfg', 'feature="fs"',
  '--cfg', 'feature="mm"',
  '--cfg', 'feature="net"',
  '--cfg', 'feature="param"',
  '--cfg', 'feature="pipe"',
  '--cfg', 'feature="thread"',
]
elif host_machine.system() == 'windows'
  os_deps += [dependency('windows-sys-0.6-rs', fallback: ['windows-sys-0.6-rs', 'dep_windows_sys'])]
endif

lib = static_library(
  'rustix',
  'src/lib.rs',
  override_options : ['rust_std=2021', 'build.rust_std=2021'],
  dependencies : [dep_errno, dep_libc, dep_bitflags] + os_deps,
  rust_abi : 'rust',
  rust_args: rustix_args,
)
