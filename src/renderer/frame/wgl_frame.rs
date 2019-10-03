use winapi::shared::windef::{HDC,RECT};
use crate::renderer::frame::Frame;
use winapi::um::winuser;

pub struct WGLFrame{
    hdc:HDC
}

impl WGLFrame{
    pub fn new(hdc:HDC)->Self{
        WGLFrame{
            hdc
        }
    }
}

impl Frame for WGLFrame{
    fn get_frame_size(&self) -> (u32, u32) {
        unsafe {
            let hwnd = winuser::WindowFromDC(self.hdc);
            let mut rect = RECT {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            };
            winuser::GetClientRect(hwnd, &mut rect);
            (rect.right as u32, rect.bottom as u32)
        }
    }
}