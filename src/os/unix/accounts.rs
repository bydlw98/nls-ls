use std::sync::Mutex;

use once_cell::sync::Lazy;
use user_utils::os::unix::{get_name_by_gid, get_name_by_uid};

use crate::config::Config;
use crate::output::{GridCell, GridCellExts};

pub fn get_username_cell_by_uid(uid: u32, config: &Config) -> GridCell {
    struct User {
        name_cell: GridCell,
        uid: u32,
    }

    static USERS_CACHE: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match USERS_CACHE.lock() {
        Ok(mut users_cache) => {
            let cache_index_option = users_cache.iter().position(|user| user.uid == uid);

            match cache_index_option {
                Some(cache_index) => users_cache[cache_index].name_cell.clone(),
                None => {
                    log::debug!("uid '{}' is not found in USERS_CACHE", uid);
                    let name_cell = username_cell(uid, config);
                    let user = User {
                        name_cell: name_cell.clone(),
                        uid,
                    };
                    users_cache.push(user);

                    name_cell
                }
            }
        }
        Err(_) => username_cell(uid, config),
    }
}

pub fn get_groupname_cell_by_gid(gid: u32, config: &Config) -> GridCell {
    struct Group {
        name_cell: GridCell,
        gid: u32,
    }

    static GROUPS_CACHE: Lazy<Mutex<Vec<Group>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match GROUPS_CACHE.lock() {
        Ok(mut groups_cache) => {
            let cache_index_option = groups_cache.iter().position(|group| group.gid == gid);

            match cache_index_option {
                Some(cache_index) => groups_cache[cache_index].name_cell.clone(),
                None => {
                    log::debug!("gid '{}' is not found in GROUPS_CACHE", gid);
                    let name_cell = groupname_cell(gid, config);
                    let group = Group {
                        name_cell: name_cell.clone(),
                        gid,
                    };
                    groups_cache.push(group);

                    name_cell
                }
            }
        }
        Err(_) => groupname_cell(gid, config),
    }
}

fn username_cell(uid: u32, config: &Config) -> GridCell {
    let owner_style = config.theme.owner_style();
    let raw_uid = uid as libc::uid_t;

    if config.numeric_uid_gid {
        GridCell::from_num_with_style(raw_uid, owner_style)
    } else {
        match get_name_by_uid(raw_uid) {
            Ok(name) => GridCell::from_str_with_style(&name.to_string_lossy(), owner_style),
            Err(_) => GridCell::from_num_with_style(raw_uid, owner_style),
        }
    }
}

fn groupname_cell(gid: libc::gid_t, config: &Config) -> GridCell {
    let group_style = config.theme.group_style();
    let raw_gid = gid as libc::gid_t;

    if config.numeric_uid_gid {
        GridCell::from_num_with_style(raw_gid, group_style)
    } else {
        match get_name_by_gid(raw_gid) {
            Ok(name) => GridCell::from_str_with_style(&name.to_string_lossy(), group_style),
            Err(_) => GridCell::from_num_with_style(raw_gid, group_style),
        }
    }
}
