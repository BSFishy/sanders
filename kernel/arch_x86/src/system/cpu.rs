//! TODO(BSFishy): document this

use system::{Cpu, Core};

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct CpuX86;

impl Cpu for CpuX86 {
    type Core = CoreX86;

    fn core(&self) -> Self::Core {
        CoreX86
    }
}

/// TODO(BSFishy): document this
/// TODO: make this take in the core id
#[derive(Debug)]
pub struct CoreX86;

impl Core for CoreX86 {
    fn id(&self) -> usize {
        todo!()
    }
}
