//! TODO(BSFishy): document this
// TODO: convert this into a module that allows for other architectures

pub mod pic;
pub mod idt;

/// TODO(BSFishy): document this
pub fn init() {
    idt::init_idt();
}

/// TODO(BSFishy): document this
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
