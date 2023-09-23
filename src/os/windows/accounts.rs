use std::io;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::sys_prelude::*;
use super::utf16_until_null_to_string_lossy;

use crate::config::Config;
use crate::output::DisplayCell;

pub fn get_accountname_cell_by_sid_ptr(sid_ptr: c::PSID, config: &Config) -> DisplayCell {
    static ACCOUNTS_CACHE: Lazy<Mutex<Vec<Account>>> =
        Lazy::new(|| Mutex::new(Vec::with_capacity(2)));
    let mut accounts_cache = ACCOUNTS_CACHE.lock().unwrap();
    let cache_index_option = accounts_cache
        .iter()
        .position(|account| account.sid_buf == sid_ptr);

    match cache_index_option {
        Some(cache_index) => accounts_cache[cache_index].name_cell.clone(),
        None => match Account::from_sid_ptr(sid_ptr, config) {
            Some(account) => {
                let name_cell = account.name_cell.clone();
                log::debug!("account '{}' is not found in ACCOUNTS_CACHE", name_cell);
                accounts_cache.push(account);

                name_cell
            }
            None => internal_get_accountname_cell_by_sid_ptr(sid_ptr, config),
        },
    }
}

fn internal_get_accountname_cell_by_sid_ptr(sid_ptr: c::PSID, config: &Config) -> DisplayCell {
    if config.numeric_uid_gid {
        get_string_sid_cell_by_sid_ptr(sid_ptr)
    } else {
        lookup_accountname_cell_by_sid_ptr(sid_ptr)
    }
}

fn lookup_accountname_cell_by_sid_ptr(sid_ptr: c::PSID) -> DisplayCell {
    let mut wide_name_length: u32 = 32;
    let mut wide_domain_length: u32 = 32;
    let mut wide_name_buf: [u16; 32] = [0; 32];
    let mut wide_domain_buf: [u16; 32] = [0; 32];
    let mut sid_name_use = c::SidTypeUnknown;

    unsafe {
        let return_code = c::LookupAccountSidW(
            ptr::null(),
            sid_ptr,
            wide_name_buf.as_mut_ptr(),
            &mut wide_name_length,
            wide_domain_buf.as_mut_ptr(),
            &mut wide_domain_length,
            &mut sid_name_use,
        );

        // If LookupAccountSidW succeeds, return_code is non-zero
        if return_code != 0 {
            DisplayCell::from(format!(
                "{}\\{}",
                utf16_until_null_to_string_lossy(&wide_domain_buf),
                utf16_until_null_to_string_lossy(&wide_name_buf)
            ))
        }
        // If GetLastError() returns ERROR_NONE_MAPPED, means
        // unable to get the name of SID
        else if c::GetLastError() == c::ERROR_NONE_MAPPED {
            log::debug!("no SID is mapped");
            get_string_sid_cell_by_sid_ptr(sid_ptr)
        } else {
            // Retry lookup SID name with correct size
            let mut wide_name = vec![0; wide_name_length as usize];
            let mut wide_domain = vec![0; wide_domain_length as usize];

            let return_code = c::LookupAccountSidW(
                ptr::null(),
                sid_ptr,
                wide_name.as_mut_ptr(),
                &mut wide_name_length,
                wide_domain.as_mut_ptr(),
                &mut wide_domain_length,
                &mut sid_name_use,
            );
            if return_code != 0 {
                DisplayCell::from(format!(
                    "{}\\{}",
                    utf16_until_null_to_string_lossy(&wide_domain),
                    utf16_until_null_to_string_lossy(&wide_name)
                ))
            } else {
                log::debug!(
                    "unable to lookup accountname: {}",
                    io::Error::last_os_error()
                );

                DisplayCell::error_left_aligned()
            }
        }
    }
}

fn get_string_sid_cell_by_sid_ptr(sid_ptr: c::PSID) -> DisplayCell {
    unsafe {
        let mut wide_cstring_ptr = MaybeUninit::<*mut u16>::uninit();
        let return_code = c::ConvertSidToStringSidW(sid_ptr, wide_cstring_ptr.as_mut_ptr());

        // On success, return_code is non-zero
        if return_code != 0 {
            let wide_cstring_ptr = wide_cstring_ptr.assume_init();
            let wide_cstring_len = c::wcslen(wide_cstring_ptr);
            let wide_cstring_array = std::slice::from_raw_parts(wide_cstring_ptr, wide_cstring_len);
            let string_sid = String::from_utf16_lossy(wide_cstring_array);

            c::LocalFree(wide_cstring_ptr as c::HLOCAL);

            DisplayCell::from(string_sid)
        } else {
            DisplayCell::error_left_aligned()
        }
    }
}

#[derive(Debug, Default)]
struct Account {
    name_cell: DisplayCell,
    sid_buf: SidBuf,
}

impl Account {
    fn from_sid_ptr(sid_ptr: c::PSID, config: &Config) -> Option<Self> {
        let sid_buf = SidBuf::from_sid_ptr(sid_ptr)?;
        let name_cell = internal_get_accountname_cell_by_sid_ptr(sid_ptr, config);

        Some(Self {
            name_cell: name_cell,
            sid_buf: sid_buf,
        })
    }
}

#[derive(Debug, Default)]
struct SidBuf(Vec<u8>);

impl SidBuf {
    pub fn as_ptr(&self) -> c::PSID {
        self.0.as_ptr() as c::PSID
    }

    pub fn from_sid_ptr(sid_ptr: c::PSID) -> Option<Self> {
        unsafe {
            let sid_length = c::GetLengthSid(sid_ptr);
            let mut buf: Vec<u8> = vec![0; sid_length as usize];
            let return_code = c::CopySid(sid_length, buf.as_mut_ptr() as c::PSID, sid_ptr);

            // On success, return_code is non-zero
            if return_code != 0 {
                Some(Self(buf))
            } else {
                None
            }
        }
    }
}

impl PartialEq<c::PSID> for SidBuf {
    fn eq(&self, other: &c::PSID) -> bool {
        unsafe { c::EqualSid(self.as_ptr(), *other) != 0 }
    }
}
