//! Declare functions defined in out-of-line asm files.
//!
//! Kernel calling conventions differ from userspace calling conventions,
//! so we also define inline function wrappers which reorder the arguments
//! so that they match with the kernel convention as closely as possible,
//! to minimize the amount of out-of-line code we need.

#[cfg(target_arch = "x86")]
use super::super::vdso_wrappers::SyscallType;

// Architectures that don't need reordering could use this.
#[cfg(any())]
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

// Some architectures' outline assembly code prefers to see the `nr` argument
// last, as that lines up the syscall calling convention with the userspace
// calling convention better.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
extern "C" {
    fn reordered_syscall0(nr: u32) -> usize;
    fn reordered_syscall0_readonly(nr: u32) -> usize;
    fn reordered_syscall1(a0: usize, nr: u32) -> usize;
    fn reordered_syscall1_readonly(a0: usize, nr: u32) -> usize;
    fn reordered_syscall1_noreturn(a0: usize, nr: u32) -> !;
    fn reordered_syscall2(a0: usize, a1: usize, nr: u32) -> usize;
    fn reordered_syscall2_readonly(a0: usize, a1: usize, nr: u32) -> usize;
    fn reordered_syscall3(a0: usize, a1: usize, a2: usize, nr: u32) -> usize;
    fn reordered_syscall3_readonly(a0: usize, a1: usize, a2: usize, nr: u32) -> usize;
    fn reordered_syscall4(a0: usize, a1: usize, a2: usize, a3: usize, nr: u32) -> usize;
    fn reordered_syscall4_readonly(a0: usize, a1: usize, a2: usize, a3: usize, nr: u32) -> usize;
    fn reordered_syscall5(a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, nr: u32) -> usize;
    fn reordered_syscall5_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
}

// Then we define inline wrapper functions that do the reordering.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
mod reorder {
    use super::*;

    #[inline]
    pub(crate) unsafe fn syscall0(nr: u32) -> usize {
        reordered_syscall0(nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
        reordered_syscall0_readonly(nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        reordered_syscall1(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
        reordered_syscall1_readonly(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        reordered_syscall1_noreturn(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        reordered_syscall2(a0, a1, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        reordered_syscall2_readonly(a0, a1, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        reordered_syscall3(a0, a1, a2, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        reordered_syscall3_readonly(a0, a1, a2, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        reordered_syscall4(a0, a1, a2, a3, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        reordered_syscall4_readonly(a0, a1, a2, a3, nr)
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
        reordered_syscall5(a0, a1, a2, a3, a4, nr)
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
        reordered_syscall5_readonly(a0, a1, a2, a3, a4, nr)
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
        reordered_syscall6(a0, a1, a2, a3, a4, a5, nr)
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
        reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, nr)
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub(crate) use reorder::*;

// riscv64's outline assembly code prefers to see the `nr` argument in a7.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
#[cfg(target_arch = "riscv64")]
extern "C" {
    fn reordered_syscall0(
        u0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall0_readonly(
        u0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall1(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall1_readonly(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall1_noreturn(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> !;
    fn reordered_syscall2(
        a0: usize,
        a1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall2_readonly(
        a0: usize,
        a1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall3(
        a0: usize,
        a1: usize,
        a2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall3_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall4(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall4_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall5(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall5_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
}

// Then we define inline wrapper functions that do the reordering.
#[cfg(target_arch = "riscv64")]
mod reorder {
    use super::*;

    #[inline]
    pub(crate) unsafe fn syscall0(nr: u32) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall0(u, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall0_readonly(u, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall1(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall1_readonly(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall1_noreturn(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall2(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall2_readonly(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall3(a0, a1, a2, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall3_readonly(a0, a1, a2, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall4(a0, a1, a2, a3, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall4_readonly(a0, a1, a2, a3, u, u, u, nr)
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
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall5(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall5_readonly(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall6(a0, a1, a2, a3, a4, a5, u, nr)
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
        let u = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, u, nr)
    }
}

#[cfg(target_arch = "riscv64")]
pub(crate) use reorder::*;

// x86 (using fastcall) prefers to pass a1 and a2 first, before a0, because
// fastcall passes the first two arguments in ecx and edx, which are the second
// and third Linux syscall arguments.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
#[cfg(target_arch = "x86")]
extern "fastcall" {
    fn reordered_syscall0(nr: u32) -> usize;
    fn reordered_syscall0_readonly(nr: u32) -> usize;
    fn reordered_syscall1(a0: usize, nr: u32) -> usize;
    fn reordered_syscall1_readonly(a0: usize, nr: u32) -> usize;
    fn reordered_syscall1_noreturn(a0: usize, nr: u32) -> !;
    fn reordered_syscall2(a1: usize, a0: usize, nr: u32) -> usize;
    fn reordered_syscall2_readonly(a1: usize, a0: usize, nr: u32) -> usize;
    fn reordered_syscall3(a1: usize, a2: usize, a0: usize, nr: u32) -> usize;
    fn reordered_syscall3_readonly(a1: usize, a2: usize, a0: usize, nr: u32) -> usize;
    fn reordered_syscall4(a1: usize, a2: usize, a0: usize, a3: usize, nr: u32) -> usize;
    fn reordered_syscall4_readonly(a1: usize, a2: usize, a0: usize, a3: usize, nr: u32) -> usize;
    fn reordered_syscall5(a1: usize, a2: usize, a0: usize, a3: usize, a4: usize, nr: u32) -> usize;
    fn reordered_syscall5_readonly(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
    fn reordered_syscall6_readonly(
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
#[cfg(target_arch = "x86")]
mod reorder {
    use super::*;

    #[inline]
    pub(crate) unsafe fn syscall0(nr: u32) -> usize {
        reordered_syscall0(nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
        reordered_syscall0_readonly(nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        reordered_syscall1(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
        reordered_syscall1_readonly(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        reordered_syscall1_noreturn(a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        reordered_syscall2(a1, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        reordered_syscall2_readonly(a1, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        reordered_syscall3(a1, a2, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        reordered_syscall3_readonly(a1, a2, a0, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        reordered_syscall4(a1, a2, a0, a3, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        reordered_syscall4_readonly(a1, a2, a0, a3, nr)
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
        reordered_syscall5(a1, a2, a0, a3, a4, nr)
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
        reordered_syscall5_readonly(a1, a2, a0, a3, a4, nr)
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
        reordered_syscall6(a1, a2, a0, a3, a4, a5, nr)
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
        reordered_syscall6_readonly(a1, a2, a0, a3, a4, a5, nr)
    }
}

#[cfg(target_arch = "x86")]
pub(crate) use reorder::*;

// x86 prefers to route all syscalls through the vDSO, though this isn't
// always possible, so it also has a special form for doing the dispatch.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
#[cfg(target_arch = "x86")]
extern "fastcall" {
    fn reordered_indirect_syscall0(nr: u32, callee: SyscallType) -> usize;
    fn reordered_indirect_syscall1(a0: usize, nr: u32, callee: SyscallType) -> usize;
    fn reordered_indirect_syscall1_noreturn(a0: usize, nr: u32, callee: SyscallType) -> !;
    fn reordered_indirect_syscall2(a1: usize, a0: usize, nr: u32, callee: SyscallType) -> usize;
    fn reordered_indirect_syscall3(
        a1: usize,
        a2: usize,
        a0: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn reordered_indirect_syscall4(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn reordered_indirect_syscall5(
        a1: usize,
        a2: usize,
        a0: usize,
        a3: usize,
        a4: usize,
        nr: u32,
        callee: SyscallType,
    ) -> usize;
    fn reordered_indirect_syscall6(
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
#[cfg(target_arch = "x86")]
mod reorder_indirect {
    use super::*;

    #[inline]
    pub(crate) unsafe fn indirect_syscall0(callee: SyscallType, nr: u32) -> usize {
        reordered_indirect_syscall0(nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall1(callee: SyscallType, nr: u32, a0: usize) -> usize {
        reordered_indirect_syscall1(a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall1_noreturn(callee: SyscallType, nr: u32, a0: usize) -> ! {
        reordered_indirect_syscall1_noreturn(a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall2(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
    ) -> usize {
        reordered_indirect_syscall2(a1, a0, nr, callee)
    }
    #[inline]
    pub(crate) unsafe fn indirect_syscall3(
        callee: SyscallType,
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
    ) -> usize {
        reordered_indirect_syscall3(a1, a2, a0, nr, callee)
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
        reordered_indirect_syscall4(a1, a2, a0, a3, nr, callee)
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
        reordered_indirect_syscall5(a1, a2, a0, a3, a4, nr, callee)
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
        reordered_indirect_syscall6(a1, a2, a0, a3, a4, a5, nr, callee)
    }
}

#[cfg(target_arch = "x86")]
pub(crate) use reorder_indirect::*;
