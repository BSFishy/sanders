//! TODO(BSFishy): document this

/// TODO(BSFishy): document this
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ProcessStatus {
    /// TODO(BSFishy): document this
    WaitingForStart = 0,
    /// TODO(BSFishy): document this
    Waiting,
    /// TODO(BSFishy): document this
    Running,
}

/// TODO(BSFishy): document this
pub struct Process {
    status: ProcessStatus,
    ebp: u32,
    esp: u32,
    pointer: fn(),
}
