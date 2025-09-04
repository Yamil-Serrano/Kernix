use std::{ffi::CString, ptr};
use windows_sys::Win32::System::Services::*;
pub fn set_service(service: &str, enable: bool) {
    unsafe {
        let scm = OpenSCManagerA(ptr::null(), ptr::null(), SC_MANAGER_ALL_ACCESS);
        if scm.is_null() { return; }

        let name = CString::new(service).unwrap();
        let svc = OpenServiceA(scm, name.as_ptr() as *const u8, SERVICE_ALL_ACCESS);
        if svc.is_null() { return; }

        ChangeServiceConfigA(
            svc,
            SERVICE_NO_CHANGE,
            if enable { SERVICE_AUTO_START } else { SERVICE_DISABLED },
            SERVICE_NO_CHANGE,
            ptr::null(),    // binary path (no change)
            ptr::null(),    // load order group (no change)
            ptr::null_mut(),// tag ID (no change)
            ptr::null(),    // dependencies (no change)
            ptr::null(),    // service start name (no change)
            ptr::null(),    // password (no change)
            ptr::null(),    // display name (no change)
        );

        CloseServiceHandle(svc);
        CloseServiceHandle(scm);

        println!("{} {}", service, if enable { "Enabled" } else { "Desabled" });
    }
}