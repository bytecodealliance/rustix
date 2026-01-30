# Copyright © 2026 Google
# SPDX-License-Identifier: MIT

project(
  'bitflags',
  'rust',
  version : '2.9.1',
  license : 'MIT OR Apache-2.0',
  meson_version : '>= 1.3.0',
)

lib = static_library(
  'bitflags',
  'src/lib.rs',
  override_options : ['rust_std=2021', 'build.rust_std=2021', 'warning_level=0'],
  rust_abi : 'rust',
)

dep_bitflags = declare_dependency(
  link_with : [lib]
)
