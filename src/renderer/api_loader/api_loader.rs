pub trait ApiLoader{
    fn init()->Self;
    fn load_func(&self,name:&str)->*const ();
}