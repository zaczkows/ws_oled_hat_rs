pub use rusttype::{Font, Scale};
use ssd1305::{Dims, Ssd1305};

#[derive(Debug)]
pub struct Params {
    pub scale: Scale,
    pub height: usize,
    pub x: i32,
    pub y: i32,
}

pub trait Renderer {
    /// Returns rendered text dimentions
    fn render_text(&self, data: &mut Ssd1305, params: &Params, text: &str) -> Dims;
    fn renders_text_size(&self, text_size: usize) -> bool;
}

impl Renderer for psf::Font {
    fn render_text(&self, data: &mut Ssd1305, params: &Params, text: &str) -> Dims {
        let mut x = params.x as usize;
        for t in text.chars() {
            let c = self.get_char(t).unwrap();
            'outer: for h in 0..c.height() {
                for w in 0..c.width() {
                    let x = w + x;
                    let y = h + params.y as usize;
                    if x >= data.width() || y >= data.height() {
                        break 'outer;
                    }
                    let render_pixel = c.get(w, h).unwrap() != 0;
                    data.set_pixel(x as usize, y as usize, render_pixel);
                }
            }
            x += c.width();
        }

        Dims {
            width: self.width() * text.len(),
            height: self.height(),
        }
    }

    fn renders_text_size(&self, text_size: usize) -> bool {
        self.height() == text_size
    }
}

#[derive(Debug)]
pub struct RustTypeFont<'a> {
    font: rusttype::Font<'a>,
}

impl<'a> RustTypeFont<'a> {
    pub fn new<'b>(font_path: &str) -> Option<RustTypeFont<'b>> {
        let data = std::fs::read(font_path);
        if data.is_err() {
            return None;
        }
        let data = data.unwrap();
        let font = Font::try_from_vec(data);
        if font.is_none() {
            return None;
        }

        Some(RustTypeFont {
            font: font.unwrap(),
        })
    }

    pub fn font(&self) -> &rusttype::Font<'a> {
        &self.font
    }
}

impl<'a> Renderer for RustTypeFont<'a> {
    fn render_text(&self, data: &mut Ssd1305, params: &Params, text: &str) -> Dims {
        let v_metrics = self.font.v_metrics(params.scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = self.font().layout(text, params.scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        let w = data.width() as i32;
        let h = data.height() as i32;
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x + params.x;
                    let y = y as i32 + bb.min.y + params.y;
                    if x >= w || y >= h {
                        return;
                    }
                    // v should be in the range 0.0 to 1.0
                    let render_pixel = v > 0.33;
                    data.set_pixel(x as usize, y as usize, render_pixel);
                })
            }
        }

        Dims {
            width,
            height: params.height,
        }
    }

    fn renders_text_size(&self, _: usize) -> bool {
        // Can render any size
        true
    }
}
