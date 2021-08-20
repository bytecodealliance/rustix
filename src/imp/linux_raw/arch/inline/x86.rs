//! 32-bit x86 Linux system calls.
//!
//! There are two forms; `indirect_*` which take a callee, which allow calling
//! through the vDSO when possible, and plain forms, which use the `int 0x80`
//! instruction.

#![allow(dead_code)]

use crate::imp::linux_raw::reg::{
    ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0,
};
use crate::imp::linux_raw::vdso_wrappers::SyscallType;

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall0(
    callee: SyscallType,
    nr: SyscallNumber,
) -> RetReg<R0> {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr.to_asm() => r0,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall1(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall1_noreturn(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
) -> ! {
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        in("eax") nr.to_asm(),
        in("ebx") a0.to_asm(),
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall2(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall3(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "call {callee}",
        callee = in(reg) callee,
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall4(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
    a3: ArgReg<A3>,
) -> RetReg<R0> {
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
        a3 = in(reg) a3.to_asm(),
        in("edi") callee,
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall5(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
    a3: ArgReg<A3>,
    a4: ArgReg<A4>,
) -> RetReg<R0> {
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
        inout("eax") &[callee as usize, a3.to_asm(), nr.to_asm()] => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub(in crate::imp::linux_raw) unsafe fn indirect_syscall6(
    callee: SyscallType,
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
    a3: ArgReg<A3>,
    a4: ArgReg<A4>,
    a5: ArgReg<A5>,
) -> RetReg<R0> {
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
        inout("eax") &[callee as usize, a3.to_asm(), a5.to_asm(), nr.to_asm()] => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall1(nr: SyscallNumber, a0: ArgReg<A0>) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall1_readonly(
    nr: SyscallNumber,
    a0: ArgReg<A0>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::imp::linux_raw) unsafe fn syscall1_noreturn(nr: SyscallNumber, a0: ArgReg<A0>) -> ! {
    asm!(
        "int $$0x80",
        in("eax") nr.to_asm(),
        in("ebx") a0.to_asm(),
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall2(
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall2_readonly(
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall3(
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall3_readonly(
    nr: SyscallNumber,
    a0: ArgReg<A0>,
    a1: ArgReg<A1>,
    a2: ArgReg<A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "int $$0x80",
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
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
    let r0;
    // a3 should go in esi, but asm! won't let us use it as an operand.
    // Temporarily swap it into place, and then swap it back afterward.
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3.to_asm(),
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
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
    let r0;
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3.to_asm(),
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
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
    let r0;
    // As in syscall 4, use xchg to handle a3. a4 should go in edi, and
    // we can use that register as an operand.
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3.to_asm(),
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
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
    let r0;
    asm!(
        "xchg esi, {a3}",
        "int $$0x80",
        "xchg esi, {a3}",
        a3 = in(reg) a3.to_asm(),
        inlateout("eax") nr.to_asm() => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
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
        inout("eax") &[a3.to_asm(), a5.to_asm(), nr.to_asm()] => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(preserves_flags)
    );
    FromAsm::from_asm(r0)
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
        inout("eax") &[a3.to_asm(), a5.to_asm(), nr.to_asm()] => r0,
        in("ebx") a0.to_asm(),
        in("ecx") a1.to_asm(),
        in("edx") a2.to_asm(),
        in("edi") a4.to_asm(),
        options(preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
