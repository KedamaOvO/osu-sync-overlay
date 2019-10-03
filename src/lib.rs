mod sync;
mod gui;
mod utils;
mod renderer;

use crate::renderer::*;
use utils::*;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use winapi::shared::ntdef::{HANDLE, LPCWSTR};
use winapi::shared::minwindef::{BOOL, DWORD, LPVOID, HINSTANCE, TRUE, HMODULE};
use winapi::shared::windef::HDC;
use detour::GenericDetour;

use simplelog::{CombinedLogger, WriteLogger, Config, LevelFilter};
use std::fs::File;
use crate::gui::UI;
use crate::sync::*;
use std::mem;
use std::collections::HashMap;
use widestring::U16CString;

type FnLoadLibraryExW = extern "stdcall" fn(LPCWSTR, HANDLE, DWORD) -> HMODULE;
type FnGLSwapBuffers = extern "stdcall" fn(HDC) -> BOOL;
type FnEGLSwapBuffers = extern "stdcall" fn(EGLDisplay, EGLSurface) -> EGLBoolean;

lazy_static! {
    static ref LOAD_LIBRARY_EX_HOOKER: GenericDetour<FnLoadLibraryExW> = {
        unsafe{
            let load_library_exw_addr:FnLoadLibraryExW = mem::transmute(get_proc_address("kernel32.dll","LoadLibraryExW"));
            debug!("LoadLibraryExW Address: 0x{:X?}",load_library_exw_addr as usize);

            let hook = GenericDetour::<FnLoadLibraryExW>::new(load_library_exw_addr,hook_load_library_ex).unwrap();
            hook.enable().unwrap();
            hook
        }
    };

    static ref GL_HOOKER: GenericDetour<FnGLSwapBuffers> = {
        unsafe{
            let swap_buffers_addr:FnGLSwapBuffers = mem::transmute(get_proc_address("gdi32.dll","SwapBuffers"));
            debug!("SwapBuffers Address: 0x{:X?}",swap_buffers_addr as usize);

            let hook = GenericDetour::<FnGLSwapBuffers>::new(swap_buffers_addr,wgl_swap_buffers).unwrap();
            hook.enable().unwrap();
            hook
        }
    };

    static ref GLES_HOOKER: GenericDetour<FnEGLSwapBuffers> = {
        unsafe{
            let swap_buffers_addr:FnEGLSwapBuffers = mem::transmute(get_proc_address("libegl.dll","eglSwapBuffers"));
            debug!("eglSwapBuffers Address: 0x{:X?}",swap_buffers_addr as usize);

            let hook = GenericDetour::<FnEGLSwapBuffers>::new(swap_buffers_addr,egl_swap_buffers).unwrap();
            hook.enable().unwrap();
            hook
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

    TRUE
}

static mut GLOBAL_CONFIG: Option<GlobalConfig> = None;
static mut OVERLAY_CONFIG: Option<OverlayConfig> = None;
static mut OVERLAY_MMFS: Option<HashMap<String, MemoryMappingFile>> = None;
const MMF_OVERLAY_CONFIG_LENGTH: usize = 65535;
const MMF_LENGTH: usize = 4096;

fn load_overlay_mmfs(configs: &[OverlayConfigItem]) -> HashMap<String, MemoryMappingFile> {
    let mut map = HashMap::new();
    for config in configs.iter() {
        let mut mmf = MemoryMappingFile::new(config.mmf(), MMF_LENGTH);
        let _ = mmf.map();
        map.insert(config.mmf().to_string(), mmf);
    }
    map
}

fn init_logger() {
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("overlay-log.log").unwrap()),
            #[cfg(debug_assertions)]
                WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("overlay-debug.log").unwrap()),
        ]
    ).unwrap();
}

fn init_config() {
    unsafe {
        GLOBAL_CONFIG = Some(GlobalConfig::new(MemoryMappingFile::new("Local\\rtpp-overlay-global-config", mem::size_of::<GlobalConfig>())));
        OVERLAY_CONFIG = Some(OverlayConfig::new(MemoryMappingFile::new("Local\\rtpp-overlay-configs", MMF_OVERLAY_CONFIG_LENGTH)));
        OVERLAY_MMFS = Some(load_overlay_mmfs(OVERLAY_CONFIG.as_ref().unwrap().config_items));
    }
}

fn init() {
    init_logger();
    init_config();

    if !module_is_loaded("libegl.dll") &&
        !module_is_loaded("opengl32.dll")
    {
        lazy_static::initialize(&LOAD_LIBRARY_EX_HOOKER);
    } else {
        lazy_static::initialize(&GL_HOOKER);
        lazy_static::initialize(&GLES_HOOKER);
        unsafe { GL_HOOKER_OK = true; }
    }
}

static mut GL_HOOKER_OK: bool = false;

#[allow(non_snake_case)]
extern "stdcall" fn hook_load_library_ex(lpLibFileName: LPCWSTR, hFile: HANDLE, dwFlags: DWORD) -> HMODULE {
    let module = (*LOAD_LIBRARY_EX_HOOKER).call(lpLibFileName, hFile, dwFlags);
    let lib_name = unsafe { U16CString::from_ptr_str(lpLibFileName) }.to_string_lossy();

    if lib_name.as_str() == "gdi32.dll" {
        info!("OpenGL ready");
        lazy_static::initialize(&GL_HOOKER);
        unsafe { GL_HOOKER_OK = true; }
    } else if lib_name.as_str() == "libegl.dll" {
        info!("OpenGL ES ready");
        lazy_static::initialize(&GLES_HOOKER);
        unsafe { GL_HOOKER_OK = true; }
    }

    //remove hooker(LoadLibraryExW)
    unsafe {
        if GL_HOOKER_OK {
            (*LOAD_LIBRARY_EX_HOOKER).disable().unwrap();
            info!("Removed LoadLibraryExW hook");
        }
    }

    module
}

static mut GL_UI: Option<UI<WGLFrame>> = None;
static mut GLES_UI: Option<UI<EGLFrame>> = None;

fn check_config_changed<F: Frame>(ui: &mut UI<F>, force: bool) {
    unsafe {
        if GLOBAL_CONFIG.as_ref().unwrap().check_changed() || force {
            GLOBAL_CONFIG = Some(GLOBAL_CONFIG.take().unwrap().reload());
        }

        if OVERLAY_CONFIG.as_ref().unwrap().check_changed() || force {
            info!("Reload overlay config");
            OVERLAY_CONFIG = Some(OVERLAY_CONFIG.take().unwrap().reload());
            info!("Reload fonts");
            ui.reload_fonts(OVERLAY_CONFIG.as_ref().unwrap(), GLOBAL_CONFIG.as_ref().unwrap().glyph_ranges.as_str());
            info!("Fonts Reloaded");
        }
    }
}

extern "stdcall" fn wgl_swap_buffers(hdc: HDC) -> BOOL {
    unsafe {
        match &mut GL_UI {
            None => {
                let wgl = WGLFrame::new(hdc);
                let renderer = UIRenderer::init(GLLoader::init());
                GL_UI = Some(UI::init(renderer, wgl));
                if let Some(ui) = GL_UI.as_mut() {
                    check_config_changed(ui, true);
                }
                info!("GL Initialized");
            }
            Some(ui) => {
                ui.render(OVERLAY_CONFIG.as_ref().unwrap(), OVERLAY_MMFS.as_ref().unwrap());
            }
        }

        if let Some(ui) = GL_UI.as_mut() {
            check_config_changed(ui, false);
        }
    }
    (*GL_HOOKER).call(hdc)
}

extern "stdcall" fn egl_swap_buffers(display: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    unsafe {
        match &mut GLES_UI {
            None => {
                let egl = EGLFrame::new(display, surface);
                let renderer = UIRenderer::init(GLESLoader::init());
                GLES_UI = Some(UI::init(renderer, egl));
                if let Some(ui) = GLES_UI.as_mut() {
                    check_config_changed(ui, true);
                }
                info!("GLES Initialized");
            }
            Some(ui) => {
                ui.render(OVERLAY_CONFIG.as_ref().unwrap(), OVERLAY_MMFS.as_ref().unwrap());
            }
        }

        if let Some(ui) = GLES_UI.as_mut() {
            check_config_changed(ui, false);
        }
    }

    (*GLES_HOOKER).call(display, surface)
}