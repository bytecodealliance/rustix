#![allow(dead_code)]

#[inline]
pub(crate) unsafe fn syscall0(nr: u32) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        out("a0") r0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        out("a0") r0,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
    asm!(
        "ecall",
        in("a7") nr as usize,
        in("a0") a0,
        options(noreturn)
    );
}

#[inline]
pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall4_readonly(
    nr: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
) -> usize {
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        options(nostack, preserves_flags, readonly)
    );
    r0
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
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        options(nostack, preserves_flags)
    );
    r0
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
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        options(nostack, preserves_flags, readonly)
    );
    r0
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
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        options(nostack, preserves_flags)
    );
    r0
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
    let r0;
    asm!(
        "ecall",
        in("a7") nr as usize,
        inlateout("a0") a0 => r0,
        in("a1") a1,
        in("a2") a2,
        in("a3") a3,
        in("a4") a4,
        in("a5") a5,
        options(nostack, preserves_flags, readonly)
    );
    r0
}
