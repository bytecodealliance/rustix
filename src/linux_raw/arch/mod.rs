//! Compilers should really have intrinsics for making system calls. They're
//! much like regular calls, with custom calling conventions, and calling
//! conventions are otherwise the compiler's job. But for now, use inline asm.

#![allow(dead_code)]

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub(crate) use aarch64::*;
#[cfg(target_arch = "x86")]
pub(crate) use x86::*;
#[cfg(target_arch = "x86_64")]
pub(crate) use x86_64::*;
