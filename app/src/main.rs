mod renderer;

use renderer::{Params, Renderer, Scale};
use ssd1305::Ssd1305;

fn load_font(path: &str) -> Option<Box<dyn Renderer>> {
    if path.ends_with(".psf") || path.ends_with(".psf.gz") {
        let p = psfu::Font::new_from_str(path);
        if p.is_ok() {
            return Some(Box::new(p.unwrap()));
        }
    } else {
        let fs = renderer::RustTypeFont::new(path);
        if fs.is_some() {
            return Some(Box::new(fs.unwrap()));
        }
    }

    None
}

fn main() {
    let mut renderers: Vec<Box<dyn Renderer>> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        renderers.push(load_font(&"/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf").unwrap());
    } else {
        for path in args {
            if let Some(font) = load_font(&path) {
                renderers.push(font);
            }
        }
    }

    let screen = Ssd1305::new();
    if screen.is_none() {
        println!("Failed to create ssd1305");
        return;
    }
    let mut screen = screen.unwrap();
    screen.begin();

    let mut params = Params {
        scale: Scale { x: 0.0, y: 0.0 },
        height: 0,
        x: 0,
        y: 0,
    };
    loop {
        screen.clear();

        params.x = 0;
        params.y = 0;
        params.height = 12;
        params.scale.x = params.height as f32 * 0.9;
        params.scale.y = params.height as f32;
        let now = time::OffsetDateTime::now_local();
        let temp: f32 = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
            / 1000.0f32;
        let date = format!("{} | {:.1}°C", &now.format("%a,%d.%m.%Y"), &temp);
        let dims = renderers[0].render_text(&mut screen, &params, &date);
        print!("\rwidth: {}, height: {}", dims.width, dims.height);

        params.x = 23;
        params.y = dims.height as i32;
        params.height = 24;
        params.scale.x = params.height as f32 * 0.9;
        params.scale.y = params.height as f32;
        let hour = now.format("%T");
        let dims = renderers[0].render_text(&mut screen, &params, &hour);
        print!("\rwidth: {}, height: {}", dims.width, dims.height);

        screen.display();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
