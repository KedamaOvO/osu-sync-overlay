mod wgl_frame;
mod egl_frame;

pub use wgl_frame::WGLFrame;
pub use egl_frame::*;

pub trait Frame {
    fn get_frame_size(&self) -> (u32, u32);
}