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

// extern crate alloc;

use core::alloc::GlobalAlloc;
use core::ptr::slice_from_raw_parts;
use bootloader::BootInfo;

pub mod logging;

pub mod arch;
pub mod ipc;
pub mod memory;
pub mod multitasking;
pub mod system;

#[cfg(test)]
mod testing;

#[doc(inline)]
pub use arch::get_system;

#[doc(inline)]
pub use system::System;

/// TODO(BSFishy): document this
pub fn init(boot_info: &'static BootInfo) {
    logging::prepare_logger();

    // Prepare the system before the individual modules are initialized.
    // This allows for architecture-specific initialization to occur that is
    // necessary for the individual modules to be initialized. For example,
    // certain registers or tables might need to be set up before initialization
    // can occur.
    let sys = get_system();
    sys.prepare();

    // Initialize all of the individual modules with any necessary information
    // they might need.
    memory::init(boot_info);
    multitasking::init(boot_info);
    ipc::init(boot_info);

    // TODO: initialize drivers somewhere, probably around here

    // Allow the architecture-specific system to initialize itself. This allows
    // for any architecture-specific preparations or initializations to be made.
    // For example, certain registers or tables might need to be set up for a
    // proper running environment.
    sys.init(boot_info);

    log::trace!("Initialized");
}

/// TODO(BSFishy): document this
pub fn run() -> ! {
    // {
    //     use x86_64::registers::control::Cr3;
    //
    //     let (frame1, flags) = Cr3::read();
    //     let (frame2, pcid) = Cr3::read_pcid();
    //     log::info!("Running!");
    //     log::info!("read: {:?}, {:?}", frame1, flags);
    //     log::info!("read_pcid: {:?}, {:?}", frame2, pcid);
    // }

    // {
    //     use x86::apic::ApicControl;
    //     use crate::arch::x86_64::Apic;
    //
    //     let apic = Apic::get();
    //     if let Some(apic) = apic {
    //         log::info!("Got xapic!");
    //         log::info!("Id: {}", apic.id());
    //         log::info!("Logical id: {}", apic.logical_id());
    //         log::info!("Version: {}", apic.version());
    //         log::info!("BSP: {}", apic.bsp());
    //     } else {
    //         log::warn!("No xapic found!");
    //     }
    // }

    // {
    //     use core::alloc::Layout;
    //     use crate::memory::ALLOCATOR;
    //
    //     #[derive(Debug)]
    //     struct Example {
    //         a: i32
    //     }
    //
    //     let layout = Layout::new::<Example>();
    //     let address = unsafe { ALLOCATOR.alloc(layout) };
    //     log::info!("Allocated: {:p}", address);
    //     // let type_address = address as *mut Example;
    //     // let heap = unsafe { &mut *type_address };
    //     // heap.a = 10;
    //     //
    //     // log::info!("{:?}", heap);
    //
    //     unsafe { ALLOCATOR.dealloc(address, layout) };
    //
    //     log::info!("Deallocated");
    // }

    // {
    //     use x86_64::instructions::segmentation::Segment;
    //     use x86_64::registers::segmentation::DS;
    //     use x86_64::instructions::tables::sgdt;
    //     use x86_64::structures::gdt::{GlobalDescriptorTable, DescriptorFlags};
    //     use core::{ptr::slice_from_raw_parts_mut, mem::size_of};
    //     use bit_field::BitField;
    //
    //     let ds = DS::get_reg();
    //     log::info!("DS: {}, {:?}", ds.index(), ds.rpl());
    //
    //     let gdt_descriptor = sgdt();
    //     log::info!("GDT: {:?}", gdt_descriptor);
    //     let next_free = (gdt_descriptor.limit + 1) as usize / size_of::<u64>();
    //     log::info!("GDT next_free: {}", next_free);
    //     let limit = (3 * size_of::<u64>() - 1) as u16;
    //     log::info!("GDT limit: {}", limit);
    //
    //     let gdt_slice = slice_from_raw_parts(gdt_descriptor.base.as_ptr::<u64>(), next_free);
    //     let gdt_slice = unsafe { gdt_slice.as_ref() }.unwrap();
    //
    //     let mut gdt_iter = gdt_slice.iter();
    //     loop {
    //         match gdt_iter.next() {
    //             Some(value) => {
    //                 if value.get_bits(40..44) == 0b1001 {
    //                     log::info!("\tSystem segment");
    //                 } else {
    //                     let flags = DescriptorFlags::from_bits(*value);
    //                     log::info!("\tFlags: 0x{:X} {:?}", value, flags);
    //                 }
    //             },
    //             None => break,
    //         }
    //     }
    //
    //     // let gdt = unsafe { GlobalDescriptorTable::from_raw_slice(gdt_slice) };
    //
    // }

    log::info!("Started!");

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
// pub mod multitasking;
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
