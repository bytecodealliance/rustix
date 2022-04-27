//! aarch64 Linux system calls.

use crate::imp::reg::{ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0};
use core::arch::asm;

#[cfg(target_pointer_width = "32")]
compile_error!("arm64-ilp32 is not supported yet");

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber<'_>) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        lateout("x0") r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_noreturn<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> ! {
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        in("x0") a0.into().to_asm(),
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        in("x4") a4.into().to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        in("x4") a4.into().to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
    a5: impl Into<ArgReg<'a, A5>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        in("x4") a4.into().to_asm(),
        in("x5") a5.into().to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6_readonly<'a>(
    nr: SyscallNumber<'_>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
    a5: impl Into<ArgReg<'a, A5>>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.into().to_asm() => r0,
        in("x1") a1.into().to_asm(),
        in("x2") a2.into().to_asm(),
        in("x3") a3.into().to_asm(),
        in("x4") a4.into().to_asm(),
        in("x5") a5.into().to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
