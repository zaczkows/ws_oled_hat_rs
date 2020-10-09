pub use rusttype::{Font, Scale};
use ssd1305::{Dims, Offset, Ssd1305};

pub trait Renderer {
    /// Returns rendered text dimentions
    fn render_text(&self, data: &mut Ssd1305, off: &Offset, text: &str) -> Dims;
    fn renders_text_size(&self, text_size: usize) -> bool;
}

impl Renderer for psfu::Font {
    fn render_text(&self, data: &mut Ssd1305, off: &Offset, text: &str) -> Dims {
        for t in text.chars() {
            let c = self.get_char(t).unwrap();
            for h in 0..c.height {
                for w in 0..c.width {
                    let r#where = h * c.width + w;
                    let x = w + off.x as usize;
                    let y = h + off.y as usize;
                    // data.buf[(x + (y / 8) * w)] |= what;
                    let render_pixel = c.d[r#where] != 0;
                    data.set_pixel(x as usize, y as usize, render_pixel);
                }
            }
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
    pub height: f32,
    pub scale: rusttype::Scale,
}

impl<'a> RustTypeFont<'a> {
    pub fn new(font_path: &str) -> Option<RustTypeFont> {
        let data = std::fs::read(&font_path);
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
            height: 0.0,
            scale: Scale::uniform(1.0),
        })
    }

    pub fn font(&self) -> &rusttype::Font<'a> {
        &self.font
    }
}

impl<'a> Renderer for RustTypeFont<'a> {
    fn render_text(&self, data: &mut Ssd1305, off: &Offset, text: &str) -> Dims {
        let v_metrics = self.font.v_metrics(self.scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = self.font().layout(text, self.scale, offset).collect();

        let pixel_height = self.height.ceil() as usize;
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
                    let x = x as i32 + bb.min.x + off.x;
                    let y = y as i32 + bb.min.y + off.y;
                    if x >= w || y >= h {
                        return;
                    }
                    // v should be in the range 0.0 to 1.0
                    let render_pixel = v > 0.33;
                    data.set_pixel(x as usize, y as usize, render_pixel);
                    // data[(x + (y / 8) * w) as usize] |= i << (y % 8);
                })
            }
        }

        Dims {
            width,
            height: pixel_height,
        }
    }

    fn renders_text_size(&self, _: usize) -> bool {
        // Can render any size
        true
    }
}
