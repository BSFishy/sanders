//! TODO: document this

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
// #![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use bootloader::BootInfo;

// #[cfg(test)]
// use {
//     bootloader::entry_point
// };

#[cfg(test)]
mod testing;

/// TODO(BSFishy): document this
pub fn init(_info: &'static BootInfo) {

}

/// TODO(BSFishy): document this
pub fn run() -> ! {
    loop {}
}

// extern crate alloc;
//
// use bootloader::BootInfo;
// use core::panic::PanicInfo;
//
// #[cfg(test)]
// use {
//     bootloader::entry_point,
//     sanders_testing::{exit_qemu, QemuExitCode},
// };
//
// #[cfg(test)]
// entry_point!(test_kernel_main);
//
// pub mod interrupts;
//
// pub mod ipc;
// pub mod memory;
// pub mod process;
//
// /// TODO(BSFishy): document this
// pub fn init(boot_info: &'static BootInfo) {
//     memory::pre_init();
//     interrupts::init();
//     interrupts::enable();
//
//     memory::init(boot_info);
// }
//
// /// TODO(BSFishy): document this
// #[allow(unused_variables)]
// pub fn handle_panic(info: &PanicInfo) -> ! {
//     log::error!("{}", info);
//
//     interrupts::hlt_loop();
// }
//
// /// TODO(BSFishy): document this
// #[allow(unused_variables)]
// pub fn handle_test_panic(info: &PanicInfo) -> ! {
//     log::error!(target: "serial", "failed");
//     log::error!(target: "serial", "Error: {}", info);
//
//     cfg_if::cfg_if! {
//         if #[cfg(test)] {
//             exit_qemu(QemuExitCode::Failed);
//         } else {
//             loop {}
//         }
//     }
// }
//
// #[cfg(test)]
// fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
//     init(boot_info);
//
//     //noinspection RsUnresolvedReference
//     test_main();
//
//     interrupts::hlt_loop();
// }
//
// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     handle_test_panic(info);
// }
