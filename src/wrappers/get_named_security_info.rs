use crate::utilities::{buf_from_os, has_bit};
use crate::SecurityDescriptor;
use std::ffi::OsStr;
use std::ptr::null_mut;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winnt::{self, PACL, PSECURITY_DESCRIPTOR, PSID};
use windows_error::WindowsError;

/// Wraps GetNamedSecurityInfoW
#[allow(non_snake_case)]
pub fn GetNamedSecurityInfo(
    name: &OsStr,
    obj_type: u32,
    sec_info: u32,
) -> Result<SecurityDescriptor, WindowsError> {
    let name = buf_from_os(name);

    let mut owner: PSID = null_mut();
    let mut group: PSID = null_mut();
    let mut dacl: PACL = null_mut();
    let mut sacl: PACL = null_mut();
    let mut sd: PSECURITY_DESCRIPTOR = null_mut();

    let result: WindowsError = unsafe {
        winapi::um::aclapi::GetNamedSecurityInfoW(
            name.as_ptr(),
            obj_type,
            sec_info,
            &mut owner,
            &mut group,
            &mut dacl,
            &mut sacl,
            &mut sd,
        )
    }
    .into();

    if result != ERROR_SUCCESS {
        return Err(result);
    }

    let owner = if has_bit(sec_info, winnt::OWNER_SECURITY_INFORMATION) {
        owner
    } else {
        null_mut()
    };

    let group = if has_bit(sec_info, winnt::GROUP_SECURITY_INFORMATION) {
        group
    } else {
        null_mut()
    };

    let dacl = if has_bit(sec_info, winnt::DACL_SECURITY_INFORMATION) {
        dacl
    } else {
        null_mut()
    };

    let sacl = if has_bit(sec_info, winnt::SACL_SECURITY_INFORMATION) {
        sacl
    } else {
        null_mut()
    };

    Ok(unsafe { SecurityDescriptor::from_raw(sd, owner, group, dacl, sacl) })
}