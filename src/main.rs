#![no_std]
#![no_main]

use colonel::{eprintk, printk, sys::{tty::{self, TTY}, vga}};
use vga::{ColorAttrib, Color16};

colonel::entrypoint!(main);


pub fn main() {
}