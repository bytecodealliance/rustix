use crate::imp::linux_raw::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};

// Some architectures' outline assembly code prefers to see the `nr` argument
// last, as that lines up the syscall calling convention with the userspace
// calling convention better.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
extern "C" {
    fn rsix_reordered_syscall0_readonly(nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1(a0: ArgReg<A0>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1_readonly(a0: ArgReg<A0>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall1_noreturn(a0: ArgReg<A0>, nr: SyscallNumber) -> !;
    fn rsix_reordered_syscall2(a0: ArgReg<A0>, a1: ArgReg<A1>, nr: SyscallNumber) -> RetReg<R0>;
    fn rsix_reordered_syscall2_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
        rsix_reordered_syscall0_readonly(nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall1(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall1_readonly(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_noreturn(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> ! {
        rsix_reordered_syscall1_noreturn(a0, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall2(a0, a1, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall2_readonly(a0, a1, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall3(a0, a1, a2, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall3_readonly(a0, a1, a2, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall4(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall4(a0, a1, a2, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall4_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall4_readonly(a0, a1, a2, a3, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall5(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall5(a0, a1, a2, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall5_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall5_readonly(a0, a1, a2, a3, a4, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall6(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall6(a0, a1, a2, a3, a4, a5, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall6_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
    ) -> RetReg<R0> {
        rsix_reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, nr)
    }
}

pub(in crate::imp::linux_raw) use reorder::*;
