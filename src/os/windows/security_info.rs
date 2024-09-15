use std::io;
use std::ptr;

use super::sys_prelude::*;
use super::FileHandle;

pub struct SecurityInfo {
    sd_ptr: c::PSECURITY_DESCRIPTOR,
    raw_owner_psid: c::PSID,
    raw_group_psid: c::PSID,
    dacl_ptr: *mut c::ACL,
    is_ok: bool,
}

impl SecurityInfo {
    pub fn from_wide_path(wide_path: &[u16], follow_links: bool) -> Result<Self, io::Error> {
        const SECURITY_INFORMATION: c::OBJECT_SECURITY_INFORMATION = c::OWNER_SECURITY_INFORMATION
            | c::GROUP_SECURITY_INFORMATION
            | c::DACL_SECURITY_INFORMATION;

        let file_handle = FileHandle::open(wide_path, c::READ_CONTROL, follow_links)?;
        let mut security_info = Self::default();

        let return_code = unsafe {
            c::GetSecurityInfo(
                file_handle.as_raw_handle(),
                c::SE_FILE_OBJECT,
                SECURITY_INFORMATION,
                &mut security_info.raw_owner_psid,
                &mut security_info.raw_group_psid,
                &mut security_info.dacl_ptr,
                ptr::null_mut(),
                &mut security_info.sd_ptr,
            )
        };

        // On success, return_code is ERROR_SUCCESS
        if return_code == c::ERROR_SUCCESS {
            security_info.is_ok = true;

            Ok(security_info)
        } else {
            Err(io::Error::from_raw_os_error(return_code as i32))
        }
    }

    pub fn owner_psid(&self) -> c::PSID {
        self.raw_owner_psid
    }

    pub fn group_psid(&self) -> c::PSID {
        self.raw_group_psid
    }

    pub fn dacl_ptr(&self) -> *const c::ACL {
        self.dacl_ptr
    }
}

impl Default for SecurityInfo {
    fn default() -> Self {
        Self {
            sd_ptr: ptr::null_mut(),
            raw_owner_psid: ptr::null_mut(),
            raw_group_psid: ptr::null_mut(),
            dacl_ptr: ptr::null_mut(),
            is_ok: false,
        }
    }
}

impl Drop for SecurityInfo {
    fn drop(&mut self) {
        if self.is_ok {
            unsafe {
                c::LocalFree(self.sd_ptr as c::HLOCAL);
            }
        }
    }
}
