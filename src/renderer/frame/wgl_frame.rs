use winapi::shared::windef::{HDC, HWND, RECT};
use crate::renderer::frame::Frame;
use winapi::um::winuser;

pub struct WGLFrame {
    hwnd: HWND,
}

impl WGLFrame {
    pub fn new(hdc: HDC) -> Self {
        WGLFrame {
            hwnd: unsafe { winuser::WindowFromDC(hdc) }
        }
    }
}

impl Frame for WGLFrame {
    fn get_frame_size(&self) -> (u32, u32) {
        let mut rect = RECT {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        unsafe {
            winuser::GetClientRect(self.hwnd, &mut rect);
        }
        (rect.right as u32, rect.bottom as u32)
    }
}