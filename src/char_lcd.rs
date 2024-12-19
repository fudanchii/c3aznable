use esp_idf_svc::hal::{delay::Ets, gpio};
use esp_idf_svc::sys::{esp, gpio_set_level, EspError};

pub struct LcdPinMap {
    pub rs: gpio::Gpio0,
    pub en: gpio::Gpio10,

    pub d4: gpio::Gpio4,
    pub d5: gpio::Gpio5,
    pub d6: gpio::Gpio6,
    pub d7: gpio::Gpio7,
}

pub struct CharLCD {
    rs: gpio::PinDriver<'static, gpio::Gpio0, gpio::Output>,
    en: gpio::PinDriver<'static, gpio::Gpio10, gpio::Output>,

    d4: gpio::PinDriver<'static, gpio::Gpio4, gpio::Output>,
    d5: gpio::PinDriver<'static, gpio::Gpio5, gpio::Output>,
    d6: gpio::PinDriver<'static, gpio::Gpio6, gpio::Output>,
    d7: gpio::PinDriver<'static, gpio::Gpio7, gpio::Output>,
}

impl CharLCD {
    pub fn init(pins: LcdPinMap) -> Result<Self, EspError> {
        let mut slf = Self {
            rs: gpio::PinDriver::output(pins.rs)?,
            en: gpio::PinDriver::output(pins.en)?,

            d4: gpio::PinDriver::output(pins.d4)?,
            d5: gpio::PinDriver::output(pins.d5)?,
            d6: gpio::PinDriver::output(pins.d6)?,
            d7: gpio::PinDriver::output(pins.d7)?,
        };

        slf.reset()?;

        Ok(slf)
    }

    pub fn reset(&mut self) -> Result<(), EspError> {
        self.rs.set_high()?;
        self.en.set_low()?;

        self.d4.set_low()?;
        self.d5.set_low()?;
        self.d6.set_low()?;
        self.d7.set_low()?;

        Ok(())
    }
}

impl lcd::Hardware for CharLCD {
    fn rs(&mut self, state: bool) {
        let result = self.rs.set_level(state.into());

        if result.is_err() {
            println!("err:rs.{}: {}", state, result.unwrap_err());
        }
    }

    fn enable(&mut self, state: bool) {
        let result = self.en.set_level(state.into());

        if result.is_err() {
            println!("err:en.{}: {}", state, result.unwrap_err());
        }
    }

    fn data(&mut self, byte: u8) {
        let result = esp!(unsafe { gpio_set_level(self.d4.pin(), ((byte >> 0) & 1) as u32) });
        if result.is_err() {
            println!("err:d4.{}: {}", (byte >> 0) & 1, result.unwrap_err());
        }

        let result = esp!(unsafe { gpio_set_level(self.d5.pin(), ((byte >> 1) & 1) as u32) });
        if result.is_err() {
            println!("err:d5.{}: {}", (byte >> 1) & 1, result.unwrap_err());
        }

        let result = esp!(unsafe { gpio_set_level(self.d6.pin(), ((byte >> 2) & 1) as u32) });
        if result.is_err() {
            println!("err:d6.{}: {}", (byte >> 2) & 1, result.unwrap_err());
        }

        let result = esp!(unsafe { gpio_set_level(self.d7.pin(), ((byte >> 3) & 1) as u32) });
        if result.is_err() {
            println!("err:d7.{}: {}", (byte >> 3) & 1, result.unwrap_err());
        }
    }
}

impl lcd::Delay for CharLCD {
    fn delay_us(&mut self, a_bit: u32) {
        Ets::delay_us(a_bit);
    }
}
