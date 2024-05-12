use std::sync::Mutex;

use compact_str::{CompactString, ToCompactString};
use once_cell::sync::Lazy;
use user_utils::windows::*;

use crate::config::Config;

pub fn get_accountname_by_psid(psid: BorrowedPsid<'_>, config: &Config) -> CompactString {
    static ACCOUNTS_CACHE: Lazy<Mutex<Vec<Account>>> =
        Lazy::new(|| Mutex::new(Vec::with_capacity(2)));
    let mut accounts_cache = ACCOUNTS_CACHE.lock().unwrap();
    let cache_index_option = accounts_cache
        .iter()
        .position(|account| account.psid == psid);

    match cache_index_option {
        Some(cache_index) => accounts_cache[cache_index].name.clone(),
        None => match Account::get_by_psid(psid, config) {
            Some(account) => {
                let accountname = account.name.clone();
                log::debug!("account '{}' is not found in ACCOUNTS_CACHE", accountname);
                accounts_cache.push(account);

                accountname
            }
            None => internal_get_accountname_by_psid(psid, config),
        },
    }
}

fn internal_get_accountname_by_psid(psid: BorrowedPsid<'_>, config: &Config) -> CompactString {
    if config.numeric_uid_gid {
        psid.convert_to_string_sid()
            .map(|string_sid| string_sid.to_compact_string())
            .unwrap_or(CompactString::new_inline("?"))
    } else {
        match psid.lookup_accountname() {
            Ok(accountname) => accountname.to_string_lossy().to_compact_string(),
            Err(_) => psid
                .convert_to_string_sid()
                .map(|string_sid| string_sid.to_compact_string())
                .unwrap_or(CompactString::new_inline("?")),
        }
    }
}

#[derive(Debug)]
struct Account {
    name: CompactString,
    psid: OwnedPsid,
}

impl Account {
    fn get_by_psid(psid: BorrowedPsid<'_>, config: &Config) -> Option<Self> {
        let sid_buf = psid.try_clone_to_owned().ok()?;
        let accountname = internal_get_accountname_by_psid(psid, config);

        Some(Self {
            name: accountname,
            psid: sid_buf,
        })
    }
}
