//! powerpc64le Linux system calls.
//!
//! On powerpc64le, Linux indicates success or failure using `cr0.SO` rather
//! than by returning a negative error code as most other architectures do. In
//! theory we could immediately translate this into a `Result`, and it'd save
//! a few branches. And in theory we could have specialized sequences for use
//! with syscalls that are known to never fail. However, those would require
//! more extensive changes in rustix's platform-independent code. For now, we
//! check the flag and negatate the error value to make PowerPC64 look like
//! other architectures.

use crate::imp::reg::{ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0};
use core::arch::asm;

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        lateout("r3") r0,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_noreturn<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> ! {
    asm!(
        "sc",
        in("r0") nr.to_asm(),
        in("r3") a0.into().to_asm(),
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        inlateout("r7") a4.into().to_asm() => _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        inlateout("r7") a4.into().to_asm() => _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
    a5: impl Into<ArgReg<'a, A5>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        inlateout("r7") a4.into().to_asm() => _,
        inlateout("r8") a5.into().to_asm() => _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
    a5: impl Into<ArgReg<'a, A5>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "sc",
        "bns 0f",
        "neg 3, 3",
        "0:",
        inlateout("r0") nr.to_asm() => _,
        inlateout("r3") a0.into().to_asm() => r0,
        inlateout("r4") a1.into().to_asm() => _,
        inlateout("r5") a2.into().to_asm() => _,
        inlateout("r6") a3.into().to_asm() => _,
        inlateout("r7") a4.into().to_asm() => _,
        inlateout("r8") a5.into().to_asm() => _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
