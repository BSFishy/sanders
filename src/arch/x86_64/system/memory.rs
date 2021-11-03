//! TODO(BSFishy): document this

use bootloader::BootInfo;
use core::alloc::Layout;
use crate::system::Memory;

/// TODO(BSFishy): document this
pub struct MemoryX86_64;

impl Memory for MemoryX86_64 {
    fn init(&self, boot_info: &'static BootInfo) {
        crate::arch::x86_64::memory::init(boot_info);

        // boot_info.memory_map.get(1).unwrap().
        log::trace!("System memory initialized");
    }

    #[inline]
    fn start_size(&self) -> (usize, usize) {
        crate::arch::x86_64::memory::start_size()
    }

    fn map(&mut self, physical_address: *mut u8, virtual_address: *mut u8) {
        todo!()
    }

    fn ensure_mapped(&mut self, pointer: *mut u8, layout: &Layout) {
        // TODO: check if the pointer is already mapped

        crate::arch::x86_64::memory::map::<x86_64::structures::paging::Size4KiB>(pointer, layout.size()).expect("Unable to map pointer");
    }
}
