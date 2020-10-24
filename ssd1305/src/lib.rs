#[derive(Debug)]
pub struct Ssd1305 {
    gpio: bcm2835_rs::Bcm2835Gpio,
    spi: Option<bcm2835_rs::Bcm2835Spi>,
    buffer: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Dims {
    pub width: usize,
    pub height: usize,
}

impl Ssd1305 {
    const RST: u8 = 25;
    const DC: u8 = 24;
    const PAGES: usize = 4;
    const WIDTH: usize = 128;
    const HEIGHT: usize = 8 * 4; // 8 pixels * 4 rows

    pub fn new() -> Option<Self> {
        let gpio = bcm2835_rs::Bcm2835Gpio::new();
        if let Some(gpio) = gpio {
            let mut s = Ssd1305 {
                gpio,
                spi: None,
                buffer: Vec::with_capacity(Ssd1305::PAGES * Ssd1305::WIDTH),
            };
            s.buffer.resize(Ssd1305::PAGES * Ssd1305::WIDTH, 0);
            Some(s)
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        Ssd1305::WIDTH
    }

    pub fn height(&self) -> usize {
        Ssd1305::HEIGHT
    }

    pub fn begin(&mut self) {
        self.gpio.fsel(Ssd1305::RST, bcm2835_rs::FunctionSelect::Outp);
        self.gpio.fsel(Ssd1305::DC, bcm2835_rs::FunctionSelect::Outp);

        self.spi = bcm2835_rs::Bcm2835Spi::new();
        if self.spi.is_none() {
            panic!("Failed to acquire SPI iface");
        }

        let spi = self.spi.as_mut().unwrap();
        spi.set_bit_order(bcm2835_rs::SpiBitOrder::MsbFirst); // The default
        spi.set_data_mode(bcm2835_rs::SpiMode::Mode0); // The default
        spi.set_clock_divider(bcm2835_rs::SpiClockDivider::Div2k); // The default
        spi.chip_select(bcm2835_rs::SpiChipSelect::Cs0); // The default
        spi.set_chip_select_polarity(bcm2835_rs::SpiChipSelect::Cs0, bcm2835_rs::PinVoltage::Low);
        // the default

        self.gpio.write(Ssd1305::RST, bcm2835_rs::PinVoltage::High);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.gpio.write(Ssd1305::RST, bcm2835_rs::PinVoltage::Low);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.gpio.write(Ssd1305::RST, bcm2835_rs::PinVoltage::High);

        self.gpio.spi_command(spi, 0xAE); //--turn off oled panel
        self.gpio.spi_command(spi, 0x04); //--turn off oled panel
        self.gpio.spi_command(spi, 0x10); //--turn off oled panel
        self.gpio.spi_command(spi, 0x40); //---set low column address
        self.gpio.spi_command(spi, 0x81); //---set high column address
        self.gpio.spi_command(spi, 0x80); //--set start line address  Set Mapping RAM Display Start Line
                                          //(0x00~0x3F)
        self.gpio.spi_command(spi, 0xA1); //--set contrast control register
        self.gpio.spi_command(spi, 0xA6); // Set SEG Output Current Brightness
        self.gpio.spi_command(spi, 0xA8); //--Set SEG/Column Mapping     0xa0×óÓÒ·´ÖÃ 0xa1Õý³£
        self.gpio.spi_command(spi, 0x1F); // Set COM/Row Scan Direction   0xc0ÉÏÏÂ·´ÖÃ 0xc8Õý³£
        self.gpio.spi_command(spi, 0xC8); //--set normal display
        self.gpio.spi_command(spi, 0xD3); //--set multiplex ratio(1 to 64)
        self.gpio.spi_command(spi, 0x00); //--1/64 duty
        self.gpio.spi_command(spi, 0xD5); //-set display offset Shift Mapping RAM Counter (0x00~0x3F)
        self.gpio.spi_command(spi, 0xF0); //-not offset
        self.gpio.spi_command(spi, 0xd8); //--set display clock divide ratio/oscillator frequency
        self.gpio.spi_command(spi, 0x05); //--set divide ratio, Set Clock as 100 Frames/Sec
        self.gpio.spi_command(spi, 0xD9); //--set pre-charge period
        self.gpio.spi_command(spi, 0xC2); // Set Pre-Charge as 15 Clocks & Discharge as 1 Clock
        self.gpio.spi_command(spi, 0xDA); //--set com pins hardware configuration
        self.gpio.spi_command(spi, 0x12);
        self.gpio.spi_command(spi, 0xDB); //--set vcomh
        self.gpio.spi_command(spi, 0x08); // Set VCOM Deselect Level
        self.gpio.spi_command(spi, 0xAF); //-Set Page Addressing Mode (0x00/0x01/0x02)
    }

    pub fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }

    pub fn display(&mut self) {
        let spi = self.spi.as_mut().expect("SPI not set!");
        for page in 0..Ssd1305::PAGES {
            // Set page address
            self.gpio.spi_command(spi, 0xB0 + page as u8);
            // set low column address
            self.gpio.spi_command(spi, 0x04);
            /* set high column address */
            self.gpio.spi_command(spi, 0x10);
            /* write data */
            self.gpio.write(Ssd1305::DC, bcm2835_rs::PinVoltage::High);
            spi.transfern(&mut self.buffer[page * Ssd1305::WIDTH..(page + 1) * Ssd1305::WIDTH]);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, is_set: bool) -> bool {
        if x > self.width() || y > self.height() {
            return false;
        }
        let w = self.width();
        if is_set {
            self.buffer[(x + (y / 8) * w)] |= 1 << (y % 8);
        } else {
            self.buffer[(x + (y / 8) * w)] &= !(1 << (y % 8));
        }
        true
    }
}

impl Drop for Ssd1305 {
    fn drop(&mut self) {
        self.spi = None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
