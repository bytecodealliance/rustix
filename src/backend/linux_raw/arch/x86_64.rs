//! x86-64 Linux system calls.

#![cfg_attr(miri, allow(unreachable_code))]
#![cfg_attr(miri, allow(unused_variables))]

#[cfg(miri)]
use crate::backend::reg::Opaque;
use crate::backend::reg::{
    ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm as _, A0, A1, A2, A3, A4, A5, R0,
};
#[cfg(not(miri))]
use core::arch::asm;
use core::ptr;

#[cfg(target_pointer_width = "32")]
compile_error!("x32 is not yet supported");

fn returns_int(ret: libc::c_int) -> *mut Opaque {
    let i = match ret {
        -1 => -std::io::Error::last_os_error().raw_os_error().unwrap() as usize,
        n => n as usize,
    };
    ptr::without_provenance_mut(i)
}

fn returns_ssize_t(ret: libc::ssize_t) -> *mut Opaque {
    let i = match ret {
        -1 => -std::io::Error::last_os_error().raw_os_error().unwrap() as usize,
        n => n as usize,
    };
    ptr::without_provenance_mut(i)
}

unsafe fn syscall(
    nr: *mut Opaque,
    a0: *mut Opaque,
    a1: *mut Opaque,
    a2: *mut Opaque,
    a3: *mut Opaque,
    a4: *mut Opaque,
    a5: *mut Opaque,
) -> *mut Opaque {
    match nr.addr() as u32 {
        linux_raw_sys::general::__NR_clock_gettime => {
            returns_int(libc::clock_gettime(a0.addr() as _, a1.cast()))
        }
        linux_raw_sys::general::__NR_read => {
            returns_ssize_t(libc::read(a0.addr() as _, a1.cast(), a2.addr() as _))
        }
        linux_raw_sys::general::__NR_write => {
            returns_ssize_t(libc::write(a0.addr() as _, a1.cast(), a2.addr() as _))
        }
        linux_raw_sys::general::__NR_madvise => {
            returns_int(libc::madvise(a0.cast(), a1.addr() as _, a2.addr() as _))
        }
        nr => panic!("unsupported syscall {}", nr),
    }
}

fn unused() -> *mut Opaque {
    ptr::NonNull::dangling().as_ptr()
}

#[inline]
pub(in crate::backend) unsafe fn syscall0_readonly(nr: SyscallNumber<'_>) -> RetReg<R0> {
    let r0;

    #[cfg(not(miri))]
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            unused(),
            unused(),
            unused(),
            unused(),
            unused(),
            unused(),
        );
    }

    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall1(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> RetReg<R0> {
    let r0;

    #[cfg(not(miri))]
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            unused(),
            unused(),
            unused(),
            unused(),
            unused(),
        );
    }

    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall1_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
) -> RetReg<R0> {
    let r0;

    #[cfg(not(miri))]
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            unused(),
            unused(),
            unused(),
            unused(),
            unused(),
        );
    }

    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall1_noreturn(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> ! {
    #[cfg(not(miri))]
    {
        asm!(
            "syscall",
            in("rax") nr.to_asm(),
            in("rdi") a0.to_asm(),
            options(nostack, noreturn)
        )
    }

    #[cfg(miri)]
    {
        match nr.to_asm().addr() as u32 {
            nr => panic!("unsupported syscall1_readonly {}", nr),
        }
    }
}

#[inline]
pub(in crate::backend) unsafe fn syscall2(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;

    #[cfg(not(miri))]
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            unused(),
            unused(),
            unused(),
            unused(),
        );
    }

    FromAsm::from_asm(r0)
}

#[inline]
pub(in crate::backend) unsafe fn syscall2_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;

    #[cfg(not(miri))]
    asm!(
        "syscall",
        inlateout("rax") nr.to_asm() => r0,
        in("rdi") a0.to_asm(),
        in("rsi") a1.to_asm(),
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            unused(),
            unused(),
            unused(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            unused(),
            unused(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            unused(),
            unused(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            unused(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            unused(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            a4.to_asm(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            a4.to_asm(),
            unused(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            a4.to_asm(),
            a5.to_asm(),
        );
    }

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

    #[cfg(not(miri))]
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

    #[cfg(miri)]
    {
        r0 = syscall(
            nr.to_asm(),
            a0.to_asm(),
            a1.to_asm(),
            a2.to_asm(),
            a3.to_asm(),
            a4.to_asm(),
            a5.to_asm(),
        );
    }

    FromAsm::from_asm(r0)
}
