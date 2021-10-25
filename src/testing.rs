//! TODO(BSFishy): document this

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use super::{init, test_main};

entry_point!(test_kernel_main);

fn test_kernel_main(info: &'static BootInfo) -> ! {
    init(info);

    test_main();

    loop {}
}

#[panic_handler]
fn handle_test_panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    // serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    // exit_qemu(QemuExitCode::Success);
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
        // serial_print!("test {} ... ", core::any::type_name::<T>());
        self();
        // serial_println!("ok");
    }
}
