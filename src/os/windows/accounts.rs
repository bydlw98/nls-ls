use std::sync::Mutex;

use compact_str::{CompactString, ToCompactString};
use once_cell::sync::Lazy;
use user_utils::os::windows::{
    convert_sid_to_string_sid, lookup_account_sid, GroupidExt, UseridExt,
};
use user_utils::{Groupid, GroupidBuf, Userid, UseridBuf};

use super::sys_prelude::*;
use crate::config::Config;

pub fn get_username_by_psid(psid: c::PSID, config: &Config) -> CompactString {
    struct User {
        name: CompactString,
        id: UseridBuf,
    }

    static USERS_CACHE: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match USERS_CACHE.lock() {
        Ok(mut users_cache) => {
            let user_id = unsafe { Userid::from_raw_psid_unchecked(psid) };
            let cache_index_option = users_cache.iter().position(|user| user.id.eq(user_id));

            match cache_index_option {
                Some(cache_index) => users_cache[cache_index].name.clone(),
                None => {
                    log::debug!("owner SID: '{}' is not found in USERS_CACHE", user_id);
                    let name = accountname_str(psid, config);
                    if let Ok(id) = user_id.try_clone() {
                        let user = User {
                            name: name.clone(),
                            id,
                        };

                        users_cache.push(user);
                    }

                    name
                }
            }
        }
        Err(_) => accountname_str(psid, config),
    }
}

pub fn get_groupname_by_psid(psid: c::PSID, config: &Config) -> CompactString {
    struct Group {
        name: CompactString,
        id: GroupidBuf,
    }

    static GROUPS_CACHE: Lazy<Mutex<Vec<Group>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2)));

    match GROUPS_CACHE.lock() {
        Ok(mut groups_cache) => {
            let group_id = unsafe { Groupid::from_raw_psid_unchecked(psid) };
            let cache_index_option = groups_cache.iter().position(|group| group.id.eq(group_id));

            match cache_index_option {
                Some(cache_index) => groups_cache[cache_index].name.clone(),
                None => {
                    log::debug!("group SID: '{}' is not found in GROUPS_CACHE", group_id);
                    let name = accountname_str(psid, config);
                    if let Ok(id) = group_id.try_clone() {
                        let group = Group {
                            name: name.clone(),
                            id,
                        };

                        groups_cache.push(group);
                    }

                    name
                }
            }
        }
        Err(_) => accountname_str(psid, config),
    }
}

fn accountname_str(psid: c::PSID, config: &Config) -> CompactString {
    if config.numeric_uid_gid {
        convert_sid_to_string_sid(psid)
            .map(|string_sid| string_sid.to_compact_string())
            .unwrap_or(CompactString::new_inline("?"))
    } else {
        match lookup_account_sid(psid) {
            Ok(name) => CompactString::new(&name.to_string_lossy()),
            Err(_) => convert_sid_to_string_sid(psid)
                .map(|string_sid| string_sid.to_compact_string())
                .unwrap_or(CompactString::new_inline("?")),
        }
    }
}
