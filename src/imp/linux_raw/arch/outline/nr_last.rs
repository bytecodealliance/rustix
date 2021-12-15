use crate::imp::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};

// Some architectures' outline assembly code prefers to see the `nr` argument
// last, as that lines up the syscall calling convention with the userspace
// calling convention better.
//
// First we declare the actual assembly routines with `*_nr_last` names and
// reordered arguments. If the signatures or calling conventions are ever
// changed, the symbol names should also be updated accordingly, to avoid
// collisions with other versions of this crate.
//
// We don't define `_readonly` versions of these because we have no way to tell
// Rust that calls to our outline assembly are readonly.
extern "C" {
    fn rustix_syscall0_nr_last(nr: SyscallNumber<'_>) -> RetReg<R0>;
    fn rustix_syscall1_nr_last(a0: ArgReg<'_, A0>, nr: SyscallNumber<'_>) -> RetReg<R0>;
    fn rustix_syscall1_noreturn_nr_last(a0: ArgReg<'_, A0>, nr: SyscallNumber<'_>) -> !;
    fn rustix_syscall2_nr_last(
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall3_nr_last(
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall4_nr_last(
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall5_nr_last(
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
        a3: ArgReg<'_, A3>,
        a4: ArgReg<'_, A4>,
        nr: SyscallNumber<'_>,
    ) -> RetReg<R0>;
    fn rustix_syscall6_nr_last(
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
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
        rustix_syscall0_nr_last(nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> RetReg<R0> {
        rustix_syscall1_nr_last(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
    ) -> RetReg<R0> {
        rustix_syscall1_nr_last(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall1_noreturn(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> ! {
        rustix_syscall1_noreturn_nr_last(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall2(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
    ) -> RetReg<R0> {
        rustix_syscall2_nr_last(a0, a1, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall2_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
    ) -> RetReg<R0> {
        rustix_syscall2_nr_last(a0, a1, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall3(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
    ) -> RetReg<R0> {
        rustix_syscall3_nr_last(a0, a1, a2, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp) unsafe fn syscall3_readonly(
        nr: SyscallNumber<'_>,
        a0: ArgReg<'_, A0>,
        a1: ArgReg<'_, A1>,
        a2: ArgReg<'_, A2>,
    ) -> RetReg<R0> {
        rustix_syscall3_nr_last(a0, a1, a2, nr)
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
        rustix_syscall4_nr_last(a0, a1, a2, a3, nr)
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
        rustix_syscall4_nr_last(a0, a1, a2, a3, nr)
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
        rustix_syscall5_nr_last(a0, a1, a2, a3, a4, nr)
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
        rustix_syscall5_nr_last(a0, a1, a2, a3, a4, nr)
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
        rustix_syscall6_nr_last(a0, a1, a2, a3, a4, a5, nr)
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
        rustix_syscall6_nr_last(a0, a1, a2, a3, a4, a5, nr)
    }
}

pub(in crate::imp) use reorder::*;
