//! TODO(BSFishy): document this

use core::fmt;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub enum InitError {
    /// TODO(BSFishy): document this
    Logger(logging::PrepareError),
    /// TODO(BSFishy): document this
    Ipc(ipc::InitError),
    /// TODO(BSFishy): document this
    Memory(memory::InitError),
    /// TODO(BSFishy): document this
    Multitasking(multitasking::InitError),
    /// TODO(BSFishy): document this
    SystemPrepare(&'static str),
    /// TODO(BSFishy): document this
    SystemInit(&'static str),
}

impl InitError {
    /// TODO(BSFishy): document this
    pub fn from_system_prepare(other: &'static str) -> Self {
        InitError::SystemPrepare(other)
    }

    /// TODO(BSFishy): document this
    pub fn from_system_init(other: &'static str) -> Self {
        InitError::SystemInit(other)
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitError::Logger(e) => write!(f, "{}", e),
            InitError::Ipc(_) => write!(f, ""),
            InitError::Memory(_) => write!(f, ""),
            InitError::Multitasking(_) => write!(f, ""),
            InitError::SystemPrepare(e) => write!(f, "error in system preparation: {}", e),
            InitError::SystemInit(e) => write!(f, "error in system initialization: {}", e),
        }
    }
}

impl From<logging::PrepareError> for InitError {
    fn from(other: logging::PrepareError) -> Self {
        InitError::Logger(other)
    }
}

impl From<ipc::InitError> for InitError {
    fn from(other: ipc::InitError) -> Self {
        InitError::Ipc(other)
    }
}

impl From<memory::InitError> for InitError {
    fn from(other: memory::InitError) -> Self {
        InitError::Memory(other)
    }
}

impl From<multitasking::InitError> for InitError {
    fn from(other: multitasking::InitError) -> Self {
        InitError::Multitasking(other)
    }
}
