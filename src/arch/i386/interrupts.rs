use crate::utils::KernelResult;

use super::idt::{self, InterruptHandler};

pub fn init() -> KernelResult<()> {
    idt::init();
    Ok(())
}

#[allow(unused)]
pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

#[allow(unused)]
pub fn set_handler(index: usize, handler: InterruptHandler) -> KernelResult<()> {
    if index >= idt::MAX_INTERRUPTS {return Err("Maximum Interrupt Index Is 256!")};
    idt::set_isr(index as u8, handler);
    Ok(())
}