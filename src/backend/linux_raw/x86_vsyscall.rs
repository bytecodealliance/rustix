//! Implement syscalls using the x86 vsyscall mechanism.
//!
//! # Safety
//!
//! Similar to syscalls.rs, this file performs raw system calls, and sometimes
//! passes them uninitialized memory buffers.
#![allow(unsafe_code)]

use super::reg::{ArgReg, RetReg, SyscallNumber, A0, A1, A2, A3, A4, A5, R0};
use crate::backend::arch::asm;
use core::mem::transmute;

#[inline]
pub(super) unsafe fn syscall0(nr: SyscallNumber<'_>) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall0(callee, nr)
}

#[inline]
pub(super) unsafe fn syscall1<'a>(nr: SyscallNumber<'a>, a0: ArgReg<'a, A0>) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall1(callee, nr, a0)
}

#[inline]
pub(super) unsafe fn syscall1_noreturn<'a>(nr: SyscallNumber<'a>, a0: ArgReg<'a, A0>) -> ! {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall1_noreturn(callee, nr, a0)
}

#[inline]
pub(super) unsafe fn syscall2<'a>(
    nr: SyscallNumber<'a>,
    a0: ArgReg<'a, A0>,
    a1: ArgReg<'a, A1>,
) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall2(callee, nr, a0, a1)
}

#[inline]
pub(super) unsafe fn syscall3<'a>(
    nr: SyscallNumber<'a>,
    a0: ArgReg<'a, A0>,
    a1: ArgReg<'a, A1>,
    a2: ArgReg<'a, A2>,
) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall3(callee, nr, a0, a1, a2)
}

#[inline]
pub(super) unsafe fn syscall4<'a>(
    nr: SyscallNumber<'a>,
    a0: ArgReg<'a, A0>,
    a1: ArgReg<'a, A1>,
    a2: ArgReg<'a, A2>,
    a3: ArgReg<'a, A3>,
) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall4(callee, nr, a0, a1, a2, a3)
}

#[inline]
pub(super) unsafe fn syscall5<'a>(
    nr: SyscallNumber<'a>,
    a0: ArgReg<'a, A0>,
    a1: ArgReg<'a, A1>,
    a2: ArgReg<'a, A2>,
    a3: ArgReg<'a, A3>,
    a4: ArgReg<'a, A4>,
) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall5(callee, nr, a0, a1, a2, a3, a4)
}

#[inline]
pub(super) unsafe fn syscall6<'a>(
    nr: SyscallNumber<'a>,
    a0: ArgReg<'a, A0>,
    a1: ArgReg<'a, A1>,
    a2: ArgReg<'a, A2>,
    a3: ArgReg<'a, A3>,
    a4: ArgReg<'a, A4>,
    a5: ArgReg<'a, A5>,
) -> RetReg<R0> {
    let callee = transmute(super::param::auxv::vsyscall());
    asm::indirect_syscall6(callee, nr, a0, a1, a2, a3, a4, a5)
}

// With the indirect call, it isn't meaningful to do a separate
// `_readonly` optimization.
#[allow(unused_imports)]
pub(super) use {
    syscall0 as syscall0_readonly, syscall1 as syscall1_readonly, syscall2 as syscall2_readonly,
    syscall3 as syscall3_readonly, syscall4 as syscall4_readonly, syscall5 as syscall5_readonly,
    syscall6 as syscall6_readonly,
};

/// The underlying syscall functions are only called from asm, using the
/// special syscall calling convention to pass arguments and return values,
/// which the signature here doesn't reflect.
pub(super) type SyscallType = unsafe extern "C" fn();
