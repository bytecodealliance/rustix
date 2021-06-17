use crate::fs::{Mode, RawMode};

/// `S_IF*` constants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// `S_IFREG`
    RegularFile,

    /// `S_IFDIR`
    Directory,

    /// `S_IFLNK`
    Symlink,

    /// `S_IFIFO`
    Fifo,

    /// `S_IFSOCK`
    Socket,

    /// `S_IFCHR`
    CharacterDevice,

    /// `S_IFBLK`
    BlockDevice,

    /// An unknown filesystem object.
    Unknown,
}

#[cfg(libc)]
impl FileType {
    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    pub const fn from_raw_mode(st_mode: RawMode) -> Self {
        match st_mode & libc::S_IFMT {
            libc::S_IFREG => Self::RegularFile,
            libc::S_IFDIR => Self::Directory,
            libc::S_IFLNK => Self::Symlink,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFIFO`.
            libc::S_IFIFO => Self::Fifo,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `S_IFSOCK`.
            libc::S_IFSOCK => Self::Socket,
            libc::S_IFCHR => Self::CharacterDevice,
            libc::S_IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
    }

    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    #[inline]
    pub const fn from_mode(st_mode: Mode) -> Self {
        Self::from_raw_mode(st_mode.bits())
    }

    /// Construct a `FileType` from the `d_type` field of a `libc::dirent`.
    #[cfg(not(target_os = "redox"))]
    pub(crate) const fn from_dirent_d_type(d_type: u8) -> Self {
        match d_type {
            libc::DT_REG => Self::RegularFile,
            libc::DT_DIR => Self::Directory,
            libc::DT_LNK => Self::Symlink,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `DT_SOCK`.
            libc::DT_SOCK => Self::Socket,
            #[cfg(not(target_os = "wasi"))] // TODO: Use WASI's `DT_FIFO`.
            libc::DT_FIFO => Self::Fifo,
            libc::DT_CHR => Self::CharacterDevice,
            libc::DT_BLK => Self::BlockDevice,
            // libc::DT_UNKNOWN |
            _ => Self::Unknown,
        }
    }
}

#[cfg(linux_raw)]
impl FileType {
    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    pub const fn from_raw_mode(st_mode: RawMode) -> Self {
        match st_mode & linux_raw_sys::general::S_IFMT {
            linux_raw_sys::general::S_IFREG => Self::RegularFile,
            linux_raw_sys::general::S_IFDIR => Self::Directory,
            linux_raw_sys::general::S_IFLNK => Self::Symlink,
            linux_raw_sys::general::S_IFIFO => Self::Fifo,
            linux_raw_sys::general::S_IFSOCK => Self::Socket,
            linux_raw_sys::general::S_IFCHR => Self::CharacterDevice,
            linux_raw_sys::general::S_IFBLK => Self::BlockDevice,
            _ => Self::Unknown,
        }
    }

    /// Construct a `FileType` from the `st_mode` field of a `Stat`.
    pub const fn from_mode(st_mode: Mode) -> Self {
        Self::from_raw_mode(st_mode.bits())
    }

    /// Construct a `FileType` from the `d_type` field of a `dirent`.
    pub(crate) const fn from_dirent_d_type(d_type: u8) -> Self {
        match d_type as u32 {
            linux_raw_sys::general::DT_REG => Self::RegularFile,
            linux_raw_sys::general::DT_DIR => Self::Directory,
            linux_raw_sys::general::DT_LNK => Self::Symlink,
            linux_raw_sys::general::DT_SOCK => Self::Socket,
            linux_raw_sys::general::DT_FIFO => Self::Fifo,
            linux_raw_sys::general::DT_CHR => Self::CharacterDevice,
            linux_raw_sys::general::DT_BLK => Self::BlockDevice,
            // linux_raw_sys::general::DT_UNKNOWN |
            _ => Self::Unknown,
        }
    }
}
