//! TODO(BSFishy): document this

use core::fmt;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub enum PrepareError {
    /// TODO(BSFishy): document this
    SetLogger(log::SetLoggerError),
}

impl fmt::Display for PrepareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrepareError::SetLogger(e) => write!(f, "{}", e),
        }
    }
}

impl From<log::SetLoggerError> for PrepareError {
    fn from(other: log::SetLoggerError) -> Self {
        PrepareError::SetLogger(other)
    }
}
