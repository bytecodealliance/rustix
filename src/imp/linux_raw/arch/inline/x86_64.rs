//! x86-64 Linux system calls.

use crate::imp::reg::{ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0};
use core::arch::asm;

#[cfg(target_pointer_width = "32")]
compile_error!("x32 is not yet supported");

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber<'_>) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::imp) unsafe fn syscall1_noreturn<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> ! {
    asm!(
        "syscall",
        in("rax") nr.to_asm(),
        in("rdi") a0.into().to_asm(),
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        in("r8") a4.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        in("r8") a4.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        in("r8") a4.into().to_asm(),
        in("r9") a5.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
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
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.into().to_asm(),
        in("rsi") a1.into().to_asm(),
        in("rdx") a2.into().to_asm(),
        in("r10") a3.into().to_asm(),
        in("r8") a4.into().to_asm(),
        in("r9") a5.into().to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
