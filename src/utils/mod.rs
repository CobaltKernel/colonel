

#[macro_export]
macro_rules! inb {
    ($port:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u8> = Port::new($port);
            port.read()
        }
    };
}

#[macro_export]
macro_rules! inw {
    ($port:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u16> = Port::new($port);
            port.read()
        }
    };
}

#[macro_export]
macro_rules! indw {
    ($port:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u32> = Port::new($port);
            port.read()
        }
    };
}

#[macro_export]
macro_rules! kmread_mut {
    ($addr:expr, $t:ty) => {
        &mut *($addr as $t)
    };
}

#[macro_export]
macro_rules! printk {
    ($($args:tt)*) => {
        $crate::sys::tty::print(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! eprintk {
    ($($args:tt)*) => {
        $crate::sys::tty::eprint(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[macro_export]
macro_rules! entrypoint {
    ($entrypoint:path) => {
        use bootloader::{BootInfo};
        use x86_64::instructions::interrupts::*;
        use colonel::version;

        bootloader::entry_point!(kernel_main);

        pub fn kernel_main(info: &'static BootInfo) -> ! {
            let func: fn() = $entrypoint;
            printk!("Colonel V{}", version!());
            func();

            disable();
            loop {x86_64::instructions::hlt();}
        }
    };
}

pub type KernelResult<T> = core::result::Result<T, &'static str>;

