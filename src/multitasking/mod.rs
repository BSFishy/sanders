//! TODO(BSFishy): document this

use bootloader::BootInfo;

pub mod process;
pub mod scheduler;
pub mod thread;

#[doc(inline)]
pub use process::{Process, Pid};

#[doc(inline)]
pub use scheduler::Scheduler;

#[doc(inline)]
pub use thread::{Thread, Tid};

/// TODO(BSFishy): document this
pub fn init(_boot_info: &'static BootInfo) {
    log::trace!("Multitasking initialized");
}
