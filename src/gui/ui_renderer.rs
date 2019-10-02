pub trait UIRenderer{
    fn get_frame_size(&self)->(u32,u32);
    fn upload_texture_data(&mut self,w:u32,h:u32,pixels:&[u8]);
    fn render(&mut self,w:u32,h:u32,cmd_lists:&imgui::DrawData);
}