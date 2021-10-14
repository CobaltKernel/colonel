use bootloader::BootInfo;

use crate::utils::KernelResult;
use crate::init;

pub fn boot(_info: &'static BootInfo) {
    init!(crate::arch::i386::interrupts::init);
}

#[macro_export]
macro_rules! init {
    ($func:path) => {
        $crate::log!("Initing {}...", stringify!($func));
        let f: fn() -> KernelResult<()> = $func;
        match f() {
            Ok(_) => $crate::printk!("[OK]\r\n"),
            Err(msg) => $crate::eprintk!("[FAILED] ({})\r\n", msg),
        }
    };

    // TODO(George): Add Arbitary Arguments To The Init Function.
    // ($func:path, $($args:tt)*) => {
    //     $crate::log!("Initing {}...", stringify!($func));

    //     match $func( $(args)* ) {
    //         Ok(_) => $crate::printk!("[OK]\r\n");
    //         Err(msg) => $crate::eprintk!("[FAILED] ({})\r\n", msg);
    //     }
    // };
}

fn _test_init() -> KernelResult<()> {
    Err("No, Just, No (ERR_UNSUPPORTED)")
}


