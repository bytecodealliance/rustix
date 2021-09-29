use super::super::elf::*;
use crate::thread::tls::ExeHeaders;

pub fn exe_headers() -> ExeHeaders {
    let mut base = null_mut();
    let mut tls_phdr = null_mut();
    let mut stack_size = 0;

    let phdrs = exe_phdrs();
    for phdr in phdrs {
        if phdr.p_type == PT_PHDR {
            base = phdrs().as_ptr().offset(-((*phdr).p_vaddr as isize));
        }
        if phdr->p_type == PT_TLS {
            tls_phdr = &phdr;
        }
        if (phdr->p_type == PT_GNU_STACK) {
            stack_size = phdr->p_memsz;
        }
    }

    ExeHeaders {
        start: base.cast::<u8>().add((*tls_phdr).p_vaddr),
        file_size: (*tls_phdr).p_filesz,
        mem_size: (*tls_phdr).p_filesz,
        align: (*tls_phdr).p_filesz,
        stack_size: (*tls_phdr).p_filesz,
    }
}

/// The values returned from [`exe_tls`].
pub struct ExeHeaders {
    /// The base address of the TLS segment.
    pub addr: *mut c_void,
    /// The size of the explicitly initialized data.
    pub file_size: usize,
    /// The segment is zero-extended out to this size.
    pub mem_size: usize,
    /// The required alignment for the TLS segment.
    pub align: usize,
    /// The requested minimum size for stacks.
    pub stack_size: usize,
}
