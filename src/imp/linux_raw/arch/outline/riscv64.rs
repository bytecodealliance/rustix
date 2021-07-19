// riscv64's outline assembly code prefers to see the `nr` argument in a7.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
extern "C" {
    fn posish_reordered_syscall0_readonly(
        u0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall1(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall1_readonly(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall1_noreturn(
        a0: usize,
        u1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> !;
    fn posish_reordered_syscall2(
        a0: usize,
        a1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall2_readonly(
        a0: usize,
        a1: usize,
        u2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall3(
        a0: usize,
        a1: usize,
        a2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall3_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        u3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall4(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall4_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        u4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        u5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        u6: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6_readonly(
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
mod reorder {
    use super::*;

    #[inline]
    pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall0_readonly(u, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall1(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall1_readonly(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall1_noreturn(a0, u, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall2(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall2_readonly(a0, a1, u, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall3(a0, a1, a2, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall3_readonly(a0, a1, a2, u, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall4(a0, a1, a2, a3, u, u, u, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall4_readonly(a0, a1, a2, a3, u, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall5(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall5_readonly(a0, a1, a2, a3, a4, u, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall6(a0, a1, a2, a3, a4, a5, u, nr)
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
        let u = std::mem::MaybeUninit::uninit().assume_init();
        posish_reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, u, nr)
    }
}

pub(crate) use reorder::*;
