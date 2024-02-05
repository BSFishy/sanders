//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use bootloader::BootInfo;

pub mod error;

pub use error::InitError;

/// TODO(BSFishy): document this
pub fn init(_boot_info: &'static BootInfo) -> Result<(), error::InitError> {
    log::trace!("Initialized multitasking");

    Ok(())
}
