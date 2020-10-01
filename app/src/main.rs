//! Render example where each glyph pixel is output as an ascii character.
// use rusttype::{point, Font, Scale};
use ssd1305::Ssd1305;
// use std::io::Write;

fn main() {
    let screen = Ssd1305::new();
    if screen.is_none() {
        println!("Failed to create ssd1305");
        return;
    }
    let mut screen = screen.unwrap();
    screen.begin();
    screen.clean();
    let data = screen.data();
    data[0] = 1;
    data[1] = 3;
    data[2] = 7;
    data[3] = 127;
    screen.display();

    /*
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
        const FONT_PATH: &str = "/home/zaczkows/.fonts/SourceCodePro-Medium.otf";
        eprintln!("No font specified ... using {}", FONT_PATH);
        let font_data = include_bytes!("/home/zaczkows/.fonts/SourceCodePro-Medium.otf");
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
    };

    // Desired font pixel height
    let height: f32 = 16.4; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = Scale {
        x: height * 3.0,
        y: height,
    };

    // The origin of a line of text is at the baseline (roughly where
    // non-descending letters sit). We don't want to clip the text, so we shift
    // it down with an offset when laying it out. v_metrics.ascent is the
    // distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Glyphs to draw for "RustType". Feel free to try other strings.
    let glyphs: Vec<_> = font.layout("RustType", scale, offset).collect();

    // Find the most visually pleasing width to display
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;

    println!("width: {}, height: {}", width, pixel_height);

    // Rasterise directly into ASCII art.
    let mut pixel_data = vec![b'*'; width * pixel_height];
    let mapping = b"*%#x+=:-. "; // The approximation of greyscale
    let mapping_scale = (mapping.len() - 1) as f32;
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                // v should be in the range 0.0 to 1.0
                let i = (v * mapping_scale + 0.5) as usize;
                // so something's wrong if you get $ in the output.
                let c = mapping.get(i).cloned().unwrap_or(b'$');
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                // There's still a possibility that the glyph clips the boundaries of the bitmap
                if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    pixel_data[(x + y * width)] = c;
                }
            })
        }
    }

    // Print it out
    let stdout = ::std::io::stdout();
    let mut handle = stdout.lock();
    for j in 0..pixel_height {
        handle
            .write_all(&pixel_data[j * width..(j + 1) * width])
            .unwrap();
        handle.write_all(b"\n").unwrap();
    }
    */
}
