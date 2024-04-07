use std::io;
use std::mem::MaybeUninit;

use once_cell::sync::OnceCell;
use user_utils::windows::{AsPsid, AsRawPsid, BorrowedPsid, OwnedPsid};

use super::security_info::SecurityInfo;
use super::sys_prelude::*;

use crate::config::Config;
use crate::utils::HasMaskSetExt;

pub fn get_rwx_permissions(security_info: &SecurityInfo, config: &Config) -> String {
    static WORLD_PSID: OnceCell<Option<OwnedPsid>> = OnceCell::new();
    let mut permissions_buf = String::with_capacity(9);

    match get_accessmask(security_info.dacl_ptr(), security_info.owner_psid()) {
        Ok(owner_accessmask) => {
            permissions_buf.push_str(&accessmask_to_rwx(owner_accessmask, config))
        }
        Err(err) => {
            eprintln!("nls: unable to get owner permissions: {}", err);
            permissions_buf.push_str("???")
        }
    }

    match get_accessmask(security_info.dacl_ptr(), security_info.group_psid()) {
        Ok(group_accessmask) => {
            permissions_buf.push_str(&accessmask_to_rwx(group_accessmask, config))
        }
        Err(err) => {
            eprintln!("nls: unable to get group permissions: {}", err);
            permissions_buf.push_str("???")
        }
    }

    match WORLD_PSID.get_or_init(|| OwnedPsid::world().ok()) {
        Some(world_psid) => match get_accessmask(security_info.dacl_ptr(), world_psid.as_psid()) {
            Ok(others_accessmask) => {
                permissions_buf.push_str(&accessmask_to_rwx(others_accessmask, config))
            }
            Err(err) => {
                eprintln!("nls: unable to get others permissions: {}", err);
                permissions_buf.push_str("???")
            }
        },
        None => permissions_buf.push_str("???"),
    }
    permissions_buf
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

pub fn get_accessmask(dacl_ptr: *const c::ACL, psid: BorrowedPsid<'_>) -> Result<u32, io::Error> {
    let raw_psid = psid.as_raw_psid();
    let mut trustee = MaybeUninit::<c::TRUSTEE_W>::uninit();
    unsafe { c::BuildTrusteeWithSidW(trustee.as_mut_ptr(), raw_psid) };
    let trustee = unsafe { trustee.assume_init() };

    let mut accessmask: u32 = 0;
    let return_code = unsafe { c::GetEffectiveRightsFromAclW(dacl_ptr, &trustee, &mut accessmask) };

    // On success, return_code is ERROR_SUCCESS
    // Else return_code is a raw os error
    if return_code == c::ERROR_SUCCESS {
        Ok(accessmask)
    } else {
        Err(io::Error::from_raw_os_error(return_code as i32))
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
