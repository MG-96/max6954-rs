#[cfg(feature = "segment7-font")]
pub mod segment7font;

use enumflags2::bitflags;
use num_enum::FromPrimitive;
pub(crate) struct AsciiFont(pub(crate) u8);

impl From<HexFont> for AsciiFont {
    fn from(value: HexFont) -> Self {
        Self(value as u8)
    }
}
impl From<char> for AsciiFont {
    fn from(value: char) -> Self {
        if value.is_ascii() {
            let u32_char = u32::from(value);
            return Self(u32_char as u8);
        }
        Self(0x20)
    }
}
impl From<AsciiFont> for u8 {
    fn from(value: AsciiFont) -> Self {
        value.0
    }
}

/// 7-Segment hexadecimal decoding
///
/// Includes a [Blank](HexFont::Blank) value which blanks the digit.
/// When a digit is blanked, it also affects its pair (e.g. _D0_ also blanks _D0a_)
#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive)]
pub(crate) enum HexFont {
    Zero = 0x00,
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
    Eight = 0x08,
    Nine = 0x09,
    A = 0x0A,
    B = 0x0B,
    C = 0x0C,
    D = 0x0D,
    E = 0x0E,
    F = 0x0F,
    #[num_enum(default)]
    Blank = 0x20,
}
impl From<char> for HexFont {
    fn from(value: char) -> Self {
        if value.is_ascii_hexdigit() {
            return (value.to_digit(16).unwrap() as u8).into();
        }
        Self::Blank
    }
}

/// Segment decoding when font decoding is disabled for 7-segment digit
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[bitflags]
pub enum Segments {
    A = 0b0100_0000,
    B = 0b0010_0000,
    C = 0b0001_0000,
    D = 0b0000_1000,
    E = 0b0000_0100,
    F = 0b0000_0010,
    G = 0b0000_0001,
    DP = 0b1000_0000,
}
