#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(colonel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use colonel::{log, printk, size_of};


colonel::entrypoint!(main);


pub fn main() {
    log!("Size Of u8, u16: {}\r\n", size_of!(u8, u16));
}

