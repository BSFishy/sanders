#![no_std]
#![no_main]

use arch::get_system;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sanders::{init, run};
use system::System;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    if let Err(error) = init(boot_info) {
        panic!("Initialization error: {}", error);
    }

    run();
}

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    // TODO: halt loop or something similar here (maybe let drivers display an error message like a blue screen?)
    let sys = get_system();
    loop {
        sys.pause();
    }
}
