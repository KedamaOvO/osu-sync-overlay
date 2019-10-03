use crate::renderer::api_loader::ApiLoader;
use winapi::um::libloaderapi;
use std::mem;
use std::ffi::CString;
use winapi::shared::ntdef::LPCSTR;
use winapi::shared::minwindef::{PROC, HMODULE};

type FnWglGetProcAddress = extern "stdcall" fn(LPCSTR) -> PROC;

pub struct GLLoader{
    module:HMODULE,
    get_proc_addr:FnWglGetProcAddress,
}

impl ApiLoader for GLLoader {
    fn init()->Self {
        let module = unsafe { libloaderapi::GetModuleHandleA(b"opengl32.dll\0".as_ptr() as *const i8) };
        let get_proc_addr: FnWglGetProcAddress = unsafe { mem::transmute(libloaderapi::GetProcAddress(module, b"wglGetProcAddress\0".as_ptr() as *const i8)) };

        GLLoader{
            module,
            get_proc_addr,
        }
    }

    fn load_func(&self,name: &str)->*const () {
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

