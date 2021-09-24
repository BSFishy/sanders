#![no_std]
#![no_main]
#![allow(dead_code)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sanders::{handle_panic, init, interrupts};

mod logging;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    logging::prepare_logger();

    init(boot_info);

    log::info!("Started!"); // TODO: do something other than this

    interrupts::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_panic(info);
}
