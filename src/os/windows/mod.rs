mod accounts;
mod mode;
mod permissions;
mod security_info;
mod sys_prelude;

use sys_prelude::*;

use std::ffi::c_void;
use std::fmt;
use std::fs::FileType;
use std::io;
use std::mem::{self, MaybeUninit};
use std::ops;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use nls_term_grid::{Alignment, GridCell};

use crate::config::{AllocatedSizeBlocks, Config};
use crate::output::GridCellExts;

use accounts::get_accountname_by_sid_ptr;
use permissions::get_rwx_permissions;
use security_info::SecurityInfo;

pub use mode::pwsh_mode_cell;

#[derive(Debug, Default)]
pub struct WindowsMetadata {
    nlink: Option<u64>,
    allocated_size: Option<u64>,
    size: Option<u64>,
    rwx_permissions: String,
    owner_string: String,
    group_string: String,
}

impl WindowsMetadata {
    pub fn get(path: &Path, follow_links: bool, config: &Config) -> Self {
        let wide_path = WideString::from_path(path);
        let mut windows_metadata = Self::default();

        if config.output_format.is_long()
            || config.sorting_order.is_size()
            || config.list_allocated_size
        {
            windows_metadata.init_from_file_standard_info(&wide_path, path, follow_links);
        }

        if config.output_format.is_long() {
            windows_metadata.init_from_security_info(&wide_path, path, follow_links, config);
        }

        windows_metadata
    }

    fn init_from_file_standard_info(&mut self, wide_path: &[u16], path: &Path, follow_links: bool) {
        match get_file_standard_info(wide_path, follow_links) {
            Ok(file_standard_info) => {
                self.allocated_size = Some(file_standard_info.AllocationSize as u64);
                self.nlink = Some(file_standard_info.NumberOfLinks as u64);
                self.size = Some(file_standard_info.EndOfFile as u64);
            }
            Err(err) => {
                eprintln!(
                    "nls: unable to get file standard info for '{}': {}",
                    path.display(),
                    err
                );
            }
        }
    }

    fn init_from_security_info(
        &mut self,
        wide_path: &[u16],
        path: &Path,
        follow_links: bool,
        config: &Config,
    ) {
        match SecurityInfo::from_wide_path(wide_path, follow_links) {
            Ok(security_info) => {
                if config.mode_format.is_rwx() {
                    self.rwx_permissions = get_rwx_permissions(&security_info, config);
                }
                if config.list_owner {
                    self.owner_string =
                        get_accountname_by_sid_ptr(security_info.sid_owner_ptr(), config);
                }
                if config.list_group {
                    self.group_string =
                        get_accountname_by_sid_ptr(security_info.sid_group_ptr(), config);
                }
            }
            Err(err) => {
                eprintln!(
                    "nls: unable to get security info for '{}': {}",
                    path.display(),
                    err
                );
                self.rwx_permissions = String::from("?????????");
                self.owner_string = String::from('?');
                self.group_string = String::from('?');
            }
        }
    }

    pub fn allocated_size(&self, config: &Config) -> Option<u64> {
        match self.allocated_size {
            Some(allocated_size) => match config.allocated_size_blocks {
                AllocatedSizeBlocks::Posix => Some(((allocated_size as f64) / 512.0).ceil() as u64),
                AllocatedSizeBlocks::Kibibytes => {
                    Some(((allocated_size as f64) / 1024.0).ceil() as u64)
                }
                AllocatedSizeBlocks::Raw => Some(allocated_size),
            },
            None => None,
        }
    }

    pub fn nlink_cell(&self, config: &Config) -> GridCell {
        let nlink_style = config.theme.nlink_style();
        match &self.nlink {
            Some(nlink) => GridCell::from_num_with_style(*nlink, nlink_style),
            None => GridCell::error_cell(Alignment::Right),
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn owner_cell(&self, config: &Config) -> GridCell {
        let owner_style = config.theme.owner_style();
        if self.owner_string == "?" {
            GridCell::error_cell(Alignment::Left)
        } else {
            GridCell::from_str_with_style(&self.owner_string, owner_style)
        }
    }

    pub fn group_cell(&self, config: &Config) -> GridCell {
        let group_style = config.theme.group_style();
        if self.group_string == "?" {
            GridCell::error_cell(Alignment::Left)
        } else {
            GridCell::from_str_with_style(&self.group_string, group_style)
        }
    }

    pub fn rwx_mode_cell(&self, file_type: Option<FileType>, config: &Config) -> GridCell {
        mode::rwx_mode_cell(file_type, &self.rwx_permissions, config)
    }
}

struct FileHandle(c::HANDLE);

impl FileHandle {
    pub fn open(
        wide_path: &[u16],
        desired_access: u32,
        follow_links: bool,
    ) -> Result<Self, io::Error> {
        let flags_and_attributes = if follow_links {
            c::FILE_FLAG_BACKUP_SEMANTICS
        } else {
            c::FILE_FLAG_BACKUP_SEMANTICS | c::FILE_FLAG_OPEN_REPARSE_POINT
        };
        unsafe {
            let hfile = c::CreateFileW(
                wide_path.as_ptr(),
                desired_access,
                c::FILE_SHARE_READ,
                ptr::null(),
                c::OPEN_EXISTING,
                flags_and_attributes,
                0,
            );

            if hfile != c::INVALID_HANDLE_VALUE {
                Ok(Self(hfile))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    pub fn as_raw_handle(&self) -> c::HANDLE {
        self.0
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        unsafe {
            if self.0 != c::INVALID_HANDLE_VALUE {
                c::CloseHandle(self.0);
            }
        }
    }
}

#[derive(Default)]
pub struct WideString(Vec<u16>);

impl WideString {
    pub fn from_path(path: &Path) -> Self {
        let mut wide_buf = path.as_os_str().encode_wide().collect::<Vec<u16>>();
        wide_buf.push(0);

        Self(wide_buf)
    }
}

impl fmt::Debug for WideString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L\"{}\"", utf16_until_null_to_string_lossy(&self))
    }
}

impl ops::Deref for WideString {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn utf16_until_null_to_string_lossy(utf16_buf: &[u16]) -> String {
    String::from_utf16_lossy(
        &utf16_buf
            .iter()
            .cloned()
            .take_while(|&c| c != 0)
            .collect::<Vec<u16>>(),
    )
}

pub fn get_file_id_identifier(path: &Path, follow_links: bool) -> Result<u128, io::Error> {
    let wide_path = WideString::from_path(path);
    let file_handle = FileHandle::open(&wide_path, 0, follow_links)?;

    unsafe {
        let mut file_id_info = MaybeUninit::<c::FILE_ID_INFO>::uninit();
        let return_code = c::GetFileInformationByHandleEx(
            file_handle.as_raw_handle(),
            c::FileIdInfo,
            file_id_info.as_mut_ptr() as *mut c_void,
            mem::size_of::<c::FILE_ID_INFO>() as u32,
        );

        if return_code != 0 {
            let file_id_info = file_id_info.assume_init();

            Ok(u128::from_le_bytes(file_id_info.FileId.Identifier))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

fn get_file_standard_info(
    wide_path: &[u16],
    follow_links: bool,
) -> Result<c::FILE_STANDARD_INFO, io::Error> {
    let file_handle = FileHandle::open(wide_path, 0, follow_links)?;

    unsafe {
        let mut file_standard_info = MaybeUninit::<c::FILE_STANDARD_INFO>::uninit();

        let return_code = c::GetFileInformationByHandleEx(
            file_handle.as_raw_handle(),
            c::FileStandardInfo,
            file_standard_info.as_mut_ptr() as *mut c_void,
            mem::size_of::<c::FILE_STANDARD_INFO>() as u32,
        );

        // On success, return_code is non-zero
        if return_code != 0 {
            let file_standard_info = file_standard_info.assume_init();

            Ok(file_standard_info)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
