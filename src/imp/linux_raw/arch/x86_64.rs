#![allow(dead_code)]

#[inline]
pub(crate) unsafe fn syscall0(nr: u32) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
    asm!(
        "syscall",
        in("rax") nr,
        in("rdi") a0,
        options(noreturn)
    )
}

#[inline]
pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let r0;
    asm!(
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        out("rcx") _,
        out("r11") _,
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
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        out("rcx") _,
        out("r11") _,
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
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        out("rcx") _,
        out("r11") _,
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
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        out("rcx") _,
        out("r11") _,
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
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        in("r9") a5,
        out("rcx") _,
        out("r11") _,
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
        "syscall",
        inout("rax") nr as usize => r0,
        in("rdi") a0,
        in("rsi") a1,
        in("rdx") a2,
        in("r10") a3,
        in("r8") a4,
        in("r9") a5,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    r0
}
