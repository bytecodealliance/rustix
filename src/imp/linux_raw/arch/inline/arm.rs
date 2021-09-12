use crate::imp::linux_raw::reg::{
    ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0,
};

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        out("r0") r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall1(nr: SyscallNumber, a0: ArgReg<A0>) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp::linux_raw) unsafe fn syscall1_noreturn(nr: SyscallNumber, a0: ArgReg<A0>) -> ! {
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        in("r0") a0.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
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
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
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
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
        in("r4") a4.to_asm(),
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
        in("r4") a4.to_asm(),
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
    asm!(
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
        in("r4") a4.to_asm(),
        in("r5") a5.to_asm(),
        options(nostack, preserves_flags)
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
        "svc 0",
        in("r7") nr.to_asm(),
        inlateout("r0") a0.to_asm() => r0,
        in("r1") a1.to_asm(),
        in("r2") a2.to_asm(),
        in("r3") a3.to_asm(),
        in("r4") a4.to_asm(),
        in("r5") a5.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
