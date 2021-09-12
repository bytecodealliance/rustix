use crate::imp::linux_raw::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};

// arm's outline assembly
extern "C" {
    fn rsix_reordered_syscall0_readonly(
        u0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall1(
        a0: ArgReg<A0>,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall1_readonly(
        a0: ArgReg<A0>,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall1_noreturn(
        a0: ArgReg<A0>,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> !;
    fn rsix_reordered_syscall2(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall2_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall3_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall4_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall5_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        u5: usize,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
    fn rsix_reordered_syscall6_readonly(
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
        a3: ArgReg<A3>,
        a4: ArgReg<A4>,
        a5: ArgReg<A5>,
        u6: usize,
        nr: SyscallNumber,
    ) -> RetReg<R0>;
}

// Then we define inline wrapper functions that do the reordering.
mod reorder {
    use super::*;

    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall0_readonly(u, u, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall1(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall1_readonly(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall1_noreturn(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
    ) -> ! {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall1_noreturn(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall2(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall2_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall2_readonly(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall3(a0, a1, a2, u, u, u, u, nr)
    }
    #[inline]
    #[must_use]
    pub(in crate::imp::linux_raw) unsafe fn syscall3_readonly(
        nr: SyscallNumber,
        a0: ArgReg<A0>,
        a1: ArgReg<A1>,
        a2: ArgReg<A2>,
    ) -> RetReg<R0> {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall3_readonly(a0, a1, a2, u, u, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall4(a0, a1, a2, a3, u, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall4_readonly(a0, a1, a2, a3, u, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall5(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall5_readonly(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall6(a0, a1, a2, a3, a4, a5, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        rsix_reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, u, nr)
    }
}

pub(in crate::imp::linux_raw) use reorder::*;
