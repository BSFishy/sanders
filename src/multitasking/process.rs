//! TODO(BSFishy): document this

// use alloc::collections::BTreeMap;
use crate::multitasking::{Thread, Tid};

/// TODO(BSFishy): document this
pub type Pid = usize;

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct Process {
    id: Pid,
    // TODO: put this in a mutex
    // threads: BTreeMap<Tid, Thread>,
}
