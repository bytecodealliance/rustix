//! x86-64 Linux system calls.

use crate::backend::reg::{
    ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm as _, A0, A1, A2, A3, A4, A5, R0,
};
use core::arch::asm;

#[cfg(target_pointer_width = "32")]
compile_error!("x32 is not yet supported");

#[inline]
pub(in crate::backend) unsafe fn syscall0_readonly(nr: SyscallNumber<'_>) -> RetReg<R0> {
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
pub(in crate::backend) unsafe fn syscall1(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall1_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall1_noreturn(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> ! {
    asm!(
        "syscall",
        in("rax") nr.to_asm(),
        in("rdi") a0.to_asm(),
        options(nostack, noreturn)
    )
}

#[inline]
pub(in crate::backend) unsafe fn syscall2(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall2_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall3(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall3_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall4(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall4_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall5(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        in("r8") a4.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall5_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        in("r8") a4.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall6(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
    a5: ArgReg<'_, A5>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        in("r8") a4.to_asm(),
        in("r9") a5.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall6_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
    a5: ArgReg<'_, A5>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        in("rdx") a2.to_asm(),
        in("r10") a3.to_asm(),
        in("r8") a4.to_asm(),
        in("r9") a5.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
