//! Manages The Two 8259 PIC Chips
//! Ported From: https://wiki.osdev.org/PIC

use spin::Mutex;

use crate::{outb, inb};

static PIC1: Mutex<u8> = Mutex::new(0);
static PIC2: Mutex<u8> = Mutex::new(0);

const MASTER_COMM: u16 = 0x20;
const MASTER_DATA: u16 = 0x21;

const SLAVE_COMM: u16 = 0xA0;
const SLAVE_DATA: u16 = 0xA1;

const SLAVE_IRQ:     u8 = 0x02;
const SLAVE_CASCADE: u8 = 0x04;

const EOI: u8 = 0x20;


const ICW1_ICW4:        u8 = 0x01;
const ICW1_SINGLE:      u8 = 0x02;
const ICW1_INTERVAL4:   u8 = 0x04;
const ICW1_LEVEL:       u8 = 0x08;
const ICW1_INIT:        u8 = 0x10;

const ICW4_8086:        u8 = 0x01;


const OCW3_IRR: u8 = 0x0A;
const OCW3_ISR: u8 = 0x0B;

pub fn interrupt_index(irq: u8) -> u8 {
    let master = {*PIC1.lock()};
    master + irq
}

pub fn eoi(irq: u8) {
    if irq >= 8 {
        outb!(SLAVE_COMM, EOI);
    }

    outb!(MASTER_COMM, EOI);
}

pub fn remap(master: u8, slave: u8) {
    let master_mask = inb!(MASTER_DATA);
    let slave_mask = inb!(SLAVE_DATA);


    // Start Initializing The PICs
    outb!(MASTER_COMM, ICW1_ICW4 | ICW1_INIT);
    outb!(SLAVE_COMM,  ICW1_ICW4 | ICW1_INIT);

    // Set The Offsets
    outb!(MASTER_DATA, master);
    outb!(SLAVE_DATA,  slave);

    // Tell The Master PIC where the Slave PIC's IRQ is.
    outb!(MASTER_DATA, SLAVE_IRQ);
    outb!(SLAVE_DATA,  SLAVE_CASCADE);

    // Use 8086 Mode.
    outb!(MASTER_DATA, ICW4_8086);
    outb!(SLAVE_DATA,  ICW4_8086);

    // Restore Masks.
    outb!(MASTER_DATA, master_mask);
    outb!(SLAVE_DATA,  slave_mask);

    {
        *PIC1.lock() = master;
        *PIC2.lock() = slave;
    }
}

fn get_ocw3(ocw3: u8) -> u16 {
    outb!(MASTER_COMM, ocw3);
    outb!(SLAVE_COMM,  ocw3);

    let slave_value = (inb!(SLAVE_COMM) as u16) << 8;
    let master_value = inb!(MASTER_COMM) as u16; 

    return (slave_value) | (master_value);
}

pub fn isr() -> u16 {
    self::get_ocw3(OCW3_ISR)
}

pub fn irr() -> u16 {
    self::get_ocw3(OCW3_IRR)
}