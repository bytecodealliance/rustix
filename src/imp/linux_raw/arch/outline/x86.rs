#![allow(dead_code)]

use crate::imp::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};
use crate::imp::vdso_wrappers::SyscallType;

// x86 (using fastcall) prefers to pass a1 and a2 first, before a0, because
// fastcall passes the first two arguments in ecx and edx, which are the second
// and third Linux syscall arguments.
//
// First we declare the actual assembly routines with `*_nr_last_fastcall`
// names and reordered arguments. If the signatures or calling conventions are
// ever changed, the symbol names should also be updated accordingly, to avoid
// collisions with other versions of this crate.
//
// We don't define `_readonly` versions of these because we have no way to tell
// Rust that calls to our outline assembly are readonly.
extern "fastcall" {
    fn rustix_syscall0_nr_last_fastcall(nr: SyscallNumber<'_>) -> RetReg<R0>;
    fn rustix_syscall1_nr_last_fastcall(a0: ArgReg<'_, A0>, nr: SyscallNumber<'_>) -> RetReg<R0>;
    fn rustix_syscall1_noreturn_nr_last_fastcall(a0: ArgReg<'_, A0>, nr: SyscallNumber<'_>) -> !;
    fn rustix_syscall2_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall3_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall4_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall5_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall6_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        a5: ArgReg<'_, A5>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber<'_>) -> RetReg<R0> {
        rustix_syscall0_nr_last_fastcall(nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> RetReg<R0> {
        rustix_syscall1_nr_last_fastcall(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
    ) -> RetReg<R0> {
        rustix_syscall1_nr_last_fastcall(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1_noreturn(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> ! {
        rustix_syscall1_noreturn_nr_last_fastcall(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall2(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
    ) -> RetReg<R0> {
        rustix_syscall2_nr_last_fastcall(a1, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall2_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
    ) -> RetReg<R0> {
        rustix_syscall2_nr_last_fastcall(a1, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall3(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
    ) -> RetReg<R0> {
        rustix_syscall3_nr_last_fastcall(a1, a2, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall3_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
    ) -> RetReg<R0> {
        rustix_syscall3_nr_last_fastcall(a1, a2, a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall4(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
    ) -> RetReg<R0> {
        rustix_syscall4_nr_last_fastcall(a1, a2, a0, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall4_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
    ) -> RetReg<R0> {
        rustix_syscall4_nr_last_fastcall(a1, a2, a0, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall5(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
    ) -> RetReg<R0> {
        rustix_syscall5_nr_last_fastcall(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall5_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
    ) -> RetReg<R0> {
        rustix_syscall5_nr_last_fastcall(a1, a2, a0, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall6(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        a5: ArgReg<'_, A5>,
    ) -> RetReg<R0> {
        rustix_syscall6_nr_last_fastcall(a1, a2, a0, a3, a4, a5, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall6_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        a5: ArgReg<'_, A5>,
    ) -> RetReg<R0> {
        rustix_syscall6_nr_last_fastcall(a1, a2, a0, a3, a4, a5, nr)
    }
}

pub(in crate::imp) use reorder::*;

// x86 prefers to route all syscalls through the vDSO, though this isn't
// always possible, so it also has a special form for doing the dispatch.
//
// First we declare the actual assembly routines with `*_nr_last_fastcall`
// names and reordered arguments. If the signatures or calling conventions are
// ever changed, the symbol names should also be updated accordingly, to avoid
// collisions with other versions of this crate.
extern "fastcall" {
    fn rustix_indirect_syscall0_nr_last_fastcall(
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall1_nr_last_fastcall(
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall1_noreturn_nr_last_fastcall(
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> !;
    fn rustix_indirect_syscall2_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall3_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall4_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall5_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
    fn rustix_indirect_syscall6_nr_last_fastcall(
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a0: ArgReg<'_, A0>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        a5: ArgReg<'_, A5>,
        nr: SyscallNumber<'_>,
        callee: SyscallType,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder_indirect {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall0(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall0_nr_last_fastcall(nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall1(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall1_nr_last_fastcall(a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall1_noreturn(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
    ) -> ! {
        rustix_indirect_syscall1_noreturn_nr_last_fastcall(a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall2(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall2_nr_last_fastcall(a1, a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall3(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall3_nr_last_fastcall(a1, a2, a0, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall4(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall4_nr_last_fastcall(a1, a2, a0, a3, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall5(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall5_nr_last_fastcall(a1, a2, a0, a3, a4, nr, callee)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn indirect_syscall6(
        callee: SyscallType,
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        a5: ArgReg<'_, A5>,
    ) -> RetReg<R0> {
        rustix_indirect_syscall6_nr_last_fastcall(a1, a2, a0, a3, a4, a5, nr, callee)
    }
}

pub(in crate::imp) use reorder_indirect::*;
