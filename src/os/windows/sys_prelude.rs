pub mod c {
    pub use libc::wcslen;
    pub use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, ERROR_NONE_MAPPED, ERROR_SUCCESS, HANDLE, HLOCAL,
        INVALID_HANDLE_VALUE, PSID,
    };
    pub use windows_sys::Win32::Security::Authorization::{
        BuildTrusteeWithSidW, ConvertSidToStringSidW, GetEffectiveRightsFromAclW, GetSecurityInfo,
        SE_FILE_OBJECT, TRUSTEE_W,
    };
    pub use windows_sys::Win32::Security::{
        CopySid, CreateWellKnownSid, EqualSid, GetLengthSid, GetSidLengthRequired,
        LookupAccountSidW, SidTypeUnknown, WinWorldSid, ACL, DACL_SECURITY_INFORMATION,
        GROUP_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    };
    pub use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FileIdInfo, FileStandardInfo, GetFileInformationByHandleEx,
        FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_GENERIC_EXECUTE,
        FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_ID_INFO, FILE_SHARE_READ, FILE_STANDARD_INFO,
        OPEN_EXISTING, READ_CONTROL,
    };
    pub use windows_sys::Win32::System::Memory::LocalFree;
}
