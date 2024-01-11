#![allow(unsafe_code)]

#[cfg(feature = "alloc")]
use crate::alloc::string::String;
use crate::backend::io::syscalls::ioctl;
use crate::fd::AsFd;
use crate::io;
use linux_raw_sys::ioctl::SIOCGIFINDEX;
#[cfg(feature = "alloc")]
use linux_raw_sys::ioctl::SIOCGIFNAME;
use linux_raw_sys::net::{ifreq, ifreq__bindgen_ty_1, ifreq__bindgen_ty_2, IFNAMSIZ};

pub(crate) fn name_to_index(fd: impl AsFd, if_name: &str) -> io::Result<u32> {
    let if_name_bytes = if_name.as_bytes();
    if if_name_bytes.len() >= IFNAMSIZ as usize {
        return Err(io::Errno::NODEV);
    }

    let mut ifreq = ifreq {
        ifr_ifrn: ifreq__bindgen_ty_1 { ifrn_name: [0; 16] },
        ifr_ifru: ifreq__bindgen_ty_2 { ifru_ivalue: 0 },
    };
    unsafe { ifreq.ifr_ifrn.ifrn_name[..if_name_bytes.len()].copy_from_slice(if_name_bytes) };

    unsafe { ioctl(fd.as_fd(), SIOCGIFINDEX, &mut ifreq as *mut ifreq as _) }?;
    let index = unsafe { ifreq.ifr_ifru.ifru_ivalue };
    Ok(index as u32)
}

#[cfg(feature = "alloc")]
pub(crate) fn index_to_name(fd: impl AsFd, index: u32) -> io::Result<String> {
    let mut ifreq = ifreq {
        ifr_ifrn: ifreq__bindgen_ty_1 { ifrn_name: [0; 16] },
        ifr_ifru: ifreq__bindgen_ty_2 {
            ifru_ivalue: index as _,
        },
    };

    unsafe { ioctl(fd.as_fd(), SIOCGIFNAME, &mut ifreq as *mut ifreq as _) }?;

    if let Some(nul_byte) = unsafe { ifreq.ifr_ifrn.ifrn_name }
        .iter()
        .position(|char| *char == 0)
    {
        let name = unsafe { ifreq.ifr_ifrn.ifrn_name }[..nul_byte]
            .iter()
            .map(|v| *v as char)
            .collect();

        Ok(name)
    } else {
        Err(io::Errno::INVAL)
    }
}
