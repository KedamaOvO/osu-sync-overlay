use super::gles2_bindings as gl;
use std::mem;
use std::ffi::c_void;
use imgui::*;
use crate::renderer::gles2_bindings::types::*;
use crate::renderer::ApiLoader;
use imgui::internal::RawWrapper;

pub struct UIRenderer {
    //vertex
    vbos:[GLuint;3],
    ibos:[GLuint;3],
    index:usize,

    //shader and uniform location
    shader:GLuint,
    texture_location:GLint,
    proj_matrix_location:GLint,
    position_location:GLint,
    uv_location:GLint,
    color_location:GLint,

    texture:GLuint,
}

impl UIRenderer {
    pub fn init<L:ApiLoader>(loader:L)->Self{
        gl::load_with(|name|{
            loader.load_func(name) as *const _
        });

        let mut renderer = UIRenderer {
            vbos: [0,0,0],
            ibos: [0,0,0],
            index:0,

            texture_location:0,
            proj_matrix_location:0,
            position_location:0,
            uv_location:0,
            color_location:0,

            shader: 0,
            texture: 0,
        };

        unsafe {
            // Backup GL status
            let mut last_texture:GLint = 0;
            let mut last_array_buffer:GLint = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture as *mut GLint);
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer as *mut GLint);

            gl::GenBuffers(3, renderer.vbos.as_mut_ptr());
            gl::GenBuffers(3, renderer.ibos.as_mut_ptr());

            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            renderer.shader = gl::CreateProgram();

            // Build shader
            let vs_bytes = include_bytes!("shaders/ui_vert.glsl");
            let vs_bytes_len = vs_bytes.len() as i32;
            let fs_bytes = include_bytes!("shaders/ui_frag.glsl");
            let fs_bytes_len = fs_bytes.len() as i32;
            gl::ShaderSource(vs, 1, mem::transmute(&vs_bytes.as_ptr()), &vs_bytes_len as *const _);
            gl::ShaderSource(fs, 1, mem::transmute(&fs_bytes.as_ptr()), &fs_bytes_len as *const _);
            gl::CompileShader(vs);
            gl::CompileShader(fs);
            gl::AttachShader(renderer.shader, vs);
            gl::AttachShader(renderer.shader, fs);
            gl::LinkProgram(renderer.shader);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            // Init texture
            let last_texture: GLint = 0;

            gl::GenTextures(1, &mut renderer.texture);
            gl::BindTexture(gl::TEXTURE_2D, renderer.texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            //gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
            // Restore texture
            gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);

            gl::UseProgram(renderer.shader);
            // Get uniform location
            renderer.texture_location = gl::GetUniformLocation(renderer.shader, b"Texture\0".as_ptr() as *const _);
            renderer.proj_matrix_location = gl::GetUniformLocation(renderer.shader, b"ProjMtx\0".as_ptr() as *const _);
            renderer.position_location = gl::GetAttribLocation(renderer.shader, b"Position\0".as_ptr() as *const _);
            renderer.uv_location = gl::GetAttribLocation(renderer.shader, b"UV\0".as_ptr() as *const _);
            renderer.color_location = gl::GetAttribLocation(renderer.shader, b"Color\0".as_ptr() as *const _);

            // Restore modified GL state
            gl::BindTexture(gl::TEXTURE_2D, last_texture as GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as GLuint);
        }

        renderer
    }

    pub fn render(&mut self,w:u32,h:u32,draw_data: &DrawData) {
        unsafe {
            // Backup GL state
            let last_active_texture: GLenum = 0;
            gl::GetIntegerv(gl::ACTIVE_TEXTURE, mem::transmute(&last_active_texture));
            gl::ActiveTexture(gl::TEXTURE0);
            let last_program = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, mem::transmute(&last_program));
            let last_texture:GLint = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, mem::transmute(&last_texture));
            let last_array_buffer:GLint = 0;
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, mem::transmute(&last_array_buffer));
            let last_element_buffer:GLint = 0;
            gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING,mem::transmute(&last_element_buffer));
            let last_viewport:[GLint;4] = [0,0,0,0];
            gl::GetIntegerv(gl::VIEWPORT, mem::transmute(last_viewport.as_ptr()));
            let last_scissor_box:[GLint;4] = [0,0,0,0];
            gl::GetIntegerv(gl::SCISSOR_BOX, mem::transmute(last_scissor_box.as_ptr()));
            let last_blend_src_rgb:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_SRC_RGB, mem::transmute(&last_blend_src_rgb));
            let last_blend_dst_rgb:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_DST_RGB, mem::transmute(&last_blend_dst_rgb));
            let last_blend_src_alpha:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_SRC_ALPHA, mem::transmute(&last_blend_src_alpha));
            let last_blend_dst_alpha:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_DST_ALPHA, mem::transmute(&last_blend_dst_alpha));
            let last_blend_equation_rgb:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_EQUATION_RGB, mem::transmute(&last_blend_equation_rgb));
            let last_blend_equation_alpha:GLenum = 0;
            gl::GetIntegerv(gl::BLEND_EQUATION_ALPHA, mem::transmute(&last_blend_equation_alpha));
            let last_enable_blend:GLboolean = gl::IsEnabled(gl::BLEND);
            let last_enable_cull_face:GLboolean = gl::IsEnabled(gl::CULL_FACE);
            let last_enable_depth_test:GLboolean = gl::IsEnabled(gl::DEPTH_TEST);
            let last_enable_scissor_test:GLboolean = gl::IsEnabled(gl::SCISSOR_TEST);

            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);

            gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
            let left = draw_data.display_pos[0];
            let right = draw_data.display_pos[0] + draw_data.display_size[0];
            let top = draw_data.display_pos[1];
            let bottom = draw_data.display_pos[1] + draw_data.display_size[1];

            let ortho_projection:[[f32;4];4] =
                [
                    [  2.0/(right-left),         0.0,                       0.0, 0.0 ],
                    [  0.0,                      2.0/(top-bottom),          0.0, 0.0 ],
                    [  0.0,                      0.0,                      -1.0, 0.0 ],
                    [ (right+left)/(left-right), (top+bottom)/(bottom-top), 0.0, 1.0 ],
                ];

            gl::UseProgram(self.shader);
            gl::Uniform1i(self.texture_location, 0);
            gl::UniformMatrix4fv(self.proj_matrix_location, 1, gl::FALSE, &ortho_projection[0][0]);

            for cmd_list in draw_data.draw_lists(){
                let vbo = self.vbos[self.index];
                let ibo = self.ibos[self.index];

                self.index = (self.index + 1) % self.vbos.len();

                gl::BindBuffer(gl::ARRAY_BUFFER,vbo);
                let vtx_buffer = cmd_list.vtx_buffer();
                gl::BufferData(gl::ARRAY_BUFFER,
                               (vtx_buffer.len() * mem::size_of::<DrawVert>()) as isize,
                               vtx_buffer.as_ptr() as *const _,
                               gl::STREAM_DRAW);

                let idx_buffer = cmd_list.idx_buffer();
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,ibo);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                               (idx_buffer.len() * mem::size_of::<u16>()) as isize,
                               idx_buffer.as_ptr() as *const _,
                               gl::STREAM_DRAW);

                gl::EnableVertexAttribArray(self.position_location as u32);
                gl::EnableVertexAttribArray(self.uv_location as u32);
                gl::EnableVertexAttribArray(self.color_location as u32);
                gl::VertexAttribPointer(self.position_location as u32, 2, gl::FLOAT, gl::FALSE, mem::size_of::<DrawVert>() as i32, 0 as *const GLvoid);
                gl::VertexAttribPointer(self.uv_location as u32, 2, gl::FLOAT, gl::FALSE, mem::size_of::<DrawVert>() as i32, 8 as *const GLvoid);
                gl::VertexAttribPointer(self.color_location as u32, 4, gl::UNSIGNED_BYTE, gl::TRUE, mem::size_of::<DrawVert>() as i32, 16 as *const GLvoid);

                gl::BindTexture(gl::TEXTURE_2D, self.texture);
                for cmd in cmd_list.commands(){
                    match cmd{
                        DrawCmd::Elements {
                            count,
                            cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                idx_offset,
                                ..
                            }} =>{
                            let clip:[u32;4]=[
                                (clip_rect[0] - draw_data.display_pos[0] * draw_data.framebuffer_scale[0]) as u32,
                                (clip_rect[1] - draw_data.display_pos[1] * draw_data.framebuffer_scale[1]) as u32,
                                (clip_rect[2] - draw_data.display_pos[0] * draw_data.framebuffer_scale[0]) as u32,
                                (clip_rect[3] - draw_data.display_pos[1] * draw_data.framebuffer_scale[1]) as u32,
                            ];

                            if clip[0] < w && clip_rect[1] < h as f32 && clip_rect[2] >= 0.0 && clip_rect[3] >= 0.0{
                                gl::Scissor(clip[0] as i32, (h - clip[3]) as i32, (clip[2] - clip[0]) as i32, (clip_rect[3] - clip_rect[1]) as i32);
                                gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_SHORT, (idx_offset*2) as *const c_void);
                            }
                        }
                        DrawCmd::ResetRenderState => (),
                        DrawCmd::RawCallback { callback, raw_cmd } => callback(cmd_list.raw(), raw_cmd),
                    }
                }
            }

            // Restore modified GL state
            gl::UseProgram(last_program);
            gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as u32);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,last_element_buffer as u32);
            gl::ActiveTexture(last_active_texture);
            gl::BlendEquationSeparate(last_blend_equation_rgb, last_blend_equation_alpha);
            gl::BlendFuncSeparate(last_blend_src_rgb, last_blend_dst_rgb, last_blend_src_alpha, last_blend_dst_alpha);
            if last_enable_blend == gl::TRUE{
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }

            if last_enable_cull_face == gl::TRUE{
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }

            if last_enable_depth_test  == gl::TRUE{
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            if last_enable_scissor_test == gl::TRUE{
                gl::Enable(gl::SCISSOR_TEST);
            } else {
                gl::Disable(gl::SCISSOR_TEST);
            }

            gl::Viewport(last_viewport[0], last_viewport[1], last_viewport[2], last_viewport[3]);
            gl::Scissor(last_scissor_box[0], last_scissor_box[1], last_scissor_box[2], last_scissor_box[3]);
        }
    }

    pub fn upload_texture_data(&mut self, w: u32, h: u32, pixels: &[u8]) {
        unsafe{
            let mut last_texture: GLint = 0;
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D,&mut last_texture);
            gl::BindTexture(gl::TEXTURE_2D,self.texture);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::ALPHA as i32,w as i32,h as i32,0,gl::ALPHA,gl::UNSIGNED_BYTE,pixels.as_ptr() as *const _);
            gl::BindTexture(gl::TEXTURE_2D,last_texture as u32);
        }
    }
}

impl Drop for UIRenderer{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteBuffers(3,self.vbos.as_mut_ptr());
            gl::DeleteBuffers(3,self.ibos.as_mut_ptr());
            gl::DeleteTextures(1,&self.texture);
            gl::DeleteProgram(self.shader);
        }
    }
}