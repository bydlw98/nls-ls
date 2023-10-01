use std::io;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::OnceLock;

use super::security_info::SecurityInfo;
use super::sys_prelude::*;

use crate::config::Config;
use crate::utils::HasMaskSetExt;

pub fn get_rwx_permissions(security_info: &SecurityInfo, config: &Config) -> String {
    static OTHERS_SIDBUF: OnceLock<Option<Vec<u8>>> = OnceLock::new();
    let mut permissions_buf = String::with_capacity(9);

    match get_accessmask(security_info.dacl_ptr(), security_info.sid_owner_ptr()) {
        Ok(owner_accessmask) => {
            permissions_buf.push_str(&accessmask_to_rwx(owner_accessmask, config))
        }
        Err(err) => {
            eprintln!("nls: unable to get owner permissions: {}", err);
            permissions_buf.push_str("???")
        }
    }

    match get_accessmask(security_info.dacl_ptr(), security_info.sid_group_ptr()) {
        Ok(group_accessmask) => {
            permissions_buf.push_str(&accessmask_to_rwx(group_accessmask, config))
        }
        Err(err) => {
            eprintln!("nls: unable to get group permissions: {}", err);
            permissions_buf.push_str("???")
        }
    }

    match OTHERS_SIDBUF.get_or_init(|| create_others_sidbuf()) {
        Some(others_sidbuf) => {
            match get_accessmask(security_info.dacl_ptr(), others_sidbuf.as_ptr() as c::PSID) {
                Ok(others_accessmask) => {
                    permissions_buf.push_str(&accessmask_to_rwx(others_accessmask, config))
                }
                Err(err) => {
                    eprintln!("nls: unable to get others permissions: {}", err);
                    permissions_buf.push_str("???")
                }
            }
        }
        None => permissions_buf.push_str("???"),
    }
    permissions_buf
}

pub fn create_others_sidbuf() -> Option<Vec<u8>> {
    unsafe {
        let mut world_sid_len = c::GetSidLengthRequired(1);
        let mut others_sidbuf: Vec<u8> = vec![0; world_sid_len as usize];

        let return_code = c::CreateWellKnownSid(
            c::WinWorldSid,
            ptr::null_mut(),
            others_sidbuf.as_mut_ptr() as c::PSID,
            &mut world_sid_len,
        );

        // On success, return_code is 0
        if return_code != 0 {
            Some(others_sidbuf)
        } else {
            log::debug!(
                "unable to create others SID: {}",
                io::Error::last_os_error()
            );
            None
        }
    }
}

fn accessmask_to_rwx(accessmask: u32, config: &Config) -> String {
    let mut rwx_string = String::with_capacity(32);
    let theme = &config.theme;

    if accessmask.has_mask_set(c::FILE_GENERIC_READ) {
        string_push_char_with_style(&mut rwx_string, 'r', theme.read_style());
    } else {
        string_push_char_with_style(&mut rwx_string, '-', theme.no_permission_style())
    }

    if accessmask.has_mask_set(c::FILE_GENERIC_WRITE) {
        string_push_char_with_style(&mut rwx_string, 'w', theme.write_style());
    } else {
        string_push_char_with_style(&mut rwx_string, '-', theme.no_permission_style())
    }

    if accessmask.has_mask_set(c::FILE_GENERIC_EXECUTE) {
        string_push_char_with_style(&mut rwx_string, 'x', theme.execute_style());
    } else {
        string_push_char_with_style(&mut rwx_string, '-', theme.no_permission_style())
    }

    rwx_string
}

pub fn get_accessmask(dacl_ptr: *const c::ACL, sid_ptr: c::PSID) -> Result<u32, io::Error> {
    unsafe {
        let mut trustee = MaybeUninit::<c::TRUSTEE_W>::uninit();
        c::BuildTrusteeWithSidW(trustee.as_mut_ptr(), sid_ptr);
        let trustee = trustee.assume_init();

        let mut accessmask: u32 = 0;
        let return_code = c::GetEffectiveRightsFromAclW(dacl_ptr, &trustee, &mut accessmask);

        // On success, return_code is ERROR_SUCCESS
        // Else return_code is a raw os error
        if return_code == c::ERROR_SUCCESS {
            Ok(accessmask)
        } else {
            Err(io::Error::from_raw_os_error(return_code as i32))
        }
    }
}

fn string_push_char_with_style(string: &mut String, ch: char, ansi_style_str: Option<&str>) {
    match ansi_style_str {
        Some(ansi_style_str) => {
            string.push_str("\x1b[");
            string.push_str(ansi_style_str);
            string.push('m');
            string.push(ch);
            string.push_str("\x1b[0m");
        }
        None => {
            string.push(ch);
        }
    }
}
