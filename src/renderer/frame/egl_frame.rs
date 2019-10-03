use crate::renderer::frame::Frame;
use crate::utils::get_proc_address;
use std::mem;

pub type EGLBoolean = winapi::ctypes::c_uint;
pub type EGLDisplay = *mut winapi::ctypes::c_void;
pub type EGLSurface = *mut winapi::ctypes::c_void;
pub type EGLint = i32;

type FnEglQuerySurface = extern "stdcall" fn(EGLDisplay, EGLSurface, EGLint, *mut EGLint);

const EGL_HEIGHT: EGLint = 0x3056;
const EGL_WIDTH: EGLint = 0x3057;

pub struct EGLFrame{
    display:EGLDisplay,
    surface:EGLSurface,

    egl_query_surface:FnEglQuerySurface,
}

impl EGLFrame{
    pub fn new(display:EGLDisplay,surface:EGLSurface)->Self{
        EGLFrame{
            display,
            surface,
            egl_query_surface:unsafe{mem::transmute(get_proc_address("libegl.dll", "eglQuerySurface"))}
        }
    }
}

impl Frame for EGLFrame{
    fn get_frame_size(&self) -> (u32, u32) {
        let mut h = 0;
        let mut w = 0;
        (self.egl_query_surface)(self.display, self.surface, EGL_HEIGHT, &mut h);
        (self.egl_query_surface)(self.display, self.surface, EGL_WIDTH, &mut w);
        (w as u32, h as u32)
    }
}
