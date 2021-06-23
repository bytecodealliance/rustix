//! 32-bit x86 Linux system calls. These use `int $0x80`, which works but
//! is slow. Ideally we should locate the vDSO, parse it, and use its
//! functions to make system calls instead.

#![allow(dead_code)]

#[inline]
pub(crate) unsafe fn syscall0(nr: u32) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall1_noreturn(nr: u32, a0: usize) -> ! {
    asm!(
        "int $$0x80",
        in("eax") nr as usize,
        in("ebx") a0,
        options(noreturn)
    )
}

#[inline]
pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let r0;
    // a3 should go in esi, but asm! won't let us use it as an operand.
    // Temporarily swap it into place, and then swap it back afterward.
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
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
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
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
    // As in syscall 4, use xchg to handle a3. a4 should go in edi, and
    // we can use that register as an operand.
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
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
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        inout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
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
    // Oof. a3 should go in esi, and a5 should go in ebp, and asm! won't
    // let us use either of those registers as operands. And we can't request
    // stack slots. And there are no other registers free. Use eax as a
    // temporary pointer to a slice, since it gets clobbered as the return
    // value anyway.
    //
    // This is another reason that syscalls should be compiler intrinsics
    // rather than inline asm.
    asm!(
        "push ebp",
        "push esi",
        "mov esi, [eax + 0]",
        "mov ebp, [eax + 4]",
        "mov eax, [eax + 8]",
        "int $$0x80",
        "pop esi",
        "pop ebp",
        inout("eax") &[a3, a5, nr as usize] => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(preserves_flags)
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
        "push ebp",
        "push esi",
        "mov esi, [eax + 0]",
        "mov ebp, [eax + 4]",
        "mov eax, [eax + 8]",
        "int $$0x80",
        "pop esi",
        "pop ebp",
        inout("eax") &[a3, a5, nr as usize] => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(preserves_flags, readonly)
    );
    r0
}
