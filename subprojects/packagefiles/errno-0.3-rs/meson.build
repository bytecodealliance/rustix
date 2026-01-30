# Copyright © 2026 Google
# SPDX-License-Identifier: MIT

project(
  'errno',
  'rust',
  version : '0.3.12',
  license : 'MIT OR Apache-2.0',
  meson_version : '>= 1.3.0',
)

os_deps = []
libc = subproject('libc-0.2-rs').get_variable('lib')
if host_machine.system() == 'windows'
  os_deps += subproject('windows-sys-0.6-rs').get_variable('lib')
endif

lib = static_library(
  'libc_errno',
  'src/lib.rs',
  override_options : ['rust_std=2018', 'build.rust_std=2018', 'warning_level=0'],
  link_with: [libc] + os_deps,
  rust_abi : 'rust',
)

dep_errno = declare_dependency(
  link_with : [lib]
)
