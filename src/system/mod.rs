//! TODO(BSFishy): document this

use bootloader::BootInfo;

pub mod cpu;
pub mod memory;

pub use cpu::{Cpu, Core};
pub use memory::Memory;

#[doc(inline)]
pub use crate::arch::get_system;

/// TODO(BSFishy): document this
pub trait System {
    /// TODO(BSFishy): document this
    type Cpu: cpu::Cpu;

    /// TODO(BSFishy): document this
    type Memory: memory::Memory;

    /// TODO(BSFishy): document this
    fn prepare(&self);

    /// TODO(BSFishy): document this
    fn init(&self, boot_info: &'static BootInfo);

    /// TODO(BSFishy): document this
    fn cpu(&self) -> Self::Cpu;

    /// TODO(BSFishy): document this
    fn memory(&self) -> Self::Memory;
}
