use winapi::um::memoryapi;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::PAGE_READWRITE;
use winapi::ctypes::c_void;
use winapi::um::memoryapi::{FILE_MAP_READ, FILE_MAP_WRITE};
use winapi::um::handleapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;


pub struct MemoryMappingFile {
    size: usize,
    mmf_handle: *mut c_void,

    mapped_ptr: *mut u8,
}

impl MemoryMappingFile {
    pub fn new(name: &str, size: usize) -> Self {
        let name_os: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();
        let mmf = unsafe { memoryapi::CreateFileMappingW(INVALID_HANDLE_VALUE, null_mut(), PAGE_READWRITE, 0, size as u32, name_os.as_ptr()) };
        MemoryMappingFile {
            mmf_handle: mmf,
            size,

            mapped_ptr: null_mut(),
        }
    }

    pub fn mapping(&mut self) -> Result<*mut u8, String> {
        unsafe {
            if self.mapped_ptr.is_null() {
                let p = memoryapi::MapViewOfFile(self.mmf_handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, self.size);
                if p.is_null() {
                    return Err("Can't get pointer from MMF.".to_string());
                }

                self.mapped_ptr = p as *mut u8;
            }
            Ok(&mut *self.mapped_ptr)
        }
    }

    pub fn get_ptr(&self) -> Option<*mut u8> {
        unsafe {
            if self.mapped_ptr.is_null() {
                return None;
            }
            Some(&mut *self.mapped_ptr)
        }
    }

    fn unmap(&mut self) {
        unsafe {
            memoryapi::UnmapViewOfFile(self.mapped_ptr as *mut c_void);
        }
        self.mapped_ptr = null_mut();
    }
}

impl Drop for MemoryMappingFile {
    fn drop(&mut self) {
        if !self.mapped_ptr.is_null() {
            self.unmap();
        }
        unsafe {
            handleapi::CloseHandle(self.mmf_handle)
        };
    }
}