//! TODO(BSFishy): document this

use bootloader::BootInfo;

use crate::get_system;
use crate::system::{System, Memory};

pub mod allocation;

use allocation::Allocator;

#[global_allocator]
pub(crate) static ALLOCATOR: Allocator = Allocator::new();

/// TODO(BSFishy): document this
pub fn init(boot_info: &'static BootInfo) {
    let system = get_system();
    system.memory().init(boot_info);

    // let (start, size) = system.memory().start_size();
    // ALLOCATOR.init(start, size);

    log::trace!("Memory initialized");
}
