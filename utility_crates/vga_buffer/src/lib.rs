//! TODO(BSFishy): document this

#![no_std]
#![no_main]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![feature(custom_test_frameworks)]
#![test_runner(sanders_testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[cfg(test)]
use {
    core::panic::PanicInfo,
    sanders_serial::serial_println,
    sanders_testing::{exit_qemu, QemuExitCode},
};

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);

    exit_qemu(QemuExitCode::Failed);

    loop {}
}

/// TODO(BSFishy): document this
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// TODO(BSFishy): document this
    Black = 0,
    /// TODO(BSFishy): document this
    Blue = 1,
    /// TODO(BSFishy): document this
    Green = 2,
    /// TODO(BSFishy): document this
    Cyan = 3,
    /// TODO(BSFishy): document this
    Red = 4,
    /// TODO(BSFishy): document this
    Magenta = 5,
    /// TODO(BSFishy): document this
    Brown = 6,
    /// TODO(BSFishy): document this
    LightGray = 7,
    /// TODO(BSFishy): document this
    DarkGray = 8,
    /// TODO(BSFishy): document this
    LightBlue = 9,
    /// TODO(BSFishy): document this
    LightGreen = 10,
    /// TODO(BSFishy): document this
    LightCyan = 11,
    /// TODO(BSFishy): document this
    LightRed = 12,
    /// TODO(BSFishy): document this
    Pink = 13,
    /// TODO(BSFishy): document this
    Yellow = 14,
    /// TODO(BSFishy): document this
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn set_foreground(self, foreground: Color) -> Self {
        ColorCode((self.0 & 0b1111_0000) | (foreground as u8))
    }

    fn set_background(self, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (self.0 & 0b0000_1111))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    /// TODO(BSFishy): document this
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// TODO(BSFishy): document this
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// TODO(BSFishy): document this
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// TODO(BSFishy): document this
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// TODO(BSFishy): document this
    pub fn set_foreground(&mut self, foreground: Color) {
        self.color_code = self.color_code.set_foreground(foreground);
    }

    /// TODO(BSFishy): document this
    pub fn set_background(&mut self, background: Color) {
        self.color_code = self.color_code.set_background(background);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// TODO: better implementation of print
/// TODO(BSFishy): document this
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
    }
}

// TODO: better implementation of print
/// TODO(BSFishy): document this
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", core::format_args!($($arg)*)));
}

/// TODO(BSFishy): document this
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        $crate::WRITER.lock().set_foreground($crate::Color::Red);

        $crate::print!($($arg)*);
    }
}

/// TODO(BSFishy): document this
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", core::format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_println_simple() {
        println!("test_println_simple output");
    }

    #[test_case]
    fn test_println_many() {
        for _ in 0..200 {
            println!("test_println_many output");
        }
    }

    #[test_case]
    fn test_println_output() {
        use core::fmt::Write;
        use x86_64::instructions::interrupts;

        let s = "Some test string that fits on a single line";
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writeln!(writer, "\n{}", s).expect("writeln failed");
            for (i, c) in s.chars().enumerate() {
                let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
                assert_eq!(char::from(screen_char.ascii_character), c);
            }
        });
    }
}
