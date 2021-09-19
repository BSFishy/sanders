#![no_std]
#![no_main]
#![allow(dead_code)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sanders::{handle_panic, init, interrupts};

#[cfg(debug_assertions)]
use sanders_vga_buffer::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    #[cfg(debug_assertions)]
    println!("Started!"); // TODO: do something other than this

    interrupts::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_panic(info);
}
