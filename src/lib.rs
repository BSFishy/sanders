//! TODO: document this

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![test_runner(sanders_testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

extern crate alloc;

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[cfg(debug_assertions)]
use {
    sanders_serial::serial_println,
    sanders_testing::{exit_qemu, QemuExitCode},
    sanders_vga_buffer::eprintln,
};

#[cfg(test)]
use bootloader::entry_point;

#[cfg(test)]
entry_point!(test_kernel_main);

pub mod gdt;
pub mod interrupts;

pub mod ipc;
pub mod memory;
pub mod process;

/// TODO(BSFishy): document this
pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::pic::PICS.lock().initialize() };

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::instructions::interrupts::enable();
        } else {
            compile_error!("Unsupported architecture");
        }
    }

    memory::init(boot_info);
}

/// TODO(BSFishy): document this
#[allow(unused_variables)]
pub fn handle_panic(info: &PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    eprintln!("{}", info); // TODO: handle this in a way that doesn't do this

    interrupts::hlt_loop();
}

/// TODO(BSFishy): document this
#[allow(unused_variables)]
pub fn handle_test_panic(info: &PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    serial_println!("failed");
    #[cfg(debug_assertions)]
    serial_println!("Error: {}", info);

    #[cfg(debug_assertions)]
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    //noinspection RsUnresolvedReference
    test_main();

    interrupts::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_test_panic(info);
}
