//! TODO(BSFishy): document this

/// TODO(BSFishy): document this
pub trait CPU {
    /// TODO(BSFishy): document this
    type Core: Core;

    /// TODO(BSFishy): document this
    fn core(&self) -> Self::Core;
}

/// TODO(BSFishy): document this
pub trait Core {
    /// TODO(BSFishy): document this
    fn id(&self) -> usize;
}
