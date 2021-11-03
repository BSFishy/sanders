//! TODO(BSFishy): document this

use bootloader::BootInfo;

/// TODO(BSFishy): document this
pub fn init(_boot_info: &'static BootInfo) {
    log::trace!("IPC initialized");
}
