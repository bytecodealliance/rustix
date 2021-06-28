//! Compilers should really have intrinsics for making system calls. They're
//! much like regular calls, with custom calling conventions, and calling
//! conventions are otherwise the compiler's job. But for now, use inline asm.
//!
//! # Safety
//!
//! This contains the `asm` statements performing the syscall instructions.
#![allow(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(target_arch = "aarch64")]
pub(crate) mod aarch64;
#[cfg(target_arch = "x86")]
pub(crate) mod x86;
#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

#[cfg(target_arch = "aarch64")]
pub(crate) use aarch64 as asm;
#[cfg(target_arch = "x86")]
pub(crate) use x86 as asm;
#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64 as asm;

// On aarch64 and x86_64, the architecture syscall instruction is fast, so
// use it directly. On x86, use vDSO wrappers.
#[cfg(target_arch = "x86")]
pub(crate) use super::vdso_wrappers::x86_via_vdso as choose;
#[cfg(target_arch = "aarch64")]
pub(crate) use asm as choose;
#[cfg(target_arch = "x86_64")]
pub(crate) use asm as choose;
