use ssd1305::{FontSettings, Ssd1305};

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
    // Desired font pixel height
    fs.height = 16.0; // to get 80 chars across (fits most terminals); adjust as desired
    fs.scale.x = fs.height * 0.9;
    fs.scale.y = fs.height;

    loop {
        screen.clear();
        let now = time::OffsetDateTime::now_local().format("%T");
        screen.text(&fs, &now);
        screen.display();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
