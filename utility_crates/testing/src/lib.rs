//! TODO(BSFishy): document this

#![no_std]
#![no_main]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::panic::PanicInfo;

use sanders_serial::{serial_print, serial_println};

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("Error: {}", info);

    exit_qemu(QemuExitCode::Failed);

    loop {}
}

/// TODO(BSFishy): document this
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

/// TODO(BSFishy): document this
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// TODO(BSFishy): document this
    Success = 0x10,

    /// TODO(BSFishy): document this
    Failed = 0x11,
}

/// TODO(BSFishy): document this
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use x86_64::instructions::port::Port;

            unsafe {
                let mut port = Port::new(0xf4);
                port.write(exit_code as u32);
            }
        } else {
            compile_error!("Unsupported architecture");
        }
    }

    unreachable!("QEMU exit function did not exit");
}

/// TODO(BSFishy): document this
pub trait Testable {
    /// TODO(BSFishy): document this
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("test {} ... ", core::any::type_name::<T>());
        self();
        serial_println!("ok");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
