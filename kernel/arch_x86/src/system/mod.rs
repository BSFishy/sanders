//! TODO(BSFishy): document this

use system::{BootInfo, System};

pub mod cpu;
pub mod memory;

pub use cpu::{CpuX86, CoreX86};
pub use memory::MemoryX86;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct SystemX86;

impl System for SystemX86 {
    type Cpu = CpuX86;
    type Memory = MemoryX86;

    fn prepare(&self) -> Result<(), &'static str> {
        log::trace!("Prepared x86 system");
        Ok(())
    }

    fn init(&self, _boot_info: &'static BootInfo) -> Result<(), &'static str> {
        log::trace!("Initialized x86 system");

        let cpuid = x86::cpuid::CpuId::new();
        if let Some(feature_info) = cpuid.get_feature_info() {
            log::debug!("Max logical processor IDs: {}", feature_info.max_logical_processor_ids());
        } else {
            log::warn!("No feature info");
        }

        if let Some(topology_info) = cpuid.get_extended_topology_info() {
            for topology in topology_info {
                log::debug!("{}[{}]: {} ({})", topology.level_type(), topology.level_number(), topology.processors(), topology.x2apic_id());
            }
        } else {
            log::warn!("No topology info");
        }

        Ok(())
    }

    fn cpu(&self) -> Self::Cpu {
        CpuX86
    }

    fn memory(&self) -> Self::Memory {
        MemoryX86
    }

    fn pause(&self) {
        x86_64::instructions::hlt();
    }

    fn interrupts_enabled(&self) -> bool {
        x86_64::instructions::interrupts::are_enabled()
    }

    fn enable_interrupts(&self) {
        x86_64::instructions::interrupts::enable();
    }

    fn disable_interrupts(&self) {
        x86_64::instructions::interrupts::disable();
    }
}
