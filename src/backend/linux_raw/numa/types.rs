use bitflags::bitflags;

bitflags! {
    /// `MPOL_*` and `MPOL_F_*` flags for use with [`mbind`].
    ///
    /// [`mbind`]: crate::io::mbind
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Mode: u32 {
    /// `MPOL_F_STATIC_NODES`
    const STATIC_NODES = linux_raw_sys::mempolicy::MPOL_F_STATIC_NODES;
    /// `MPOL_F_RELATIVE_NODES`
    const RELATIVE_NODES = linux_raw_sys::mempolicy::MPOL_F_RELATIVE_NODES;
    /// `MPOL_F_NUMA_BALANCING`
    const NUMA_BALANCING = linux_raw_sys::mempolicy::MPOL_F_NUMA_BALANCING;

    /// `MPOL_DEFAULT`
    const DEFAULT = linux_raw_sys::mempolicy::MPOL_DEFAULT as u32;
    /// `MPOL_PREFERRED`
    const PREFERRED = linux_raw_sys::mempolicy::MPOL_PREFERRED as u32;
    /// `MPOL_BIND`
    const BIND = linux_raw_sys::mempolicy::MPOL_BIND as u32;
    /// `MPOL_INTERLEAVE`
    const INTERLEAVE = linux_raw_sys::mempolicy::MPOL_INTERLEAVE as u32;
    /// `MPOL_LOCAL`
    const LOCAL = linux_raw_sys::mempolicy::MPOL_LOCAL as u32;
    /// `MPOL_PREFERRED_MANY`
    const PREFERRED_MANY  = linux_raw_sys::mempolicy::MPOL_PREFERRED_MANY as u32;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

bitflags! {
    /// `MPOL_MF_*` flags for use with [`mbind`].
    ///
    /// [`mbind`]: crate::io::mbind
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ModeFlags: u32 {
    /// `MPOL_MF_STRICT`
    const STRICT = linux_raw_sys::mempolicy::MPOL_MF_STRICT;
    /// `MPOL_MF_MOVE`
    const MOVE = linux_raw_sys::mempolicy::MPOL_MF_MOVE;
    /// `MPOL_MF_MOVE_ALL`
    const MOVE_ALL = linux_raw_sys::mempolicy::MPOL_MF_MOVE_ALL;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}
