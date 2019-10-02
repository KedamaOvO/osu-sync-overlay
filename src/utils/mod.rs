use winapi::um::{winuser, libloaderapi};
use winapi::shared::windef::{HDC, RECT};
use crate::gles::egldef::{EGLDisplay, EGLSurface, EGLint};
use std::sync::Once;
use std::mem;
use std::ffi::CString;
use winapi::shared::minwindef::FARPROC;

pub unsafe fn get_proc_address(module_name: &str, func_name: &str) -> FARPROC{
    let module_c_name = CString::new(module_name).unwrap();
    let func_c_name = CString::new(func_name).unwrap();
    let module = libloaderapi::GetModuleHandleA(module_c_name.as_ptr() as *const i8);
    return libloaderapi::GetProcAddress(module, func_c_name.as_ptr() as *const i8);
}

//WGL
pub fn get_wgl_window_size(hdc: HDC) -> (u32, u32) {
    unsafe {
        let hwnd = winuser::WindowFromDC(hdc);
        let mut rect = RECT {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        winuser::GetClientRect(hwnd, &mut rect);
        return (rect.right as u32, rect.bottom as u32);
    }
}

pub fn module_is_loaded(module_name: &str)->bool{
    let module_c_name = CString::new(module_name).unwrap();
    let handle = unsafe{libloaderapi::GetModuleHandleA(module_c_name.as_ptr() as *const i8)};
    !handle.is_null()
}

//EGL
type FnEglQuerySurface = extern "stdcall" fn(EGLDisplay, EGLSurface, EGLint, *mut EGLint);

static ONEC: Once = Once::new();
const EGL_HEIGHT: EGLint = 0x3056;
const EGL_WIDTH: EGLint = 0x3057;

pub fn get_egl_window_size(display: EGLDisplay, surface: EGLSurface) -> (u32, u32) {
    #[allow(non_upper_case_globals)]
    static mut eglQuerySurface: Option<FnEglQuerySurface> = None;
    ONEC.call_once(|| {
        unsafe {
            let module = libloaderapi::GetModuleHandleA(b"libegl.dll\0".as_ptr() as *const i8);
            eglQuerySurface = mem::transmute(
                libloaderapi::GetProcAddress(module, b"eglQuerySurface\0".as_ptr() as *const i8),
            );
        }
    });

    unsafe {
        let mut h = 0;
        let mut w = 0;
        eglQuerySurface.unwrap()(display, surface, EGL_HEIGHT, &mut h);
        eglQuerySurface.unwrap()(display, surface, EGL_WIDTH, &mut w);
        (w as u32, h as u32)
    }
}
