//! 32-bit x86 Linux system calls.
//!
//! There are two forms; `indirect_*` which take a callee, which allow calling
//! through the vDSO when possible, and plain forms, which use the `int 0x80`
//! instruction.

#![allow(dead_code)]

use super::super::super::vdso_wrappers::SyscallType;

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall0(callee: SyscallType, nr: u32) -> usize {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr as usize => r0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall1(callee: SyscallType, nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
pub(crate) unsafe fn indirect_syscall1_noreturn(callee: SyscallType, nr: u32, a0: usize) -> ! {
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        in("eax") nr as usize,
        in("ebx") a0,
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall2(
    callee: SyscallType,
    nr: u32,
    a0: usize,
    a1: usize,
) -> usize {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall3(
    callee: SyscallType,
    nr: u32,
    a0: usize,
    a1: usize,
    a2: usize,
) -> usize {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall4(
    callee: SyscallType,
    nr: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
) -> usize {
    let r0;
    // a3 should go in esi, but asm! won't let us use it as an operand.
    // temporarily swap it into place, and then swap it back afterward.
    //
    // Note that we hard-code the callee operand to use edi instead of
    // `in(reg)` because even though we can't name esi as an operand,
    // the compiler can use esi to satisfy `in(reg)`.
    asm!(
        "xchg esi, {a3}",
        "call edi",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        in("edi") callee,
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn indirect_syscall5(
    callee: SyscallType,
    nr: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> usize {
    let r0;
    // Oof. a3 should go in esi, and asm! won't let us use that register as
    // an operand. And we can't request stack slots. And there are no other
    // registers free. Use eax as a temporary pointer to a slice, since it
    // gets clobbered as the return value anyway.
    asm!(
        "push ebp",
        "push esi",
        "push [eax + 0]",
        "mov esi, [eax + 4]",
        "mov eax, [eax + 8]",
        "call [esp]",
        "pop esi",
        "pop esi",
        "pop ebp",
        inout("eax") &[callee as usize, a3, nr as usize] => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(preserves_flags)
    );
    r0
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
    let r0;
    // Oof again. a3 should go in esi, and a5 should go in ebp, and asm! won't
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
        "push [eax + 0]",
        "mov esi, [eax + 4]",
        "mov ebp, [eax + 8]",
        "mov eax, [eax + 12]",
        "call [esp]",
        "pop esi",
        "pop esi",
        "pop ebp",
        inout("eax") &[callee as usize, a3, a5, nr as usize] => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall0_readonly(nr: u32) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall1(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall1_readonly(nr: u32, a0: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
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
#[must_use]
pub(crate) unsafe fn syscall2(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall2_readonly(nr: u32, a0: usize, a1: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall3(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall3_readonly(nr: u32, a0: usize, a1: usize, a2: usize) -> usize {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
#[must_use]
pub(crate) unsafe fn syscall4(nr: u32, a0: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let r0;
    // a3 should go in esi, but asm! won't let us use it as an operand.
    // Temporarily swap it into place, and then swap it back afterward.
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3,
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
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
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
#[must_use]
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
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(nostack, preserves_flags)
    );
    r0
}

#[inline]
#[must_use]
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
        inlateout("eax") nr as usize => r0,
        in("ebx") a0,
        in("ecx") a1,
        in("edx") a2,
        in("edi") a4,
        options(nostack, preserves_flags, readonly)
    );
    r0
}

#[inline]
#[must_use]
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
#[must_use]
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
