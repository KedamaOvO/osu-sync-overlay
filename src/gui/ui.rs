use imgui::*;
use std::collections::{HashSet, HashMap};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs::File;
use std::io::Read;
use crate::gui::UIRenderer;
use crate::sync::{OverlayConfigItem, OverlayConfig, MemoryMappingFile};
use std::time::Instant;
use std::mem;

const ERROR_WINDOW_ETA:f32 = 5.0f32;

pub struct UI<R:UIRenderer>{
    ctx:Context,
    renderer:R,
    fonts:HashMap<String,FontId>,
    font_glyph_ranges:HashMap<&'static str,FontGlyphRanges>,

    prev_time:Instant,

    error_messages:Vec<String>,
    error_window_eta:f32,
}

impl<R> UI<R>
    where R:UIRenderer
{
    pub fn init(renderer:R) -> Self{
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
            fonts:HashMap::new(),
            font_glyph_ranges,

            prev_time:Instant::now(),

            error_messages:Vec::new(),
            error_window_eta:0.0,
        }
    }

    fn load_fonts(&mut self,configs:&[OverlayConfigItem],glyph_ranges:&str){
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

        for config in configs.iter() {
            let path = config.font_path.as_str();
            font_set.insert((path.to_string(),config.font_size));
        }

        let font_glyph_ranges = &self.font_glyph_ranges;
        // Add fonts
        let ids:Vec<((&String,i32),FontId)> = font_set.iter().map(|f|{
            let font_id = match File::open(&f.0) {
                Ok(mut ttf)=> {
                    let mut buf = Vec::new();
                    ttf.read_to_end(&mut buf);
                    let font_id = fonts.add_font(&[FontSource::TtfData {
                        data: buf.as_slice(),
                        size_pixels: f.1 as f32,
                        config: Some(FontConfig {
                            glyph_ranges: font_glyph_ranges.get(glyph_ranges).unwrap().clone(),
                            ..FontConfig::default()
                        }),
                    }]);
                    font_id
                }
                _ => {
                    error!("Can't open {}",&f.0);
                    default_font
                }
            };
            info!("Added font({}, {})",(f.0).as_str(),f.1);
            ((&f.0,f.1),font_id)
        }).collect();

        // MMF -> FontId
        for config in configs.iter(){
            let key = config.mmf.as_str();
            let path = &config.font_path;

            let r = ids.iter().find(|t| t.0 == (path,config.font_size)).unwrap();
            mmf_to_font_map.insert(key.to_string(),r.1);
        }

        self.fonts = mmf_to_font_map;

        let fonts_tex = fonts.build_alpha8_texture();
        info!("Build fonts texture");
        self.renderer.upload_texture_data(fonts_tex.width,fonts_tex.height,fonts_tex.data)
    }

    pub fn reload_fonts(&mut self,config:&OverlayConfig,glyph_ranges:&str){
        self.ctx.fonts().clear();
        self.load_fonts(&config.config_items,glyph_ranges);
    }

    pub fn render(&mut self,config:&OverlayConfig,mmfs:&HashMap<String,MemoryMappingFile>){
        //update time
        let now = self.ctx.io_mut().update_delta_time(self.prev_time);
        self.prev_time = now;

        let frame_size= self.renderer.get_frame_size();
        self.ctx.io_mut().display_size = [frame_size.0 as f32,frame_size.1 as f32];

        self.layout(&config.config_items,mmfs,|renderer,layout_data|{
            renderer.render(layout_data)
        });
    }

    pub fn layout<F:FnOnce(&mut R,&DrawData)>(
        &mut self,
        configs:&[OverlayConfigItem],
        mmfs:&HashMap<String,MemoryMappingFile>,
        render:F)
    {
        let dt = self.ctx.io().delta_time;
        let ui = self.ctx.frame();

        Window::new(im_str!("123"))
            .title_bar(true)
            .build(&ui,||{
                ui.text("ASD");
            });

        for config in configs.iter() {
            let bg = ui.push_style_color(StyleColor::WindowBg,config.background_color);
            let border = ui.push_style_color(StyleColor::Border,config.border_color);
            let title = CString::new(config.mmf.as_str()).unwrap();

            let win = Window::new(unsafe{mem::transmute(title.as_c_str())})
                .title_bar(false)
                .always_auto_resize(true)
                .save_settings(false)
                .mouse_inputs(false)
                .no_inputs()
                .bg_alpha(config.background_color[3])
                .position([config.x as f32,config.y as f32],Condition::Always)
                .position_pivot(config.pivot)
                .begin(&ui)
                .unwrap();

            let fontid = self.fonts.get(config.mmf.as_str());
            let font = match fontid {
                Some(f) => ui.push_font(*f),
                None => {
                    self.error_messages.push(format!("[MMF: {}] Can't load Font({}, {}). use default font.", config.mmf, config.font_path, config.font_path));
                    if !self.error_messages.is_empty() && self.error_window_eta <= 0.0 {
                        self.error_window_eta = ERROR_WINDOW_ETA;
                    }

                    ui.push_font(*self.fonts.get("__default__").unwrap())
                }
            };

            let mmf = mmfs.get(config.mmf.as_str()).unwrap();
            let text = unsafe { CStr::from_ptr(mmf.get_ptr().unwrap() as *const c_char).to_str().unwrap() };
            ui.text_colored(config.text_color, text);

            font.pop(&ui);

            bg.pop(&ui);
            border.pop(&ui);

            win.end(&ui);
        }

        let (w,h) = self.renderer.get_frame_size();

        if !self.error_messages.is_empty(){
            let win = Window::new(im_str!("error-window"))
                .title_bar(false)
                .always_auto_resize(true)
                .save_settings(false)
                .mouse_inputs(false)
                .no_inputs()
                .position([w as f32 / 2.0 ,(h * 5) as f32 / 6.0 ],Condition::Always)
                .position_pivot([0.5f32,0.5f32])
                .begin(&ui)
                .unwrap();

            for msg in self.error_messages.iter() {
                ui.text_colored([1.0, 1.0, 1.0, 1.0], msg);
            }
            ui.text_colored([0.0, 1.0, 0.0, 1.0], format!("Close window after {}(s)", self.error_window_eta));
            win.end(&ui);
            self.error_window_eta -= dt;
            if self.error_window_eta < 0.0{
                self.error_messages.clear();
            }
        }
        let layout_data = ui.render();
        render(&mut self.renderer,layout_data);
    }
}