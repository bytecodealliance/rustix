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

impl FileType {
    /// Construct a `FileType` from the `st_mode` field of a `LibcStat`.
    pub fn from_libc_stat_st_mode(st_mode: libc::mode_t) -> Self {
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

    /// Construct a `FileType` from the `d_type` field of a `libc::dirent`.
    pub(crate) fn from_dirent_d_type(d_type: u8) -> Self {
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
            /* libc::DT_UNKNOWN | */ _ => Self::Unknown,
        }
    }
}
