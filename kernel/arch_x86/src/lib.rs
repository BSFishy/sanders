//! TODO(BSFishy): document this

#![no_std]
#![cfg(any(doc, target_arch = "x86", target_arch = "x86_64"))]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

pub mod system;

pub use crate::system::SystemX86;
