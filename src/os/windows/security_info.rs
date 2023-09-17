use std::io;
use std::path::Path;
use std::ptr;

use super::sys_prelude::*;
use super::FileHandle;
use crate::config::Config;

pub struct SecurityInfo {
    sd_ptr: c::PSECURITY_DESCRIPTOR,
    sid_owner_ptr: c::PSID,
    sid_group_ptr: c::PSID,
    is_ok: bool,
}

impl SecurityInfo {
    pub fn from_wide_path(
        wide_path: &[u16],
        path: &Path,
        config: &Config,
    ) -> Result<Self, io::Error> {
        let file_handle = FileHandle::open(wide_path, c::READ_CONTROL)?;
        let mut security_info = Self::default();

        unsafe {
            let return_code = c::GetSecurityInfo(
                file_handle.raw_handle(),
                c::SE_FILE_OBJECT,
                c::OWNER_SECURITY_INFORMATION | c::GROUP_SECURITY_INFORMATION,
                &mut security_info.sid_owner_ptr,
                &mut security_info.sid_group_ptr,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut security_info.sd_ptr,
            );

            // On success, return_code is ERROR_SUCCESS
            if return_code == c::ERROR_SUCCESS {
                security_info.is_ok = true;

                Ok(security_info)
            } else {
                Err(io::Error::from_raw_os_error(return_code as i32))
            }
        }
    }

    pub fn sid_owner_ptr(&self) -> c::PSID {
        self.sid_owner_ptr
    }

    pub fn sid_group_ptr(&self) -> c::PSID {
        self.sid_group_ptr
    }
}

impl Default for SecurityInfo {
    fn default() -> Self {
        Self {
            sd_ptr: ptr::null_mut(),
            sid_owner_ptr: ptr::null_mut(),
            sid_group_ptr: ptr::null_mut(),
            is_ok: false,
        }
    }
}

impl Drop for SecurityInfo {
    fn drop(&mut self) {
        unsafe {
            if self.is_ok {
                c::LocalFree(self.sd_ptr as c::HLOCAL);
            }
        }
    }
}
