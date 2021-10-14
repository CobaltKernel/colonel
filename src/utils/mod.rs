

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
macro_rules! outb {
    ($port:expr, $value:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u8> = Port::new($port);
            port.write($value);
        }
    };
}

#[macro_export]
macro_rules! outw {
    ($port:expr, $value:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u16> = Port::new($port);
            port.write($value);
        }
    };
}

#[macro_export]
macro_rules! outdw {
    ($port:expr, $value:expr) => {
        unsafe { 
            use x86_64::instructions::port::*;
            let mut port: Port<u8> = Port::new($port);
            port.write($value);
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
        $crate::sys::tty::print(format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! eprintk {
    ($($args:tt)*) => {
        $crate::sys::tty::eprint(format_args!($($args)*))
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
            printk!("Colonel V{}\r\n", version!());
            $crate::boot(info);
            #[cfg(test)]
                test_main();

            func();

            disable();
            loop {x86_64::instructions::hlt();}
        }
    };
}

#[macro_export]
macro_rules! log {
    ($fmt:expr) => {
        $crate::printk!(concat!("{}[LOG]:{} ",$fmt), $crate::sys::csi::FG_GREEN, $crate::sys::csi::RESET);
    };

    ($fmt:expr, $($args:tt)*) => {
        $crate::printk!(concat!("{}[LOG]:{} ",$fmt), $crate::sys::csi::FG_GREEN, $crate::sys::csi::RESET, $($args)*);
    };
}

#[macro_export]
macro_rules! size_of {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };

    ($($t:ty),*) => {
        {
            let mut sum = 0;
            $(sum += core::mem::size_of::<$t>();)*
            sum
        }
    };
}

pub type KernelResult<T> = core::result::Result<T, &'static str>;
pub type KString = &'static str;


