//! # s&ers serial
//!
//! s&ers serial is a crate that provides easy-to-use macros that interface with serial port 1.
//! It is used primarily to relay testing information to the terminal.
//! It is necessary to print the information to the terminal, because the QEMU instances are started in headless mode, and numerous instances are started.

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    /// Access to the first serial port.
    ///
    /// This is direct hardware access to serial port 1.
    /// This is used by s&ers to interface with the terminal used to start the virtual machine.
    /// What this allows us to do is give feedback outside of the virtual machine, i.e. about test results.
    /// To use this, you could directly access the serial port, provided by the `uart_16550` crate, or you could use one of the `serial_print` or `serial_println` macros.
    /// These mimic the macros provided by the standard library, but print to the serial port.
    ///
    /// **Printing hello world:**
    /// ```no_run
    /// fn run_test<T: TestCase>(test: T) {
    ///     serial_print!("{}...\t", core::any::type_name::<T>());
    ///     test();
    ///     serial_println!("[ok]");
    /// }
    /// ```
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use x86_64::instructions::interrupts;

            interrupts::without_interrupts(|| {
                SERIAL1
                    .lock()
                    .write_fmt(args)
                    .expect("Printing to serial failed");
            });
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}

/// Print to serial port 1.
///
/// This will print a string to serial port 1.
/// s&ers uses this to relay testing information to the terminal when running in a virtual machine.
/// It mimics the similarly named `print` macro in the standard library.
/// It uses [`crate::SERIAL1`] to interface with the serial port.
///
/// ```
/// # #[macro_use] extern crate sanders_serial;
/// # fn main() {
/// serial_print!("Hello world!");
/// # }
/// ```
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

/// Print a line to serial port 1.
///
/// This will print a string, appending a new line to the end, to serial port 1.
/// s&ers uses this to relay testing information to the terminal when running in a virtual machine.
/// It mimics the similarly named `println` macro in the standard library.
/// It uses [`crate::SERIAL1`] to interface with the serial port.
///
/// ```rust
/// # #[macro_use] extern crate sanders_serial;
/// # fn main() {
/// serial_println!("Hello world!");
/// # }
/// ```
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
