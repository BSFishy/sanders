//! TODO(BSFishy): document this

pub mod idt;
pub mod pic;

/// TODO(BSFishy): document this
pub fn init() {
    idt::init_idt();
}

/// TODO(BSFishy): document this
pub fn hlt_loop() -> ! {
    loop {
        // TODO: make this support multiple architectures
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                x86_64::instructions::hlt();
            } else {
                compile_error!("Unsupported architecture");
            }
        }
    }
}
