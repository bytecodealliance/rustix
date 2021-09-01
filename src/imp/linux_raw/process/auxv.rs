//! Linux auxv support.
//!
//! Currently this uses /proc/self/auxv (after doing extensive checks
//! that it is in fact procfs from the kernel). In the future, it may make
//! sense to add a way to initialize the auxv state from the auxv passed
//! into the process by the kernel instead.
//!
//! # Safety
//!
//! This uses `slice::from_raw_parts` to read from a file into an
//! array of `Elf_auxv_t` records.
#![allow(unsafe_code)]
#![allow(non_snake_case)]

use crate::io;
use crate::io::{pread, proc_self_auxv};
use linux_raw_sys::general::{AT_HWCAP, AT_NULL, AT_PAGESZ, AT_SYSINFO_EHDR};
use linux_raw_sys::v5_4::general::AT_HWCAP2;
use once_cell::sync::OnceCell;
use std::mem::size_of;
use std::slice;

#[inline]
pub(crate) fn page_size() -> usize {
    auxv().page_size
}

#[inline]
pub(crate) fn linux_hwcap() -> (usize, usize) {
    let auxv = auxv();
    (auxv.hwcap, auxv.hwcap2)
}

#[inline]
pub(in super::super) fn sysinfo_ehdr() -> usize {
    auxv().sysinfo_ehdr
}

struct Auxv {
    page_size: usize,
    hwcap: usize,
    hwcap2: usize,
    sysinfo_ehdr: usize,
}

#[inline]
fn auxv() -> &'static Auxv {
    static AUXV: OnceCell<Auxv> = OnceCell::new();
    AUXV.get_or_init(initialize)
}

fn initialize() -> Auxv {
    // Some architectures need to do some setup before we can make syscalls.
    super::super::vdso_wrappers::init_before_auxv();

    // Open /proc/self/auxv and confirm it's the real thing from the kernel.
    let fd = proc_self_auxv().unwrap();

    let mut auxv = Auxv {
        page_size: 0,
        hwcap: 0,
        hwcap2: 0,
        sysinfo_ehdr: 0,
    };

    // A buffer for `Elf_auxv_t` records.
    let mut buffer = [Elf_auxv_t {
        a_type: 0,
        a_val: 0,
    }; 16];

    // Safety: Use `slice::from_raw_parts` to get a byte-slice view of `buffer`.
    let byte_slice = unsafe {
        slice::from_raw_parts_mut(
            (&mut buffer as *mut Elf_auxv_t).cast::<u8>(),
            buffer.len() * size_of::<Elf_auxv_t>(),
        )
    };

    // Iterate over the auxv records until we find AT_SYSINFO_EHDR, which
    // points to the vDSO ELF header.
    let mut offset = 0;
    loop {
        match pread(&fd, byte_slice, offset) {
            Ok(0) => break,
            Ok(n) => {
                let elf_auxv_slice = &buffer[..n / size_of::<Elf_auxv_t>()];
                for elf_auxv in elf_auxv_slice {
                    let (a_type, a_val) = (elf_auxv.a_type, elf_auxv.a_val);
                    match a_type as u32 {
                        AT_PAGESZ => auxv.page_size = a_val,
                        AT_HWCAP => auxv.hwcap = a_val,
                        AT_HWCAP2 => auxv.hwcap2 = a_val,
                        AT_SYSINFO_EHDR => auxv.sysinfo_ehdr = a_val,
                        AT_NULL => break,
                        _ => (),
                    }
                }
                offset += (elf_auxv_slice.len() * size_of::<Elf_auxv_t>()) as u64;
            }
            Err(io::Error::INTR) => continue,
            Err(err) => Err(err).unwrap(),
        }
    }

    auxv
}

// ELF ABI

#[repr(C)]
#[derive(Copy, Clone)]
struct Elf_auxv_t {
    a_type: usize,
    a_val: usize,
}
