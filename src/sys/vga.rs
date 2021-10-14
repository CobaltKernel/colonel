
use core::fmt::Display;

use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::{kmread_mut, utils::KernelResult};
use x86_64::instructions::interrupts::without_interrupts;

pub const BUFFER_WIDTH: usize =  80;
pub const BUFFER_HEIGHT: usize = 25;

const CGA_ADDR: usize = 0xB8000;


pub const CRTC_ADDR: u16 = 0x3D4;
pub const CRTC_DATA: u16 = 0x3D5;

lazy_static! {
    static ref CGA_BUFFER: Mutex<&'static mut ScreenBuffer> = Mutex::new(ScreenBuffer::get_cga());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Color16 {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorAttrib(u8);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Character {
    code_point: u8,
    color: ColorAttrib,
}

#[repr(transparent)]
struct ScreenBuffer {
    data: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}



impl Color16 {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from(value: u8) -> Self {
        match value {
             0 => Self::Black,
             1 => Self::Blue,
             2 => Self::Green,
             3 => Self::Cyan,
             4 => Self::Red,
             5 => Self::Magenta,
             6 => Self::Brown,
             7 => Self::LightGray,
             8 => Self::DarkGray,
             9 => Self::LightBlue,
            10 => Self::LightGreen,
            11 => Self::LightCyan,
            12 => Self::LightRed,
            13 => Self::Pink,
            14 => Self::Yellow,
            15 => Self::White,
            _ => panic!("Invalid Value, Must Be Between 0 & 15")
        }
    }

    pub fn from_ansi(code: u8) -> Color16 {
        match code {
            30 => Color16::Black,
            31 => Color16::Red,
            32 => Color16::Green,
            33 => Color16::Brown,
            34 => Color16::Blue,
            35 => Color16::Magenta,
            36 => Color16::Cyan,
            37 => Color16::LightGray,
            90 => Color16::DarkGray,
            91 => Color16::LightRed,
            92 => Color16::LightGreen,
            93 => Color16::Yellow,
            94 => Color16::LightBlue,
            95 => Color16::Pink,
            96 => Color16::LightCyan,
            97 => Color16::White,
            _  => Color16::Black, // Error
        }
    }}

impl ColorAttrib {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_color16(fg: Color16, bg: Color16) -> Self {
        let mut value = 0;
        value |= bg.as_u8();
        value = value << 4;
        value |= fg.as_u8();
        Self::new(value)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn fg(&self) -> Color16 {
        let fg = self.as_u8() & 0x0F;
        Color16::from(fg)
    }

    pub fn bg(&self) -> Color16 {
        let bg = (self.as_u8() & 0xF0) >> 4;
        Color16::from(bg)
    }

}

impl Display for ColorAttrib {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[fg: {:?}, bg: {:?}]", self.fg(), self.bg())
    }
}

impl Character {
    pub fn new(chr: char, color: ColorAttrib) -> Self {
        Self {
            code_point: chr as u8,
            color,
        }
    }

    pub fn color(&self) -> ColorAttrib {
        self.color
    }

    pub fn code_point(&self) -> char {
        self.code_point as char
    }
}

impl ScreenBuffer {
    pub fn get_cga() -> &'static mut ScreenBuffer {
        return unsafe { kmread_mut!(CGA_ADDR, *mut ScreenBuffer) };
    }

    pub fn put_char(&mut self, x: usize, y: usize, chr: Character) -> KernelResult<()> {
        if x >= BUFFER_WIDTH {
            return Err("X Is Out Of Range.");
        } else if y >= BUFFER_HEIGHT {
            return Err("Y Is Out Of Range.");
        } else {
            self.data[y][x].write(chr);
            return Ok(());
        }
    }

    pub fn char(&mut self, x: usize, y: usize) -> KernelResult<Character> {
        if x >= BUFFER_WIDTH {
            return Err("X Is Out Of Range.");
        } else if y >= BUFFER_HEIGHT {
            return Err("Y Is Out Of Range.");
        } else {
            return Ok(self.data[y][x].read());
        }
    }
}


pub fn put_char(x: usize, y: usize, chr: char, color: ColorAttrib) -> KernelResult<()> {
   let mut res = Err(""); 
    without_interrupts(|| {
        res = CGA_BUFFER.lock().put_char(x, y, Character::new(chr, color));
    });
    res
}

pub fn get_char(x: usize, y: usize) -> KernelResult<Character> {
    let mut res = Err("");
    without_interrupts(|| {
        res = CGA_BUFFER.lock().char(x, y);
    });
    res
}

pub fn width() -> usize {
    BUFFER_WIDTH
}

pub fn height() -> usize {
    BUFFER_HEIGHT
}