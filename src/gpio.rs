///
/// GPIO features and usage documentation.
/// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
///
use std::fs::{OpenOptions};
use std::io;
use memmap::{MmapOptions, MmapMut};
use register::{FieldValue};

pub mod registers;

pub enum Mode {
    Input = 0,
    Output = 1,
    Alt0 = 4,
    Alt1 = 5,
    Alt2 = 6,
    Alt3 = 7,
    Alt4 = 3,
    Alt5 = 2,
}

impl Mode {
    pub fn as_field_value(self, pin: u8) -> FieldValue<u32, registers::GPFSEL::Register> {
        registers::pin_fsel_field(pin).val(self as u32)
    }
}

pub enum Level {
    Low = 0,
    High = 1
}

impl Level {
    pub fn is_high(&self) -> bool {
        match self {
            Level::Low => false,
            Level::High => true,
        }
    }

    pub fn is_low(&self) -> bool {
        match self {
            Level::Low => true,
            Level::High => false,
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s = match self {
            Level::Low => "Low",
            Level::High => "High"
        };
        write!(f, "{}", s)
    }
}

pub enum PullUpDown {
    Off = 0,
    Down = 1,
    Up = 2
}

pub struct Gpio {
    mmap: MmapMut,
}

#[derive(Debug)]
pub struct Error {
    pin: u8
}

pub fn validate_pin(pin: u8) -> Result<(), Error> {
    if pin > 54 {
        return Err(Error { pin })
    }
    Ok(())
}

impl Drop for Gpio {
    fn drop(&mut self) {

    }
}

impl Gpio {
    pub fn new() -> Result<Gpio, io::Error> {
        let gpio_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/gpiomem")?;

        let mmap = unsafe { MmapOptions::new().len(registers::GPIO_MEM_SIZE).map_mut(&gpio_file)? };

        Ok(Gpio {
            mmap
        })
    }

    pub fn set_mode(&mut self, pin: u8, mode: Mode) -> Result<(), Error> {
        validate_pin(pin)?;
        let regs = self.mmap.as_mut_ptr() as *mut registers::GpioRegisters;
        let field_val = mode.as_field_value(pin);

        match pin / 10  {
            1 => unsafe { (*regs).GPFSEL1.modify(field_val) },
            2 => unsafe { (*regs).GPFSEL2.modify(field_val) },
            3 => unsafe { (*regs).GPFSEL3.modify(field_val) },
            4 => unsafe { (*regs).GPFSEL4.modify(field_val) },
            5 => unsafe { (*regs).GPFSEL5.modify(field_val) },
            _ => panic!("Valid pin > 54 not possible")
        };
        Ok(())
    }

    pub fn read(&self, pin: u8) -> Result<Level, Error> {
        validate_pin(pin)?;
        let regs = self.mmap.as_ptr() as *const registers::GpioRegisters;

        let set = match pin / 32 {
            0 => unsafe { (*regs).GPLEV0.is_set(registers::pin_lev_field(pin)) },
            1 => unsafe { (*regs).GPLEV1.is_set(registers::pin_lev_field(pin)) },
            _ => panic!("Valid pin > 54 not possible")
        };

        match set {
            true => Ok(Level::High),
            false => Ok(Level::Low)
        }
    }

    pub fn set(&mut self, pin: u8) -> Result<(), Error> {
        validate_pin(pin)?;
        let regs = self.mmap.as_mut_ptr() as *mut registers::GpioRegisters;

        
        match pin / 32 {
            0 => unsafe { (*regs).GPSET0.modify(registers::pin_set_value(pin)) },
            1 => unsafe { (*regs).GPSET1.modify(registers::pin_set_value(pin)) },
            _ => panic!("Valid pin > 54 not possible")
        };

        Ok(())
    }

    pub fn clear(&mut self, pin: u8) -> Result<(), Error> {
        validate_pin(pin)?;
        let regs = self.mmap.as_mut_ptr() as *mut registers::GpioRegisters;

        match pin / 32 {
            0 => unsafe { (*regs).GPCLR0.modify(registers::pin_clear_value(pin)) },
            1 => unsafe { (*regs).GPCLR1.modify(registers::pin_clear_value(pin)) },
            _ => panic!("Valid pin > 54 not possible")
        };

        Ok(())
    }

    /// Sets actuates the pull-up or pull-down resistors of the supplied pins.
    /// This method is not safe to run simiultaneusly on the same Raspberry PI.
    /// Doing so has undefined behaviour.
    pub fn set_pullupdown(&mut self, pins: &[u8], updown: PullUpDown) -> Result<(), Error> {
        for pin in pins {
            validate_pin(*pin)?;
        }
        let regs = self.mmap.as_mut_ptr() as *mut registers::GpioRegisters;

        unsafe { (*regs).GPPUD.modify(registers::GPPUD::PUD.val(updown as u32)); }
        // Required wait of 150 cycles. 1 ms more than enough but this should only be run at
        // startup and is not time critical.
        std::thread::sleep(std::time::Duration::from_millis(1));
        for pin in pins {
            match pin / 32 {
                0 => unsafe { (*regs).GPPUDCLK0.write(registers::pin_pudclk_value(*pin)) },
                1 => unsafe { (*regs).GPPUDCLK1.write(registers::pin_pudclk_value(*pin)) },
                _ => panic!("Valid pin > 54 not possible")
            };
        }
        // Required wait of 150 cycles.
        std::thread::sleep(std::time::Duration::from_millis(1));
        unsafe { (*regs).GPPUDCLK0.set(0) };
        unsafe { (*regs).GPPUD.modify(registers::GPPUD::PUD::OFF) };

        Ok(())
    }
}
