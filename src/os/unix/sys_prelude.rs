#[allow(clippy::unnecessary_cast)]
pub mod c {
    pub const S_IFMT: u32 = libc::S_IFMT as u32;
    pub const S_IFBLK: u32 = libc::S_IFBLK as u32;
    pub const S_IFCHR: u32 = libc::S_IFCHR as u32;
    pub const S_IFDIR: u32 = libc::S_IFDIR as u32;
    pub const S_IFIFO: u32 = libc::S_IFIFO as u32;
    pub const S_IFLNK: u32 = libc::S_IFLNK as u32;
    pub const S_IFREG: u32 = libc::S_IFREG as u32;
    pub const S_IFSOCK: u32 = libc::S_IFSOCK as u32;

    pub const S_IRUSR: u32 = libc::S_IRUSR as u32;
    pub const S_IWUSR: u32 = libc::S_IWUSR as u32;
    pub const S_IXUSR: u32 = libc::S_IXUSR as u32;
    pub const S_ISUID: u32 = libc::S_ISUID as u32;

    pub const S_IRGRP: u32 = libc::S_IRGRP as u32;
    pub const S_IWGRP: u32 = libc::S_IWGRP as u32;
    pub const S_IXGRP: u32 = libc::S_IXGRP as u32;
    pub const S_ISGID: u32 = libc::S_ISGID as u32;

    pub const S_IROTH: u32 = libc::S_IROTH as u32;
    pub const S_IWOTH: u32 = libc::S_IWOTH as u32;
    pub const S_IXOTH: u32 = libc::S_IXOTH as u32;
    pub const S_ISVTX: u32 = libc::S_ISVTX as u32;

    pub use libc::getgrgid_r;
    pub use libc::getpwuid_r;
    pub use libc::group;
    pub use libc::passwd;
}
