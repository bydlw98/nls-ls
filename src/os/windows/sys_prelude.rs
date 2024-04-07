pub mod c {
    pub use windows_sys::Win32::Foundation::{
        CloseHandle, LocalFree, ERROR_SUCCESS, HANDLE, HLOCAL, INVALID_HANDLE_VALUE, PSID,
    };
    pub use windows_sys::Win32::Security::Authorization::{
        BuildTrusteeWithSidW, GetEffectiveRightsFromAclW, GetSecurityInfo, SE_FILE_OBJECT,
        TRUSTEE_W,
    };
    pub use windows_sys::Win32::Security::{
        ACL, DACL_SECURITY_INFORMATION, GROUP_SECURITY_INFORMATION, OBJECT_SECURITY_INFORMATION,
        OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
    };
    pub use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FileIdInfo, FileStandardInfo, GetFileInformationByHandleEx,
        FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_HIDDEN,
        FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_SYSTEM,
        FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_GENERIC_EXECUTE,
        FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_ID_INFO, FILE_SHARE_READ, FILE_STANDARD_INFO,
        OPEN_EXISTING, READ_CONTROL,
    };
}
