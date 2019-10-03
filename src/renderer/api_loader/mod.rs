mod gles_loader;
mod gl_loader;

pub use gl_loader::GLLoader;
pub use gles_loader::GLESLoader;

pub trait ApiLoader{
    fn init()->Self;
    fn load_func(&self,name:&str)->*const ();
}