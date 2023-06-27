#![no_std]
//! Platform-agnostic embedded-hal driver for the Maxim Integrated Max6954 display driver.
//!
//! ## Resources
//!
//! [Datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX6954.pdf)
//!
//! # Examples
//! ```no_run
//! todo!()
//! ```

use embedded_hal::spi::{SpiDevice, Operation};
use enumflags2::{bitflags, BitFlags};
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug)]
pub enum Error<SPI> {
    Spi(SPI),
}
pub struct Max6954<SPI> {
    spi: SPI,
}

impl<SPI: SpiDevice> Max6954<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
    pub fn destroy(self) -> SPI {
        self.spi
    }

    pub fn write_register(&mut self, address: Register, data: u8) -> Result<(), Error<SPI::Error>> {
        self.spi.transaction(&mut [
            Operation::Write(&[address as u8, data]),
            ]).map_err(Error::Spi)?;
        Ok(())
    }

    pub fn set_digit(&mut self, digit: Digit, plane: Plane, data: u8) -> Result<(), Error<SPI::Error>> {
        self.write_register(digit.register(plane), data)
    }

    /// Sets [Register::Configuration].
    pub fn set_configuration(&mut self, config: BitFlags<Configuration>) -> Result<(), Error<SPI::Error>> {
        self.write_register(Register::Configuration, config.bits_c())
    }

    /// Unblanks all registers by setting _D0P0P1-D7P0P1_ to 0x00.
    /// 
    /// Registers are set to 0b0010_0000 after startup which blanks all digits (including _D0a - D7a_)
    pub fn unblank(&mut self) -> Result<(), Error<SPI::Error>> {
        for digit in 0..8 {
            self.write_register(Digit::try_from(digit).unwrap().register(Plane::Both), 0)?;
        }
        Ok(())
    }
}

/// All available registers
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum Register {
    NoOp                    = 0x00,
    DecodeMode              = 0x01,
    /// Sets the Global intensity from 0x00-0x0F
    GlobalIntensity         = 0x02,
    ScanLimit               = 0x03,
    /// [Configuration]
    Configuration           = 0x04,
    GpioData                = 0x05,
    PortConfiguration       = 0x06,
    DisplayTest             = 0x07,
    /// __write:__ KEY_A Mask
    /// 
    /// __read:__ KEY_A Debounce
    KeyAMaskDebounce        = 0x08,
    /// __write:__ KEY_A Mask
    /// 
    /// __read:__ KEY_A Debounce
    KeyBMaskDebounce        = 0x09,
    /// __write:__ KEY_A Mask
    /// 
    /// __read:__ KEY_A Debounce
    KeyCMaskDebounce        = 0x0A,
    /// __write:__ KEY_A Mask
    /// 
    /// __read:__ KEY_A Debounce
    KeyDMaskDebounce        = 0x0B,
    /// __write:__ Digit Type
    /// 
    /// __read:__ KEY_A pressed
    KeyAPressedDigitType    = 0x0C,
    /// read only
    KeyBPressed             = 0x0D,
    /// read only
    KeyCPressed             = 0x0E,
    /// read only
    KeyDPressed             = 0x0F,
    Intensity10             = 0x10,
    Intensity32             = 0x11,
    Intensity54             = 0x12,
    Intensity76             = 0x13,
    Intensity10a            = 0x14,
    Intensity32a            = 0x15,
    Intensity54a            = 0x16,
    Intensity76a            = 0x17,
    D0P0                    = 0x20,
    D1P0                    = 0x21,
    D2P0                    = 0x22,
    D3P0                    = 0x23,
    D4P0                    = 0x24,
    D5P0                    = 0x25,
    D6P0                    = 0x26,
    D7P0                    = 0x27,
    D0aP0                   = 0x28,
    D1aP0                   = 0x29,
    D2aP0                   = 0x2A,
    D3aP0                   = 0x2B,
    D4aP0                   = 0x2C,
    D5aP0                   = 0x2D,
    D6aP0                   = 0x2E,
    D7aP0                   = 0x2F,
    D0P1                    = 0x40,
    D1P1                    = 0x41,
    D2P1                    = 0x42,
    D3P1                    = 0x43,
    D4P1                    = 0x44,
    D5P1                    = 0x45,
    D6P1                    = 0x46,
    D7P1                    = 0x47,
    D0aP1                   = 0x48,
    D1aP1                   = 0x49,
    D2aP1                   = 0x4A,
    D3aP1                   = 0x4B,
    D4aP1                   = 0x4C,
    D5aP1                   = 0x4D,
    D6aP1                   = 0x4E,
    D7aP1                   = 0x4F,
    D0P0P1                  = 0x60,
    D1P0P1                  = 0x61,
    D2P0P1                  = 0x62,
    D3P0P1                  = 0x63,
    D4P0P1                  = 0x64,
    D5P0P1                  = 0x65,
    D6P0P1                  = 0x66,
    D7P0P1                  = 0x67,
    D0aP0P1                 = 0x68,
    D1aP0P1                 = 0x69,
    D2aP0P1                 = 0x6A,
    D3aP0P1                 = 0x6B,
    D4aP0P1                 = 0x6C,
    D5aP0P1                 = 0x6D,
    D6aP0P1                 = 0x6E,
    D7aP0P1                 = 0x6F,
}

/// Addressable digits
/// 
/// (Digits _D0a - D7a_ are 7-Segment only)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum Digit {
    D0  = 0x00,
    D1  = 0x01,
    D2  = 0x02,
    D3  = 0x03,
    D4  = 0x04,
    D5  = 0x05,
    D6  = 0x06,
    D7  = 0x07,
    D0a = 0x08,
    D1a = 0x09,
    D2a = 0x0a,
    D3a = 0x0b,
    D4a = 0x0c,
    D5a = 0x0d,
    D6a = 0x0e,
    D7a = 0x0f,
}
impl Digit {
    fn register(&self, plane: Plane) -> Register {
        match (plane, self) {
            (Plane::P0, Digit::D0) => Register::D0P0,
            (Plane::P0, Digit::D1) => Register::D1P0,
            (Plane::P0, Digit::D2) => Register::D2P0,
            (Plane::P0, Digit::D3) => Register::D3P0,
            (Plane::P0, Digit::D4) => Register::D4P0,
            (Plane::P0, Digit::D5) => Register::D5P0,
            (Plane::P0, Digit::D6) => Register::D6P0,
            (Plane::P0, Digit::D7) => Register::D7P0,
            (Plane::P0, Digit::D0a) => Register::D0aP0,
            (Plane::P0, Digit::D1a) => Register::D1aP0,
            (Plane::P0, Digit::D2a) => Register::D2aP0,
            (Plane::P0, Digit::D3a) => Register::D3aP0,
            (Plane::P0, Digit::D4a) => Register::D4aP0,
            (Plane::P0, Digit::D5a) => Register::D5aP0,
            (Plane::P0, Digit::D6a) => Register::D6aP0,
            (Plane::P0, Digit::D7a) => Register::D7aP0,
            (Plane::P1, Digit::D0) => Register::D0P1,
            (Plane::P1, Digit::D1) => Register::D1P1,
            (Plane::P1, Digit::D2) => Register::D2P1,
            (Plane::P1, Digit::D3) => Register::D3P1,
            (Plane::P1, Digit::D4) => Register::D4P1,
            (Plane::P1, Digit::D5) => Register::D5P1,
            (Plane::P1, Digit::D6) => Register::D6P1,
            (Plane::P1, Digit::D7) => Register::D7P1,
            (Plane::P1, Digit::D0a) => Register::D0aP1,
            (Plane::P1, Digit::D1a) => Register::D1aP1,
            (Plane::P1, Digit::D2a) => Register::D2aP1,
            (Plane::P1, Digit::D3a) => Register::D3aP1,
            (Plane::P1, Digit::D4a) => Register::D4aP1,
            (Plane::P1, Digit::D5a) => Register::D5aP1,
            (Plane::P1, Digit::D6a) => Register::D6aP1,
            (Plane::P1, Digit::D7a) => Register::D7aP1,
            (Plane::Both, Digit::D0) => Register::D0P0P1,
            (Plane::Both, Digit::D1) => Register::D1P0P1,
            (Plane::Both, Digit::D2) => Register::D2P0P1,
            (Plane::Both, Digit::D3) => Register::D3P0P1,
            (Plane::Both, Digit::D4) => Register::D4P0P1,
            (Plane::Both, Digit::D5) => Register::D5P0P1,
            (Plane::Both, Digit::D6) => Register::D6P0P1,
            (Plane::Both, Digit::D7) => Register::D7P0P1,
            (Plane::Both, Digit::D0a) => Register::D0aP0P1,
            (Plane::Both, Digit::D1a) => Register::D1aP0P1,
            (Plane::Both, Digit::D2a) => Register::D2aP0P1,
            (Plane::Both, Digit::D3a) => Register::D3aP0P1,
            (Plane::Both, Digit::D4a) => Register::D4aP0P1,
            (Plane::Both, Digit::D5a) => Register::D5aP0P1,
            (Plane::Both, Digit::D6a) => Register::D6aP0P1,
            (Plane::Both, Digit::D7a) => Register::D7aP0P1,
        }
    }
}

/// Data planes
/// 
/// Each digit has a corresponding register in both planes.
/// A digit's data can be updated in plane P0, plane P1, or both planes at the same time.
/// When enabled by configuring [Configuration::E], the Display switches between plane 0 and plane 1
pub enum Plane {
    P0,
    P1,
    Both,
}

/// Configuration Register
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Configuration {
    /// Returns the current phase of the blink timing (read only - write is ignored) (P)
    /// 
    /// P1 Blink Phase = 0
    /// 
    /// P0 Blink Phase = 1
    BlinkPhase = 0b1000_0000,
    /// Global or digit-by-digit intensity (I)
    GlobalIntensity = 0b0100_0000,
    /// Clear digit data for both planes P0 and P1 (transient bit) (R)
    ClearData = 0b0010_0000,
    /// Reset blink timing on CS falling edge (transient bit) (T)
    BlinkSync = 0b0001_0000,
    /// Enable blink function globally (E)
    GlobalBlink = 0b0000_1000,
    /// Blink rate (0.5Hz when set, default 1Hz) (B)
    /// 
    /// Depends on the oscillator frequency.
    /// Values given are for 4MHz.
    BlinkRate = 0b0000_0100,
    /// Disables shutdown when set (S)
    NotShutdown = 0b0000_0001,
    // X = 0b0000_0010, // Not used
}
