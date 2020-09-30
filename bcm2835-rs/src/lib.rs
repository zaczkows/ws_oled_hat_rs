mod bindings;

#[derive(Debug)]
pub enum PinVoltage {
    High = bindings::HIGH as isize,
    Low = bindings::LOW as isize,
}

#[derive(Debug)]
pub enum FunctionSelect {
    Inpt = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_INPT as isize,
    Outp = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_OUTP as isize,
    Alt0 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT0 as isize,
    Alt1 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT1 as isize,
    Alt2 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT2 as isize,
    Alt3 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT3 as isize,
    Alt4 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT4 as isize,
    Alt5 = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_ALT5 as isize,
    // Mask = bindings::bcm2835FunctionSelect_BCM2835_GPIO_FSEL_MASK as isize,
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

    pub fn write(&mut self, pin: u8, on: PinVoltage) {
        unsafe { bindings::bcm2835_gpio_write(pin, on as u8) }
    }

    pub fn fsel(&mut self, pin: u8, function: FunctionSelect) {
        unsafe { bindings::bcm2835_gpio_fsel(pin, function as u8) }
    }

    pub fn spi_command(&mut self, spi: &mut Bcm2835Spi, cmd: u8) {
        const DC: u8 = 24;
        self.write(DC, PinVoltage::Low);
        spi.transfer(cmd);
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
pub enum SpiBitOrder {
    LsbFirst = bindings::bcm2835SPIBitOrder_BCM2835_SPI_BIT_ORDER_LSBFIRST as isize,
    MsbFirst = bindings::bcm2835SPIBitOrder_BCM2835_SPI_BIT_ORDER_MSBFIRST as isize,
}

#[derive(Debug)]
pub enum SpiMode {
    Mode0 = bindings::bcm2835SPIMode_BCM2835_SPI_MODE0 as isize,
    Mode1 = bindings::bcm2835SPIMode_BCM2835_SPI_MODE1 as isize,
    Mode2 = bindings::bcm2835SPIMode_BCM2835_SPI_MODE2 as isize,
    Mode3 = bindings::bcm2835SPIMode_BCM2835_SPI_MODE3 as isize,
}

#[derive(Debug)]
pub enum SpiClockDivider {
    Div65k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_65536 as isize,
    Div32k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_32768 as isize,
    Div16k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_16384 as isize,
    Div8k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_8192 as isize,
    Div4k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_4096 as isize,
    Div2k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_2048 as isize,
    Div1k = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_1024 as isize,
    Div512 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_512 as isize,
    Div256 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_256 as isize,
    Div128 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_128 as isize,
    Div64 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_64 as isize,
    Div32 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_32 as isize,
    Div16 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_16 as isize,
    Div8 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_8 as isize,
    Div4 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_4 as isize,
    Div2 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_2 as isize,
    Div1 = bindings::bcm2835SPIClockDivider_BCM2835_SPI_CLOCK_DIVIDER_1 as isize,
}

#[derive(Debug)]
pub enum SpiChipSelect {
    Cs0 = bindings::bcm2835SPIChipSelect_BCM2835_SPI_CS0 as isize,
    Cs1 = bindings::bcm2835SPIChipSelect_BCM2835_SPI_CS1 as isize,
    Cs2 = bindings::bcm2835SPIChipSelect_BCM2835_SPI_CS2 as isize,
    None = bindings::bcm2835SPIChipSelect_BCM2835_SPI_CS_NONE as isize,
}

#[derive(Debug)]
pub struct Bcm2835Spi {}

impl Bcm2835Spi {
    pub fn new() -> Option<Self> {
        let success: bool = unsafe { bindings::bcm2835_spi_begin() != 0 };
        if success {
            Some(Bcm2835Spi {})
        } else {
            None
        }
    }

    pub fn transfer(&mut self, data: u8) -> u8 {
        unsafe { bindings::bcm2835_spi_transfer(data) }
    }

    pub fn transfern(&mut self, data: &mut [u8]) {
        unsafe {
            bindings::bcm2835_spi_transfern(
                data.as_mut_ptr() as *mut ::std::os::raw::c_char,
                data.len() as u32,
            )
        }
    }

    pub fn set_bit_order(&mut self, order: SpiBitOrder) {
        unsafe {
            bindings::bcm2835_spi_setBitOrder(order as u8);
        }
    }
    pub fn set_data_mode(&mut self, mode: SpiMode) {
        unsafe {
            bindings::bcm2835_spi_setDataMode(mode as u8);
        }
    }
    pub fn set_clock_divider(&mut self, divider: SpiClockDivider) {
        unsafe {
            bindings::bcm2835_spi_setClockDivider(divider as u16);
        }
    }
    pub fn chip_select(&mut self, chip_select: SpiChipSelect) {
        unsafe {
            bindings::bcm2835_spi_chipSelect(chip_select as u8);
        }
    }
    pub fn set_chip_select_polarity(
        &mut self,
        chip_select: SpiChipSelect,
        pin_voltage: PinVoltage,
    ) {
        unsafe {
            bindings::bcm2835_spi_setChipSelectPolarity(chip_select as u8, pin_voltage as u8);
        }
    }
}

impl Drop for Bcm2835Spi {
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
