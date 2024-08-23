use crate::backend::c;
use crate::backend::conv::ret_usize;
use crate::timespec::Timespec;
use crate::{futex, io};
use core::sync::atomic::AtomicU32;

pub(crate) unsafe fn futex_val2(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    val2: u32,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    // The least-significant four bytes of the timeout pointer are used as `val2`.
    // ["the kernel casts the timeout value first to unsigned long, then to uint32_t"](https://man7.org/linux/man-pages/man2/futex.2.html),
    // so we perform that exact conversion in reverse to create the pointer.
    let timeout = val2 as usize as *const Timespec;

    #[cfg(all(
        target_pointer_width = "32",
        not(any(target_arch = "aarch64", target_arch = "x86_64"))
    ))]
    {
        // TODO: Upstream this to the libc crate.
        #[allow(non_upper_case_globals)]
        const SYS_futex_time64: i32 = linux_raw_sys::general::__NR_futex_time64 as i32;

        syscall! {
            fn futex_time64(
                uaddr: *const AtomicU32,
                futex_op: c::c_int,
                val: u32,
                timeout: *const Timespec,
                uaddr2: *const AtomicU32,
                val3: u32
            ) via SYS_futex_time64 -> c::ssize_t
        }

        ret_usize(futex_time64(
            uaddr,
            op as i32 | flags.bits() as i32,
            val,
            timeout,
            uaddr2,
            val3,
        ))
    }

    #[cfg(any(
        target_pointer_width = "64",
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    {
        syscall! {
            fn futex(
                uaddr: *const AtomicU32,
                futex_op: c::c_int,
                val: u32,
                timeout: *const linux_raw_sys::general::__kernel_timespec,
                uaddr2: *const AtomicU32,
                val3: u32
            ) via SYS_futex -> c::c_long
        }

        ret_usize(futex(
            uaddr,
            op as i32 | flags.bits() as i32,
            val,
            timeout.cast(),
            uaddr2,
            val3,
        ) as isize)
    }
}

pub(crate) unsafe fn futex_timeout(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    timeout: *const Timespec,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    #[cfg(all(
        target_pointer_width = "32",
        not(any(target_arch = "aarch64", target_arch = "x86_64"))
    ))]
    {
        // TODO: Upstream this to the libc crate.
        #[allow(non_upper_case_globals)]
        const SYS_futex_time64: i32 = linux_raw_sys::general::__NR_futex_time64 as i32;

        syscall! {
            fn futex_time64(
                uaddr: *const AtomicU32,
                futex_op: c::c_int,
                val: u32,
                timeout: *const Timespec,
                uaddr2: *const AtomicU32,
                val3: u32
            ) via SYS_futex_time64 -> c::ssize_t
        }

        ret_usize(futex_time64(
            uaddr,
            op as i32 | flags.bits() as i32,
            val,
            timeout,
            uaddr2,
            val3,
        ))
        .or_else(|err| {
            // See the comments in `rustix_clock_gettime_via_syscall` about
            // emulation.
            if err == io::Errno::NOSYS {
                futex_old_timespec(uaddr, op, flags, val, timeout, uaddr2, val3)
            } else {
                Err(err)
            }
        })
    }

    #[cfg(any(
        target_pointer_width = "64",
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    {
        syscall! {
            fn futex(
                uaddr: *const AtomicU32,
                futex_op: c::c_int,
                val: u32,
                timeout: *const linux_raw_sys::general::__kernel_timespec,
                uaddr2: *const AtomicU32,
                val3: u32
            ) via SYS_futex -> c::c_long
        }

        ret_usize(futex(
            uaddr,
            op as i32 | flags.bits() as i32,
            val,
            timeout.cast(),
            uaddr2,
            val3,
        ) as isize)
    }
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "aarch64", target_arch = "x86_64"))
))]
unsafe fn futex_old_timespec(
    uaddr: *const AtomicU32,
    op: super::types::Operation,
    flags: futex::Flags,
    val: u32,
    timeout: *const Timespec,
    uaddr2: *const AtomicU32,
    val3: u32,
) -> io::Result<usize> {
    syscall! {
        fn futex(
            uaddr: *const AtomicU32,
            futex_op: c::c_int,
            val: u32,
            timeout: *const linux_raw_sys::general::__kernel_old_timespec,
            uaddr2: *const AtomicU32,
            val3: u32
        ) via SYS_futex -> c::c_long
    }

    let old_timeout = if timeout.is_null() {
        None
    } else {
        Some(linux_raw_sys::general::__kernel_old_timespec {
            tv_sec: (*timeout).tv_sec.try_into().map_err(|_| io::Errno::INVAL)?,
            tv_nsec: (*timeout)
                .tv_nsec
                .try_into()
                .map_err(|_| io::Errno::INVAL)?,
        })
    };
    ret_usize(futex(
        uaddr,
        op as i32 | flags.bits() as i32,
        val,
        old_timeout
            .as_ref()
            .map(|timeout| timeout as *const linux_raw_sys::general::__kernel_old_timespec)
            .unwrap_or(core::ptr::null()),
        uaddr2,
        val3,
    ) as isize)
}
