use ssd1305::{FontSettings, Offset, Ssd1305};

fn main() {
    let screen = Ssd1305::new();
    if screen.is_none() {
        println!("Failed to create ssd1305");
        return;
    }
    let mut screen = screen.unwrap();
    screen.begin();

    let path = if let Some(font_path) = std::env::args().nth(1) {
        font_path
    } else {
        String::from("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf")
    };

    let fs = FontSettings::new(&path);
    let mut fs = fs.unwrap();

    let mut offset = Offset { x: 0, y: 0 };
    loop {
        screen.clear();

        offset.x = 0;
        offset.y = 0;
        fs.height = 12.0;
        fs.scale.x = fs.height * 0.9;
        fs.scale.y = fs.height;
        let now = time::OffsetDateTime::now_local();
        let temp: f32 = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap()
            / 1000.0f32;
        let date = format!("{} | {:.1}°C", &now.format("%a,%d/%m/%Y"), &temp);
        let dims = screen.text(&fs, &offset, &date);
        print!("\rwidth: {}, height: {}", dims.0, dims.1);

        offset.x = 23;
        offset.y = dims.1 as i32;
        fs.height = 24.0;
        fs.scale.x = fs.height * 0.9;
        fs.scale.y = fs.height;
        let hour = now.format("%T");
        let dims = screen.text(&fs, &offset, &hour);
        print!("\rwidth: {}, height: {}", dims.0, dims.1);

        screen.display();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
