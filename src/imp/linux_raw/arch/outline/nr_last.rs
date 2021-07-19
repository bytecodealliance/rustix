// Some architectures' outline assembly code prefers to see the `nr` argument
// last, as that lines up the syscall calling convention with the userspace
// calling convention better.
//
// First we declare the actual assembly routines with `reordered_*` names and
// reorgered arguments.
extern "C" {
    fn posish_reordered_syscall0_readonly(nr: u32) -> usize;
    fn posish_reordered_syscall1(a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall1_readonly(a0: usize, nr: u32) -> usize;
    fn posish_reordered_syscall1_noreturn(a0: usize, nr: u32) -> !;
    fn posish_reordered_syscall2(a0: usize, a1: usize, nr: u32) -> usize;
    fn posish_reordered_syscall2_readonly(a0: usize, a1: usize, nr: u32) -> usize;
    fn posish_reordered_syscall3(a0: usize, a1: usize, a2: usize, nr: u32) -> usize;
    fn posish_reordered_syscall3_readonly(a0: usize, a1: usize, a2: usize, nr: u32) -> usize;
    fn posish_reordered_syscall4(a0: usize, a1: usize, a2: usize, a3: usize, nr: u32) -> usize;
    fn posish_reordered_syscall4_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall5_readonly(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        nr: u32,
    ) -> usize;
    fn posish_reordered_syscall6_readonly(
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
        posish_reordered_syscall2(a0, a1, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
        posish_reordered_syscall2_readonly(a0, a1, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        posish_reordered_syscall3(a0, a1, a2, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
        posish_reordered_syscall3_readonly(a0, a1, a2, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
        posish_reordered_syscall4(a0, a1, a2, a3, nr)
    }
    #[inline]
    pub(crate) unsafe fn syscall4_readonly(
        nr: u32,
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
    ) -> usize {
        posish_reordered_syscall4_readonly(a0, a1, a2, a3, nr)
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
        posish_reordered_syscall5(a0, a1, a2, a3, a4, nr)
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
        posish_reordered_syscall5_readonly(a0, a1, a2, a3, a4, nr)
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
        posish_reordered_syscall6(a0, a1, a2, a3, a4, a5, nr)
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
        posish_reordered_syscall6_readonly(a0, a1, a2, a3, a4, a5, nr)
    }
}

pub(crate) use reorder::*;
