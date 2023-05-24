#[cfg(any(linux_kernel, target_os = "freebsd", target_os = "illumos"))]
use {crate::backend::c, bitflags::bitflags};

#[cfg(any(linux_kernel, target_os = "freebsd", target_os = "illumos"))]
bitflags! {
    /// `EFD_*` flags for use with [`eventfd`].
    ///
    /// [`eventfd`]: crate::io::eventfd
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct EventfdFlags: c::c_int {
        /// `EFD_CLOEXEC`
        const CLOEXEC = c::EFD_CLOEXEC;
        /// `EFD_NONBLOCK`
        const NONBLOCK = c::EFD_NONBLOCK;
        /// `EFD_SEMAPHORE`
        const SEMAPHORE = c::EFD_SEMAPHORE;
    }
}
