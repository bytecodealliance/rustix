//! Declare functions defined in out-of-line asm files.

#[cfg(target_arch = "x86")]
use super::super::vdso_wrappers::SyscallType;

extern "C" {
    pub(crate) fn syscall0(nr: u32) -> usize;
    pub(crate) fn syscall0_readonly(nr: u32) -> usize;
    pub(crate) fn syscall1(nr: u32, a0: usize) -> usize;
    pub(crate) fn syscall1_readonly(nr: u32, a0: usize) -> usize;
    pub(crate) fn syscall1_noreturn(nr: u32, a0: usize) -> !;
    pub(crate) fn syscall2(nr: u32, a0: usize, a1: usize) -> usize;
    pub(crate) fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize;
    pub(crate) fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize;
    pub(crate) fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize;
    pub(crate) fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize;
    pub(crate) fn syscall4_readonly(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize;
    pub(crate) fn syscall5(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize)
        -> usize;
    pub(crate) fn syscall5_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize;
    pub(crate) fn syscall6(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize;
    pub(crate) fn syscall6_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize;
}
#[cfg(target_arch = "x86")]
extern "C" {
    pub(crate) fn indirect_syscall0(callee: SyscallType, nr: u32) -> usize;
    pub(crate) fn indirect_syscall1(callee: SyscallType, nr: u32, a0: usize) -> usize;
    pub(crate) fn indirect_syscall1_noreturn(callee: SyscallType, nr: u32, a0: usize) -> !;
    pub(crate) fn indirect_syscall2(callee: SyscallType, nr: u32, a0: usize, a1: usize) -> usize;
    pub(crate) fn indirect_syscall3(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
    ) -> usize;
    pub(crate) fn indirect_syscall4(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize;
    pub(crate) fn indirect_syscall5(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize;
    pub(crate) fn indirect_syscall6(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize;
}
