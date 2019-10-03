use winapi::um::libloaderapi;
use std::ffi::CString;
use winapi::shared::minwindef::FARPROC;

pub unsafe fn get_proc_address(module_name: &str, func_name: &str) -> FARPROC {
    let module_c_name = CString::new(module_name).unwrap();
    let func_c_name = CString::new(func_name).unwrap();
    let module = libloaderapi::GetModuleHandleA(module_c_name.as_ptr() as *const i8);
    libloaderapi::GetProcAddress(module, func_c_name.as_ptr() as *const i8)
}

//WGL


pub fn module_is_loaded(module_name: &str) -> bool {
    let module_c_name = CString::new(module_name).unwrap();
    let handle = unsafe { libloaderapi::GetModuleHandleA(module_c_name.as_ptr() as *const i8) };
    !handle.is_null()
}

