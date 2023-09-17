use std::io;
use std::mem::MaybeUninit;
use std::ptr;

use super::sys_prelude::*;
use super::utf16_null_terminated_to_string_lossy;

use crate::output::DisplayCell;

pub fn get_accountname_cell_by_sid_ptr(sid_ptr: c::PSID) -> DisplayCell {
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
                utf16_null_terminated_to_string_lossy(&wide_domain_buf),
                utf16_null_terminated_to_string_lossy(&wide_name_buf)
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
                    utf16_null_terminated_to_string_lossy(&wide_domain),
                    utf16_null_terminated_to_string_lossy(&wide_name)
                ))
            } else {
                log::debug!("unable to lookup accountname: {}", io::Error::last_os_error());

                DisplayCell::error_left_aligned()
            }
        }
    }
}

pub fn get_string_sid_cell_by_sid_ptr(sid_ptr: c::PSID) -> DisplayCell {
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
