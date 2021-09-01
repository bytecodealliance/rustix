//! Parse the Linux vDSO.
//!
//! The following code is transliterated from
//! tools/testing/selftests/vDSO/parse_vdso.c in Linux 5.11, which is licensed
//! with Creative Commons Zero License, version 1.0,
//! available at <http://creativecommons.org/publicdomain/zero/1.0/legalcode>
//!
//! # Safety
//!
//! Parsing the vDSO involves a lot of raw pointer manipulation. This
//! implementation follows Linux's reference implementation, and adds several
//! additional safety checks.
#![allow(unsafe_code)]
#![allow(non_snake_case)]

use crate::io::{self, madvise, Advice};
use std::ffi::CStr;
use std::mem::{align_of, size_of};
use std::os::raw::{c_char, c_void};
use std::ptr::null;

pub(super) struct Vdso {
    // Load information
    load_addr: usize,
    load_end: usize,    // the end of the `PT_LOAD` segment
    load_offset: usize, // load_addr - recorded vaddr

    // Symbol table
    symtab: *const Elf_Sym,
    symstrings: *const c_char,
    bucket: *const u32,
    chain: *const u32,
    nbucket: u32,
    //nchain: u32,

    // Version table
    versym: *const u16,
    verdef: *const Elf_Verdef,
}

// Straight from the ELF specification.
fn elf_hash(name: &CStr) -> u32 {
    let mut h: u32 = 0;
    for b in name.to_bytes() {
        h = (h << 4).wrapping_add(u32::from(*b));
        let g = h & 0xf000_0000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= !g;
    }
    h
}

/// Cast `value` to a pointer type, doing some checks for validity.
fn make_pointer<T>(value: usize) -> Option<*const T> {
    if value == 0 || value.checked_add(size_of::<T>()).is_none() || value % align_of::<T>() != 0 {
        return None;
    }

    Some(value as *const T)
}

/// Create a `Vdso` value by parsing the vDSO at the given address.
///
/// # Safety
///
/// `base` must be a valid pointer to an ELF image in memory.
unsafe fn init_from_sysinfo_ehdr(base: usize) -> Option<Vdso> {
    let mut vdso = Vdso {
        load_addr: base,
        load_end: base,
        load_offset: 0,
        symtab: null(),
        symstrings: null(),
        bucket: null(),
        chain: null(),
        nbucket: 0,
        //nchain: 0,
        versym: null(),
        verdef: null(),
    };

    // Check that we're not attempting to parse our own ELF image.
    extern "C" {
        static __ehdr_start: c_void;
    }
    let ehdr_start: *const c_void = &__ehdr_start;
    if base == (ehdr_start as usize) {
        return None;
    }

    // Check that the vDSO is page-aligned and appropriately mapped.
    if madvise(base as *mut c_void, size_of::<Elf_Ehdr>(), Advice::Normal).is_err() {
        return None;
    }

    let hdr = &*make_pointer::<Elf_Ehdr>(base)?;

    if hdr.e_ident[..SELFMAG] != ELFMAG {
        return None; // Wrong ELF magic
    }
    if hdr.e_ident[EI_CLASS] != ELFCLASS {
        return None; // Wrong ELF class
    }
    if hdr.e_ident[EI_DATA] != ELFDATA {
        return None; // Wrong ELF data
    }
    if !matches!(hdr.e_ident[EI_OSABI], ELFOSABI_SYSV | ELFOSABI_LINUX) {
        return None; // Unrecognized ELF OS ABI
    }
    if hdr.e_ident[EI_ABIVERSION] != ELFABIVERSION {
        return None; // Unrecognized ELF ABI version
    }
    if hdr.e_type != ET_DYN {
        return None; // Wrong ELF type
    }
    // Verify that the `e_machine` matches the architecture we're running as.
    // This helps catch cases where we're running under qemu.
    if hdr.e_machine != EM_CURRENT {
        return None; // Wrong machine type
    }

    // If ELF is extended, we'll need to adjust.
    if hdr.e_ident[EI_VERSION] != EV_CURRENT
        || hdr.e_ehsize as usize != size_of::<Elf_Ehdr>()
        || hdr.e_phentsize as usize != size_of::<Elf_Phdr>()
    {
        return None;
    }
    // We don't currently support extra-large numbers of segments.
    if hdr.e_phnum == PN_XNUM {
        return None;
    }

    // If `e_phoff` is zero, it's more likely that we're looking at memory that
    // has been zeroed than that the kernel has somehow aliased the `Ehdr` and
    // the `Phdr`.
    if hdr.e_phoff < size_of::<Elf_Ehdr>() {
        return None;
    }

    // Check that the vDSO is not writable, since that would indicate that this
    // isn't the kernel vDSO. Here we're just using `clock_getres` just as an
    // arbitrary system call which writes to a buffer and fails with `EFAULT`
    // if the buffer is not writeable.
    {
        use super::arch::choose::syscall2;
        use super::conv::{clockid_t, ret, void_star};
        use super::reg::nr;
        use crate::time::ClockId;
        use linux_raw_sys::general::__NR_clock_getres;
        if ret(syscall2(
            nr(__NR_clock_getres),
            clockid_t(ClockId::Monotonic),
            void_star(base as *mut c_void),
        )) != Err(io::Error::FAULT)
        {
            // We can't gracefully fail here because we would seem to have just
            // mutated some unknown memory.
            std::process::abort();
        }
    }

    let pt = make_pointer::<Elf_Phdr>(base.checked_add(hdr.e_phoff)?)?;
    let mut dyn_: *const Elf_Dyn = null();
    let mut num_dyn = 0;

    // We need two things from the segment table: the load offset
    // and the dynamic table.
    let mut found_vaddr = false;
    for i in 0..hdr.e_phnum {
        let phdr = &*pt.add(i as usize);
        if phdr.p_flags & PF_W != 0 {
            // Don't trust any vDSO that claims to be loading writeable
            // segments into memory.
            return None;
        }
        if phdr.p_type == PT_LOAD && !found_vaddr {
            // The segment should be readable and executable, because it
            // contains the symbol table and the function bodies.
            if phdr.p_flags & (PF_R | PF_X) != (PF_R | PF_X) {
                return None;
            }
            found_vaddr = true;
            vdso.load_end = base.checked_add(phdr.p_offset.checked_add(phdr.p_memsz)?)?;
            vdso.load_offset = base.wrapping_add(phdr.p_offset.wrapping_sub(phdr.p_vaddr));
        } else if phdr.p_type == PT_DYNAMIC {
            // If `p_offset` is zero, it's more likely that we're looking at memory
            // that has been zeroed than that the kernel has somehow aliased the
            // `Ehdr` and the `Elf_Dyn` array.
            if phdr.p_offset < size_of::<Elf_Ehdr>() {
                return None;
            }

            dyn_ = make_pointer::<Elf_Dyn>(base.checked_add(phdr.p_offset)?)?;
            num_dyn = phdr.p_memsz / size_of::<Elf_Dyn>();
        } else if phdr.p_type == PT_INTERP {
            // Don't trust any ELF image that has an "interpreter", which is
            // likely to be a user ELF image rather than the kernel vDSO.
            return None;
        } else if phdr.p_type == PT_GNU_RELRO {
            // Don't trust any ELF image that uses GNU RELRO, which is likely
            // to be a user ELF image rather than the kernel vDSO.
            return None;
        }
    }

    if !found_vaddr || dyn_.is_null() {
        return None; // Failed
    }

    // Fish out the useful bits of the dynamic table.
    let mut hash: *const u32 = null();
    vdso.symstrings = null();
    vdso.symtab = null();
    vdso.versym = null();
    vdso.verdef = null();
    let mut i = 0;
    loop {
        if i == num_dyn {
            return None;
        }
        let d = &*dyn_.add(i);
        match d.d_tag {
            DT_STRTAB => {
                vdso.symstrings = make_pointer::<c_char>(d.d_val.checked_add(vdso.load_offset)?)?;
            }
            DT_SYMTAB => {
                vdso.symtab = make_pointer::<Elf_Sym>(d.d_val.checked_add(vdso.load_offset)?)?;
            }
            DT_HASH => {
                hash = make_pointer::<u32>(d.d_val.checked_add(vdso.load_offset)?)?;
            }
            DT_VERSYM => {
                vdso.versym = make_pointer::<u16>(d.d_val.checked_add(vdso.load_offset)?)?;
            }
            DT_VERDEF => {
                vdso.verdef = make_pointer::<Elf_Verdef>(d.d_val.checked_add(vdso.load_offset)?)?;
            }
            DT_SYMENT => {
                if d.d_val != size_of::<Elf_Sym>() {
                    return None; // Failed
                }
            }
            DT_NULL => break,
            _ => {}
        }
        i = i.checked_add(1)?;
    }
    if vdso.symstrings.is_null() || vdso.symtab.is_null() || hash.is_null() {
        return None; // Failed
    }

    if vdso.verdef.is_null() {
        vdso.versym = null();
    }

    // Parse the hash table header.
    vdso.nbucket = *hash.add(0);
    //vdso.nchain = *hash.add(1);
    vdso.bucket = hash.add(2);
    vdso.chain = hash.add(vdso.nbucket as usize + 2);

    // That's all we need.
    Some(vdso)
}

impl Vdso {
    /// Parse the vDSO.
    ///
    /// Returns None if the vDSO can't be located or if it doesn't conform
    /// to our expectations.
    #[inline]
    pub(super) fn new() -> Option<Self> {
        init_from_proc_self_auxv()
    }

    /// Check the version for a symbol.
    ///
    /// # Safety
    ///
    /// The raw pointers inside `self` must be valid.
    unsafe fn match_version(&self, mut ver: u16, name: &CStr, hash: u32) -> bool {
        // This is a helper function to check if the version indexed by
        // ver matches name (which hashes to hash).
        //
        // The version definition table is a mess, and I don't know how
        // to do this in better than linear time without allocating memory
        // to build an index.  I also don't know why the table has
        // variable size entries in the first place.
        //
        // For added fun, I can't find a comprehensible specification of how
        // to parse all the weird flags in the table.
        //
        // So I just parse the whole table every time.

        // First step: find the version definition
        ver &= 0x7fff; // Apparently bit 15 means "hidden"
        let mut def = self.verdef;
        loop {
            if (*def).vd_version != VER_DEF_CURRENT {
                return false; // Failed
            }

            if ((*def).vd_flags & VER_FLG_BASE) == 0 && ((*def).vd_ndx & 0x7fff) == ver {
                break;
            }

            if (*def).vd_next == 0 {
                return false; // No definition.
            }

            def = def
                .cast::<c_char>()
                .add((*def).vd_next as usize)
                .cast::<Elf_Verdef>();
        }

        // Now figure out whether it matches.
        let aux = &*(def.cast::<c_char>())
            .add((*def).vd_aux as usize)
            .cast::<Elf_Verdaux>();
        (*def).vd_hash == hash
            && (name == CStr::from_ptr(self.symstrings.add(aux.vda_name as usize)))
    }

    /// Look up a symbol in the vDSO.
    pub(super) fn sym(&self, version: &CStr, name: &CStr) -> *const c_void {
        let ver_hash = elf_hash(version);
        let name_hash = elf_hash(name);

        // Safety: The pointers in `self` must be valid.
        unsafe {
            let mut chain = *self.bucket.add((name_hash % self.nbucket) as usize);

            while chain != STN_UNDEF {
                let sym = &*self.symtab.add(chain as usize);

                // Check for a defined global or weak function w/ right name.
                if ELF_ST_TYPE(sym.st_info) != STT_FUNC ||
		       (ELF_ST_BIND(sym.st_info) != STB_GLOBAL &&
		        ELF_ST_BIND(sym.st_info) != STB_WEAK) ||
		       sym.st_shndx == SHN_UNDEF ||
		       sym.st_shndx == SHN_ABS ||
               ELF_ST_VISIBILITY(sym.st_other) != STV_DEFAULT ||
		       (name != CStr::from_ptr(self.symstrings.add(sym.st_name as usize))) ||
		       // Check symbol version.
		       (!self.versym.is_null()
		        && !self.match_version(*self.versym.add(chain as usize),
					       version, ver_hash))
                {
                    chain = *self.chain.add(chain as usize);
                    continue;
                }

                let sum = self.load_offset.checked_add(sym.st_value).unwrap();
                assert!(sum >= self.load_addr && sum <= self.load_end);
                return sum as *const c_void;
            }
        }

        null()
    }
}

// Find the `AT_SYSINFO_EHDR` in auxv records in memory. We don't currently
// have direct access to the auxv records in memory, so we use /proc/self/auxv
// instead.
//
// # Safety
//
// `elf_auxv` must point to a valid array of AUXV records terminated by an
// `AT_NULL` record.
/*
unsafe fn init_from_auxv(elf_auxv: *const Elf_auxv_t) -> Option<Vdso> {
    let mut i = 0;
    while (*elf_auxv.add(i)).a_type != AT_NULL {
        if (*elf_auxv.add(i)).a_type == AT_SYSINFO_EHDR {
            return init_from_sysinfo_ehdr((*elf_auxv.add(i)).a_val);
        }
        i += 1;
    }

    None
}
*/

// Find the `AT_SYSINFO_EHDR` in auxv records in /proc/self/auxv.
fn init_from_proc_self_auxv() -> Option<Vdso> {
    // Safety: `sysinfo_ehdr` does extensive checks to ensure that the value
    // we get really is an `AT_SYSINFO_EHDR` value from the kernel.
    unsafe { init_from_sysinfo_ehdr(super::process::sysinfo_ehdr()) }
}

// ELF ABI

const SELFMAG: usize = 4;
const ELFMAG: [u8; SELFMAG] = [0x7f, b'E', b'L', b'F'];
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;
const EV_CURRENT: u8 = 1;
#[cfg(target_pointer_width = "32")]
const ELFCLASS: u8 = 1; // ELFCLASS32
#[cfg(target_pointer_width = "64")]
const ELFCLASS: u8 = 2; // ELFCLASS64
#[cfg(target_endian = "little")]
const ELFDATA: u8 = 1; // ELFDATA2LSB
#[cfg(target_endian = "big")]
const ELFDATA: u8 = 2; // ELFDATA2MSB
const ELFOSABI_SYSV: u8 = 0;
const ELFOSABI_LINUX: u8 = 3;
// At present all of our supported platforms use 0.
const ELFABIVERSION: u8 = 0;
const ET_DYN: u16 = 3;
const EI_NIDENT: usize = 16;
const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xfff1;
const PN_XNUM: u16 = 0xffff;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_GNU_RELRO: u32 = 0x6474_e552;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const DT_NULL: i32 = 0;
const DT_HASH: i32 = 4;
const DT_STRTAB: i32 = 5;
const DT_SYMTAB: i32 = 6;
const DT_SYMENT: i32 = 11;
const DT_VERSYM: i32 = 0x6fff_fff0;
const DT_VERDEF: i32 = 0x6fff_fffc;
const STB_WEAK: u8 = 2;
const STB_GLOBAL: u8 = 1;
const STT_FUNC: u8 = 2;
const STN_UNDEF: u32 = 0;
const VER_FLG_BASE: u16 = 0x1;
const VER_DEF_CURRENT: u16 = 1;
const STV_DEFAULT: u8 = 0;
#[cfg(target_arch = "x86")]
const EM_CURRENT: u16 = 3; // EM_386
#[cfg(target_arch = "x86_64")]
const EM_CURRENT: u16 = 62; // EM_X86_64
#[cfg(target_arch = "aarch64")]
const EM_CURRENT: u16 = 183; // EM_AARCH64
#[cfg(target_arch = "riscv64")]
const EM_CURRENT: u16 = 243; // EM_RISCV

#[inline]
const fn ELF_ST_VISIBILITY(o: u8) -> u8 {
    o & 0x03
}

#[inline]
const fn ELF_ST_BIND(val: u8) -> u8 {
    val >> 4
}

#[inline]
const fn ELF_ST_TYPE(val: u8) -> u8 {
    val & 0xf
}

#[repr(C)]
struct Elf_Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: usize,
    e_phoff: usize,
    e_shoff: usize,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
struct Elf_Phdr {
    p_type: u32,
    p_offset: usize,
    p_vaddr: usize,
    p_paddr: usize,
    p_filesz: usize,
    p_memsz: usize,
    p_flags: u32,
    p_align: usize,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct Elf_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: usize,
    p_vaddr: usize,
    p_paddr: usize,
    p_filesz: usize,
    p_memsz: usize,
    p_align: usize,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
struct Elf_Sym {
    st_name: u32,
    st_value: usize,
    st_size: usize,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct Elf_Sym {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: usize,
    st_size: usize,
}

#[repr(C)]
struct Elf_Dyn {
    d_tag: i32,
    d_val: usize,
}

#[repr(C)]
struct Elf_Verdef {
    vd_version: u16,
    vd_flags: u16,
    vd_ndx: u16,
    vd_cnt: u16,
    vd_hash: u32,
    vd_aux: u32,
    vd_next: u32,
}

#[repr(C)]
struct Elf_Verdaux {
    vda_name: u32,
    _vda_next: u32,
}
