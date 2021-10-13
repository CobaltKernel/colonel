use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use super::vga::{self, ColorAttrib, get_char, put_char};
pub use super::vga::Color16;


lazy_static! {
    static ref STDOUT: Mutex<TTY> = Mutex::new(TTY::new());
}



pub fn print(args: core::fmt::Arguments) {
    STDOUT.lock().write_fmt(args).expect("Yeest");
}

pub fn eprint(args: core::fmt::Arguments) {
    let mut t = STDOUT.lock();
    t.set_bg(Color16::Red);
    t.write_fmt(args).expect("Yeest");
    t.set_bg(Color16::Blue);

}

pub struct TTY {
    style: ColorAttrib,
    column: usize, row: usize,
}

impl TTY {

    pub fn new() -> Self {
        Self {
             column: 0,
             row: 0,
             style: ColorAttrib::new(0x1F),
        }
    }

    pub fn print_char(&mut self, chr: char) {
        if (' '..='~').contains(&chr) { put_char(self.column, self.row, chr, self.style).expect("Failed To Print"); self.column += 1}
        else if chr == '\n' {self.newline();}
        else if chr == '\r' {self.carriage_return();}

        
        if self.column >= vga::width() {
            self.newline();
        }
    }

    pub fn print_str(&mut self, text: &str) {
        for chr in text.chars() {
            self.print_char(chr);
        }
    }

    pub fn set_fg(&mut self, fg: Color16) {
        let color = ColorAttrib::from_color16(fg,self.style.bg());
        self.style = color;
    }

    pub fn set_bg(&mut self, bg: Color16) {
        let color = ColorAttrib::from_color16(self.style.fg(),bg);
        self.style = color;
    }

    pub fn set_row(&mut self, y: usize) {
        self.row = y;
    }

    pub fn set_column(&mut self, x: usize) {
        self.column = x;
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn style(&self) -> ColorAttrib {
        self.style
    }

    fn newline(&mut self) {
        if self.row() < vga::height() {
            self.row += 1;
            self.carriage_return();
        } else {
            self.shift_up();
        }
    }

    fn shift_up(&mut self) {
        for y in 1..vga::height() {
            for x in 0..vga::width() {
                let chr = get_char(x, y).expect("");
                put_char(x, y - 1, chr.code_point(), chr.color()).expect("");
            }
        }
        for x in 0..vga::width() {
            put_char(x, vga::height()-1, ' ', self.style()).expect("");
        }
    }

    fn carriage_return(&mut self) {
        self.set_column(0);
    }
}

impl Write for TTY {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_str(s);
        Ok(())
    }
}
