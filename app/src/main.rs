//! Render example where each glyph pixel is output as an ascii character.
use rusttype::{point, Font, Scale};
use ssd1305::Ssd1305;

fn main() {
    let screen = Ssd1305::new();
    if screen.is_none() {
        println!("Failed to create ssd1305");
        return;
    }
    let mut screen = screen.unwrap();
    screen.begin();

    let font = if let Some(font_path) = std::env::args().nth(1) {
        let font_path = std::env::current_dir().unwrap().join(font_path);
        let data = std::fs::read(&font_path).unwrap();
        Font::try_from_vec(data).unwrap_or_else(|| {
            panic!(format!(
                "error constructing a Font from data at {:?}",
                font_path
            ));
        })
    } else {
        const FONT_PATH: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
        eprintln!("No font specified ... using {}", FONT_PATH);
        let font_data = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
    };

    // Desired font pixel height
    let height: f32 = 18.0; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    // The origin of a line of text is at the baseline (roughly where
    // non-descending letters sit). We don't want to clip the text, so we shift
    // it down with an offset when laying it out. v_metrics.ascent is the
    // distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    loop {
        let now = time::OffsetDateTime::now_local();
        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<_> = font.layout(&now.format("%T"), scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        print!("width: {}, height: {}\r", width, pixel_height);

        let w = screen.width() as i32;
        let h = screen.height() as i32;
        let data = screen.data();
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= w || y >= h {
                        return;
                    }
                    // v should be in the range 0.0 to 1.0
                    let i = if v > 0.43 { 1 } else { 0 };
                    data[(x + (y / 8) * w) as usize] |= i << (y % 8);
                })
            }
        }
        screen.display();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
