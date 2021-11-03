//! TODO(BSFishy): document this

use crate::system::{Cpu, Core};

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct CpuX86_64;

impl Cpu for CpuX86_64 {
    type Core = CoreX86_64;

    fn core(&self) -> Self::Core {
        CoreX86_64
    }
}

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct CoreX86_64;

impl Core for CoreX86_64 {
    fn id(&self) -> usize {
        todo!()
    }
}
