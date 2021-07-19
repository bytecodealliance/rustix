#![allow(dead_code)]

use super::super::super::vdso_wrappers::SyscallType;

// x86 (using fastcall) prefers to pass a1 and a2 first, before a0, because
// fastcall passes the first two arguments in ecx and edx, which are the second
// and third Linux syscall arguments.
//
// First we declare the actual assembly routines with `posish_reordered_*`
// names and reorgered arguments.
extern "fastcall" {
    fn posish_reordered_syscall0_readonly(nr: u32) -> usize;
    fn posish_reordered_syscall1(a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall1_readonly(a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall1_noreturn(a0: usize, nr: u32) -> !;
    fn posish_reordered_syscall2(a1: usize, a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall2_readonly(a1: usize, a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall3(a1: usize, a2: usize, a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall3_readonly(a1: usize, a2: usize, a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall4(a1: usize, a2: usize, a0: usize, a3: usize, nr: u32) -> usize;
    fn posish_reordered_syscall4_readonly(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5_readonly(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6_readonly(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder {
    use super::*;

    #[inline]
    pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
        posish_reordered_syscall0_readonly(nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        posish_reordered_syscall1(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
        posish_reordered_syscall1_readonly(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        posish_reordered_syscall1_noreturn(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        posish_reordered_syscall2(a1, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        posish_reordered_syscall2_readonly(a1, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        posish_reordered_syscall3(a1, a2, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        posish_reordered_syscall3_readonly(a1, a2, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        posish_reordered_syscall4(a1, a2, a0, a3, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        posish_reordered_syscall4_readonly(a1, a2, a0, a3, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall5(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize {
        posish_reordered_syscall5(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall5_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize {
        posish_reordered_syscall5_readonly(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall6(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize {
        posish_reordered_syscall6(a1, a2, a0, a3, a4, a5, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall6_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize {
        posish_reordered_syscall6_readonly(a1, a2, a0, a3, a4, a5, nr)
    }
}

pub(crate) use reorder::*;

// x86 prefers to route all syscalls through the vDSO, though this isn't
// always possible, so it also has a special form for doing the dispatch.
//
// First we declare the actual assembly routines with `posish_reordered_*`
// names and reorgered arguments.
extern "fastcall" {
    fn posish_reordered_indirect_syscall0(nr: u32, callee: SyscallType) -> usize;
    fn posish_reordered_indirect_syscall1(a0: usize, nr: u32, callee: SyscallType) -> usize;
    fn posish_reordered_indirect_syscall1_noreturn(a0: usize, nr: u32, callee: SyscallType) -> !;
    fn posish_reordered_indirect_syscall2(
        a1: usize,
        a0: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn posish_reordered_indirect_syscall3(
        a1: usize,
        a2: usize,
        a0: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn posish_reordered_indirect_syscall4(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn posish_reordered_indirect_syscall5(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn posish_reordered_indirect_syscall6(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder_indirect {
    use super::*;

    #[inline]
    pub(crate) unsafe fn indirect_syscall0(callee: SyscallType, nr: u32) -> usize {
        posish_reordered_indirect_syscall0(nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall1(callee: SyscallType, nr: u32, a0: usize) -> usize {
        posish_reordered_indirect_syscall1(a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall1_noreturn(callee: SyscallType, nr: u32, a0: usize) -> ! {
        posish_reordered_indirect_syscall1_noreturn(a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall2(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
    ) -> usize {
        posish_reordered_indirect_syscall2(a1, a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall3(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
    ) -> usize {
        posish_reordered_indirect_syscall3(a1, a2, a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall4(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        posish_reordered_indirect_syscall4(a1, a2, a0, a3, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall5(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
    ) -> usize {
        posish_reordered_indirect_syscall5(a1, a2, a0, a3, a4, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall6(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    ) -> usize {
        posish_reordered_indirect_syscall6(a1, a2, a0, a3, a4, a5, nr, callee)
    }
}

pub(crate) use reorder_indirect::*;
