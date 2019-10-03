use crate::renderer::api_loader::ApiLoader;
use winapi::um::libloaderapi;
use std::mem;
use std::ffi::{CString, c_void};
use winapi::shared::minwindef::HMODULE;
use crate::utils;
use std::os::raw::c_char;

type FnEglGetProcAddress = extern "stdcall" fn(*const c_char) -> *mut c_void;

pub struct GLESLoader {
    module: HMODULE,
    get_proc_addr: FnEglGetProcAddress,
}

impl ApiLoader for GLESLoader {
    fn init() -> Self {
        let glesmodule = unsafe { libloaderapi::GetModuleHandleA(b"libGLESv2.dll\0".as_ptr() as *const i8) };
        let get_proc_addr: FnEglGetProcAddress = unsafe { mem::transmute(utils::get_proc_address("libEGL.dll", "eglGetProcAddress")) };

        GLESLoader {
            module: glesmodule,
            get_proc_addr,
        }
    }

    fn load_func(&self, name: &str) -> *const () {
        unsafe {
            let cname = CString::new(name).unwrap();
            let proc = libloaderapi::GetProcAddress(self.module, cname.as_ptr() as *const i8);
            if !proc.is_null() {
                return proc as *const _;
            }

            (self.get_proc_addr)(cname.as_ptr() as *const _) as *const _
        }
    }
}
