mod egl_def;
use egl_def::*;

#[macro_use] extern crate lazy_static;
use winapi::shared::ntdef::{HANDLE, LPCWSTR,LPCSTR};
use winapi::shared::minwindef::{BOOL, DWORD, LPVOID, HINSTANCE, TRUE, HMODULE};
use winapi::shared::windef::{HDC};
use winapi::um::libloaderapi;
use winapi::um::consoleapi;
use detour::GenericDetour;

use std::sync::Mutex;
use std::mem;

type FnLoadLibraryExW = extern "stdcall" fn(LPCWSTR, HANDLE, DWORD) -> HMODULE;
type FnGLSwapBuffers = extern "stdcall" fn (HDC) -> BOOL;
type FnEGLSwapBuffers = extern "stdcall" fn(EGLDisplay,EGLSurface)->EGLBoolean;

lazy_static!{
    static ref LOAD_LIBRARY_EX_HOOKER: Mutex<GenericDetour<FnLoadLibraryExW>> = {
        unsafe{
            let module = libloaderapi::GetModuleHandleA(b"kernel32.dll\0".as_ptr() as *const i8);
            eprintln!("Kernel32 Module: 0x{:X?}",module as i32);

            let load_library_exw_addr:FnLoadLibraryExW = mem::transmute(
                libloaderapi::GetProcAddress(module,b"LoadLibraryExW\0".as_ptr() as *const i8),
            );
            eprintln!("LoadLibraryExW Address: 0x{:X?}",load_library_exw_addr as i32);

            let mut hook = GenericDetour::<FnLoadLibraryExW>::new(load_library_exw_addr,hook_load_library_ex).unwrap();
            hook.enable().unwrap();
            return Mutex::new(hook);
        }
    };

    static ref GL_HOOKER: GenericDetour<FnGLSwapBuffers> = {
        unsafe{
            let module = libloaderapi::GetModuleHandleA(b"gdi32.dll\0".as_ptr() as *const i8);
            eprintln!("GDI Module: 0x{:X?}",module as i32);

            let swap_buffers_addr:FnGLSwapBuffers = mem::transmute(
                libloaderapi::GetProcAddress(module,b"SwapBuffers\0".as_ptr() as *const i8),
            );
            eprintln!("SwapBuffers Address: 0x{:X?}",swap_buffers_addr as i32);

            let mut hook = GenericDetour::<FnGLSwapBuffers>::new(swap_buffers_addr,gl_swap_buffers).unwrap();
            hook.enable().unwrap();
            return hook;
        }
    };

    static ref GLES_HOOKER: GenericDetour<FnEGLSwapBuffers> = {
        unsafe{
            let module = libloaderapi::GetModuleHandleA(b"libegl.dll\0".as_ptr() as *const i8);
            eprintln!("EGL Module: 0x{:X?}",module as i32);

            let swap_buffers_addr:FnEGLSwapBuffers = mem::transmute(
                libloaderapi::GetProcAddress(module,b"eglSwapBuffers\0".as_ptr() as *const i8),
            );

            eprintln!("eglSwapBuffers Address: 0x{:X?}",swap_buffers_addr as i32);
            let mut hook = GenericDetour::<FnEGLSwapBuffers>::new(swap_buffers_addr,egl_swap_buffers).unwrap();
            hook.enable().unwrap();
            return hook;
        }
    };
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "stdcall" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{

    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    return TRUE;
}

#[cfg(debug_assertions)]
fn enable_debug_console(){
    unsafe{
        consoleapi::AllocConsole();

        libc::freopen(b"conout$\0".as_ptr() as *const i8,
                      b"w\0".as_ptr() as *const i8,
                      libc::fdopen(2, b"w\0".as_ptr() as *const i8));
    }

}

fn init() {
    enable_debug_console();

    lazy_static::initialize(&LOAD_LIBRARY_EX_HOOKER);
}

#[allow(non_snake_case)]
extern "stdcall" fn hook_load_library_ex(lpLibFileName:LPCWSTR,hFile:HANDLE,dwFlags:DWORD) -> HMODULE{
    let module = (*LOAD_LIBRARY_EX_HOOKER).lock().unwrap().call(lpLibFileName,hFile,dwFlags);

    if strcmp(lpLibFileName,b"gdi32.dll\0".as_ptr()){
        lazy_static::initialize(&GL_HOOKER);
    } else if strcmp(lpLibFileName,b"libegl.dll\0".as_ptr()){
        lazy_static::initialize(&GLES_HOOKER);
    }

    //remove hooker(LoadLibraryExW)
    unsafe {
        if (*GL_HOOKER).is_enabled() && (*GLES_HOOKER).is_enabled() {
            (*LOAD_LIBRARY_EX_HOOKER).lock().unwrap().disable().unwrap();
            eprintln!("Removed LoadLibraryExW Hook");
        }
    }

    return module;

    fn strcmp(a:*const u16,b:*const u8)->bool{
        unsafe{
            while *a != 0 && *b !=0{
                if (*a) as u16 != *b as u16{
                    return false;
                }
                b.add(1);
                a.add(1);
            }
        }
        true
    }
}

extern "stdcall" fn gl_swap_buffers(hdc:HDC) -> BOOL{
    eprintln!("GL");
    return (*GL_HOOKER).call(hdc);
}

extern "stdcall" fn egl_swap_buffers(display:EGLDisplay,surface:EGLSurface)->EGLBoolean{
    eprintln!("EGL");
    return (*GLES_HOOKER).call(display,surface);
}