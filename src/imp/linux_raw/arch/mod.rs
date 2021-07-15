//! # Safety
//!
//! This contains the inline `asm` statements performing the syscall
//! instructions and FFI declarations declaring the out-of-line ("outline")
//! syscall instructions.
#![allow(unsafe_code)]

// When inline asm is available, use it.
#[cfg(linux_raw_inline_asm)]
pub(crate) mod inline;
#[cfg(linux_raw_inline_asm)]
pub(crate) use self::inline as asm;

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

// On x86, use vDSO wrappers for all syscalls. We could use the architecture
// syscall instruction (int 0x80), but the vDSO kernel_vsyscall mechanism is
// much faster.
#[cfg(target_arch = "x86")]
pub(crate) use super::vdso_wrappers::x86_via_vdso as choose;
//#[cfg(target_arch = "x86")]
//pub(crate) use self::asm as choose;
