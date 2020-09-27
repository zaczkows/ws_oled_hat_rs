mod bindings;

use bindings::bcm2835SPIBitOrder as Bcm1835SpiBitOrder;
use bindings::bcm2835SPIChipSelect as Bcm2835SpiChipSelect;
use bindings::bcm2835SPIClockDivider as Bcm2835SpiClockDivider;
use bindings::bcm2835SPIMode as Bcm2835SpiMode;

#[derive(Debug)]
pub enum PinVoltage {
    High = bindings::HIGH as isize,
    Low = bindings::LOW as isize,
}

#[derive(Debug)]
pub struct Bcm2835Gpio {}

impl Bcm2835Gpio {
    pub fn new() -> Option<Self> {
        let success: bool = unsafe { bindings::bcm2835_init() != 0 };
        if success {
            Some(Bcm2835Gpio {})
        } else {
            None
        }
    }

    pub fn write(&mut self, pin: u8, data: u8) {
        unsafe { bindings::bcm2835_gpio_write(pin, data) }
    }

    pub fn fsel(&mut self, pin: u8, mode: u8) {
        unsafe { bindings::bcm2835_gpio_fsel(pin, mode) }
    }
}

impl Drop for Bcm2835Gpio {
    fn drop(&mut self) {
        let success: bool = unsafe { bindings::bcm2835_close() != 0 };
        if !success {
            eprintln!("Failed to close bcm2835");
        }
    }
}

#[derive(Debug)]
pub struct Bcm2835Spi<'a> {
    gpio: &'a mut Bcm2835Gpio,
}

impl<'a> Bcm2835Spi<'a> {
    pub fn new(gpio: &'a mut Bcm2835Gpio) -> Option<Self> {
        let success: bool = unsafe { bindings::bcm2835_spi_begin() == 1 };
        if success {
            Some(Bcm2835Spi { gpio })
        } else {
            None
        }
    }

    pub fn transfer(&mut self, data: u8) -> u8 {
        unsafe { bindings::bcm2835_spi_transfer(data) }
    }

    pub fn set_bit_order(&mut self, order: Bcm1835SpiBitOrder) {
        unsafe {
            bindings::bcm2835_spi_setBitOrder(order as u8);
        }
    }
    pub fn set_data_mode(&mut self, mode: Bcm2835SpiMode) {
        unsafe {
            bindings::bcm2835_spi_setDataMode(mode as u8);
        }
    }
    pub fn set_clock_divider(&mut self, divider: Bcm2835SpiClockDivider) {
        unsafe {
            bindings::bcm2835_spi_setClockDivider(divider as u16);
        }
    }
    pub fn chip_select(&mut self, chip_select: Bcm2835SpiChipSelect) {
        unsafe {
            bindings::bcm2835_spi_chipSelect(chip_select as u8);
        }
    }
    pub fn set_chip_select_polarity(
        &mut self,
        chip_select: Bcm2835SpiChipSelect,
        pin_voltage: PinVoltage,
    ) {
        unsafe {
            bindings::bcm2835_spi_setChipSelectPolarity(chip_select as u8, pin_voltage as u8);
        }
    }

    pub fn command(&mut self, cmd: u8) {
        const DC: u8 = 24;
        self.gpio.write(DC, PinVoltage::Low as u8);
        self.transfer(cmd);
    }
}

impl<'a> Drop for Bcm2835Spi<'a> {
    fn drop(&mut self) {
        unsafe { bindings::bcm2835_spi_end() }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert!(Bcm2835Gpio::new().is_none());
    }
}
