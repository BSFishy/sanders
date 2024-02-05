//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

pub mod cpu;
pub mod memory;

pub use bootloader::BootInfo;
pub use cpu::{Cpu, Core};
pub use memory::Memory;

/// TODO(BSFishy): document this
pub trait System {
    /// TODO(BSFishy): document this
    type Cpu: cpu::Cpu;

    /// TODO(BSFishy): document this
    type Memory: memory::Memory;

    /// TODO(BSFishy): document this
    fn prepare(&self) -> Result<(), &'static str>;

    /// TODO(BSFishy): document this
    fn init(&self, boot_info: &'static BootInfo) -> Result<(), &'static str>;

    /// TODO(BSFishy): document this
    fn cpu(&self) -> Self::Cpu;

    /// TODO(BSFishy): document this
    fn memory(&self) -> Self::Memory;

    /// TODO(BSFishy): document this
    fn pause(&self);

    /// TODO(BSFishy): document this
    fn interrupts_enabled(&self) -> bool;

    /// TODO(BSFishy): document this
    fn enable_interrupts(&self);

    /// TODO(BSFishy): document this
    fn disable_interrupts(&self);

    /// TODO(BSFishy): document this
    fn pause_interrupts(&self) -> bool {
        let enabled = self.interrupts_enabled();
        self.disable_interrupts();
        enabled
    }

    /// TODO(BSFishy): document this
    fn restore_interrupts(&self, enabled: bool) {
        if enabled {
            self.enable_interrupts();
        }
    }
}

