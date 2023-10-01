use super::sys_prelude::*;

use std::ffi::{c_char, CStr};
use std::mem;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::config::Config;
use crate::output::DisplayCell;

pub fn get_username_cell_by_uid(uid: u32, config: &Config) -> DisplayCell {
    static USERS_CACHE: Lazy<Mutex<Vec<Account>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match USERS_CACHE.lock() {
        Ok(mut users_cache) => {
            let cache_index_option = users_cache
                .iter()
                .position(|account| account.numeric_id == uid);

            match cache_index_option {
                Some(cache_index) => users_cache[cache_index].name_cell.clone(),
                None => {
                    log::debug!("uid '{}' is not found in USERS_CACHE", uid);
                    let user = Account::get_by_uid(uid, config);
                    let username_cell = user.name_cell.clone();
                    users_cache.push(user);

                    username_cell
                }
            }
        }
        Err(_) => {
            let user = Account::get_by_uid(uid, config);

            user.name_cell
        }
    }
}

pub fn get_groupname_cell_by_gid(gid: u32, config: &Config) -> DisplayCell {
    static GROUPS_CACHE: Lazy<Mutex<Vec<Account>>> =
        Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match GROUPS_CACHE.lock() {
        Ok(mut groups_cache) => {
            let cache_index_option = groups_cache
                .iter()
                .position(|account| account.numeric_id == gid);

            match cache_index_option {
                Some(cache_index) => groups_cache[cache_index].name_cell.clone(),
                None => {
                    log::debug!("gid '{}' is not found in GROUPS_CACHE", gid);
                    let group = Account::get_by_gid(gid, config);
                    let groupname_cell = group.name_cell.clone();
                    groups_cache.push(group);

                    groupname_cell
                }
            }
        }
        Err(_) => {
            let group = Account::get_by_gid(gid, config);

            group.name_cell
        }
    }
}

#[derive(Debug, Default)]
struct Account {
    name_cell: DisplayCell,
    numeric_id: u32,
}

impl Account {
    pub fn get_by_uid(uid: u32, config: &Config) -> Self {
        let owner_style = config.theme.owner_style();

        if config.numeric_uid_gid {
            Self {
                name_cell: DisplayCell::from_num_with_style(uid as u64, owner_style, true),
                numeric_id: uid,
            }
        } else {
            let mut pwd: c::passwd = unsafe { mem::zeroed() };
            const BUFLEN: usize = 2048;
            let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
            let mut result: *mut c::passwd = std::ptr::null_mut();

            unsafe {
                let return_code =
                    c::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), BUFLEN, &mut result);

                // On success, return_code is 0 and result is a pointer to pwd,
                if return_code == 0 && (result == &mut pwd) {
                    let username_string = CStr::from_ptr(pwd.pw_name).to_string_lossy();

                    Self {
                        name_cell: DisplayCell::from_str_with_style(
                            &username_string,
                            owner_style,
                            true,
                        ),
                        numeric_id: uid,
                    }
                } else {
                    Self {
                        name_cell: DisplayCell::from_num_with_style(uid as u64, owner_style, true),
                        numeric_id: uid,
                    }
                }
            }
        }
    }

    pub fn get_by_gid(gid: u32, config: &Config) -> Self {
        let group_style = config.theme.group_style();

        if config.numeric_uid_gid {
            Self {
                name_cell: DisplayCell::from_num_with_style(gid as u64, group_style, true),
                numeric_id: gid,
            }
        } else {
            let mut grp: c::group = unsafe { mem::zeroed() };
            const BUFLEN: usize = 2048;
            let mut buf: [c_char; BUFLEN] = [0; BUFLEN];
            let mut result: *mut c::group = std::ptr::null_mut();

            unsafe {
                let return_code =
                    c::getgrgid_r(gid, &mut grp, buf.as_mut_ptr(), BUFLEN, &mut result);

                // On success, return_code is 0 and result is a pointer to grp,
                if return_code == 0 && (result == &mut grp) {
                    let groupname_string = CStr::from_ptr(grp.gr_name).to_string_lossy();
                    let groupname_cell =
                        DisplayCell::from_str_with_style(&groupname_string, group_style, true);

                    Self {
                        name_cell: groupname_cell,
                        numeric_id: gid,
                    }
                } else {
                    Self {
                        name_cell: DisplayCell::from_num_with_style(gid as u64, group_style, true),
                        numeric_id: gid,
                    }
                }
            }
        }
    }
}
