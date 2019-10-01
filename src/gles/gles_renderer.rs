use super::gles2_bindings as gl;
use std::mem;
use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use winapi::um::libloaderapi;
use crate::utils;
use crate::gui::UIRenderer;
use imgui::DrawData;
use crate::gles::egldef::{EGLSurface, EGLDisplay};
use crate::utils::get_egl_window_size;
use crate::gles::gles2_bindings::types::*;

type FnEglGetProcAddress = extern "stdcall" fn(*const c_char) -> *mut c_void;

pub struct GLESRenderer {
    vbo:[GLuint;3],
    ibo:[GLuint;3],
    index:i32,

    shader:GLuint,
    texture:GLuint,

    frame_size:(u32,u32),
    display: EGLDisplay,
    surface: EGLSurface,
}

impl GLESRenderer {
    pub fn load_func(){
        let glesmodule = unsafe { libloaderapi::GetModuleHandleA(b"libGLESv2.dll\0".as_ptr() as *const i8) };
        let get_proc_addr: FnEglGetProcAddress = unsafe { mem::transmute(utils::get_proc_address("libEGL.dll", "eglGetProcAddress")) };

        //GLES Loader
        gl::load_with(|name| {
            unsafe {
                let cname = CString::new(name).unwrap();
                let proc = libloaderapi::GetProcAddress(glesmodule, cname.as_ptr() as *const i8);
                if !proc.is_null() {
                    return mem::transmute(proc);
                }

                return mem::transmute(get_proc_addr(cname.as_ptr()));
            }
        });
    }

    pub fn init(display: EGLDisplay, surface: EGLSurface)->Self{
        let frame_size = get_egl_window_size(display,surface);
        let mut renderer = GLESRenderer {
            vbo: [0,0,0],
            ibo: [0,0,0],
            index:0,

            shader: 0,
            texture: 0,
            display,
            surface,
            frame_size
        };

        renderer
    }
}

impl UIRenderer for GLESRenderer{
    fn get_frame_size(&self) -> (u32, u32) {
        self.frame_size
    }

    fn upload_texture_data(&mut self, w: u32, h: u32, pixels: &[u8]) {
        unsafe{
            let mut last_texture: GLint = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D,&mut last_texture);
            gl::BindTexture(gl::TEXTURE_2D,self.texture);
            gl::TexImage2D(self.texture,0,gl::ALPHA as i32,w as i32,h as i32,0,gl::ALPHA,gl::UNSIGNED_BYTE,pixels.as_ptr() as *const _);
            gl::BindTexture(gl::TEXTURE_2D,last_texture as u32);
        }
    }

    fn render(&mut self, cmd_lists: &DrawData) {
        unimplemented!()
    }
}

impl Drop for GLESRenderer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(3,self.vbo.as_mut_ptr());
            gl::DeleteBuffers(3,self.ibo.as_mut_ptr());
            gl::DeleteTextures(1,&mut self.texture);
            gl::DeleteProgram(self.shader);
        }
    }
}