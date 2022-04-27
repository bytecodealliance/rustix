//! mipsel Linux system calls.
//!
//! On mipsel, Linux indicates success or failure using `$a3` rather
//! than by returning a negative error code as most other architectures do.
//!
//! Mips-family platforms have a special calling convention for `__NR_pipe`,
//! however we use `__NR_pipe2` instead to avoid having to implement it.

use crate::imp::reg::{
    ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, A6, R0,
};
use core::arch::asm;

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_noreturn<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
) -> ! {
    asm!(
        "syscall",
        in("$2" /*$v0*/) nr.to_asm(),
        in("$4" /*$a0*/) a0.into().to_asm(),
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
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        lateout("$7" /*$a3*/) err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.into().to_asm(),
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.into().to_asm(),
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.into().to_asm(),
        in(reg) a5.into().to_asm(),
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
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
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.into().to_asm(),
        in(reg) a5.into().to_asm(),
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall7_readonly<'a>(
    nr: SyscallNumber<'a>,
    a0: impl Into<ArgReg<'a, A0>>,
    a1: impl Into<ArgReg<'a, A1>>,
    a2: impl Into<ArgReg<'a, A2>>,
    a3: impl Into<ArgReg<'a, A3>>,
    a4: impl Into<ArgReg<'a, A4>>,
    a5: impl Into<ArgReg<'a, A5>>,
    a6: impl Into<ArgReg<'a, A6>>,
) -> RetReg<R0> {
    let x0;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "sw {}, 24($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) a4.into().to_asm(),
        in(reg) a5.into().to_asm(),
        in(reg) a6.into().to_asm(),
        inlateout("$2" /*$v0*/) nr.to_asm() => x0,
        in("$4" /*$a0*/) a0.into().to_asm(),
        in("$5" /*$a1*/) a1.into().to_asm(),
        in("$6" /*$a2*/) a2.into().to_asm(),
        inlateout("$7" /*$a3*/) a3.into().to_asm() => err,
        lateout("$8" /*$t0*/) _,
        lateout("$9" /*$t1*/) _,
        lateout("$10" /*$t2*/) _,
        lateout("$11" /*$t3*/) _,
        lateout("$12" /*$t4*/) _,
        lateout("$13" /*$t5*/) _,
        lateout("$14" /*$t6*/) _,
        lateout("$15" /*$t7*/) _,
        lateout("$24" /*$t8*/) _,
        lateout("$25" /*$t9*/) _,
        options(preserves_flags, readonly)
    );
    FromAsm::from_asm(if err != 0 {
        (x0 as usize).wrapping_neg() as *mut _
    } else {
        x0
    })
}
