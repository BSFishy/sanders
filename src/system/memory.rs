//! TODO(BSFishy): document this

use bootloader::BootInfo;
use core::alloc::Layout;

/// TODO(BSFishy): document this
pub trait Memory {
    /// TODO(BSFishy): document this
    fn init(&self, boot_info: &'static BootInfo);

    /// TODO(BSFishy): document this
    fn start_size(&self) -> (usize, usize);

    /// TODO(BSFishy): document this
    fn map(&mut self, physical_address: *mut u8, virtual_address: *mut u8);

    /// TODO(BSFishy): document this
    fn ensure_mapped(&mut self, pointer: *mut u8, layout: &Layout);
}
