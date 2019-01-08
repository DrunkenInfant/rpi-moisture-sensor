use crate::gpio::Gpio;
use crate::gpio::Error;

pub trait Sensor: Clone + Copy {
    fn init(&self, gpio: &mut Gpio) -> Result<(), Error>;
    fn clear(&self, gpio: &mut Gpio) -> Result<(), Error>;
    fn read(&self, gpio: &mut Gpio) -> Result<u32, Error>;
}
