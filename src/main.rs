#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sanders::{init, run};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    run();
}

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    loop {}
}

// #![allow(dead_code)]

// use bootloader::{entry_point, BootInfo};
// use core::panic::PanicInfo;
// use sanders::{handle_panic, init, interrupts};
//
// mod logging;
//
// entry_point!(kernel_main);
//
// fn kernel_main(boot_info: &'static BootInfo) -> ! {
//     logging::prepare_logger();
//
//     init(boot_info);
//
//     log::info!("Started!"); // TODO: do something other than this
//
//     interrupts::hlt_loop();
// }
//
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     handle_panic(info);
// }
