use core::fmt::Display;

use crate::utils::KString;

use super::vga::Color16;

pub const RESET: KString = "\x1b[0m";
pub const BG_RED: KString = "\x1b[44m";
pub const BG_BLUE: KString = "\x1b[41m";
pub const FG_GREEN: KString = "\x1b[32m";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ColorFg {
    Reset = 0,
    Black = 30,
    Blue = 31,
    Green = 32,
    Cyan = 33,
    Red = 34,
    Magenta = 35,
    Brown = 36,
    LightGray = 37,
    DarkGray = 90,
    LightBlue = 91,
    LightGreen = 92,
    LightCyan = 93,
    LightRed = 94,
    Pink = 95,
    Yellow = 96,
    White = 97,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ColorBg {
    Reset = 0,
    Black = 40,
    Blue = 41,
    Green = 42,
    Cyan = 43,
    Red = 44,
    Magenta = 45,
    Brown = 46,
    LightGray = 47,
    DarkGray = 100,
    LightBlue = 101,
    LightGreen = 102,
    LightCyan = 103,
    LightRed = 104,
    Pink = 105,
    Yellow = 106,
    White = 107,
}

impl ColorFg {
    pub fn as_color16(&self) -> Color16 {
        Color16::from_ansi(*self as u8)
    }
}

impl Display for ColorFg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\x1b[{}m", self)
    }
}

impl ColorBg {
    pub fn as_color16(&self) -> Color16 {
        Color16::from_ansi(*self as u8)
    }
}

impl Display for ColorBg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\x1b[{}m", self)
    }
}


