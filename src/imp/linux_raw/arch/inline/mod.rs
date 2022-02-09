//! Compilers should really have intrinsics for making system calls. They're
//! much like regular calls, with custom calling conventions, and calling
//! conventions are otherwise the compiler's job. But for now, use inline asm.

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "mips")]
mod mips;
#[cfg(target_arch = "mips64")]
mod mips64;
#[cfg(target_arch = "powerpc64")]
mod powerpc64;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub(in crate::imp) use self::aarch64::*;
#[cfg(target_arch = "arm")]
pub(in crate::imp) use self::arm::*;
#[cfg(target_arch = "mips")]
pub(in crate::imp) use self::mips::*;
#[cfg(target_arch = "mips64")]
pub(in crate::imp) use self::mips64::*;
#[cfg(target_arch = "powerpc64")]
pub(in crate::imp) use self::powerpc64::*;
#[cfg(target_arch = "riscv64")]
pub(in crate::imp) use self::riscv64::*;
#[cfg(target_arch = "x86")]
pub(in crate::imp) use self::x86::*;
#[cfg(target_arch = "x86_64")]
pub(in crate::imp) use self::x86_64::*;
