use crate::gpio::Gpio;

pub trait Sensor {
    type Value;
    type Error : std::fmt::Debug;
    fn init(&self, gpio: &mut Gpio) -> Result<(), Self::Error>;
    fn clear(&self, gpio: &mut Gpio) -> Result<(), Self::Error>;
    fn read(&self, gpio: &mut Gpio) -> Result<Self::Value, Self::Error>;
}
