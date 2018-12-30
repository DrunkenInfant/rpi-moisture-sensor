use std::time::Duration;
use crate::gpio::{Gpio, Mode, Level, PullUpDown, Error}; // as GpioError}

pub struct MoistSensor<'a> {
    pwr_pin: u8,
    val_pin: u8,
    gpio: &'a mut Gpio
}

impl<'a> Drop for MoistSensor<'a> {
    fn drop(&mut self) {
        let _ = self.gpio.clear(self.pwr_pin);
        let _ = self.gpio.set_mode(self.val_pin, Mode::Input);
        let _ = self.gpio.set_mode(self.pwr_pin, Mode::Input);
    }
}

impl<'a> MoistSensor<'a> {
    pub fn new(pwr_pin: u8, val_pin: u8, gpio: &mut Gpio) -> MoistSensor {
        MoistSensor {
            pwr_pin,
            val_pin,
            gpio
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.gpio.set_mode(self.val_pin, Mode::Input).unwrap();
        self.gpio.set_mode(self.pwr_pin, Mode::Output).unwrap();
        self.gpio.set_pullupdown(&[self.val_pin], PullUpDown::Up).unwrap();
        self.gpio.set_pullupdown(&[self.pwr_pin], PullUpDown::Off).unwrap();
        Ok(())
    }

    pub fn read(&mut self) -> Result<u32, Error> {
        self.gpio.set(self.pwr_pin)?;
        std::thread::sleep(Duration::from_millis(2));
        let res = match self.gpio.read(self.val_pin)? {
            Level::High => 1,
            Level::Low => 0
        };
        self.gpio.clear(self.pwr_pin)?;
        Ok(res)
    }
}
