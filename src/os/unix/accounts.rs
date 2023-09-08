use super::sys_prelude::*;

use std::ffi::{c_char, CStr};
use std::mem;

use crate::config::Config;
use crate::output::DisplayCell;

pub fn get_username_cell_by_uid(uid: u32, config: &Config) -> DisplayCell {
    if config.numeric_uid_gid {
        DisplayCell::from_ascii_string(uid.to_string(), true)
    } else {
        let mut pwd: c::passwd = unsafe { mem::zeroed() };
        const BUFLEN: usize = 2048;
        let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
        let mut result: *mut c::passwd = std::ptr::null_mut();

        unsafe {
            let return_code = c::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), BUFLEN, &mut result);

            // On success, return_code is 0 and result is a pointer to pwd,
            if return_code == 0 && (result == &mut pwd) {
                let username_string = CStr::from_ptr(pwd.pw_name).to_string_lossy().to_string();

                DisplayCell::from(username_string)
            } else {
                DisplayCell::from_ascii_string(uid.to_string(), true)
            }
        }
    }
}

pub fn get_groupname_cell_by_gid(gid: u32, config: &Config) -> DisplayCell {
    if config.numeric_uid_gid {
        DisplayCell::from_ascii_string(gid.to_string(), true)
    } else {
        let mut grp: c::group = unsafe { std::mem::zeroed() };
        const BUFLEN: usize = 2048;
        let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
        let mut result: *mut c::group = std::ptr::null_mut();

        unsafe {
            let return_code = c::getgrgid_r(gid, &mut grp, buf.as_mut_ptr(), BUFLEN, &mut result);

            // On success, return_code is 0 and result is a pointer to grp,
            if return_code == 0 && (result == &mut grp) {
                let groupname_string = CStr::from_ptr(grp.gr_name).to_string_lossy().to_string();

                DisplayCell::from(groupname_string)
            } else {
                DisplayCell::from_ascii_string(gid.to_string(), true)
            }
        }
    }
}
