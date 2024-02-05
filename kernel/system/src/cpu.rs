//! TODO(BSFishy): document this

/// TODO(BSFishy): document this
pub trait Cpu {
    /// TODO(BSFishy): document this
    type Core: Core;

    /// TODO(BSFishy): document this
    fn core(&self) -> Self::Core;

    /// TODO(BSFishy): document this
    fn id(&self) -> usize {
        self.core().id()
    }
}

/// TODO(BSFishy): document this
pub trait Core {
    /// TODO(BSFishy): document this
    fn id(&self) -> usize;
}
