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
        x86_64::instructions::hlt();
    }
}
