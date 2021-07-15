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

// When inline asm is available, use it.
#[cfg(all(linux_raw_inline_asm, target_arch = "aarch64"))]
pub(crate) mod aarch64;
#[cfg(all(linux_raw_inline_asm, target_arch = "riscv64"))]
pub(crate) mod riscv64;
#[cfg(all(linux_raw_inline_asm, target_arch = "x86"))]
pub(crate) mod x86;
#[cfg(all(linux_raw_inline_asm, target_arch = "x86_64"))]
pub(crate) mod x86_64;
#[cfg(all(linux_raw_inline_asm, target_arch = "aarch64"))]
pub(crate) use self::aarch64 as asm;
#[cfg(all(linux_raw_inline_asm, target_arch = "riscv64"))]
pub(crate) use self::riscv64 as asm;
#[cfg(all(linux_raw_inline_asm, target_arch = "x86"))]
pub(crate) use self::x86 as asm;
#[cfg(all(linux_raw_inline_asm, target_arch = "x86_64"))]
pub(crate) use self::x86_64 as asm;

// When inline asm isn't available, use out-of-line asm.
#[cfg(not(linux_raw_inline_asm))]
pub(crate) mod outline;
#[cfg(not(linux_raw_inline_asm))]
pub(crate) use self::outline as asm;

// On aarch64, x86_64, and riscv64, the architecture syscall instruction is
// fast, so use it directly.
#[cfg(target_arch = "aarch64")]
pub(crate) use self::asm as choose;
#[cfg(target_arch = "x86_64")]
pub(crate) use self::asm as choose;
#[cfg(target_arch = "riscv64")]
pub(crate) use self::asm as choose;

// On x86, use vDSO wrappers. We could use the architecture syscall
// instruction, but the vDSO kernel_vsyscall mechanism is much faster.
#[cfg(target_arch = "x86")]
pub(crate) use super::vdso_wrappers::x86_via_vdso as choose;
//#[cfg(target_arch = "x86")]
//pub(crate) use self::asm as choose;
