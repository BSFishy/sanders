//! TODO(BSFishy): document this

pub mod idt;
pub mod pic;

/// TODO(BSFishy): document this
pub fn init() {
    idt::init_idt();
    unsafe { pic::PICS.lock().initialize() };
}

/// TODO(BSFishy): document this
pub fn enable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::instructions::interrupts::enable();
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}

/// TODO(BSFishy): document this
pub fn disable() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::instructions::interrupts::disable();
        } else {
            compile_error!("Unsupported architecture");
        }
    }
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
