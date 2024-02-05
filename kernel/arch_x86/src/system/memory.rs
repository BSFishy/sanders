//! TODO(BSFishy): document this

use system::{BootInfo, Memory};

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct MemoryX86;

impl Memory for MemoryX86 {
    fn init(&self, _boot_info: &'static BootInfo) {
        todo!()
    }
}
