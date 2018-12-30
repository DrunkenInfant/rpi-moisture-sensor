pub const GPIO_MEM_SIZE: usize = 168;

use register::{mmio::ReadOnly, mmio::ReadWrite, Field, FieldValue, register_bitfields};

// Descriptions taken from
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    /// Reserved
    RESERVED [ RESERVED 0 ],
    /// GPIO Function Select
    GPFSEL [
        FSEL9 OFFSET(27) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL8 OFFSET(24) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL7 OFFSET(21) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL6 OFFSET(18) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL5 OFFSET(15) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL4 OFFSET(12) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL3 OFFSET(9) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL2 OFFSET(6) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL1 OFFSET(3) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ],
        FSEL0 OFFSET(0) NUMBITS(3) [
            IN = 0b000,
            OUT = 0b001,
            ALT0 = 0b100,
            ALT1 = 0b101,
            ALT2 = 0b110,
            ALT3 = 0b111,
            ALT4 = 0b011,
            ALT5 = 0b010
        ]
    ],
    GPSET [
        SET31 OFFSET(31) NUMBITS(1),
        SET30 OFFSET(30) NUMBITS(1),
        SET29 OFFSET(29) NUMBITS(1),
        SET28 OFFSET(28) NUMBITS(1),
        SET27 OFFSET(27) NUMBITS(1),
        SET26 OFFSET(26) NUMBITS(1),
        SET25 OFFSET(25) NUMBITS(1),
        SET24 OFFSET(24) NUMBITS(1),
        SET23 OFFSET(23) NUMBITS(1),
        SET22 OFFSET(22) NUMBITS(1),
        SET21 OFFSET(21) NUMBITS(1),
        SET20 OFFSET(20) NUMBITS(1),
        SET19 OFFSET(19) NUMBITS(1),
        SET18 OFFSET(18) NUMBITS(1),
        SET17 OFFSET(17) NUMBITS(1),
        SET16 OFFSET(16) NUMBITS(1),
        SET15 OFFSET(15) NUMBITS(1),
        SET14 OFFSET(14) NUMBITS(1),
        SET13 OFFSET(13) NUMBITS(1),
        SET12 OFFSET(12) NUMBITS(1),
        SET11 OFFSET(11) NUMBITS(1),
        SET10 OFFSET(10) NUMBITS(1),
        SET9 OFFSET(9) NUMBITS(1),
        SET8 OFFSET(8) NUMBITS(1),
        SET7 OFFSET(7) NUMBITS(1),
        SET6 OFFSET(6) NUMBITS(1),
        SET5 OFFSET(5) NUMBITS(1),
        SET4 OFFSET(4) NUMBITS(1),
        SET3 OFFSET(3) NUMBITS(1),
        SET2 OFFSET(2) NUMBITS(1),
        SET1 OFFSET(1) NUMBITS(1),
        SET0 OFFSET(0) NUMBITS(1)
    ],
    GPCLR [
        CLR31 OFFSET(31) NUMBITS(1),
        CLR30 OFFSET(30) NUMBITS(1),
        CLR29 OFFSET(29) NUMBITS(1),
        CLR28 OFFSET(28) NUMBITS(1),
        CLR27 OFFSET(27) NUMBITS(1),
        CLR26 OFFSET(26) NUMBITS(1),
        CLR25 OFFSET(25) NUMBITS(1),
        CLR24 OFFSET(24) NUMBITS(1),
        CLR23 OFFSET(23) NUMBITS(1),
        CLR22 OFFSET(22) NUMBITS(1),
        CLR21 OFFSET(21) NUMBITS(1),
        CLR20 OFFSET(20) NUMBITS(1),
        CLR19 OFFSET(19) NUMBITS(1),
        CLR18 OFFSET(18) NUMBITS(1),
        CLR17 OFFSET(17) NUMBITS(1),
        CLR16 OFFSET(16) NUMBITS(1),
        CLR15 OFFSET(15) NUMBITS(1),
        CLR14 OFFSET(14) NUMBITS(1),
        CLR13 OFFSET(13) NUMBITS(1),
        CLR12 OFFSET(12) NUMBITS(1),
        CLR11 OFFSET(11) NUMBITS(1),
        CLR10 OFFSET(10) NUMBITS(1),
        CLR9 OFFSET(9) NUMBITS(1),
        CLR8 OFFSET(8) NUMBITS(1),
        CLR7 OFFSET(7) NUMBITS(1),
        CLR6 OFFSET(6) NUMBITS(1),
        CLR5 OFFSET(5) NUMBITS(1),
        CLR4 OFFSET(4) NUMBITS(1),
        CLR3 OFFSET(3) NUMBITS(1),
        CLR2 OFFSET(2) NUMBITS(1),
        CLR1 OFFSET(1) NUMBITS(1),
        CLR0 OFFSET(0) NUMBITS(1)
    ],
    GPLEV [
        LEV31 OFFSET(31) NUMBITS(1),
        LEV30 OFFSET(30) NUMBITS(1),
        LEV29 OFFSET(29) NUMBITS(1),
        LEV28 OFFSET(28) NUMBITS(1),
        LEV27 OFFSET(27) NUMBITS(1),
        LEV26 OFFSET(26) NUMBITS(1),
        LEV25 OFFSET(25) NUMBITS(1),
        LEV24 OFFSET(24) NUMBITS(1),
        LEV23 OFFSET(23) NUMBITS(1),
        LEV22 OFFSET(22) NUMBITS(1),
        LEV21 OFFSET(21) NUMBITS(1),
        LEV20 OFFSET(20) NUMBITS(1),
        LEV19 OFFSET(19) NUMBITS(1),
        LEV18 OFFSET(18) NUMBITS(1),
        LEV17 OFFSET(17) NUMBITS(1),
        LEV16 OFFSET(16) NUMBITS(1),
        LEV15 OFFSET(15) NUMBITS(1),
        LEV14 OFFSET(14) NUMBITS(1),
        LEV13 OFFSET(13) NUMBITS(1),
        LEV12 OFFSET(12) NUMBITS(1),
        LEV11 OFFSET(11) NUMBITS(1),
        LEV10 OFFSET(10) NUMBITS(1),
        LEV9 OFFSET(9) NUMBITS(1),
        LEV8 OFFSET(8) NUMBITS(1),
        LEV7 OFFSET(7) NUMBITS(1),
        LEV6 OFFSET(6) NUMBITS(1),
        LEV5 OFFSET(5) NUMBITS(1),
        LEV4 OFFSET(4) NUMBITS(1),
        LEV3 OFFSET(3) NUMBITS(1),
        LEV2 OFFSET(2) NUMBITS(1),
        LEV1 OFFSET(1) NUMBITS(1),
        LEV0 OFFSET(0) NUMBITS(1)
    ],
    GPEDS [ RESERVED 0 ],
    GPREN [ RESERVED 0 ],
    GPFEN [ RESERVED 0 ],
    GPHEN [ RESERVED 0 ],
    GPLEN [ RESERVED 0 ],
    GPAREN [ RESERVED 0 ],
    GPAFEN [ RESERVED 0 ],
    GPPUD [
        PUD OFFSET(0) NUMBITS(2) [
            OFF = 0b00,
            Down = 0b01,
            Up = 0b10
        ]
    ],
    GPPUDCLK [
        PUDCLK31 OFFSET(31) NUMBITS(1),
        PUDCLK30 OFFSET(30) NUMBITS(1),
        PUDCLK29 OFFSET(29) NUMBITS(1),
        PUDCLK28 OFFSET(28) NUMBITS(1),
        PUDCLK27 OFFSET(27) NUMBITS(1),
        PUDCLK26 OFFSET(26) NUMBITS(1),
        PUDCLK25 OFFSET(25) NUMBITS(1),
        PUDCLK24 OFFSET(24) NUMBITS(1),
        PUDCLK23 OFFSET(23) NUMBITS(1),
        PUDCLK22 OFFSET(22) NUMBITS(1),
        PUDCLK21 OFFSET(21) NUMBITS(1),
        PUDCLK20 OFFSET(20) NUMBITS(1),
        PUDCLK19 OFFSET(19) NUMBITS(1),
        PUDCLK18 OFFSET(18) NUMBITS(1),
        PUDCLK17 OFFSET(17) NUMBITS(1),
        PUDCLK16 OFFSET(16) NUMBITS(1),
        PUDCLK15 OFFSET(15) NUMBITS(1),
        PUDCLK14 OFFSET(14) NUMBITS(1),
        PUDCLK13 OFFSET(13) NUMBITS(1),
        PUDCLK12 OFFSET(12) NUMBITS(1),
        PUDCLK11 OFFSET(11) NUMBITS(1),
        PUDCLK10 OFFSET(10) NUMBITS(1),
        PUDCLK9 OFFSET(9) NUMBITS(1),
        PUDCLK8 OFFSET(8) NUMBITS(1),
        PUDCLK7 OFFSET(7) NUMBITS(1),
        PUDCLK6 OFFSET(6) NUMBITS(1),
        PUDCLK5 OFFSET(5) NUMBITS(1),
        PUDCLK4 OFFSET(4) NUMBITS(1),
        PUDCLK3 OFFSET(3) NUMBITS(1),
        PUDCLK2 OFFSET(2) NUMBITS(1),
        PUDCLK1 OFFSET(1) NUMBITS(1),
        PUDCLK0 OFFSET(0) NUMBITS(1)
    ],
    TEST [ TEST 0 ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct GpioRegisters {
    pub GPFSEL0: ReadWrite<u32, GPFSEL::Register>, // 0x00
    pub GPFSEL1: ReadWrite<u32, GPFSEL::Register>, // 0x04
    pub GPFSEL2: ReadWrite<u32, GPFSEL::Register>, // 0x08
    pub GPFSEL3: ReadWrite<u32, GPFSEL::Register>, // 0x08
    pub GPFSEL4: ReadWrite<u32, GPFSEL::Register>, // 0x08
    pub GPFSEL5: ReadWrite<u32, GPFSEL::Register>, // 0x08
    pub RESERVED00: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPSET0: ReadWrite<u32, GPSET::Register>, // 0x08
    pub GPSET1: ReadWrite<u32, GPSET::Register>, // 0x08
    pub RESERVED01: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPCLR0: ReadWrite<u32, GPCLR::Register>, // 0x08
    pub GPCLR1: ReadWrite<u32, GPCLR::Register>, // 0x08
    pub RESERVED02: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPLEV0: ReadWrite<u32, GPLEV::Register>, // 0x08
    pub GPLEV1: ReadWrite<u32, GPLEV::Register>, // 0x08
    pub RESERVED03: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPEDS0: ReadWrite<u32, GPEDS::Register>, // 0x08
    pub GPEDS1: ReadWrite<u32, GPEDS::Register>, // 0x08
    pub RESERVED04: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPREN0: ReadWrite<u32, GPREN::Register>, // 0x08
    pub GPREN1: ReadWrite<u32, GPREN::Register>, // 0x08
    pub RESERVED05: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPFEN0: ReadWrite<u32, GPFEN::Register>, // 0x08
    pub GPFEN1: ReadWrite<u32, GPFEN::Register>, // 0x08
    pub RESERVED06: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPHEN0: ReadWrite<u32, GPHEN::Register>, // 0x08
    pub GPHEN1: ReadWrite<u32, GPHEN::Register>, // 0x08
    pub RESERVED07: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPLEN0: ReadWrite<u32, GPLEN::Register>, // 0x08
    pub GPLEN1: ReadWrite<u32, GPLEN::Register>, // 0x08
    pub RESERVED08: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPAREN0: ReadWrite<u32, GPAREN::Register>, // 0x08
    pub GPAREN1: ReadWrite<u32, GPAREN::Register>, // 0x08
    pub RESERVED09: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPAFEN0: ReadWrite<u32, GPAFEN::Register>, // 0x08
    pub GPAFEN1: ReadWrite<u32, GPAFEN::Register>, // 0x08
    pub RESERVED10: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub GPPUD: ReadWrite<u32, GPPUD::Register>, // 0x08
    pub GPPUDCLK0: ReadWrite<u32, GPPUDCLK::Register>, // 0x08
    pub GPPUDCLK1: ReadWrite<u32, GPPUDCLK::Register>, // 0x08
    pub RESERVED11: ReadOnly<u32, RESERVED::Register>, // 0x08
    pub TEST: ReadOnly<u32, TEST::Register>, // 0x08
}

pub fn pin_fsel_field(pin: u8) -> Field<u32, GPFSEL::Register> {
    match pin % 10 {
        0 => GPFSEL::FSEL0,
        1 => GPFSEL::FSEL1,
        2 => GPFSEL::FSEL2,
        3 => GPFSEL::FSEL3,
        4 => GPFSEL::FSEL4,
        5 => GPFSEL::FSEL5,
        6 => GPFSEL::FSEL6,
        7 => GPFSEL::FSEL7,
        8 => GPFSEL::FSEL8,
        9 => GPFSEL::FSEL9,
        v => panic!("ERROR: u8 % 10 = {}", v)
    }
}

pub fn pin_set_value(pin: u8) -> FieldValue<u32, GPSET::Register> {
    match pin % 32 {
        0 => GPSET::SET0::SET,
        1 => GPSET::SET1::SET,
        2 => GPSET::SET2::SET,
        3 => GPSET::SET3::SET,
        4 => GPSET::SET4::SET,
        5 => GPSET::SET5::SET,
        6 => GPSET::SET6::SET,
        7 => GPSET::SET7::SET,
        8 => GPSET::SET8::SET,
        9 => GPSET::SET9::SET,
        10 => GPSET::SET10::SET,
        11 => GPSET::SET11::SET,
        12 => GPSET::SET12::SET,
        13 => GPSET::SET13::SET,
        14 => GPSET::SET14::SET,
        15 => GPSET::SET15::SET,
        16 => GPSET::SET16::SET,
        17 => GPSET::SET17::SET,
        18 => GPSET::SET18::SET,
        19 => GPSET::SET19::SET,
        20 => GPSET::SET20::SET,
        21 => GPSET::SET21::SET,
        22 => GPSET::SET22::SET,
        23 => GPSET::SET23::SET,
        24 => GPSET::SET24::SET,
        25 => GPSET::SET25::SET,
        26 => GPSET::SET26::SET,
        27 => GPSET::SET27::SET,
        28 => GPSET::SET28::SET,
        29 => GPSET::SET29::SET,
        30 => GPSET::SET30::SET,
        31 => GPSET::SET31::SET,
        v => panic!("u8 % 32 = {}", v)
    }
}

pub fn pin_clear_value(pin: u8) -> FieldValue<u32, GPCLR::Register> {
    match pin % 32 {
        0 => GPCLR::CLR0::SET,
        1 => GPCLR::CLR1::SET,
        2 => GPCLR::CLR2::SET,
        3 => GPCLR::CLR3::SET,
        4 => GPCLR::CLR4::SET,
        5 => GPCLR::CLR5::SET,
        6 => GPCLR::CLR6::SET,
        7 => GPCLR::CLR7::SET,
        8 => GPCLR::CLR8::SET,
        9 => GPCLR::CLR9::SET,
        10 => GPCLR::CLR10::SET,
        11 => GPCLR::CLR11::SET,
        12 => GPCLR::CLR12::SET,
        13 => GPCLR::CLR13::SET,
        14 => GPCLR::CLR14::SET,
        15 => GPCLR::CLR15::SET,
        16 => GPCLR::CLR16::SET,
        17 => GPCLR::CLR17::SET,
        18 => GPCLR::CLR18::SET,
        19 => GPCLR::CLR19::SET,
        20 => GPCLR::CLR20::SET,
        21 => GPCLR::CLR21::SET,
        22 => GPCLR::CLR22::SET,
        23 => GPCLR::CLR23::SET,
        24 => GPCLR::CLR24::SET,
        25 => GPCLR::CLR25::SET,
        26 => GPCLR::CLR26::SET,
        27 => GPCLR::CLR27::SET,
        28 => GPCLR::CLR28::SET,
        29 => GPCLR::CLR29::SET,
        30 => GPCLR::CLR30::SET,
        31 => GPCLR::CLR31::SET,
        v => panic!("u8 % 32 = {}", v)
    }
}

pub fn pin_lev_field(pin: u8) -> Field<u32, GPLEV::Register> {
    match pin % 32 {
        0 => GPLEV::LEV0,
        1 => GPLEV::LEV1,
        2 => GPLEV::LEV2,
        3 => GPLEV::LEV3,
        4 => GPLEV::LEV4,
        5 => GPLEV::LEV5,
        6 => GPLEV::LEV6,
        7 => GPLEV::LEV7,
        8 => GPLEV::LEV8,
        9 => GPLEV::LEV9,
        10 => GPLEV::LEV10,
        11 => GPLEV::LEV11,
        12 => GPLEV::LEV12,
        13 => GPLEV::LEV13,
        14 => GPLEV::LEV14,
        15 => GPLEV::LEV15,
        16 => GPLEV::LEV16,
        17 => GPLEV::LEV17,
        18 => GPLEV::LEV18,
        19 => GPLEV::LEV19,
        20 => GPLEV::LEV20,
        21 => GPLEV::LEV21,
        22 => GPLEV::LEV22,
        23 => GPLEV::LEV23,
        24 => GPLEV::LEV24,
        25 => GPLEV::LEV25,
        26 => GPLEV::LEV26,
        27 => GPLEV::LEV27,
        28 => GPLEV::LEV28,
        29 => GPLEV::LEV29,
        30 => GPLEV::LEV30,
        31 => GPLEV::LEV31,
        v => panic!("u8 % 32 = {}", v)
    }
}

pub fn pin_pudclk_value(pin: u8) -> FieldValue<u32, GPPUDCLK::Register> {
    match pin % 32 {
        0 => GPPUDCLK::PUDCLK0::SET,
        1 => GPPUDCLK::PUDCLK1::SET,
        2 => GPPUDCLK::PUDCLK2::SET,
        3 => GPPUDCLK::PUDCLK3::SET,
        4 => GPPUDCLK::PUDCLK4::SET,
        5 => GPPUDCLK::PUDCLK5::SET,
        6 => GPPUDCLK::PUDCLK6::SET,
        7 => GPPUDCLK::PUDCLK7::SET,
        8 => GPPUDCLK::PUDCLK8::SET,
        9 => GPPUDCLK::PUDCLK9::SET,
        10 => GPPUDCLK::PUDCLK10::SET,
        11 => GPPUDCLK::PUDCLK11::SET,
        12 => GPPUDCLK::PUDCLK12::SET,
        13 => GPPUDCLK::PUDCLK13::SET,
        14 => GPPUDCLK::PUDCLK14::SET,
        15 => GPPUDCLK::PUDCLK15::SET,
        16 => GPPUDCLK::PUDCLK16::SET,
        17 => GPPUDCLK::PUDCLK17::SET,
        18 => GPPUDCLK::PUDCLK18::SET,
        19 => GPPUDCLK::PUDCLK19::SET,
        20 => GPPUDCLK::PUDCLK20::SET,
        21 => GPPUDCLK::PUDCLK21::SET,
        22 => GPPUDCLK::PUDCLK22::SET,
        23 => GPPUDCLK::PUDCLK23::SET,
        24 => GPPUDCLK::PUDCLK24::SET,
        25 => GPPUDCLK::PUDCLK25::SET,
        26 => GPPUDCLK::PUDCLK26::SET,
        27 => GPPUDCLK::PUDCLK27::SET,
        28 => GPPUDCLK::PUDCLK28::SET,
        29 => GPPUDCLK::PUDCLK29::SET,
        30 => GPPUDCLK::PUDCLK30::SET,
        31 => GPPUDCLK::PUDCLK31::SET,
        v => panic!("u8 % 32 = {}", v)
    }
}
