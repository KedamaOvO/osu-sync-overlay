use crate::sync::MemoryMappingFile;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct GlobalConfig{
    //was_changed:bool,
    //_padding:[u8;3],
    pub glyph_ranges:String,//[u8;64],

    mmf:MemoryMappingFile,
}

#[repr(C)]
pub struct OverlayConfigItem{
    mmf:[c_char;128],
    font_path:[c_char;512],
    pub x:i32,
    pub y:i32,
    pub text_color:[f32;4],
    pub background_color:[f32;4],
    pub border_color:[f32;4],
    pub pivot:[f32;2],

    pub font_size:i32,
    pub font_scale:f32,
    pub visibility:bool,
}

#[repr(C)]
pub struct OverlayConfig<'a>{
    //was_changed:bool,
    //_padding:[u8;3],
    //pub config_count:i32,
    pub config_items:&'a [OverlayConfigItem],

    mmf:MemoryMappingFile,
}

impl GlobalConfig {
    pub fn new(mut mmf:MemoryMappingFile)->Self{
        unsafe{
            let mut ptr = mmf.map().unwrap();
            //skip was_changed and padding
            ptr = ptr.add(4);

            let ptr = ptr as *const c_char;
            let glyph_ranges = CStr::from_ptr(ptr).to_str().unwrap().to_string();
            GlobalConfig{
                glyph_ranges,
                mmf
            }
        }
    }

    pub fn check_changed(&self) ->bool{
        unsafe {*(self.mmf.get_ptr().unwrap() as *const bool)}
    }

    pub fn reload(self)->Self{
        let mut config = self;
        unsafe {
            *(config.mmf.map().unwrap() as *mut bool) = false;
        }
        GlobalConfig::new(config.mmf)
    }
}

const OVERLAY_CONFIG_OFFSET:usize = 713;

impl<'a> OverlayConfig<'a>{
    pub fn new(mut mmf:MemoryMappingFile)->Self{
        unsafe{
            let mut ptr = mmf.map().unwrap();

            //skip was_changed and padding
            ptr = ptr.add(4);

            let mut ptr = ptr as *const usize;
            let count = *ptr;
            ptr = ptr.add(1);

            let ptr = ptr as *const OverlayConfigItem;
            let items = std::slice::from_raw_parts(ptr,count);

            OverlayConfig{
                config_items: items,
                mmf
            }
        }
    }

    pub fn check_changed(&self) ->bool{
        unsafe {*(self.mmf.get_ptr().unwrap() as *const bool)}
    }

    pub fn reload(self)->Self{
        let mut config = self;
        unsafe {
            *(config.mmf.map().unwrap() as *mut bool) = false;
        }
        OverlayConfig::new(config.mmf)
    }
}

impl OverlayConfigItem{
//    pub fn new(ptr:*const u8)->Self{
//        unsafe {
//            let mut ptr = ptr as *const c_char;
//            let mmf = CStr::from_ptr(ptr);
//            ptr = ptr.add(128);
//            let font_path = CStr::from_ptr(ptr);
//            ptr = ptr.add(512);
//
//            let mut ptr = ptr as *const i32;
//            let x= *ptr;
//            ptr = ptr.add(1);
//            let y=*ptr;
//            ptr = ptr.add(1);
//
//            let mut ptr = ptr as *const [f32;4];
//            let text_color = *ptr;
//            ptr = ptr.add(1);
//            let background_color = *ptr;
//            ptr = ptr.add(1);
//            let border_color = *ptr;
//            ptr = ptr.add(1);
//
//            let mut ptr = ptr as *const [f32;2];
//            let pivot = *ptr;
//            ptr = ptr.add(1);
//
//            let mut ptr = ptr as *const i32;
//            let font_size = *ptr;
//            ptr = ptr.add(1);
//
//            let ptr = ptr as *const f32;
//            let font_scale = *ptr;
//            ptr.add(1);
//
//            let ptr = ptr as *const bool;
//            let visibility = *ptr;
//
//            OverlayConfigItem{
//                mmf:mmf.to_str().unwrap().to_string(),
//                font_path:font_path.to_str().unwrap().to_string(),
//                x,y,
//                text_color,
//                background_color,
//                border_color,
//                pivot,
//                font_size,
//                font_scale,
//                visibility
//            }
//        }
//    }

    pub fn mmf(&self)->&str{
        let str = unsafe{CStr::from_ptr(self.mmf.as_ptr())};
        str.to_str().unwrap()
    }

    pub fn font_path(&self)->&str{
        let str = unsafe{CStr::from_ptr(self.font_path.as_ptr())};
        str.to_str().unwrap()
    }
}