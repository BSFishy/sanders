//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use bootloader::{BootInfo, bootinfo::{MemoryMap, MemoryRegionType}};
use frame_allocator::FrameManager;

pub mod error;

pub use error::InitError;

/// TODO(BSFishy): document this
pub static FRAME_MANAGER: FrameManager = FrameManager::new();

/// TODO(BSFishy): document this
pub fn init(boot_info: &'static BootInfo) -> Result<(), error::InitError> {
    init_frame_mapper(boot_info.physical_memory_offset, &boot_info.memory_map);

    match FRAME_MANAGER.alloc_order(0) {
        Ok(address) => {
            log::info!("Allocated: 0x{:X}", address);

            match FRAME_MANAGER.alloc_order(0) {
                Ok(address) => {
                    log::info!("Allocated: 0x{:X}", address);

                    unsafe { FRAME_MANAGER.dealloc(address, 0) };
                },
                Err(e) => {
                    log::error!("Unable to allocate: {}", e);
                }
            }

            match FRAME_MANAGER.alloc_order(0) {
                Ok(address) => {
                    log::info!("Allocated: 0x{:X}", address);

                    unsafe { FRAME_MANAGER.dealloc(address, 0) };
                },
                Err(e) => {
                    log::error!("Unable to allocate: {}", e);
                }
            }

            unsafe { FRAME_MANAGER.dealloc(address, 0) };
        },
        Err(e) => {
            log::error!("Unable to allocate: {}", e);
        }
    }

    log::trace!("Initialized memory");

    Ok(())
}

fn init_frame_mapper(physical_memory_offset: u64, memory_map: &'static MemoryMap) {
    let memory_areas = memory_map.iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable)
        .map(|region| ((physical_memory_offset + region.range.start_addr()) as usize, (region.range.end_addr() - region.range.start_addr()) as usize));

    // SAFETY: the memory areas are provided by the bootloader directly
    unsafe { FRAME_MANAGER.init(memory_areas) };
}
