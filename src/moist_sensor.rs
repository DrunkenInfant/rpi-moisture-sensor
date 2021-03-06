use std::time::Duration;
use crate::gpio::{Gpio, Mode, Level, PullUpDown, Error}; // as GpioError}
use crate::sensor::Sensor;

#[derive(Clone, Copy)]
pub struct MoistSensor {
    pwr_pin: u8,
    val_pin: u8,
    pwr_wait: u64
}

impl Sensor for MoistSensor {
    fn init(&self, gpio: &mut Gpio) -> Result<(), Error> {
        self.init(gpio)
    }

    fn read(&self, gpio: &mut Gpio) -> Result<u32, Error> {
        self.read(gpio)
    }

    fn clear(&self, gpio: &mut Gpio) -> Result<(), Error> {
        self.clear(gpio)
    }
}

impl MoistSensor {
    pub fn new(pwr_pin: u8, val_pin: u8, pwr_wait: u64) -> MoistSensor {
        MoistSensor {
            pwr_pin,
            val_pin,
            pwr_wait
        }
    }

    pub fn init(&self, gpio: &mut Gpio) -> Result<(), Error> {
        gpio.set_mode(self.val_pin, Mode::Input).unwrap();
        gpio.set_mode(self.pwr_pin, Mode::Output).unwrap();
        gpio.set_pullupdown(&[self.val_pin], PullUpDown::Up).unwrap();
        gpio.set_pullupdown(&[self.pwr_pin], PullUpDown::Off).unwrap();
        Ok(())
    }

    pub fn clear(&self, gpio: &mut Gpio) -> Result<(), Error> {
        let _ = gpio.clear(self.pwr_pin)?;
        let _ = gpio.set_mode(self.val_pin, Mode::Input)?;
        let _ = gpio.set_mode(self.pwr_pin, Mode::Input)?;
        Ok(())
    }

    pub fn read(&self, gpio: &mut Gpio) -> Result<u32, Error> {
        gpio.set(self.pwr_pin)?;
        std::thread::sleep(Duration::from_millis(self.pwr_wait));
        let res = match gpio.read(self.val_pin)? {
            Level::High => 0,
            Level::Low => 1
        };
        gpio.clear(self.pwr_pin)?;
        Ok(res)
    }
}
