use super::sys_prelude::*;

use std::ffi::{c_char, CStr};
use std::mem::MaybeUninit;

use crate::output::DisplayCell;

pub fn get_username_cell_by_uid(uid: u32) -> DisplayCell {
    let mut pwd = MaybeUninit::<c::passwd>::uninit();
    const BUFLEN: usize = 2048;
    let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
    let mut result: *mut c::passwd = std::ptr::null_mut();

    unsafe {
        let return_code =
            c::getpwuid_r(uid, pwd.as_mut_ptr(), buf.as_mut_ptr(), BUFLEN, &mut result);

        if return_code < 0 {
            DisplayCell::from_ascii_string(uid.to_string(), true)
        } else {
            let pwd = pwd.assume_init();
            let name_string = CStr::from_ptr(pwd.pw_name).to_string_lossy().to_string();

            DisplayCell::from(name_string)
        }
    }
}

pub fn get_groupname_cell_by_gid(gid: u32) -> DisplayCell {
    let mut grp = MaybeUninit::<c::group>::uninit();
    const BUFLEN: usize = 2048;
    let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
    let mut result: *mut c::group = std::ptr::null_mut();

    unsafe {
        let return_code =
            c::getgrgid_r(gid, grp.as_mut_ptr(), buf.as_mut_ptr(), BUFLEN, &mut result);

        if return_code < 0 {
            DisplayCell::from_ascii_string(gid.to_string(), true)
        } else {
            let grp = grp.assume_init();
            let groupname_string = CStr::from_ptr(grp.gr_name).to_string_lossy().to_string();

            DisplayCell::from(groupname_string)
        }
    }
}
