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

use crate::config::{AllocatedSizeBlocks, Config};
use crate::output::DisplayCell;

use accounts::get_accountname_cell_by_sid_ptr;
use permissions::get_rwx_permissions;
use security_info::SecurityInfo;

#[derive(Debug, Default)]
pub struct WindowsMetadata {
    nlink: Option<u32>,
    allocated_size: Option<u64>,
    size: Option<u64>,
    rwx_permissions: String,
    owner_cell: DisplayCell,
    group_cell: DisplayCell,
}

impl WindowsMetadata {
    pub fn get(path: &Path, config: &Config) -> Self {
        let wide_path = WideString::from_path(path);
        let mut windows_metadata = Self::default();

        if config.output_format.is_long()
            || config.sorting_order.is_size()
            || config.list_allocated_size
        {
            windows_metadata.init_file_standard_info(&wide_path, path);
        }

        if config.output_format.is_long() {
            windows_metadata.init_security_info(&wide_path, path, config);
        }

        windows_metadata
    }

    fn init_file_standard_info(&mut self, wide_path: &[u16], path: &Path) {
        match FileHandle::open(&wide_path, 0) {
            Ok(file_handle) => self.get_file_standard_info_by_handle(&file_handle, path),
            Err(err) => {
                eprintln!(
                    "nls: unable to get nlink, allocated size for '{}': {}",
                    path.display(),
                    err
                );

                return;
            }
        }
    }

    fn get_file_standard_info_by_handle(&mut self, file_handle: &FileHandle, path: &Path) {
        unsafe {
            let mut file_standard_info = MaybeUninit::<c::FILE_STANDARD_INFO>::uninit();

            // On success, return_code is non-zero
            let return_code = c::GetFileInformationByHandleEx(
                file_handle.raw_handle(),
                c::FileStandardInfo,
                file_standard_info.as_mut_ptr() as *mut c_void,
                mem::size_of::<c::FILE_STANDARD_INFO>() as u32,
            );

            if return_code != 0 {
                let file_standard_info = file_standard_info.assume_init();
                self.allocated_size = Some(file_standard_info.AllocationSize as u64);
                self.nlink = Some(file_standard_info.NumberOfLinks);
                self.size = Some(file_standard_info.EndOfFile as u64);
            } else {
                eprintln!(
                    "nls: unable to get nlink, allocated size for '{}': {}",
                    path.display(),
                    io::Error::last_os_error()
                );
            }
        }
    }

    fn init_security_info(&mut self, wide_path: &[u16], path: &Path, config: &Config) {
        match SecurityInfo::from_wide_path(wide_path) {
            Ok(security_info) => {
                self.rwx_permissions = get_rwx_permissions(&security_info);
                if config.list_owner {
                    self.owner_cell =
                        get_accountname_cell_by_sid_ptr(security_info.sid_owner_ptr(), config);
                }
                if config.list_group {
                    self.group_cell =
                        get_accountname_cell_by_sid_ptr(security_info.sid_group_ptr(), config);
                }
            }
            Err(err) => {
                eprintln!(
                    "nls: unable to get security info for '{}': {}",
                    path.display(),
                    err
                );
                self.rwx_permissions = String::from("?????????");
                self.owner_cell = DisplayCell::error_left_aligned();
                self.group_cell = DisplayCell::error_left_aligned();
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

    pub fn nlink_cell(&self) -> DisplayCell {
        match &self.nlink {
            Some(nlink) => DisplayCell::from_ascii_string(nlink.to_string(), false),
            None => DisplayCell::error_right_aligned(),
        }
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }

    pub fn owner_cell(&self) -> DisplayCell {
        self.owner_cell.clone()
    }

    pub fn group_cell(&self) -> DisplayCell {
        self.group_cell.clone()
    }

    pub fn rwx_mode_cell(&self, file_type: Option<FileType>) -> DisplayCell {
        mode::rwx_mode_cell(file_type, &self.rwx_permissions)
    }
}

struct FileHandle(c::HANDLE);

impl FileHandle {
    pub fn open(wide_path: &[u16], desired_access: u32) -> Result<Self, io::Error> {
        let flags_and_attributes = c::FILE_FLAG_BACKUP_SEMANTICS | c::FILE_FLAG_OPEN_REPARSE_POINT;
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

    pub fn raw_handle(&self) -> c::HANDLE {
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

    pub fn from_utf16_parts(utf16_buf: &[u16], length: usize) -> Self {
        let mut wide_buf = Self(utf16_buf[0..length].to_vec());

        match wide_buf.last() {
            Some(wch) => {
                if *wch != 0 {
                    wide_buf.0.push(0);
                }
            }
            None => wide_buf.0.push(0),
        }

        wide_buf
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

pub fn get_file_id_by_path(path: &Path) -> Result<u128, io::Error> {
    let wide_path = WideString::from_path(path);
    let file_handle = FileHandle::open(&wide_path, 0)?;

    unsafe {
        let mut file_id_info = MaybeUninit::<c::FILE_ID_INFO>::uninit();
        let return_code = c::GetFileInformationByHandleEx(
            file_handle.raw_handle(),
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
