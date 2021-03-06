use imgui::*;
use std::collections::{HashSet, HashMap};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs::File;
use std::io::Read;
use crate::renderer::{UIRenderer, Frame};
use crate::sync::{OverlayConfigs, MemoryMappingFile};
use std::time::Instant;
use std::mem;

const ERROR_WINDOW_ETA:f32 = 5.0f32;

pub struct UI<F:Frame>{
    ctx:Context,
    renderer:UIRenderer,
    frame:F,

    fonts:HashMap<String,FontId>,
    font_glyph_ranges:HashMap<&'static str,FontGlyphRanges>,

    prev_time:Instant,

    error_messages:Vec<(String,f32)>,
}

impl<F:Frame> UI<F>
{
    pub fn init(renderer:UIRenderer,frame:F) -> Self{
        let mut ctx = Context::create();
        ctx.set_ini_filename(None);
        ctx.io_mut().display_framebuffer_scale = [1.0,1.0];

        let mut font_glyph_ranges = HashMap::new();
        font_glyph_ranges.insert("Default",FontGlyphRanges::default());
        font_glyph_ranges.insert("Chinese",FontGlyphRanges::chinese_simplified_common());
        font_glyph_ranges.insert("ChineseFull",FontGlyphRanges::chinese_full());
        font_glyph_ranges.insert("Cyrillic",FontGlyphRanges::cyrillic());
        font_glyph_ranges.insert("Japanese",FontGlyphRanges::japanese());
        font_glyph_ranges.insert("Korean",FontGlyphRanges::korean());
        font_glyph_ranges.insert("Thai",FontGlyphRanges::thai());
        font_glyph_ranges.insert("Vietnamese",FontGlyphRanges::vietnamese());

        UI{
            ctx,
            renderer,
            frame,

            fonts:HashMap::new(),
            font_glyph_ranges,

            prev_time:Instant::now(),

            error_messages:Vec::new(),
        }
    }

    fn load_fonts(&mut self,configs:&OverlayConfigs,glyph_ranges:&str){
        let mut fonts = self.ctx.fonts();
        let mut mmf_to_font_map = HashMap::new();

        // Add default font
        let default_font = fonts.add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: 14.0,
                    glyph_ranges:self.font_glyph_ranges.get(glyph_ranges).unwrap().clone(),
                    ..FontConfig::default()
                }),
            }]);
        mmf_to_font_map.insert("__default__".to_string(),default_font);

        let mut font_set = HashSet::new();

        for (_,font_path,config) in configs.iter() {
            let path = font_path;
            font_set.insert((path,config.font_size));
        }

        let font_glyph_ranges = &self.font_glyph_ranges;
        let msgs = &mut self.error_messages;
        // Add fonts
        let ids:Vec<((&str,i32),FontId)> = font_set.iter().map(|&(path,size)|{
            let font_id = match File::open(path) {
                Ok(mut ttf)=> {
                    let mut buf = Vec::new();
                    let _ = ttf.read_to_end(&mut buf);
                    let font = fonts.add_font(&[FontSource::TtfData {
                        data: buf.as_slice(),
                        size_pixels: size as f32,
                        config: Some(FontConfig {
                            glyph_ranges: font_glyph_ranges.get(glyph_ranges).unwrap().clone(),
                            ..FontConfig::default()
                        }),
                    }]);
                    info!("Added font({}, {})",path,size);
                    font
                }
                _ => {
                    let msg = format!("Can't load Font({}, {}). use default font.",path, size);
                    error!("{}",msg);
                    msgs.push((msg,ERROR_WINDOW_ETA));
                    default_font
                }
            };

            ((path,size),font_id)
        }).collect();

        // MMF -> FontId
        for (mmf_str,font_path,config) in configs.iter(){
            let key = mmf_str;
            let path = font_path;

            let r = ids.iter().find(|t| t.0 == (path,config.font_size)).unwrap();
            mmf_to_font_map.insert(key.to_string(),r.1);
        }

        self.fonts = mmf_to_font_map;

        let fonts_tex = fonts.build_alpha8_texture();
        info!("Builded fonts texture({}x{})",fonts_tex.width,fonts_tex.height);
        self.renderer.upload_texture_data(fonts_tex.width,fonts_tex.height,fonts_tex.data)
    }

    pub fn reload_fonts(&mut self, config:&OverlayConfigs, glyph_ranges:&str){
        self.ctx.fonts().clear();
        self.load_fonts(config,glyph_ranges);
    }

    pub fn render(&mut self, configs:&OverlayConfigs, mmfs:&HashMap<String,MemoryMappingFile>){
        //update time
        let now = self.ctx.io_mut().update_delta_time(self.prev_time);
        self.prev_time = now;

        let (w,h)= self.frame.get_frame_size();
        self.ctx.io_mut().display_size = [w as f32,h as f32];

        self.layout(&configs,mmfs,|renderer,layout_data|{
            renderer.render(w,h,layout_data,)
        });
    }

    pub fn push_error_message(&mut self,msg:String){
        self.error_messages.push((msg,ERROR_WINDOW_ETA));
    }

    pub fn layout<Func:FnOnce(&mut UIRenderer,&DrawData)>(
        &mut self,
        configs:&OverlayConfigs,
        mmfs:&HashMap<String,MemoryMappingFile>,
        render:Func)
    {
        let [w,h] = self.ctx.io().display_size;
        let dt = self.ctx.io().delta_time;
        let ui = self.ctx.frame();

        for (mmf_str,_,config) in configs.iter() {
            if !config.visibility{
                continue;
            }

            if let Some(mmf) = mmfs.get(mmf_str) {
                let text = unsafe { CStr::from_ptr(mmf.get_ptr().unwrap() as *const c_char) }.to_str().unwrap();
                let bg = ui.push_style_color(StyleColor::WindowBg, config.background_color);
                let border = ui.push_style_color(StyleColor::Border, config.border_color);
                let title = CString::new(mmf_str).unwrap();

                let win = Window::new(unsafe { mem::transmute(title.as_c_str()) })
                    .title_bar(false)
                    .always_auto_resize(true)
                    .save_settings(false)
                    .mouse_inputs(false)
                    .no_inputs()
                    .bg_alpha(config.background_color[3])
                    .position([config.x as f32, config.y as f32], Condition::Always)
                    .position_pivot(config.pivot)
                    .begin(&ui)
                    .unwrap();

                let fontid = self.fonts.get(mmf_str);
                let font = match fontid {
                    Some(f) => ui.push_font(*f),
                    None => ui.push_font(*self.fonts.get("__default__").unwrap())
                };

                ui.set_window_font_scale(config.font_scale);
                ui.text_colored(config.text_color, text);
                ui.set_window_font_scale(1.0);

                font.pop(&ui);

                bg.pop(&ui);
                border.pop(&ui);

                win.end(&ui);
            }
        }

        if !self.error_messages.is_empty(){
            let win = Window::new(im_str!("error-window"))
                .title_bar(false)
                .always_auto_resize(true)
                .save_settings(false)
                .mouse_inputs(false)
                .no_inputs()
                .position([w / 2.0 ,h * 5.0 / 6.0 ],Condition::Always)
                .position_pivot([0.5f32,0.5f32])
                .begin(&ui)
                .unwrap();


            for msg in self.error_messages.iter_mut() {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], format!("({:.1}) {}",msg.1,msg.0));
                msg.1 -= dt;
            }

            self.error_messages.retain(|msg|{
                msg.1 >= 0.0
            });

            win.end(&ui);
        }
        let layout_data = ui.render();
        render(&mut self.renderer,layout_data);
    }
}