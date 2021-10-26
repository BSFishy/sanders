//! TODO(BSFishy): document this

pub mod cpu;

pub use cpu::CPU;

/// TODO(BSFishy): document this
pub trait System {
    /// TODO(BSFishy): document this
    type CPU: CPU;

    /// TODO(BSFishy): document this
    fn cpu(&self) -> Self::CPU;
}
