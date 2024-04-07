use std::sync::Mutex;

use nls_term_grid::GridCell;
use once_cell::sync::Lazy;
use user_utils::unix::*;

use crate::config::Config;
use crate::output::GridCellExts;

pub fn get_username_cell_by_uid(uid: u32, config: &Config) -> GridCell {
    static USERS_CACHE: Lazy<Mutex<Vec<Account<OwnedUid>>>> =
        Lazy::new(|| Mutex::new(Vec::with_capacity(2)));
    let borrowed_uid = BorrowedUid::borrow_raw(uid as libc::uid_t);

    match USERS_CACHE.lock() {
        Ok(mut users_cache) => {
            let cache_index_option = users_cache
                .iter()
                .position(|account| account.id == borrowed_uid);

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

pub fn get_groupname_cell_by_gid(gid: u32, config: &Config) -> GridCell {
    static GROUPS_CACHE: Lazy<Mutex<Vec<Account<OwnedGid>>>> =
        Lazy::new(|| Mutex::new(Vec::with_capacity(2)));
    let borrowed_gid = BorrowedGid::borrow_raw(gid as libc::gid_t);

    match GROUPS_CACHE.lock() {
        Ok(mut groups_cache) => {
            let cache_index_option = groups_cache
                .iter()
                .position(|account| account.id == borrowed_gid);

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

struct Account<T> {
    name_cell: GridCell,
    id: T,
}

impl Account<OwnedUid> {
    fn get_by_uid(uid: u32, config: &Config) -> Self {
        let owner_style = config.theme.owner_style();
        let borrowed_uid = BorrowedUid::borrow_raw(uid as libc::uid_t);
        let owned_uid = borrowed_uid.try_clone_to_owned().unwrap();

        if config.numeric_uid_gid {
            Self {
                name_cell: GridCell::from_num_with_style(uid as u64, owner_style),
                id: owned_uid,
            }
        } else {
            match borrowed_uid.lookup_username() {
                Ok(username) => Self {
                    name_cell: GridCell::from_str_with_style(
                        &username.to_string_lossy(),
                        owner_style,
                    ),
                    id: owned_uid,
                },
                _ => Self {
                    name_cell: GridCell::from_num_with_style(uid as u64, owner_style),
                    id: owned_uid,
                },
            }
        }
    }
}

impl Account<OwnedGid> {
    fn get_by_gid(gid: u32, config: &Config) -> Self {
        let group_style = config.theme.group_style();
        let borrowed_gid = BorrowedGid::borrow_raw(gid as libc::gid_t);
        let owned_gid = borrowed_gid.try_clone_to_owned().unwrap();

        if config.numeric_uid_gid {
            Self {
                name_cell: GridCell::from_num_with_style(gid as u64, group_style),
                id: owned_gid,
            }
        } else {
            match borrowed_gid.lookup_groupname() {
                Ok(groupname) => Self {
                    name_cell: GridCell::from_str_with_style(
                        &groupname.to_string_lossy(),
                        group_style,
                    ),
                    id: owned_gid,
                },
                _ => Self {
                    name_cell: GridCell::from_num_with_style(gid as u64, group_style),
                    id: owned_gid,
                },
            }
        }
    }
}
