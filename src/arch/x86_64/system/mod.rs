//! TODO(BSFishy): document this

use bootloader::BootInfo;
use crate::system::System;

pub mod cpu;
pub mod memory;

#[doc(inline)]
pub use cpu::CpuX86_64;

#[doc(inline)]
pub use memory::MemoryX86_64;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct SystemX86_64;

impl System for SystemX86_64 {
    type Cpu = CpuX86_64;

    type Memory = MemoryX86_64;

    fn prepare(&self) {
        log::trace!("System prepared");
    }

    fn init(&self, _boot_info: &'static BootInfo) {
        log::trace!("System initialized");
    }

    fn cpu(&self) -> Self::Cpu {
        CpuX86_64
    }

    fn memory(&self) -> Self::Memory {
        MemoryX86_64
    }
}
