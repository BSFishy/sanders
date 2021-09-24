use log::{Metadata, Record};

#[cfg(debug_assertions)]
use sanders_vga_buffer::{println, set_print_foreground, Color as PrintColor};

static LOGGER: KernelLogger = KernelLogger;

pub fn prepare_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::instructions::interrupts::without_interrupts(|| {
                log::set_logger(&LOGGER).expect("Unable to set logger");
            });
        } else {
            compile_error!("Unsupported architecture");
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            log::set_max_level(log::LevelFilter::Trace);
        } else {
            log::set_max_level(log::LevelFilter::Off);
        }
    }
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
            } else {
                panic!("No logging in release builds!");
            }
        }
    }

    fn flush(&self) {}
}
