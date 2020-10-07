pub use rusttype::{Font, Scale};

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

impl<'a> super::Renderer for RustTypeFont<'a> {
    fn render_text(&self, data: &mut super::Data, off: &super::Offset, text: &str) -> super::Dims {
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

        let w = data.dims.width as i32;
        let h = data.dims.height as i32;
        let data = &mut data.buf;
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x + off.x;
                    let y = y as i32 + bb.min.y + off.y;
                    if x >= w || y >= h {
                        return;
                    }
                    // v should be in the range 0.0 to 1.0
                    let i = if v > 0.33 { 1 } else { 0 };
                    data[(x + (y / 8) * w) as usize] |= i << (y % 8);
                })
            }
        }

        super::Dims {
            width,
            height: pixel_height,
        }
    }

    fn renders_text_size(&self, _: usize) -> bool {
        // Can render any size
        true
    }
}
