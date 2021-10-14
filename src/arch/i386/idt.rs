

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{instructions::interrupts::without_interrupts, structures::idt::InterruptDescriptorTable};
use x86_64::structures::idt::InterruptStackFrame;
use super::pics::*;


pub const HANDLER_COUNT: usize = 16;
pub const MAX_INTERRUPTS: usize = 256;

const PIC_MASTER_OFFSET: u8 = 0x20;
const PIC_SLAVE_OFFSET: u8 = PIC_MASTER_OFFSET + 8;

pub type InterruptHandler = fn();

fn default_isr() {}

pub fn init() {
    remap(
        PIC_MASTER_OFFSET, 
         PIC_SLAVE_OFFSET    
    );

    IDT.load();
}

lazy_static! {
    static ref HANDLERS: Mutex<[InterruptHandler; HANDLER_COUNT]> = Mutex::new([default_isr; HANDLER_COUNT]);

    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[interrupt_index(0) as usize].set_handler_fn(irq_0);
        idt[interrupt_index(1) as usize].set_handler_fn(irq_1);
        idt[interrupt_index(2) as usize].set_handler_fn(irq_2);
        idt[interrupt_index(3) as usize].set_handler_fn(irq_3);
        idt
    };
}



pub fn set_isr(irq: u8, handler: InterruptHandler) {
    without_interrupts(|| {
        HANDLERS.lock()[irq as usize] = handler;
    });
}

#[macro_export]
macro_rules! gen_isr {
    ($irq:expr, $f:ident) => {
        pub extern "x86-interrupt" fn $f(_stack_frame: InterruptStackFrame) {
            let handlers = HANDLERS.lock();
            handlers[$irq]();
            eoi(interrupt_index($irq));
        }
    };
}

gen_isr!(0, irq_0);
gen_isr!(1, irq_1);
gen_isr!(2, irq_2);
gen_isr!(3, irq_3);
gen_isr!(4, irq_4);
gen_isr!(5, irq_5);
gen_isr!(6, irq_6);
gen_isr!(7, irq_7);
gen_isr!(8, irq_8);
gen_isr!(9, irq_9);