//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use log::{Metadata, Record};

#[cfg(debug_assertions)]
use {
    serial::{serial_print, serial_println},
    vga_buffer::{println, set_print_foreground, Color as PrintColor},
};

pub mod error;

pub use error::PrepareError;

static LOGGER: KernelLogger = KernelLogger;

/// TODO(BSFishy): document this
pub fn prepare_logger() -> Result<(), PrepareError> {
    log::set_logger(&LOGGER)?;

    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            log::set_max_level(log::LevelFilter::Trace);
        } else {
            log::set_max_level(log::LevelFilter::Info);
        }
    }

    Ok(())
}

struct KernelLogger;

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO: enable this if there is at least one driver that supports logs
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                true
            } else {
                false
            }
        }
    }

    fn log(&self, record: &Record) {
        // TODO: log to drivers that support it
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                if !self.enabled(record.metadata()) {
                    return
                }

                match record.target() {
                    "serial" => {
                        match record.level() {
                            log::Level::Info => {
                                serial_println!("{}", record.args());
                            },
                            lvl => {
                                serial_println!("{}: {}", lvl, record.args())
                            }
                        }
                    },
                    "serial_print" => {
                        serial_print!("{}", record.args());
                    },
                    _ => {
                        match record.level() {
                            log::Level::Trace | log::Level::Debug => {
                                set_print_foreground!(PrintColor::LightGray);

                                println!("{}: {}", record.level(), record.args());
                            }
                            log::Level::Info => {
                                set_print_foreground!(PrintColor::White);

                                println!("{}", record.args());
                            }
                            log::Level::Warn => {
                                set_print_foreground!(PrintColor::Yellow);

                                println!("{}: {}", record.level(), record.args());
                            }
                            log::Level::Error => {
                                set_print_foreground!(PrintColor::Red);

                                println!("{}: {}", record.level(), record.args());
                            }
                        }
                    }
                }
            } else {
                panic!("No logging in release builds!");
            }
        }
    }

    fn flush(&self) {}
}
