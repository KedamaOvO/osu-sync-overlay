use crate::sync::MemoryMappingFile;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct GlobalConfig {
    //was_changed:bool,
    //_padding:[u8;3],
    pub glyph_ranges: String,//[u8;64],

    mmf: MemoryMappingFile,
}

#[repr(C)]
pub struct OverlayConfigItem {
    mmf: [c_char; 128],
    font_path: [c_char; 512],
    pub x: i32,
    pub y: i32,
    pub text_color: [f32; 4],
    pub background_color: [f32; 4],
    pub border_color: [f32; 4],
    pub pivot: [f32; 2],

    pub font_size: i32,
    pub font_scale: f32,
    pub visibility: bool,
}

#[repr(C)]
pub struct OverlayConfigs<'a> {
    //was_changed:bool,
    //_padding:[u8;3],
    //pub config_count:i32,

    config_items: &'a [OverlayConfigItem],
    mmf_strs:Vec<String>,
    font_paths:Vec<String>,

    mmf: MemoryMappingFile,
}

pub struct OverlayConfigsIter<'a>
{
    configs:&'a OverlayConfigs<'a>,
    index:usize,
}

impl GlobalConfig {
    pub fn new(mut mmf: MemoryMappingFile) -> Self {
        unsafe {
            let mut ptr = mmf.mapping().unwrap();
            //skip was_changed and padding
            ptr = ptr.add(4);

            let ptr = ptr as *const c_char;
            let glyph_ranges = CStr::from_ptr(ptr).to_str().unwrap().to_string();
            GlobalConfig {
                glyph_ranges,
                mmf,
            }
        }
    }

    pub fn check_changed(&self) -> bool {
        unsafe { *(self.mmf.get_ptr().unwrap() as *const bool) }
    }

    pub fn reload(self) -> Self {
        let mut config = self;
        unsafe {
            *(config.mmf.mapping().unwrap() as *mut bool) = false;
        }
        GlobalConfig::new(config.mmf)
    }
}

impl<'a> OverlayConfigs<'a> {
    pub fn new(mut mmf: MemoryMappingFile) -> Self {
        unsafe {
            let mut ptr = mmf.mapping().unwrap();

            //skip was_changed and padding
            ptr = ptr.add(4);

            let mut ptr = ptr as *const usize;
            let count = *ptr;
            ptr = ptr.add(1);

            let ptr = ptr as *const OverlayConfigItem;
            let items = std::slice::from_raw_parts(ptr, count);

            let mut mmfs = Vec::new();
            let mut font_paths = Vec::new();

            for config in items.iter(){
                mmfs.push(CStr::from_ptr(config.mmf.as_ptr()).to_str().unwrap().to_owned());
                font_paths.push(CStr::from_ptr(config.font_path.as_ptr()).to_str().unwrap().to_owned());
            }

            OverlayConfigs {
                config_items: items,
                mmf,
                mmf_strs: mmfs,
                font_paths,
            }
        }
    }

    pub fn check_changed(&self) -> bool {
        unsafe { *(self.mmf.get_ptr().unwrap() as *const bool) }
    }

    pub fn reload(self) -> Self {
        let mut config = self;
        unsafe {
            *(config.mmf.mapping().unwrap() as *mut bool) = false;
        }
        OverlayConfigs::new(config.mmf)
    }

    pub fn iter(&self)->OverlayConfigsIter{
        OverlayConfigsIter{
            configs:self,
            index:0,
        }
    }
}

impl<'a> Iterator for OverlayConfigsIter<'a>{
    type Item = (&'a str,&'a str,&'a OverlayConfigItem);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.configs.config_items.len() {
            let r = (
                self.configs.mmf_strs[self.index].as_str(),
                self.configs.font_paths[self.index].as_str(),
                &self.configs.config_items[self.index]
            );
            self.index += 1;
            return Some(r)
        }
        None
    }
}