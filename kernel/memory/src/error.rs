//! TODO(BSFishy): document this

use core::fmt;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct InitError {

}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, ""),
        }
    }
}
