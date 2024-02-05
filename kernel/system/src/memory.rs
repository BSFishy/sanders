//! TODO(BSFishy): document this

use bootloader::BootInfo;

/// TODO(BSFishy): document this
pub trait Memory {
    /// TODO(BSFishy): document this
    fn init(&self, boot_info: &'static BootInfo);
}
