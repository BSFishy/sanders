//! TODO(BSFishy): document this

use super::CPU;

pub mod pic;

/// TODO(BSFishy): document this
#[allow(non_camel_case_types)]
pub struct x86_64CPU;

impl CPU for x86_64CPU {
    fn id(&self) -> usize {
        pic::PIC.lock().id().unwrap_or(0)
    }
}
